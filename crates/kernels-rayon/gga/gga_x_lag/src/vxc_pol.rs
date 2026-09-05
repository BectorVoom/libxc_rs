//! GGA_X_LAG vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_lag.c`
//! by tools/translate_rayon/from_maple.py, then rewritten to
//! `wide::f64x8` by simd.py. Eight grid points per step; every lane runs maple2c's expression
//! sequence in its original order.
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]
use libxc_rkernel_math::constants::*;
use libxc_rkernel_math::simd;
use libxc_rkernel_math::wide::{f64x8, CmpEq, CmpGe, CmpGt, CmpLe, CmpLt, CmpNe};

const V_ZERO: f64x8 = f64x8::new([0.0; 8]);
const V_ONE: f64x8 = f64x8::new([1.0; 8]);

// Transcendentals in exact mode come from `libxc_rkernel_math::simd`,
// which is bit-identical / correctly-rounded per lane to the scalar calls
// the scalar kernel makes. In exact mode, the SIMD kernel produces output
// bit-identical to its scalar form.

/// Load 8 consecutive grid points.
///
/// The tail is padded by repeating the last element, not by zero-filling:
/// these formulas divide by rho, so a zero lane would raise inf/NaN in lanes
/// whose results are then discarded -- harmless to the answer, but it makes
/// any real NaN impossible to spot while debugging.
#[inline(always)]
fn load(s: &[f64], ip: usize, np: usize) -> f64x8 {
    if ip + 8 <= np {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        f64x8::new(b)
    } else {
        let mut b = [s[np - 1]; 8];
        b[..np - ip].copy_from_slice(&s[ip..np]);
        f64x8::new(b)
    }
}

/// Accumulate 8 consecutive grid points into an output array.
///
/// `+=`, not `=`. The scalar kernel writes `out[ip] += v`; a plain store is a
/// different operation in two ways. It keeps the sign of a negative zero where
/// `0.0 + -0.0` gives `+0.0` -- a bit difference the fingerprint gate reports
/// as a rejection even though no value changed (`gga_x_pbepow fxc` was
/// rejected on exactly this, 273 of 200,000 `v2sigma2` elements) -- and it
/// would discard whatever a caller had already put in the buffer.
#[inline(always)]
fn store_add(s: &mut [f64], ip: usize, m: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        let r: [f64; 8] = (f64x8::new(b) + acc).into();
        s[ip..ip + 8].copy_from_slice(&r);
    } else {
        for k in 0..m {
            s[ip + k] += a[k];
        }
    }
}

/// Load 8 elements with a given stride and offset.
#[inline(always)]
fn load_strided(s: &[f64], ip: usize, np: usize, stride: usize, offset: usize) -> f64x8 {
    let mut b = [0.0f64; 8];
    if ip + 8 <= np {
        let base = ip * stride + offset;
        b[0] = s[base];
        b[1] = s[base + stride];
        b[2] = s[base + 2 * stride];
        b[3] = s[base + 3 * stride];
        b[4] = s[base + 4 * stride];
        b[5] = s[base + 5 * stride];
        b[6] = s[base + 6 * stride];
        b[7] = s[base + 7 * stride];
    } else {
        for k in 0..8 {
            let p = (ip + k).min(np - 1);
            b[k] = s[p * stride + offset];
        }
    }
    f64x8::new(b)
}

/// Accumulate 8 elements with a given stride and offset.
///
/// `+=`, not `=`: the scalar kernel this was translated from writes
/// `out[ip * stride + offset] += v`, and a plain store is not the same
/// operation. It differs on the sign of zero -- `0.0 + -0.0` is `+0.0`
/// while a store of `-0.0` keeps the sign -- which is a bit difference
/// the fingerprint gate sees, and it would silently drop a caller's
/// existing contribution if one were ever there.
///
/// The read is not free on this path: a polarized `kxc`/`lxc` kernel
/// writes many strided outputs per point, and `lda_c_pw_erf kxc pol`
/// measured 84 -> 114 ns/pt (1.36x). It is charged anyway, because the
/// scalar kernel this is compared against does the same read. Gathering
/// into a vector, adding once and scattering back was tried and is no
/// faster (117 ns/pt), so the cost is the load itself, not scheduling.
#[inline(always)]
fn store_strided(s: &mut [f64], ip: usize, m: usize, stride: usize, offset: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let base = ip * stride + offset;
        s[base] += a[0];
        s[base + stride] += a[1];
        s[base + 2 * stride] += a[2];
        s[base + 3 * stride] += a[3];
        s[base + 4 * stride] += a[4];
        s[base + 5 * stride] += a[5];
        s[base + 6 * stride] += a[6];
        s[base + 7 * stride] += a[7];
    } else {
        for k in 0..m {
            s[(ip + k) * stride + offset] += a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_lag_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho0 = load_strided(rho, ip, np, 2, 0);
        let v_rho1 = load_strided(rho, ip, np, 2, 1);
        let v_sigma0 = load_strided(sigma, ip, np, 3, 0);
        let v_sigma1 = load_strided(sigma, ip, np, 3, 1);
        let v_sigma2 = load_strided(sigma, ip, np, 3, 2);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        let mut acc_vsigma_0 = V_ZERO;
        let mut acc_vsigma_1 = V_ZERO;
        let mut acc_vsigma_2 = V_ZERO;
        {
            let t1 = (v_rho0).simd_le(dens_threshold);
            let t2 = f64x8::splat(M_CBRT3);
            let t3 = v_rho0 + v_rho1;
            let t4 = f64x8::splat(1.0) / t3;
            let t7 = (f64x8::splat(2.0) * v_rho0 * t4).simd_le(zeta_threshold);
            let t8 = zeta_threshold - f64x8::splat(1.0);
            let t11 = (f64x8::splat(2.0) * v_rho1 * t4).simd_le(zeta_threshold);
            let t12 = -t8;
            let t13 = v_rho0 - v_rho1;
            let t15 = ((t7).select(t8, (t11).select(t12, t13 * t4)));
            let t16 = f64x8::splat(1.0) + t15;
            let t17 = (t16).simd_le(zeta_threshold);
            let t18 = (simd::cbrt(zeta_threshold));
            let t19 = t18 * zeta_threshold;
            let t20 = (simd::cbrt(t16));
            let t22 = ((t17).select(t19, t20 * t16));
            let t23 = t2 * t22;
            let t24 = (simd::cbrt(t3));
            let t25 = f64x8::splat(M_CBRT6);
            let t26 = t25 * t25;
            let t27 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t28 = (simd::cbrt(t27));
            let t29 = f64x8::splat(1.0) / t28;
            let t30 = t26 * t29;
            let t31 = ((v_sigma0).sqrt());
            let t32 = (simd::cbrt(v_rho0));
            let t34 = f64x8::splat(1.0) / t32 / v_rho0;
            let t36 = t30 * t31 * t34;
            let t37 = (simd::pow(t36, f64x8::splat(2.626712)));
            let t40 = f64x8::splat(1.0) + f64x8::splat(0.00013471619689594795) * t37;
            let t41 = (simd::pow(t40, -f64x8::splat(0.657946)));
            let t42 = t24 * t37 * t41;
            let t45 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(1.540002877192757e-05) * t23 * t42));
            let t46 = (v_rho1).simd_le(dens_threshold);
            let t47 = -t13;
            let t49 = ((t11).select(t8, (t7).select(t12, t47 * t4)));
            let t50 = f64x8::splat(1.0) + t49;
            let t51 = (t50).simd_le(zeta_threshold);
            let t52 = (simd::cbrt(t50));
            let t54 = ((t51).select(t19, t52 * t50));
            let t55 = t2 * t54;
            let t56 = ((v_sigma2).sqrt());
            let t57 = (simd::cbrt(v_rho1));
            let t59 = f64x8::splat(1.0) / t57 / v_rho1;
            let t61 = t30 * t56 * t59;
            let t62 = (simd::pow(t61, f64x8::splat(2.626712)));
            let t65 = f64x8::splat(1.0) + f64x8::splat(0.00013471619689594795) * t62;
            let t66 = (simd::pow(t65, -f64x8::splat(0.657946)));
            let t67 = t24 * t62 * t66;
            let t70 = ((t46).select(f64x8::splat(0.0), -f64x8::splat(1.540002877192757e-05) * t55 * t67));
            let tzk0 = t45 + t70;
            acc_zk = tzk0;
            let t71 = t3 * t3;
            let t72 = f64x8::splat(1.0) / t71;
            let t73 = t13 * t72;
            let t75 = ((t7).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), t4 - t73)));
            let t78 = ((t17).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t20 * t75));
            let t79 = t2 * t78;
            let t82 = t24 * t24;
            let t83 = f64x8::splat(1.0) / t82;
            let t85 = t83 * t37 * t41;
            let t87 = f64x8::splat(5.133342923975857e-06) * t23 * t85;
            let t88 = (simd::pow(t36, f64x8::splat(1.626712)));
            let t89 = t24 * t88;
            let t90 = t23 * t89;
            let t91 = t41 * t26;
            let t92 = t29 * t31;
            let t93 = v_rho0 * v_rho0;
            let t95 = f64x8::splat(1.0) / t32 / t93;
            let t96 = t92 * t95;
            let t97 = t91 * t96;
            let t100 = (simd::pow(t36, f64x8::splat(4.253424)));
            let t101 = t24 * t100;
            let t102 = t23 * t101;
            let t103 = (simd::pow(t40, -f64x8::splat(1.657946)));
            let t104 = t103 * t26;
            let t105 = t104 * t96;
            let t109 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(1.540002877192757e-05) * t79 * t42 - t87 + f64x8::splat(5.393525383408988e-05) * t90 * t97 - f64x8::splat(4.780604235623332e-09) * t102 * t105));
            let t110 = t47 * t72;
            let t112 = ((t11).select(f64x8::splat(0.0), (t7).select(f64x8::splat(0.0), -t4 - t110)));
            let t115 = ((t51).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t52 * t112));
            let t116 = t2 * t115;
            let t120 = t83 * t62 * t66;
            let t122 = f64x8::splat(5.133342923975857e-06) * t55 * t120;
            let t124 = ((t46).select(f64x8::splat(0.0), -f64x8::splat(1.540002877192757e-05) * t116 * t67 - t122));
            let tvrho0 = t45 + t70 + t3 * (t109 + t124);
            acc_vrho_0 = tvrho0;
            let t128 = ((t7).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), -t4 - t73)));
            let t131 = ((t17).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t20 * t128));
            let t132 = t2 * t131;
            let t136 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(1.540002877192757e-05) * t132 * t42 - t87));
            let t138 = ((t11).select(f64x8::splat(0.0), (t7).select(f64x8::splat(0.0), t4 - t110)));
            let t141 = ((t51).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t52 * t138));
            let t142 = t2 * t141;
            let t145 = (simd::pow(t61, f64x8::splat(1.626712)));
            let t146 = t24 * t145;
            let t147 = t55 * t146;
            let t148 = t66 * t26;
            let t149 = t29 * t56;
            let t150 = v_rho1 * v_rho1;
            let t152 = f64x8::splat(1.0) / t57 / t150;
            let t153 = t149 * t152;
            let t154 = t148 * t153;
            let t157 = (simd::pow(t61, f64x8::splat(4.253424)));
            let t158 = t24 * t157;
            let t159 = t55 * t158;
            let t160 = (simd::pow(t65, -f64x8::splat(1.657946)));
            let t161 = t160 * t26;
            let t162 = t161 * t153;
            let t166 = ((t46).select(f64x8::splat(0.0), -f64x8::splat(1.540002877192757e-05) * t142 * t67 - t122 + f64x8::splat(5.393525383408988e-05) * t147 * t154 - f64x8::splat(4.780604235623332e-09) * t159 * t162));
            let tvrho1 = t45 + t70 + t3 * (t136 + t166);
            acc_vrho_1 = tvrho1;
            let t169 = f64x8::splat(1.0) / t31;
            let t170 = t29 * t169;
            let t171 = t170 * t34;
            let t172 = t91 * t171;
            let t175 = t104 * t171;
            let t179 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(2.0225720187783704e-05) * t90 * t172 + f64x8::splat(1.7927265883587494e-09) * t102 * t175));
            let tvsigma0 = t3 * t179;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t180 = f64x8::splat(1.0) / t56;
            let t181 = t29 * t180;
            let t182 = t181 * t59;
            let t183 = t148 * t182;
            let t186 = t161 * t182;
            let t190 = ((t46).select(f64x8::splat(0.0), -f64x8::splat(2.0225720187783704e-05) * t147 * t183 + f64x8::splat(1.7927265883587494e-09) * t159 * t186));
            let tvsigma2 = t3 * t190;
            acc_vsigma_2 = tvsigma2;
        }
        store_add(zk, ip, m, acc_zk);
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(vsigma, ip, m, 3, 0, acc_vsigma_0);
        store_strided(vsigma, ip, m, 3, 1, acc_vsigma_1);
        store_strided(vsigma, ip, m, 3, 2, acc_vsigma_2);
        ip += 8;
    }
}

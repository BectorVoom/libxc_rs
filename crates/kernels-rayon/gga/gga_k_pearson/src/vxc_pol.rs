//! GGA_K_PEARSON vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_pearson.c`
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
pub fn gga_k_pearson_vxc_pol(
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
            let t3 = t2 * t2;
            let t4 = f64x8::splat(M_CBRTPI);
            let t6 = t3 * t4 * f64x8::splat(M_PI);
            let t7 = v_rho0 + v_rho1;
            let t8 = f64x8::splat(1.0) / t7;
            let t11 = (f64x8::splat(2.0) * v_rho0 * t8).simd_le(zeta_threshold);
            let t12 = zeta_threshold - f64x8::splat(1.0);
            let t15 = (f64x8::splat(2.0) * v_rho1 * t8).simd_le(zeta_threshold);
            let t16 = -t12;
            let t17 = v_rho0 - v_rho1;
            let t19 = ((t11).select(t12, (t15).select(t16, t17 * t8)));
            let t20 = f64x8::splat(1.0) + t19;
            let t21 = (t20).simd_le(zeta_threshold);
            let t22 = (simd::cbrt(zeta_threshold));
            let t23 = t22 * t22;
            let t24 = t23 * zeta_threshold;
            let t25 = (simd::cbrt(t20));
            let t26 = t25 * t25;
            let t28 = ((t21).select(t24, t26 * t20));
            let t29 = (simd::cbrt(t7));
            let t30 = t29 * t29;
            let t31 = t28 * t30;
            let t32 = f64x8::splat(M_CBRT6);
            let t33 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t34 = (simd::cbrt(t33));
            let t35 = t34 * t34;
            let t37 = t32 / t35;
            let t38 = v_rho0 * v_rho0;
            let t39 = (simd::cbrt(v_rho0));
            let t40 = t39 * t39;
            let t42 = f64x8::splat(1.0) / t40 / t38;
            let t44 = t33 * t33;
            let t45 = f64x8::splat(1.0) / t44;
            let t46 = v_sigma0 * v_sigma0;
            let t47 = t46 * v_sigma0;
            let t49 = t38 * t38;
            let t50 = t49 * t49;
            let t54 = f64x8::splat(1.0) + t45 * t47 / t50 / f64x8::splat(2304.0);
            let t55 = f64x8::splat(1.0) / t54;
            let t59 = f64x8::splat(1.0) + f64x8::splat(5.0) / f64x8::splat(648.0) * t37 * v_sigma0 * t42 * t55;
            let t63 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t31 * t59));
            let t64 = (v_rho1).simd_le(dens_threshold);
            let t65 = -t17;
            let t67 = ((t15).select(t12, (t11).select(t16, t65 * t8)));
            let t68 = f64x8::splat(1.0) + t67;
            let t69 = (t68).simd_le(zeta_threshold);
            let t70 = (simd::cbrt(t68));
            let t71 = t70 * t70;
            let t73 = ((t69).select(t24, t71 * t68));
            let t74 = t73 * t30;
            let t75 = v_rho1 * v_rho1;
            let t76 = (simd::cbrt(v_rho1));
            let t77 = t76 * t76;
            let t79 = f64x8::splat(1.0) / t77 / t75;
            let t81 = v_sigma2 * v_sigma2;
            let t82 = t81 * v_sigma2;
            let t84 = t75 * t75;
            let t85 = t84 * t84;
            let t89 = f64x8::splat(1.0) + t45 * t82 / t85 / f64x8::splat(2304.0);
            let t90 = f64x8::splat(1.0) / t89;
            let t94 = f64x8::splat(1.0) + f64x8::splat(5.0) / f64x8::splat(648.0) * t37 * v_sigma2 * t79 * t90;
            let t98 = ((t64).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t74 * t94));
            let tzk0 = t63 + t98;
            acc_zk = tzk0;
            let t99 = t7 * t7;
            let t100 = f64x8::splat(1.0) / t99;
            let t101 = t17 * t100;
            let t103 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), t8 - t101)));
            let t106 = ((t21).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t103));
            let t107 = t106 * t30;
            let t111 = f64x8::splat(1.0) / t29;
            let t112 = t28 * t111;
            let t115 = t6 * t112 * t59 / f64x8::splat(10.0);
            let t116 = t38 * v_rho0;
            let t118 = f64x8::splat(1.0) / t40 / t116;
            let t123 = t46 * t46;
            let t124 = t37 * t123;
            let t125 = t50 * t116;
            let t127 = f64x8::splat(1.0) / t40 / t125;
            let t128 = t54 * t54;
            let t129 = f64x8::splat(1.0) / t128;
            let t134 = -f64x8::splat(5.0) / f64x8::splat(243.0) * t37 * v_sigma0 * t118 * t55 + f64x8::splat(5.0) / f64x8::splat(186624.0) * t124 * t127 * t129 * t45;
            let t139 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t107 * t59 + t115 + f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t31 * t134));
            let t140 = t65 * t100;
            let t142 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), -t8 - t140)));
            let t145 = ((t69).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t71 * t142));
            let t146 = t145 * t30;
            let t150 = t73 * t111;
            let t153 = t6 * t150 * t94 / f64x8::splat(10.0);
            let t155 = ((t64).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t146 * t94 + t153));
            let tvrho0 = t63 + t98 + t7 * (t139 + t155);
            acc_vrho_0 = tvrho0;
            let t159 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), -t8 - t101)));
            let t162 = ((t21).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t159));
            let t163 = t162 * t30;
            let t168 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t163 * t59 + t115));
            let t170 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), t8 - t140)));
            let t173 = ((t69).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t71 * t170));
            let t174 = t173 * t30;
            let t178 = t75 * v_rho1;
            let t180 = f64x8::splat(1.0) / t77 / t178;
            let t185 = t81 * t81;
            let t186 = t37 * t185;
            let t187 = t85 * t178;
            let t189 = f64x8::splat(1.0) / t77 / t187;
            let t190 = t89 * t89;
            let t191 = f64x8::splat(1.0) / t190;
            let t196 = -f64x8::splat(5.0) / f64x8::splat(243.0) * t37 * v_sigma2 * t180 * t90 + f64x8::splat(5.0) / f64x8::splat(186624.0) * t186 * t189 * t191 * t45;
            let t201 = ((t64).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t174 * t94 + t153 + f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t74 * t196));
            let tvrho1 = t63 + t98 + t7 * (t168 + t201);
            acc_vrho_1 = tvrho1;
            let t208 = t50 * t38;
            let t210 = f64x8::splat(1.0) / t40 / t208;
            let t212 = t210 * t129 * t45;
            let t215 = f64x8::splat(5.0) / f64x8::splat(648.0) * t37 * t42 * t55 - f64x8::splat(5.0) / f64x8::splat(497664.0) * t37 * t47 * t212;
            let t219 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t31 * t215));
            let tvsigma0 = t7 * t219;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t224 = t85 * t75;
            let t226 = f64x8::splat(1.0) / t77 / t224;
            let t228 = t226 * t191 * t45;
            let t231 = f64x8::splat(5.0) / f64x8::splat(648.0) * t37 * t79 * t90 - f64x8::splat(5.0) / f64x8::splat(497664.0) * t37 * t82 * t228;
            let t235 = ((t64).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t74 * t231));
            let tvsigma2 = t7 * t235;
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

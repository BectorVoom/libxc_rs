//! GGA_K_EXP4 vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_exp4.c`
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
pub fn gga_k_exp4_vxc_pol(
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
            let t36 = f64x8::splat(1.0) / t35;
            let t37 = t32 * t36;
            let t38 = v_rho0 * v_rho0;
            let t39 = (simd::cbrt(v_rho0));
            let t40 = t39 * t39;
            let t42 = f64x8::splat(1.0) / t40 / t38;
            let t46 = (simd::exp(-f64x8::splat(8.325416666666667) * t37 * v_sigma0 * t42));
            let t48 = t32 * t32;
            let t51 = t48 / t34 / t33;
            let t52 = v_sigma0 * v_sigma0;
            let t53 = t38 * t38;
            let t54 = t53 * v_rho0;
            let t56 = f64x8::splat(1.0) / t39 / t54;
            let t60 = (simd::exp(-f64x8::splat(0.007547916666666666) * t51 * t52 * t56));
            let t62 = f64x8::splat(2.0788) - f64x8::splat(0.8524) * t46 - f64x8::splat(1.2264) * t60;
            let t66 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t31 * t62));
            let t67 = (v_rho1).simd_le(dens_threshold);
            let t68 = -t17;
            let t70 = ((t15).select(t12, (t11).select(t16, t68 * t8)));
            let t71 = f64x8::splat(1.0) + t70;
            let t72 = (t71).simd_le(zeta_threshold);
            let t73 = (simd::cbrt(t71));
            let t74 = t73 * t73;
            let t76 = ((t72).select(t24, t74 * t71));
            let t77 = t76 * t30;
            let t78 = v_rho1 * v_rho1;
            let t79 = (simd::cbrt(v_rho1));
            let t80 = t79 * t79;
            let t82 = f64x8::splat(1.0) / t80 / t78;
            let t86 = (simd::exp(-f64x8::splat(8.325416666666667) * t37 * v_sigma2 * t82));
            let t88 = v_sigma2 * v_sigma2;
            let t89 = t78 * t78;
            let t90 = t89 * v_rho1;
            let t92 = f64x8::splat(1.0) / t79 / t90;
            let t96 = (simd::exp(-f64x8::splat(0.007547916666666666) * t51 * t88 * t92));
            let t98 = f64x8::splat(2.0788) - f64x8::splat(0.8524) * t86 - f64x8::splat(1.2264) * t96;
            let t102 = ((t67).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t77 * t98));
            let tzk0 = t66 + t102;
            acc_zk = tzk0;
            let t103 = t7 * t7;
            let t104 = f64x8::splat(1.0) / t103;
            let t105 = t17 * t104;
            let t107 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), t8 - t105)));
            let t110 = ((t21).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t107));
            let t111 = t110 * t30;
            let t115 = f64x8::splat(1.0) / t29;
            let t116 = t28 * t115;
            let t119 = t6 * t116 * t62 / f64x8::splat(10.0);
            let t120 = t38 * v_rho0;
            let t122 = f64x8::splat(1.0) / t40 / t120;
            let t127 = t53 * t38;
            let t129 = f64x8::splat(1.0) / t39 / t127;
            let t134 = -f64x8::splat(18.92422711111111) * t37 * v_sigma0 * t122 * t46 - f64x8::splat(0.049369413333333334) * t51 * t52 * t129 * t60;
            let t139 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t111 * t62 + t119 + f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t31 * t134));
            let t140 = t68 * t104;
            let t142 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), -t8 - t140)));
            let t145 = ((t72).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t74 * t142));
            let t146 = t145 * t30;
            let t150 = t76 * t115;
            let t153 = t6 * t150 * t98 / f64x8::splat(10.0);
            let t155 = ((t67).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t146 * t98 + t153));
            let tvrho0 = t66 + t102 + t7 * (t139 + t155);
            acc_vrho_0 = tvrho0;
            let t159 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), -t8 - t105)));
            let t162 = ((t21).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t159));
            let t163 = t162 * t30;
            let t168 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t163 * t62 + t119));
            let t170 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), t8 - t140)));
            let t173 = ((t72).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t74 * t170));
            let t174 = t173 * t30;
            let t178 = t78 * v_rho1;
            let t180 = f64x8::splat(1.0) / t80 / t178;
            let t185 = t89 * t78;
            let t187 = f64x8::splat(1.0) / t79 / t185;
            let t192 = -f64x8::splat(18.92422711111111) * t37 * v_sigma2 * t180 * t86 - f64x8::splat(0.049369413333333334) * t51 * t88 * t187 * t96;
            let t197 = ((t67).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t174 * t98 + t153 + f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t77 * t192));
            let tvrho1 = t66 + t102 + t7 * (t168 + t197);
            acc_vrho_1 = tvrho1;
            let t207 = f64x8::splat(7.096585166666666) * t37 * t42 * t46 + f64x8::splat(0.01851353) * t51 * v_sigma0 * t56 * t60;
            let t211 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t31 * t207));
            let tvsigma0 = t7 * t211;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t219 = f64x8::splat(7.096585166666666) * t37 * t82 * t86 + f64x8::splat(0.01851353) * t51 * v_sigma2 * t92 * t96;
            let t223 = ((t67).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t77 * t219));
            let tvsigma2 = t7 * t223;
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

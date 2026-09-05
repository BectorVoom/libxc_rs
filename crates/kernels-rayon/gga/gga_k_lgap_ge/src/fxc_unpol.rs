//! GGA_K_LGAP_GE fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_lgap_ge.c`
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

#[allow(unused_variables, non_snake_case)]
pub fn gga_k_lgap_ge_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    param_mu_0: f64,
    param_mu_1: f64,
    param_mu_2: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_mu_0 = f64x8::splat(param_mu_0);
    let param_mu_1 = f64x8::splat(param_mu_1);
    let param_mu_2 = f64x8::splat(param_mu_2);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
        let mut acc_v2rho2 = V_ZERO;
        let mut acc_v2rhosigma = V_ZERO;
        let mut acc_v2sigma2 = V_ZERO;
        {
            let t2 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = t3 * t3;
            let t5 = f64x8::splat(M_CBRTPI);
            let t7 = t4 * t5 * f64x8::splat(M_PI);
            let t8 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t9 = zeta_threshold - f64x8::splat(1.0);
            let t11 = ((t8).select(t9, (t8).select(-t9, f64x8::splat(0.0))));
            let t12 = f64x8::splat(1.0) + t11;
            let t14 = (simd::cbrt(zeta_threshold));
            let t15 = t14 * t14;
            let t17 = (simd::cbrt(t12));
            let t18 = t17 * t17;
            let t20 = (((t12).simd_le(zeta_threshold)).select(t15 * zeta_threshold, t18 * t12));
            let t21 = (simd::cbrt(v_rho));
            let t22 = t21 * t21;
            let t23 = t20 * t22;
            let t25 = f64x8::splat(M_CBRT6);
            let t26 = t25 * t25;
            let t28 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t29 = (simd::cbrt(t28));
            let t31 = param_mu_0 * t26 / t29;
            let t32 = ((v_sigma).sqrt());
            let t33 = f64x8::splat(M_CBRT2);
            let t34 = t32 * t33;
            let t36 = f64x8::splat(1.0) / t21 / v_rho;
            let t41 = param_mu_1 * t25;
            let t42 = t29 * t29;
            let t43 = f64x8::splat(1.0) / t42;
            let t44 = t41 * t43;
            let t45 = t33 * t33;
            let t46 = v_sigma * t45;
            let t47 = v_rho * v_rho;
            let t49 = f64x8::splat(1.0) / t22 / t47;
            let t55 = param_mu_2 / t28;
            let t56 = t32 * v_sigma;
            let t57 = t47 * t47;
            let t58 = f64x8::splat(1.0) / t57;
            let t62 = f64x8::splat(1.0) + t31 * t34 * t36 / f64x8::splat(12.0) + t44 * t46 * t49 / f64x8::splat(24.0) + t55 * t56 * t58 / f64x8::splat(24.0);
            let t66 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t62));
            let tzk0 = f64x8::splat(2.0) * t66;
            acc_zk = tzk0;
            let t68 = t20 / t21;
            let t73 = f64x8::splat(1.0) / t21 / t47;
            let t77 = t47 * v_rho;
            let t79 = f64x8::splat(1.0) / t22 / t77;
            let t83 = t57 * v_rho;
            let t84 = f64x8::splat(1.0) / t83;
            let t88 = -t31 * t34 * t73 / f64x8::splat(9.0) - t44 * t46 * t79 / f64x8::splat(9.0) - t55 * t56 * t84 / f64x8::splat(6.0);
            let t93 = ((t2).select(f64x8::splat(0.0), t7 * t68 * t62 / f64x8::splat(10.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t88));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t93 + f64x8::splat(2.0) * t66;
            acc_vrho = tvrho0;
            let t96 = f64x8::splat(1.0) / t32;
            let t97 = t96 * t33;
            let t101 = t43 * t45;
            let t108 = t31 * t97 * t36 / f64x8::splat(24.0) + t41 * t101 * t49 / f64x8::splat(24.0) + t55 * t32 * t58 / f64x8::splat(16.0);
            let t112 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t108));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t112;
            acc_vsigma = tvsigma0;
            let t115 = t20 * t36;
            let t123 = f64x8::splat(1.0) / t21 / t77;
            let t128 = f64x8::splat(1.0) / t22 / t57;
            let t132 = t57 * t47;
            let t133 = f64x8::splat(1.0) / t132;
            let t137 = f64x8::splat(7.0) / f64x8::splat(27.0) * t31 * t34 * t123 + f64x8::splat(11.0) / f64x8::splat(27.0) * t44 * t46 * t128 + f64x8::splat(5.0) / f64x8::splat(6.0) * t55 * t56 * t133;
            let t142 = ((t2).select(f64x8::splat(0.0), -t7 * t115 * t62 / f64x8::splat(30.0) + t7 * t68 * t88 / f64x8::splat(5.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t137));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t142 + f64x8::splat(4.0) * t93;
            acc_v2rho2 = tv2rho20;
            let t157 = -t31 * t97 * t73 / f64x8::splat(18.0) - t41 * t101 * t79 / f64x8::splat(9.0) - t55 * t32 * t84 / f64x8::splat(4.0);
            let t162 = ((t2).select(f64x8::splat(0.0), t7 * t68 * t108 / f64x8::splat(10.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t157));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t162 + f64x8::splat(2.0) * t112;
            acc_v2rhosigma = tv2rhosigma0;
            let t165 = f64x8::splat(1.0) / t56;
            let t166 = t165 * t33;
            let t173 = -t31 * t166 * t36 / f64x8::splat(48.0) + t55 * t96 * t58 / f64x8::splat(32.0);
            let t177 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t173));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t177;
            acc_v2sigma2 = tv2sigma20;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        store_add(v2rho2, ip, m, acc_v2rho2);
        store_add(v2rhosigma, ip, m, acc_v2rhosigma);
        store_add(v2sigma2, ip, m, acc_v2sigma2);
        ip += 8;
    }
}

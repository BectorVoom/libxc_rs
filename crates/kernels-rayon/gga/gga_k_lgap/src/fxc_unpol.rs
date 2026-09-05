//! GGA_K_LGAP fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_lgap.c`
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
pub fn gga_k_lgap_fxc_unpol(
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
    param_kappa: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_mu_0 = f64x8::splat(param_mu_0);
    let param_mu_1 = f64x8::splat(param_mu_1);
    let param_mu_2 = f64x8::splat(param_mu_2);
    let param_kappa = f64x8::splat(param_kappa);
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
            let t63 = (simd::exp(-t31 * t34 * t36 / f64x8::splat(12.0) - t44 * t46 * t49 / f64x8::splat(24.0) - t55 * t56 * t58 / f64x8::splat(24.0)));
            let t66 = f64x8::splat(1.0) + param_kappa * (f64x8::splat(1.0) - t63);
            let t70 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t66));
            let tzk0 = f64x8::splat(2.0) * t70;
            acc_zk = tzk0;
            let t71 = f64x8::splat(1.0) / t21;
            let t72 = t20 * t71;
            let t76 = t7 * t20;
            let t77 = t22 * param_kappa;
            let t79 = f64x8::splat(1.0) / t21 / t47;
            let t83 = t47 * v_rho;
            let t85 = f64x8::splat(1.0) / t22 / t83;
            let t89 = t57 * v_rho;
            let t90 = f64x8::splat(1.0) / t89;
            let t94 = t31 * t34 * t79 / f64x8::splat(9.0) + t44 * t46 * t85 / f64x8::splat(9.0) + t55 * t56 * t90 / f64x8::splat(6.0);
            let t95 = t94 * t63;
            let t100 = ((t2).select(f64x8::splat(0.0), t7 * t72 * t66 / f64x8::splat(10.0) - f64x8::splat(3.0) / f64x8::splat(20.0) * t76 * t77 * t95));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t100 + f64x8::splat(2.0) * t70;
            acc_vrho = tvrho0;
            let t103 = f64x8::splat(1.0) / t32;
            let t104 = t103 * t33;
            let t108 = t43 * t45;
            let t115 = -t31 * t104 * t36 / f64x8::splat(24.0) - t41 * t108 * t49 / f64x8::splat(24.0) - t55 * t32 * t58 / f64x8::splat(16.0);
            let t116 = t115 * t63;
            let t120 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(20.0) * t76 * t77 * t116));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t120;
            acc_vsigma = tvsigma0;
            let t123 = t20 * t36;
            let t127 = t71 * param_kappa;
            let t132 = f64x8::splat(1.0) / t21 / t83;
            let t137 = f64x8::splat(1.0) / t22 / t57;
            let t141 = t57 * t47;
            let t142 = f64x8::splat(1.0) / t141;
            let t146 = -f64x8::splat(7.0) / f64x8::splat(27.0) * t31 * t34 * t132 - f64x8::splat(11.0) / f64x8::splat(27.0) * t44 * t46 * t137 - f64x8::splat(5.0) / f64x8::splat(6.0) * t55 * t56 * t142;
            let t147 = t146 * t63;
            let t151 = t94 * t94;
            let t152 = t151 * t63;
            let t157 = ((t2).select(f64x8::splat(0.0), -t7 * t123 * t66 / f64x8::splat(30.0) - t76 * t127 * t95 / f64x8::splat(5.0) - f64x8::splat(3.0) / f64x8::splat(20.0) * t76 * t77 * t147 - f64x8::splat(3.0) / f64x8::splat(20.0) * t76 * t77 * t152));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t157 + f64x8::splat(4.0) * t100;
            acc_v2rho2 = tv2rho20;
            let t172 = t31 * t104 * t79 / f64x8::splat(18.0) + t41 * t108 * t85 / f64x8::splat(9.0) + t55 * t32 * t90 / f64x8::splat(4.0);
            let t173 = t172 * t63;
            let t177 = t7 * t23;
            let t178 = param_kappa * t115;
            let t179 = t178 * t95;
            let t183 = ((t2).select(f64x8::splat(0.0), -t76 * t127 * t116 / f64x8::splat(10.0) - f64x8::splat(3.0) / f64x8::splat(20.0) * t76 * t77 * t173 - f64x8::splat(3.0) / f64x8::splat(20.0) * t177 * t179));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t183 + f64x8::splat(2.0) * t120;
            acc_v2rhosigma = tv2rhosigma0;
            let t186 = f64x8::splat(1.0) / t56;
            let t187 = t186 * t33;
            let t194 = t31 * t187 * t36 / f64x8::splat(48.0) - t55 * t103 * t58 / f64x8::splat(32.0);
            let t195 = t194 * t63;
            let t198 = t115 * t115;
            let t199 = t198 * t63;
            let t204 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(20.0) * t76 * t77 * t195 - f64x8::splat(3.0) / f64x8::splat(20.0) * t76 * t77 * t199));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t204;
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

//! GGA_X_LSRPBE kxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_lsrpbe.c`
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
pub fn gga_x_lsrpbe_kxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    v3rho3: &mut [f64],
    v3rho2sigma: &mut [f64],
    v3rhosigma2: &mut [f64],
    v3sigma3: &mut [f64],
    param_mu: f64,
    param_kappa: f64,
    param_alpha: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_mu = f64x8::splat(param_mu);
    let param_kappa = f64x8::splat(param_kappa);
    let param_alpha = f64x8::splat(param_alpha);
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
        let mut acc_v3rho3 = V_ZERO;
        let mut acc_v3rho2sigma = V_ZERO;
        let mut acc_v3rhosigma2 = V_ZERO;
        let mut acc_v3sigma3 = V_ZERO;
        {
            let t2 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = f64x8::splat(M_CBRTPI);
            let t6 = t3 / t4;
            let t7 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t8 = zeta_threshold - f64x8::splat(1.0);
            let t10 = ((t7).select(t8, (t7).select(-t8, f64x8::splat(0.0))));
            let t11 = f64x8::splat(1.0) + t10;
            let t13 = (simd::cbrt(zeta_threshold));
            let t15 = (simd::cbrt(t11));
            let t17 = (((t11).simd_le(zeta_threshold)).select(t13 * zeta_threshold, t15 * t11));
            let t18 = (simd::cbrt(v_rho));
            let t19 = t17 * t18;
            let t20 = f64x8::splat(M_CBRT6);
            let t22 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t23 = (simd::cbrt(t22));
            let t24 = t23 * t23;
            let t25 = f64x8::splat(1.0) / t24;
            let t26 = param_mu * t20 * t25;
            let t27 = f64x8::splat(M_CBRT2);
            let t28 = t27 * t27;
            let t29 = v_sigma * t28;
            let t30 = v_rho * v_rho;
            let t31 = t18 * t18;
            let t33 = f64x8::splat(1.0) / t31 / t30;
            let t34 = f64x8::splat(1.0) / param_kappa;
            let t39 = (simd::exp(-t26 * t29 * t33 * t34 / f64x8::splat(24.0)));
            let t42 = param_kappa + f64x8::splat(1.0);
            let t48 = (simd::exp(-param_alpha * t20 * t25 * t29 * t33 / f64x8::splat(24.0)));
            let t51 = f64x8::splat(1.0) + param_kappa * (f64x8::splat(1.0) - t39) - t42 * (f64x8::splat(1.0) - t48);
            let t55 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t51));
            let tzk0 = f64x8::splat(2.0) * t55;
            acc_zk = tzk0;
            let t57 = t17 / t31;
            let t61 = t30 * v_rho;
            let t63 = f64x8::splat(1.0) / t31 / t61;
            let t67 = t42 * param_alpha;
            let t68 = t20 * t25;
            let t69 = t67 * t68;
            let t70 = t63 * t48;
            let t74 = -t26 * t29 * t63 * t39 / f64x8::splat(9.0) + t69 * t29 * t70 / f64x8::splat(9.0);
            let t79 = ((t2).select(f64x8::splat(0.0), -t6 * t57 * t51 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t74));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t79 + f64x8::splat(2.0) * t55;
            acc_vrho = tvrho0;
            let t85 = t67 * t20;
            let t86 = t25 * t28;
            let t91 = t26 * t28 * t33 * t39 / f64x8::splat(24.0) - t85 * t86 * t33 * t48 / f64x8::splat(24.0);
            let t95 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t91));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t95;
            acc_vsigma = tvsigma0;
            let t100 = t17 / t31 / v_rho;
            let t107 = t30 * t30;
            let t109 = f64x8::splat(1.0) / t31 / t107;
            let t114 = param_mu * param_mu;
            let t115 = t20 * t20;
            let t116 = t114 * t115;
            let t118 = f64x8::splat(1.0) / t23 / t22;
            let t119 = v_sigma * v_sigma;
            let t121 = t116 * t118 * t119;
            let t124 = f64x8::splat(1.0) / t18 / t107 / t61;
            let t125 = t27 * t124;
            let t126 = t34 * t39;
            let t127 = t125 * t126;
            let t130 = t109 * t48;
            let t134 = param_alpha * param_alpha;
            let t135 = t42 * t134;
            let t137 = t135 * t115 * t118;
            let t138 = t119 * t27;
            let t139 = t124 * t48;
            let t143 = f64x8::splat(11.0) / f64x8::splat(27.0) * t26 * t29 * t109 * t39 - f64x8::splat(2.0) / f64x8::splat(81.0) * t121 * t127 - f64x8::splat(11.0) / f64x8::splat(27.0) * t69 * t29 * t130 + f64x8::splat(2.0) / f64x8::splat(81.0) * t137 * t138 * t139;
            let t148 = ((t2).select(f64x8::splat(0.0), t6 * t100 * t51 / f64x8::splat(12.0) - t6 * t57 * t74 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t143));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t148 + f64x8::splat(4.0) * t79;
            acc_v2rho2 = tv2rho20;
            let t158 = t118 * t27;
            let t159 = t116 * t158;
            let t160 = t107 * t30;
            let t162 = f64x8::splat(1.0) / t18 / t160;
            let t170 = t27 * t162;
            let t171 = v_sigma * t48;
            let t175 = -t26 * t28 * t63 * t39 / f64x8::splat(9.0) + t159 * t162 * v_sigma * t126 / f64x8::splat(108.0) + t85 * t86 * t70 / f64x8::splat(9.0) - t137 * t170 * t171 / f64x8::splat(108.0);
            let t180 = ((t2).select(f64x8::splat(0.0), -t6 * t57 * t91 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t175));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t180 + f64x8::splat(2.0) * t95;
            acc_v2rhosigma = tv2rhosigma0;
            let t183 = t116 * t118;
            let t184 = t107 * v_rho;
            let t186 = f64x8::splat(1.0) / t18 / t184;
            let t190 = t135 * t115;
            let t195 = -t183 * t27 * t186 * t126 / f64x8::splat(288.0) + t190 * t158 * t186 * t48 / f64x8::splat(288.0);
            let t199 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t195));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t199;
            acc_v2sigma2 = tv2sigma20;
            let t202 = t17 * t33;
            let t213 = f64x8::splat(1.0) / t31 / t184;
            let t218 = t107 * t107;
            let t220 = f64x8::splat(1.0) / t18 / t218;
            let t221 = t27 * t220;
            let t226 = t22 * t22;
            let t227 = f64x8::splat(1.0) / t226;
            let t228 = t114 * param_mu * t227;
            let t229 = t119 * v_sigma;
            let t230 = t228 * t229;
            let t231 = t218 * t61;
            let t232 = f64x8::splat(1.0) / t231;
            let t233 = param_kappa * param_kappa;
            let t234 = f64x8::splat(1.0) / t233;
            let t239 = t213 * t48;
            let t248 = t42 * t134 * param_alpha;
            let t249 = t248 * t227;
            let t254 = -f64x8::splat(154.0) / f64x8::splat(81.0) * t26 * t29 * t213 * t39 + f64x8::splat(22.0) / f64x8::splat(81.0) * t121 * t221 * t126 - f64x8::splat(8.0) / f64x8::splat(243.0) * t230 * t232 * t234 * t39 + f64x8::splat(154.0) / f64x8::splat(81.0) * t69 * t29 * t239 - f64x8::splat(22.0) / f64x8::splat(81.0) * t137 * t138 * t220 * t48 + f64x8::splat(8.0) / f64x8::splat(243.0) * t249 * t229 * t232 * t48;
            let t259 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(5.0) / f64x8::splat(36.0) * t6 * t202 * t51 + t6 * t100 * t74 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t57 * t143 - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t254));
            let tv3rho30 = f64x8::splat(2.0) * v_rho * t259 + f64x8::splat(6.0) * t148;
            acc_v3rho3 = tv3rho30;
            let t277 = t218 * t30;
            let t278 = f64x8::splat(1.0) / t277;
            let t279 = t228 * t278;
            let t281 = t119 * t234 * t39;
            let t294 = f64x8::splat(11.0) / f64x8::splat(27.0) * t26 * t28 * t109 * t39 - t159 * t124 * v_sigma * t126 / f64x8::splat(12.0) + t279 * t281 / f64x8::splat(81.0) - f64x8::splat(11.0) / f64x8::splat(27.0) * t85 * t86 * t130 + t137 * t125 * t171 / f64x8::splat(12.0) - t249 * t278 * t119 * t48 / f64x8::splat(81.0);
            let t299 = ((t2).select(f64x8::splat(0.0), t6 * t100 * t91 / f64x8::splat(12.0) - t6 * t57 * t175 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t294));
            let tv3rho2sigma0 = f64x8::splat(2.0) * v_rho * t299 + f64x8::splat(4.0) * t180;
            acc_v3rho2sigma = tv3rho2sigma0;
            let t308 = t218 * v_rho;
            let t309 = f64x8::splat(1.0) / t308;
            let t312 = t234 * v_sigma * t39;
            let t323 = t183 * t170 * t126 / f64x8::splat(54.0) - t228 * t309 * t312 / f64x8::splat(216.0) - t190 * t158 * t162 * t48 / f64x8::splat(54.0) + t249 * t309 * v_sigma * t48 / f64x8::splat(216.0);
            let t328 = ((t2).select(f64x8::splat(0.0), -t6 * t57 * t195 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t323));
            let tv3rhosigma20 = f64x8::splat(2.0) * v_rho * t328 + f64x8::splat(2.0) * t199;
            acc_v3rhosigma2 = tv3rhosigma20;
            let t331 = f64x8::splat(1.0) / t218;
            let t339 = -t248 * t227 * t331 * t48 / f64x8::splat(576.0) + t228 * t331 * t234 * t39 / f64x8::splat(576.0);
            let t343 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t339));
            let tv3sigma30 = f64x8::splat(2.0) * v_rho * t343;
            acc_v3sigma3 = tv3sigma30;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        store_add(v2rho2, ip, m, acc_v2rho2);
        store_add(v2rhosigma, ip, m, acc_v2rhosigma);
        store_add(v2sigma2, ip, m, acc_v2sigma2);
        store_add(v3rho3, ip, m, acc_v3rho3);
        store_add(v3rho2sigma, ip, m, acc_v3rho2sigma);
        store_add(v3rhosigma2, ip, m, acc_v3rhosigma2);
        store_add(v3sigma3, ip, m, acc_v3sigma3);
        ip += 8;
    }
}

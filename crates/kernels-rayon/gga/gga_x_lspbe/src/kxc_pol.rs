//! GGA_X_LSPBE kxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_lspbe.c`
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

/// Store 8 elements with a given stride and offset.
#[inline(always)]
fn store_strided(s: &mut [f64], ip: usize, m: usize, stride: usize, offset: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let base = ip * stride + offset;
        s[base] = a[0];
        s[base + stride] = a[1];
        s[base + 2 * stride] = a[2];
        s[base + 3 * stride] = a[3];
        s[base + 4 * stride] = a[4];
        s[base + 5 * stride] = a[5];
        s[base + 6 * stride] = a[6];
        s[base + 7 * stride] = a[7];
    } else {
        for k in 0..m {
            s[(ip + k) * stride + offset] = a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_lspbe_kxc_pol(
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
        let mut acc_v2rho2_0 = V_ZERO;
        let mut acc_v2rho2_1 = V_ZERO;
        let mut acc_v2rho2_2 = V_ZERO;
        let mut acc_v2rhosigma_0 = V_ZERO;
        let mut acc_v2rhosigma_1 = V_ZERO;
        let mut acc_v2rhosigma_2 = V_ZERO;
        let mut acc_v2rhosigma_3 = V_ZERO;
        let mut acc_v2rhosigma_4 = V_ZERO;
        let mut acc_v2rhosigma_5 = V_ZERO;
        let mut acc_v2sigma2_0 = V_ZERO;
        let mut acc_v2sigma2_1 = V_ZERO;
        let mut acc_v2sigma2_2 = V_ZERO;
        let mut acc_v2sigma2_3 = V_ZERO;
        let mut acc_v2sigma2_4 = V_ZERO;
        let mut acc_v2sigma2_5 = V_ZERO;
        let mut acc_v3rho3_0 = V_ZERO;
        let mut acc_v3rho3_1 = V_ZERO;
        let mut acc_v3rho3_2 = V_ZERO;
        let mut acc_v3rho3_3 = V_ZERO;
        let mut acc_v3rho2sigma_0 = V_ZERO;
        let mut acc_v3rho2sigma_1 = V_ZERO;
        let mut acc_v3rho2sigma_2 = V_ZERO;
        let mut acc_v3rho2sigma_3 = V_ZERO;
        let mut acc_v3rho2sigma_4 = V_ZERO;
        let mut acc_v3rho2sigma_5 = V_ZERO;
        let mut acc_v3rho2sigma_6 = V_ZERO;
        let mut acc_v3rho2sigma_7 = V_ZERO;
        let mut acc_v3rho2sigma_8 = V_ZERO;
        let mut acc_v3rhosigma2_0 = V_ZERO;
        let mut acc_v3rhosigma2_1 = V_ZERO;
        let mut acc_v3rhosigma2_2 = V_ZERO;
        let mut acc_v3rhosigma2_3 = V_ZERO;
        let mut acc_v3rhosigma2_4 = V_ZERO;
        let mut acc_v3rhosigma2_5 = V_ZERO;
        let mut acc_v3rhosigma2_6 = V_ZERO;
        let mut acc_v3rhosigma2_7 = V_ZERO;
        let mut acc_v3rhosigma2_8 = V_ZERO;
        let mut acc_v3rhosigma2_9 = V_ZERO;
        let mut acc_v3rhosigma2_10 = V_ZERO;
        let mut acc_v3rhosigma2_11 = V_ZERO;
        let mut acc_v3sigma3_0 = V_ZERO;
        let mut acc_v3sigma3_1 = V_ZERO;
        let mut acc_v3sigma3_2 = V_ZERO;
        let mut acc_v3sigma3_3 = V_ZERO;
        let mut acc_v3sigma3_4 = V_ZERO;
        let mut acc_v3sigma3_5 = V_ZERO;
        let mut acc_v3sigma3_6 = V_ZERO;
        let mut acc_v3sigma3_7 = V_ZERO;
        let mut acc_v3sigma3_8 = V_ZERO;
        let mut acc_v3sigma3_9 = V_ZERO;
        {
            let t1 = (v_rho0).simd_le(dens_threshold);
            let t2 = f64x8::splat(M_CBRT3);
            let t3 = f64x8::splat(M_CBRTPI);
            let t5 = t2 / t3;
            let t6 = v_rho0 + v_rho1;
            let t7 = f64x8::splat(1.0) / t6;
            let t10 = (f64x8::splat(2.0) * v_rho0 * t7).simd_le(zeta_threshold);
            let t11 = zeta_threshold - f64x8::splat(1.0);
            let t14 = (f64x8::splat(2.0) * v_rho1 * t7).simd_le(zeta_threshold);
            let t15 = -t11;
            let t16 = v_rho0 - v_rho1;
            let t18 = ((t10).select(t11, (t14).select(t15, t16 * t7)));
            let t19 = f64x8::splat(1.0) + t18;
            let t20 = (t19).simd_le(zeta_threshold);
            let t21 = (simd::cbrt(zeta_threshold));
            let t22 = t21 * zeta_threshold;
            let t23 = (simd::cbrt(t19));
            let t25 = ((t20).select(t22, t23 * t19));
            let t26 = (simd::cbrt(t6));
            let t27 = t25 * t26;
            let t28 = f64x8::splat(M_CBRT6);
            let t29 = param_mu * t28;
            let t30 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t31 = (simd::cbrt(t30));
            let t32 = t31 * t31;
            let t33 = f64x8::splat(1.0) / t32;
            let t34 = t33 * v_sigma0;
            let t35 = v_rho0 * v_rho0;
            let t36 = (simd::cbrt(v_rho0));
            let t37 = t36 * t36;
            let t39 = f64x8::splat(1.0) / t37 / t35;
            let t40 = t34 * t39;
            let t43 = param_kappa + t29 * t40 / f64x8::splat(24.0);
            let t48 = param_kappa + f64x8::splat(1.0);
            let t49 = param_alpha * t28;
            let t52 = (simd::exp(-t49 * t40 / f64x8::splat(24.0)));
            let t55 = f64x8::splat(1.0) + param_kappa * (f64x8::splat(1.0) - param_kappa / t43) - t48 * (f64x8::splat(1.0) - t52);
            let t59 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t55));
            let t60 = (v_rho1).simd_le(dens_threshold);
            let t61 = -t16;
            let t63 = ((t14).select(t11, (t10).select(t15, t61 * t7)));
            let t64 = f64x8::splat(1.0) + t63;
            let t65 = (t64).simd_le(zeta_threshold);
            let t66 = (simd::cbrt(t64));
            let t68 = ((t65).select(t22, t66 * t64));
            let t69 = t68 * t26;
            let t70 = t33 * v_sigma2;
            let t71 = v_rho1 * v_rho1;
            let t72 = (simd::cbrt(v_rho1));
            let t73 = t72 * t72;
            let t75 = f64x8::splat(1.0) / t73 / t71;
            let t76 = t70 * t75;
            let t79 = param_kappa + t29 * t76 / f64x8::splat(24.0);
            let t86 = (simd::exp(-t49 * t76 / f64x8::splat(24.0)));
            let t89 = f64x8::splat(1.0) + param_kappa * (f64x8::splat(1.0) - param_kappa / t79) - t48 * (f64x8::splat(1.0) - t86);
            let t93 = ((t60).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t69 * t89));
            let tzk0 = t59 + t93;
            acc_zk = tzk0;
            let t94 = t6 * t6;
            let t95 = f64x8::splat(1.0) / t94;
            let t96 = t16 * t95;
            let t98 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t7 - t96)));
            let t101 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t98));
            let t102 = t101 * t26;
            let t106 = t26 * t26;
            let t107 = f64x8::splat(1.0) / t106;
            let t108 = t25 * t107;
            let t111 = t5 * t108 * t55 / f64x8::splat(8.0);
            let t112 = param_kappa * param_kappa;
            let t113 = t43 * t43;
            let t116 = t112 / t113 * param_mu;
            let t117 = t28 * t33;
            let t118 = t35 * v_rho0;
            let t120 = f64x8::splat(1.0) / t37 / t118;
            let t125 = t48 * param_alpha * t28;
            let t130 = -t116 * t117 * v_sigma0 * t120 / f64x8::splat(9.0) + t125 * t34 * t120 * t52 / f64x8::splat(9.0);
            let t135 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t102 * t55 - t111 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t130));
            let t136 = t61 * t95;
            let t138 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -t7 - t136)));
            let t141 = ((t65).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t66 * t138));
            let t142 = t141 * t26;
            let t146 = t68 * t107;
            let t149 = t5 * t146 * t89 / f64x8::splat(8.0);
            let t151 = ((t60).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t142 * t89 - t149));
            let tvrho0 = t59 + t93 + t6 * (t135 + t151);
            acc_vrho_0 = tvrho0;
            let t155 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -t7 - t96)));
            let t158 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t155));
            let t159 = t158 * t26;
            let t164 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t159 * t55 - t111));
            let t166 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), t7 - t136)));
            let t169 = ((t65).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t66 * t166));
            let t170 = t169 * t26;
            let t174 = t79 * t79;
            let t177 = t112 / t174 * param_mu;
            let t178 = t71 * v_rho1;
            let t180 = f64x8::splat(1.0) / t73 / t178;
            let t188 = -t177 * t117 * v_sigma2 * t180 / f64x8::splat(9.0) + t125 * t70 * t180 * t86 / f64x8::splat(9.0);
            let t193 = ((t60).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t170 * t89 - t149 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t69 * t188));
            let tvrho1 = t59 + t93 + t6 * (t164 + t193);
            acc_vrho_1 = tvrho1;
            let t202 = -t125 * t33 * t39 * t52 / f64x8::splat(24.0) + t116 * t117 * t39 / f64x8::splat(24.0);
            let t206 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t202));
            let tvsigma0 = t6 * t206;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t213 = -t125 * t33 * t75 * t86 / f64x8::splat(24.0) + t177 * t117 * t75 / f64x8::splat(24.0);
            let t217 = ((t60).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t69 * t213));
            let tvsigma2 = t6 * t217;
            acc_vsigma_2 = tvsigma2;
            let t220 = t23 * t23;
            let t221 = f64x8::splat(1.0) / t220;
            let t222 = t98 * t98;
            let t225 = t94 * t6;
            let t226 = f64x8::splat(1.0) / t225;
            let t227 = t16 * t226;
            let t230 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -f64x8::splat(2.0) * t95 + f64x8::splat(2.0) * t227)));
            let t234 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t221 * t222 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t230));
            let t235 = t234 * t26;
            let t239 = t101 * t107;
            let t241 = t5 * t239 * t55;
            let t247 = f64x8::splat(1.0) / t106 / t6;
            let t248 = t25 * t247;
            let t251 = t5 * t248 * t55 / f64x8::splat(12.0);
            let t253 = t5 * t108 * t130;
            let t258 = param_mu * param_mu;
            let t259 = t112 / t113 / t43 * t258;
            let t260 = t28 * t28;
            let t262 = f64x8::splat(1.0) / t31 / t30;
            let t263 = t260 * t262;
            let t264 = v_sigma0 * v_sigma0;
            let t265 = t35 * t35;
            let t268 = f64x8::splat(1.0) / t36 / t265 / t118;
            let t274 = f64x8::splat(1.0) / t37 / t265;
            let t283 = param_alpha * param_alpha;
            let t285 = t48 * t283 * t260;
            let t286 = t262 * t264;
            let t291 = -f64x8::splat(2.0) / f64x8::splat(81.0) * t259 * t263 * t264 * t268 + f64x8::splat(11.0) / f64x8::splat(27.0) * t116 * t117 * v_sigma0 * t274 - f64x8::splat(11.0) / f64x8::splat(27.0) * t125 * t34 * t274 * t52 + t285 * t286 * t268 * t52 / f64x8::splat(81.0);
            let t296 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t235 * t55 - t241 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t102 * t130 + t251 - t253 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t291));
            let t297 = t66 * t66;
            let t298 = f64x8::splat(1.0) / t297;
            let t299 = t138 * t138;
            let t302 = t61 * t226;
            let t305 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), f64x8::splat(2.0) * t95 + f64x8::splat(2.0) * t302)));
            let t309 = ((t65).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t298 * t299 + f64x8::splat(4.0) / f64x8::splat(3.0) * t66 * t305));
            let t310 = t309 * t26;
            let t314 = t141 * t107;
            let t316 = t5 * t314 * t89;
            let t318 = t68 * t247;
            let t321 = t5 * t318 * t89 / f64x8::splat(12.0);
            let t323 = ((t60).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t310 * t89 - t316 / f64x8::splat(4.0) + t321));
            let tv2rho20 = f64x8::splat(2.0) * t135 + f64x8::splat(2.0) * t151 + t6 * (t296 + t323);
            acc_v2rho2_0 = tv2rho20;
            let t326 = t221 * t155;
            let t330 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), f64x8::splat(2.0) * t227)));
            let t334 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t326 * t98 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t330));
            let t335 = t334 * t26;
            let t339 = t158 * t107;
            let t341 = t5 * t339 * t55;
            let t349 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t335 * t55 - t341 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t159 * t130 - t241 / f64x8::splat(8.0) + t251 - t253 / f64x8::splat(8.0)));
            let t350 = t298 * t166;
            let t354 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), f64x8::splat(2.0) * t302)));
            let t358 = ((t65).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t350 * t138 + f64x8::splat(4.0) / f64x8::splat(3.0) * t66 * t354));
            let t359 = t358 * t26;
            let t363 = t169 * t107;
            let t365 = t5 * t363 * t89;
            let t372 = t5 * t146 * t188;
            let t375 = ((t60).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t359 * t89 - t365 / f64x8::splat(8.0) - t316 / f64x8::splat(8.0) + t321 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t142 * t188 - t372 / f64x8::splat(8.0)));
            let tv2rho21 = t135 + t151 + t164 + t193 + t6 * (t349 + t375);
            acc_v2rho2_1 = tv2rho21;
            let t380 = t155 * t155;
            let t385 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), f64x8::splat(2.0) * t95 + f64x8::splat(2.0) * t227)));
            let t389 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t221 * t380 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t385));
            let t390 = t389 * t26;
            let t396 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t390 * t55 - t341 / f64x8::splat(4.0) + t251));
            let t397 = t166 * t166;
            let t402 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -f64x8::splat(2.0) * t95 + f64x8::splat(2.0) * t302)));
            let t406 = ((t65).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t298 * t397 + f64x8::splat(4.0) / f64x8::splat(3.0) * t66 * t402));
            let t407 = t406 * t26;
            let t419 = t112 / t174 / t79 * t258;
            let t420 = v_sigma2 * v_sigma2;
            let t421 = t71 * t71;
            let t424 = f64x8::splat(1.0) / t72 / t421 / t178;
            let t430 = f64x8::splat(1.0) / t73 / t421;
            let t439 = t262 * t420;
            let t444 = -f64x8::splat(2.0) / f64x8::splat(81.0) * t419 * t263 * t420 * t424 + f64x8::splat(11.0) / f64x8::splat(27.0) * t177 * t117 * v_sigma2 * t430 - f64x8::splat(11.0) / f64x8::splat(27.0) * t125 * t70 * t430 * t86 + t285 * t439 * t424 * t86 / f64x8::splat(81.0);
            let t449 = ((t60).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t407 * t89 - t365 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t170 * t188 + t321 - t372 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t69 * t444));
            let tv2rho22 = f64x8::splat(2.0) * t164 + f64x8::splat(2.0) * t193 + t6 * (t396 + t449);
            acc_v2rho2_2 = tv2rho22;
            let t457 = t5 * t108 * t202 / f64x8::splat(8.0);
            let t458 = t265 * t35;
            let t460 = f64x8::splat(1.0) / t36 / t458;
            let t472 = t262 * t460;
            let t473 = v_sigma0 * t52;
            let t477 = t259 * t263 * t460 * v_sigma0 / f64x8::splat(108.0) - t116 * t117 * t120 / f64x8::splat(9.0) + t125 * t33 * t120 * t52 / f64x8::splat(9.0) - t285 * t472 * t473 / f64x8::splat(216.0);
            let t482 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t102 * t202 - t457 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t477));
            let tv2rhosigma0 = t6 * t482 + t206;
            acc_v2rhosigma_0 = tv2rhosigma0;
            let tv2rhosigma1 = f64x8::splat(0.0);
            acc_v2rhosigma_1 = tv2rhosigma1;
            let t489 = t5 * t146 * t213 / f64x8::splat(8.0);
            let t491 = ((t60).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t142 * t213 - t489));
            let tv2rhosigma2 = t6 * t491 + t217;
            acc_v2rhosigma_2 = tv2rhosigma2;
            let t497 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t159 * t202 - t457));
            let tv2rhosigma3 = t6 * t497 + t206;
            acc_v2rhosigma_3 = tv2rhosigma3;
            let tv2rhosigma4 = f64x8::splat(0.0);
            acc_v2rhosigma_4 = tv2rhosigma4;
            let t502 = t421 * t71;
            let t504 = f64x8::splat(1.0) / t72 / t502;
            let t516 = t262 * t504;
            let t517 = v_sigma2 * t86;
            let t521 = t419 * t263 * t504 * v_sigma2 / f64x8::splat(108.0) - t177 * t117 * t180 / f64x8::splat(9.0) + t125 * t33 * t180 * t86 / f64x8::splat(9.0) - t285 * t516 * t517 / f64x8::splat(216.0);
            let t526 = ((t60).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t170 * t213 - t489 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t69 * t521));
            let tv2rhosigma5 = t6 * t526 + t217;
            acc_v2rhosigma_5 = tv2rhosigma5;
            let t528 = t265 * v_rho0;
            let t530 = f64x8::splat(1.0) / t36 / t528;
            let t538 = -t259 * t263 * t530 / f64x8::splat(288.0) + t285 * t262 * t530 * t52 / f64x8::splat(576.0);
            let t542 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t538));
            let tv2sigma20 = t6 * t542;
            acc_v2sigma2_0 = tv2sigma20;
            let tv2sigma21 = f64x8::splat(0.0);
            acc_v2sigma2_1 = tv2sigma21;
            let tv2sigma22 = f64x8::splat(0.0);
            acc_v2sigma2_2 = tv2sigma22;
            let tv2sigma23 = f64x8::splat(0.0);
            acc_v2sigma2_3 = tv2sigma23;
            let tv2sigma24 = f64x8::splat(0.0);
            acc_v2sigma2_4 = tv2sigma24;
            let t543 = t421 * v_rho1;
            let t545 = f64x8::splat(1.0) / t72 / t543;
            let t553 = -t419 * t263 * t545 / f64x8::splat(288.0) + t285 * t262 * t545 * t86 / f64x8::splat(576.0);
            let t557 = ((t60).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t69 * t553));
            let tv2sigma25 = t6 * t557;
            acc_v2sigma2_5 = tv2sigma25;
            let t561 = f64x8::splat(1.0) / t220 / t19;
            let t562 = t222 * t98;
            let t565 = t221 * t98;
            let t568 = t94 * t94;
            let t569 = f64x8::splat(1.0) / t568;
            let t570 = t16 * t569;
            let t573 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), f64x8::splat(6.0) * t226 - f64x8::splat(6.0) * t570)));
            let t577 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t561 * t562 + f64x8::splat(4.0) / f64x8::splat(3.0) * t565 * t230 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t573));
            let t578 = t577 * t26;
            let t582 = t234 * t107;
            let t584 = t5 * t582 * t55;
            let t589 = t101 * t247;
            let t591 = t5 * t589 * t55;
            let t594 = t5 * t239 * t130;
            let t600 = f64x8::splat(1.0) / t106 / t94;
            let t601 = t25 * t600;
            let t604 = f64x8::splat(5.0) / f64x8::splat(36.0) * t5 * t601 * t55;
            let t606 = t5 * t248 * t130;
            let t609 = t5 * t108 * t291;
            let t611 = t113 * t113;
            let t613 = t112 / t611;
            let t614 = t258 * param_mu;
            let t615 = t613 * t614;
            let t616 = t30 * t30;
            let t617 = f64x8::splat(1.0) / t616;
            let t618 = t264 * v_sigma0;
            let t619 = t617 * t618;
            let t620 = t265 * t265;
            let t621 = t620 * t118;
            let t622 = f64x8::splat(1.0) / t621;
            let t627 = f64x8::splat(1.0) / t36 / t620;
            let t633 = f64x8::splat(1.0) / t37 / t528;
            let t647 = t48 * t283 * param_alpha;
            let t648 = t647 * t617;
            let t653 = -f64x8::splat(4.0) / f64x8::splat(81.0) * t615 * t619 * t622 + f64x8::splat(22.0) / f64x8::splat(81.0) * t259 * t263 * t264 * t627 - f64x8::splat(154.0) / f64x8::splat(81.0) * t116 * t117 * v_sigma0 * t633 + f64x8::splat(154.0) / f64x8::splat(81.0) * t125 * t34 * t633 * t52 - f64x8::splat(11.0) / f64x8::splat(81.0) * t285 * t286 * t627 * t52 + f64x8::splat(2.0) / f64x8::splat(243.0) * t648 * t618 * t622 * t52;
            let t658 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t578 * t55 - f64x8::splat(3.0) / f64x8::splat(8.0) * t584 - f64x8::splat(9.0) / f64x8::splat(8.0) * t5 * t235 * t130 + t591 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t594 - f64x8::splat(9.0) / f64x8::splat(8.0) * t5 * t102 * t291 - t604 + t606 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t609 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t653));
            let t660 = f64x8::splat(1.0) / t297 / t64;
            let t661 = t299 * t138;
            let t664 = t298 * t138;
            let t667 = t61 * t569;
            let t670 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -f64x8::splat(6.0) * t226 - f64x8::splat(6.0) * t667)));
            let t674 = ((t65).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t660 * t661 + f64x8::splat(4.0) / f64x8::splat(3.0) * t664 * t305 + f64x8::splat(4.0) / f64x8::splat(3.0) * t66 * t670));
            let t675 = t674 * t26;
            let t679 = t309 * t107;
            let t681 = t5 * t679 * t89;
            let t683 = t141 * t247;
            let t685 = t5 * t683 * t89;
            let t687 = t68 * t600;
            let t690 = f64x8::splat(5.0) / f64x8::splat(36.0) * t5 * t687 * t89;
            let t692 = ((t60).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t675 * t89 - f64x8::splat(3.0) / f64x8::splat(8.0) * t681 + t685 / f64x8::splat(4.0) - t690));
            let tv3rho30 = f64x8::splat(3.0) * t296 + f64x8::splat(3.0) * t323 + t6 * (t658 + t692);
            acc_v3rho3_0 = tv3rho30;
            let t695 = f64x8::splat(2.0) * t349;
            let t696 = f64x8::splat(2.0) * t375;
            let t697 = t561 * t155;
            let t700 = t221 * t330;
            let t705 = f64x8::splat(2.0) * t226;
            let t706 = f64x8::splat(6.0) * t570;
            let t708 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t705 - t706)));
            let t712 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t697 * t222 + f64x8::splat(8.0) / f64x8::splat(9.0) * t700 * t98 + f64x8::splat(4.0) / f64x8::splat(9.0) * t326 * t230 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t708));
            let t713 = t712 * t26;
            let t717 = t334 * t107;
            let t720 = t5 * t717 * t55 / f64x8::splat(4.0);
            let t724 = t158 * t247;
            let t726 = t5 * t724 * t55;
            let t730 = t5 * t339 * t130 / f64x8::splat(4.0);
            let t739 = -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t713 * t55 - t720 - f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t335 * t130 + t726 / f64x8::splat(12.0) - t730 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t159 * t291 - t584 / f64x8::splat(8.0) + t591 / f64x8::splat(6.0) - t594 / f64x8::splat(4.0) - t604 + t606 / f64x8::splat(6.0) - t609 / f64x8::splat(8.0);
            let t740 = ((t1).select(f64x8::splat(0.0), t739));
            let t741 = t660 * t166;
            let t744 = t298 * t354;
            let t749 = f64x8::splat(6.0) * t667;
            let t751 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -t705 - t749)));
            let t755 = ((t65).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t741 * t299 + f64x8::splat(8.0) / f64x8::splat(9.0) * t744 * t138 + f64x8::splat(4.0) / f64x8::splat(9.0) * t350 * t305 + f64x8::splat(4.0) / f64x8::splat(3.0) * t66 * t751));
            let t756 = t755 * t26;
            let t760 = t358 * t107;
            let t763 = t5 * t760 * t89 / f64x8::splat(4.0);
            let t764 = t169 * t247;
            let t766 = t5 * t764 * t89;
            let t775 = t5 * t314 * t188 / f64x8::splat(4.0);
            let t777 = t5 * t318 * t188;
            let t780 = ((t60).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t756 * t89 - t763 + t766 / f64x8::splat(12.0) - t681 / f64x8::splat(8.0) + t685 / f64x8::splat(6.0) - t690 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t310 * t188 - t775 + t777 / f64x8::splat(12.0)));
            let tv3rho31 = t296 + t323 + t695 + t696 + t6 * (t740 + t780);
            acc_v3rho3_1 = tv3rho31;
            let t783 = t561 * t380;
            let t788 = t221 * t385;
            let t792 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -t705 - t706)));
            let t796 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t783 * t98 + f64x8::splat(8.0) / f64x8::splat(9.0) * t326 * t330 + f64x8::splat(4.0) / f64x8::splat(9.0) * t788 * t98 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t792));
            let t797 = t796 * t26;
            let t801 = t389 * t107;
            let t803 = t5 * t801 * t55;
            let t812 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t797 * t55 - t803 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t390 * t130 - t720 + t726 / f64x8::splat(6.0) - t730 + t591 / f64x8::splat(12.0) - t604 + t606 / f64x8::splat(12.0)));
            let t813 = t660 * t397;
            let t818 = t298 * t402;
            let t822 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), t705 - t749)));
            let t826 = ((t65).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t813 * t138 + f64x8::splat(8.0) / f64x8::splat(9.0) * t350 * t354 + f64x8::splat(4.0) / f64x8::splat(9.0) * t818 * t138 + f64x8::splat(4.0) / f64x8::splat(3.0) * t66 * t822));
            let t827 = t826 * t26;
            let t831 = t406 * t107;
            let t833 = t5 * t831 * t89;
            let t840 = t5 * t363 * t188;
            let t848 = t5 * t146 * t444;
            let t850 = -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t827 * t89 - t833 / f64x8::splat(8.0) - t763 + t766 / f64x8::splat(6.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t359 * t188 - t840 / f64x8::splat(4.0) + t685 / f64x8::splat(12.0) - t690 - t775 + t777 / f64x8::splat(6.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t142 * t444 - t848 / f64x8::splat(8.0);
            let t851 = ((t60).select(f64x8::splat(0.0), t850));
            let tv3rho32 = t695 + t696 + t396 + t449 + t6 * (t812 + t851);
            acc_v3rho3_2 = tv3rho32;
            let t856 = t380 * t155;
            let t863 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -f64x8::splat(6.0) * t226 - f64x8::splat(6.0) * t570)));
            let t867 = ((t20).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t561 * t856 + f64x8::splat(4.0) / f64x8::splat(3.0) * t326 * t385 + f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t863));
            let t868 = t867 * t26;
            let t875 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t868 * t55 - f64x8::splat(3.0) / f64x8::splat(8.0) * t803 + t726 / f64x8::splat(4.0) - t604));
            let t876 = t397 * t166;
            let t883 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), f64x8::splat(6.0) * t226 - f64x8::splat(6.0) * t667)));
            let t887 = ((t65).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t660 * t876 + f64x8::splat(4.0) / f64x8::splat(3.0) * t350 * t402 + f64x8::splat(4.0) / f64x8::splat(3.0) * t66 * t883));
            let t888 = t887 * t26;
            let t903 = t174 * t174;
            let t905 = t112 / t903;
            let t906 = t905 * t614;
            let t907 = t420 * v_sigma2;
            let t908 = t617 * t907;
            let t909 = t421 * t421;
            let t910 = t909 * t178;
            let t911 = f64x8::splat(1.0) / t910;
            let t916 = f64x8::splat(1.0) / t72 / t909;
            let t922 = f64x8::splat(1.0) / t73 / t543;
            let t939 = -f64x8::splat(4.0) / f64x8::splat(81.0) * t906 * t908 * t911 + f64x8::splat(22.0) / f64x8::splat(81.0) * t419 * t263 * t420 * t916 - f64x8::splat(154.0) / f64x8::splat(81.0) * t177 * t117 * v_sigma2 * t922 + f64x8::splat(154.0) / f64x8::splat(81.0) * t125 * t70 * t922 * t86 - f64x8::splat(11.0) / f64x8::splat(81.0) * t285 * t439 * t916 * t86 + f64x8::splat(2.0) / f64x8::splat(243.0) * t648 * t907 * t911 * t86;
            let t944 = ((t60).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t888 * t89 - f64x8::splat(3.0) / f64x8::splat(8.0) * t833 - f64x8::splat(9.0) / f64x8::splat(8.0) * t5 * t407 * t188 + t766 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t840 - f64x8::splat(9.0) / f64x8::splat(8.0) * t5 * t170 * t444 - t690 + t777 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t848 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t69 * t939));
            let tv3rho33 = f64x8::splat(3.0) * t396 + f64x8::splat(3.0) * t449 + t6 * (t875 + t944);
            acc_v3rho3_3 = tv3rho33;
            let t952 = t5 * t239 * t202;
            let t959 = t5 * t248 * t202 / f64x8::splat(12.0);
            let t961 = t5 * t108 * t477;
            let t963 = t620 * t35;
            let t964 = f64x8::splat(1.0) / t963;
            let t965 = t617 * t964;
            let t980 = t262 * t268;
            let t988 = t615 * t965 * t264 / f64x8::splat(54.0) - t259 * t263 * t268 * v_sigma0 / f64x8::splat(12.0) + f64x8::splat(11.0) / f64x8::splat(27.0) * t116 * t117 * t274 - f64x8::splat(11.0) / f64x8::splat(27.0) * t125 * t33 * t274 * t52 + t285 * t980 * t473 / f64x8::splat(24.0) - t648 * t964 * t264 * t52 / f64x8::splat(324.0);
            let t993 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t235 * t202 - t952 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t102 * t477 + t959 - t961 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t988));
            let tv3rho2sigma0 = t6 * t993 + f64x8::splat(2.0) * t482;
            acc_v3rho2sigma_0 = tv3rho2sigma0;
            let tv3rho2sigma1 = f64x8::splat(0.0);
            acc_v3rho2sigma_1 = tv3rho2sigma1;
            let t1000 = t5 * t314 * t213;
            let t1004 = t5 * t318 * t213 / f64x8::splat(12.0);
            let t1006 = ((t60).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t310 * t213 - t1000 / f64x8::splat(4.0) + t1004));
            let tv3rho2sigma2 = t6 * t1006 + f64x8::splat(2.0) * t491;
            acc_v3rho2sigma_2 = tv3rho2sigma2;
            let t1012 = t5 * t339 * t202;
            let t1020 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t335 * t202 - t1012 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t159 * t477 - t952 / f64x8::splat(8.0) + t959 - t961 / f64x8::splat(8.0)));
            let tv3rho2sigma3 = t6 * t1020 + t482 + t497;
            acc_v3rho2sigma_3 = tv3rho2sigma3;
            let tv3rho2sigma4 = f64x8::splat(0.0);
            acc_v3rho2sigma_4 = tv3rho2sigma4;
            let t1026 = t5 * t363 * t213;
            let t1033 = t5 * t146 * t521;
            let t1036 = ((t60).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t359 * t213 - t1026 / f64x8::splat(8.0) - t1000 / f64x8::splat(8.0) + t1004 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t142 * t521 - t1033 / f64x8::splat(8.0)));
            let tv3rho2sigma5 = t6 * t1036 + t491 + t526;
            acc_v3rho2sigma_5 = tv3rho2sigma5;
            let t1044 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t390 * t202 - t1012 / f64x8::splat(4.0) + t959));
            let tv3rho2sigma6 = t6 * t1044 + f64x8::splat(2.0) * t497;
            acc_v3rho2sigma_6 = tv3rho2sigma6;
            let tv3rho2sigma7 = f64x8::splat(0.0);
            acc_v3rho2sigma_7 = tv3rho2sigma7;
            let t1055 = t909 * t71;
            let t1056 = f64x8::splat(1.0) / t1055;
            let t1057 = t617 * t1056;
            let t1072 = t262 * t424;
            let t1080 = t906 * t1057 * t420 / f64x8::splat(54.0) - t419 * t263 * t424 * v_sigma2 / f64x8::splat(12.0) + f64x8::splat(11.0) / f64x8::splat(27.0) * t177 * t117 * t430 - f64x8::splat(11.0) / f64x8::splat(27.0) * t125 * t33 * t430 * t86 + t285 * t1072 * t517 / f64x8::splat(24.0) - t648 * t1056 * t420 * t86 / f64x8::splat(324.0);
            let t1085 = ((t60).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t407 * t213 - t1026 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t170 * t521 + t1004 - t1033 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t69 * t1080));
            let tv3rho2sigma8 = t6 * t1085 + f64x8::splat(2.0) * t526;
            acc_v3rho2sigma_8 = tv3rho2sigma8;
            let t1092 = t5 * t108 * t538 / f64x8::splat(8.0);
            let t1093 = t620 * v_rho0;
            let t1094 = f64x8::splat(1.0) / t1093;
            let t1095 = t617 * t1094;
            let t1109 = -t615 * t1095 * v_sigma0 / f64x8::splat(144.0) + t259 * t263 * t460 / f64x8::splat(54.0) - t285 * t472 * t52 / f64x8::splat(108.0) + t648 * t1094 * v_sigma0 * t52 / f64x8::splat(864.0);
            let t1114 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t102 * t538 - t1092 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t1109));
            let tv3rhosigma20 = t6 * t1114 + t542;
            acc_v3rhosigma2_0 = tv3rhosigma20;
            let tv3rhosigma21 = f64x8::splat(0.0);
            acc_v3rhosigma2_1 = tv3rhosigma21;
            let tv3rhosigma22 = f64x8::splat(0.0);
            acc_v3rhosigma2_2 = tv3rhosigma22;
            let tv3rhosigma23 = f64x8::splat(0.0);
            acc_v3rhosigma2_3 = tv3rhosigma23;
            let tv3rhosigma24 = f64x8::splat(0.0);
            acc_v3rhosigma2_4 = tv3rhosigma24;
            let t1121 = t5 * t146 * t553 / f64x8::splat(8.0);
            let t1123 = ((t60).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t142 * t553 - t1121));
            let tv3rhosigma25 = t6 * t1123 + t557;
            acc_v3rhosigma2_5 = tv3rhosigma25;
            let t1129 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t159 * t538 - t1092));
            let tv3rhosigma26 = t6 * t1129 + t542;
            acc_v3rhosigma2_6 = tv3rhosigma26;
            let tv3rhosigma27 = f64x8::splat(0.0);
            acc_v3rhosigma2_7 = tv3rhosigma27;
            let tv3rhosigma28 = f64x8::splat(0.0);
            acc_v3rhosigma2_8 = tv3rhosigma28;
            let tv3rhosigma29 = f64x8::splat(0.0);
            acc_v3rhosigma2_9 = tv3rhosigma29;
            let tv3rhosigma210 = f64x8::splat(0.0);
            acc_v3rhosigma2_10 = tv3rhosigma210;
            let t1134 = t909 * v_rho1;
            let t1135 = f64x8::splat(1.0) / t1134;
            let t1136 = t617 * t1135;
            let t1150 = -t906 * t1136 * v_sigma2 / f64x8::splat(144.0) + t419 * t263 * t504 / f64x8::splat(54.0) - t285 * t516 * t86 / f64x8::splat(108.0) + t648 * t1135 * v_sigma2 * t86 / f64x8::splat(864.0);
            let t1155 = ((t60).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t170 * t553 - t1121 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t69 * t1150));
            let tv3rhosigma211 = t6 * t1155 + t557;
            acc_v3rhosigma2_11 = tv3rhosigma211;
            let t1157 = t614 * t617;
            let t1158 = f64x8::splat(1.0) / t620;
            let t1166 = t613 * t1157 * t1158 / f64x8::splat(384.0) - t647 * t617 * t1158 * t52 / f64x8::splat(2304.0);
            let t1170 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t1166));
            let tv3sigma30 = t6 * t1170;
            acc_v3sigma3_0 = tv3sigma30;
            let tv3sigma31 = f64x8::splat(0.0);
            acc_v3sigma3_1 = tv3sigma31;
            let tv3sigma32 = f64x8::splat(0.0);
            acc_v3sigma3_2 = tv3sigma32;
            let tv3sigma33 = f64x8::splat(0.0);
            acc_v3sigma3_3 = tv3sigma33;
            let tv3sigma34 = f64x8::splat(0.0);
            acc_v3sigma3_4 = tv3sigma34;
            let tv3sigma35 = f64x8::splat(0.0);
            acc_v3sigma3_5 = tv3sigma35;
            let tv3sigma36 = f64x8::splat(0.0);
            acc_v3sigma3_6 = tv3sigma36;
            let tv3sigma37 = f64x8::splat(0.0);
            acc_v3sigma3_7 = tv3sigma37;
            let tv3sigma38 = f64x8::splat(0.0);
            acc_v3sigma3_8 = tv3sigma38;
            let t1171 = f64x8::splat(1.0) / t909;
            let t1179 = t905 * t1157 * t1171 / f64x8::splat(384.0) - t647 * t617 * t1171 * t86 / f64x8::splat(2304.0);
            let t1183 = ((t60).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t69 * t1179));
            let tv3sigma39 = t6 * t1183;
            acc_v3sigma3_9 = tv3sigma39;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(vsigma, ip, m, 3, 0, acc_vsigma_0);
        store_strided(vsigma, ip, m, 3, 1, acc_vsigma_1);
        store_strided(vsigma, ip, m, 3, 2, acc_vsigma_2);
        store_strided(v2rho2, ip, m, 3, 0, acc_v2rho2_0);
        store_strided(v2rho2, ip, m, 3, 1, acc_v2rho2_1);
        store_strided(v2rho2, ip, m, 3, 2, acc_v2rho2_2);
        store_strided(v2rhosigma, ip, m, 6, 0, acc_v2rhosigma_0);
        store_strided(v2rhosigma, ip, m, 6, 1, acc_v2rhosigma_1);
        store_strided(v2rhosigma, ip, m, 6, 2, acc_v2rhosigma_2);
        store_strided(v2rhosigma, ip, m, 6, 3, acc_v2rhosigma_3);
        store_strided(v2rhosigma, ip, m, 6, 4, acc_v2rhosigma_4);
        store_strided(v2rhosigma, ip, m, 6, 5, acc_v2rhosigma_5);
        store_strided(v2sigma2, ip, m, 6, 0, acc_v2sigma2_0);
        store_strided(v2sigma2, ip, m, 6, 1, acc_v2sigma2_1);
        store_strided(v2sigma2, ip, m, 6, 2, acc_v2sigma2_2);
        store_strided(v2sigma2, ip, m, 6, 3, acc_v2sigma2_3);
        store_strided(v2sigma2, ip, m, 6, 4, acc_v2sigma2_4);
        store_strided(v2sigma2, ip, m, 6, 5, acc_v2sigma2_5);
        store_strided(v3rho3, ip, m, 4, 0, acc_v3rho3_0);
        store_strided(v3rho3, ip, m, 4, 1, acc_v3rho3_1);
        store_strided(v3rho3, ip, m, 4, 2, acc_v3rho3_2);
        store_strided(v3rho3, ip, m, 4, 3, acc_v3rho3_3);
        store_strided(v3rho2sigma, ip, m, 9, 0, acc_v3rho2sigma_0);
        store_strided(v3rho2sigma, ip, m, 9, 1, acc_v3rho2sigma_1);
        store_strided(v3rho2sigma, ip, m, 9, 2, acc_v3rho2sigma_2);
        store_strided(v3rho2sigma, ip, m, 9, 3, acc_v3rho2sigma_3);
        store_strided(v3rho2sigma, ip, m, 9, 4, acc_v3rho2sigma_4);
        store_strided(v3rho2sigma, ip, m, 9, 5, acc_v3rho2sigma_5);
        store_strided(v3rho2sigma, ip, m, 9, 6, acc_v3rho2sigma_6);
        store_strided(v3rho2sigma, ip, m, 9, 7, acc_v3rho2sigma_7);
        store_strided(v3rho2sigma, ip, m, 9, 8, acc_v3rho2sigma_8);
        store_strided(v3rhosigma2, ip, m, 12, 0, acc_v3rhosigma2_0);
        store_strided(v3rhosigma2, ip, m, 12, 1, acc_v3rhosigma2_1);
        store_strided(v3rhosigma2, ip, m, 12, 2, acc_v3rhosigma2_2);
        store_strided(v3rhosigma2, ip, m, 12, 3, acc_v3rhosigma2_3);
        store_strided(v3rhosigma2, ip, m, 12, 4, acc_v3rhosigma2_4);
        store_strided(v3rhosigma2, ip, m, 12, 5, acc_v3rhosigma2_5);
        store_strided(v3rhosigma2, ip, m, 12, 6, acc_v3rhosigma2_6);
        store_strided(v3rhosigma2, ip, m, 12, 7, acc_v3rhosigma2_7);
        store_strided(v3rhosigma2, ip, m, 12, 8, acc_v3rhosigma2_8);
        store_strided(v3rhosigma2, ip, m, 12, 9, acc_v3rhosigma2_9);
        store_strided(v3rhosigma2, ip, m, 12, 10, acc_v3rhosigma2_10);
        store_strided(v3rhosigma2, ip, m, 12, 11, acc_v3rhosigma2_11);
        store_strided(v3sigma3, ip, m, 10, 0, acc_v3sigma3_0);
        store_strided(v3sigma3, ip, m, 10, 1, acc_v3sigma3_1);
        store_strided(v3sigma3, ip, m, 10, 2, acc_v3sigma3_2);
        store_strided(v3sigma3, ip, m, 10, 3, acc_v3sigma3_3);
        store_strided(v3sigma3, ip, m, 10, 4, acc_v3sigma3_4);
        store_strided(v3sigma3, ip, m, 10, 5, acc_v3sigma3_5);
        store_strided(v3sigma3, ip, m, 10, 6, acc_v3sigma3_6);
        store_strided(v3sigma3, ip, m, 10, 7, acc_v3sigma3_7);
        store_strided(v3sigma3, ip, m, 10, 8, acc_v3sigma3_8);
        store_strided(v3sigma3, ip, m, 10, 9, acc_v3sigma3_9);
        ip += 8;
    }
}

//! GGA_X_LSPBE lxc unpol kernel — explicit SIMD (bit-exact).
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
pub fn gga_x_lspbe_lxc_unpol(
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
    v4rho4: &mut [f64],
    v4rho3sigma: &mut [f64],
    v4rho2sigma2: &mut [f64],
    v4rhosigma3: &mut [f64],
    v4sigma4: &mut [f64],
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
        let mut acc_v4rho4 = V_ZERO;
        let mut acc_v4rho3sigma = V_ZERO;
        let mut acc_v4rho2sigma2 = V_ZERO;
        let mut acc_v4rhosigma3 = V_ZERO;
        let mut acc_v4sigma4 = V_ZERO;
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
            let t21 = param_mu * t20;
            let t22 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t23 = (simd::cbrt(t22));
            let t24 = t23 * t23;
            let t25 = f64x8::splat(1.0) / t24;
            let t27 = f64x8::splat(M_CBRT2);
            let t28 = t27 * t27;
            let t29 = v_sigma * t28;
            let t30 = v_rho * v_rho;
            let t31 = t18 * t18;
            let t33 = f64x8::splat(1.0) / t31 / t30;
            let t34 = t29 * t33;
            let t37 = param_kappa + t21 * t25 * t34 / f64x8::splat(24.0);
            let t42 = param_kappa + f64x8::splat(1.0);
            let t47 = (simd::exp(-param_alpha * t20 * t25 * t34 / f64x8::splat(24.0)));
            let t50 = f64x8::splat(1.0) + param_kappa * (f64x8::splat(1.0) - param_kappa / t37) - t42 * (f64x8::splat(1.0) - t47);
            let t54 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t50));
            let tzk0 = f64x8::splat(2.0) * t54;
            acc_zk = tzk0;
            let t56 = t17 / t31;
            let t60 = param_kappa * param_kappa;
            let t61 = t37 * t37;
            let t63 = t60 / t61;
            let t64 = t63 * t21;
            let t65 = t25 * v_sigma;
            let t66 = t30 * v_rho;
            let t68 = f64x8::splat(1.0) / t31 / t66;
            let t69 = t28 * t68;
            let t72 = t42 * param_alpha;
            let t73 = t20 * t25;
            let t74 = t72 * t73;
            let t75 = t68 * t47;
            let t79 = t74 * t29 * t75 / f64x8::splat(9.0) - t64 * t65 * t69 / f64x8::splat(9.0);
            let t84 = ((t2).select(f64x8::splat(0.0), -t6 * t56 * t50 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t79));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t84 + f64x8::splat(2.0) * t54;
            acc_vrho = tvrho0;
            let t87 = t63 * param_mu;
            let t91 = t72 * t20;
            let t92 = t25 * t28;
            let t97 = t87 * t73 * t28 * t33 / f64x8::splat(24.0) - t91 * t92 * t33 * t47 / f64x8::splat(24.0);
            let t101 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t97));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t101;
            acc_vsigma = tvsigma0;
            let t106 = t17 / t31 / v_rho;
            let t115 = t60 / t61 / t37;
            let t116 = param_mu * param_mu;
            let t117 = t20 * t20;
            let t119 = t115 * t116 * t117;
            let t121 = f64x8::splat(1.0) / t23 / t22;
            let t122 = v_sigma * v_sigma;
            let t123 = t121 * t122;
            let t124 = t30 * t30;
            let t127 = f64x8::splat(1.0) / t18 / t124 / t66;
            let t128 = t27 * t127;
            let t133 = f64x8::splat(1.0) / t31 / t124;
            let t134 = t28 * t133;
            let t138 = t133 * t47;
            let t142 = param_alpha * param_alpha;
            let t143 = t42 * t142;
            let t144 = t117 * t121;
            let t145 = t143 * t144;
            let t146 = t122 * t27;
            let t147 = t127 * t47;
            let t151 = -f64x8::splat(4.0) / f64x8::splat(81.0) * t119 * t123 * t128 + f64x8::splat(11.0) / f64x8::splat(27.0) * t64 * t65 * t134 - f64x8::splat(11.0) / f64x8::splat(27.0) * t74 * t29 * t138 + f64x8::splat(2.0) / f64x8::splat(81.0) * t145 * t146 * t147;
            let t156 = ((t2).select(f64x8::splat(0.0), t6 * t106 * t50 / f64x8::splat(12.0) - t6 * t56 * t79 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t151));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t156 + f64x8::splat(4.0) * t84;
            acc_v2rho2 = tv2rho20;
            let t162 = t121 * t27;
            let t163 = t124 * t30;
            let t165 = f64x8::splat(1.0) / t18 / t163;
            let t176 = t27 * t165;
            let t177 = v_sigma * t47;
            let t181 = t119 * t162 * t165 * v_sigma / f64x8::splat(54.0) - t87 * t73 * t69 / f64x8::splat(9.0) + t91 * t92 * t75 / f64x8::splat(9.0) - t145 * t176 * t177 / f64x8::splat(108.0);
            let t186 = ((t2).select(f64x8::splat(0.0), -t6 * t56 * t97 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t181));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t186 + f64x8::splat(2.0) * t101;
            acc_v2rhosigma = tv2rhosigma0;
            let t189 = t115 * t116;
            let t190 = t124 * v_rho;
            let t192 = f64x8::splat(1.0) / t18 / t190;
            let t197 = t143 * t117;
            let t202 = -t189 * t144 * t27 * t192 / f64x8::splat(144.0) + t197 * t162 * t192 * t47 / f64x8::splat(288.0);
            let t206 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t202));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t206;
            acc_v2sigma2 = tv2sigma20;
            let t209 = t17 * t33;
            let t219 = t61 * t61;
            let t221 = t60 / t219;
            let t222 = t116 * param_mu;
            let t223 = t221 * t222;
            let t224 = t22 * t22;
            let t225 = f64x8::splat(1.0) / t224;
            let t226 = t122 * v_sigma;
            let t227 = t225 * t226;
            let t228 = t124 * t124;
            let t229 = t228 * t66;
            let t230 = f64x8::splat(1.0) / t229;
            let t235 = f64x8::splat(1.0) / t18 / t228;
            let t236 = t27 * t235;
            let t241 = f64x8::splat(1.0) / t31 / t190;
            let t242 = t28 * t241;
            let t246 = t241 * t47;
            let t255 = t42 * t142 * param_alpha;
            let t256 = t255 * t225;
            let t261 = -f64x8::splat(16.0) / f64x8::splat(81.0) * t223 * t227 * t230 + f64x8::splat(44.0) / f64x8::splat(81.0) * t119 * t123 * t236 - f64x8::splat(154.0) / f64x8::splat(81.0) * t64 * t65 * t242 + f64x8::splat(154.0) / f64x8::splat(81.0) * t74 * t29 * t246 - f64x8::splat(22.0) / f64x8::splat(81.0) * t145 * t146 * t235 * t47 + f64x8::splat(8.0) / f64x8::splat(243.0) * t256 * t226 * t230 * t47;
            let t266 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(5.0) / f64x8::splat(36.0) * t6 * t209 * t50 + t6 * t106 * t79 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t56 * t151 - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t261));
            let tv3rho30 = f64x8::splat(2.0) * v_rho * t266 + f64x8::splat(6.0) * t156;
            acc_v3rho3 = tv3rho30;
            let t276 = t228 * t30;
            let t277 = f64x8::splat(1.0) / t276;
            let t278 = t225 * t277;
            let t299 = f64x8::splat(2.0) / f64x8::splat(27.0) * t223 * t278 * t122 - t119 * t162 * t127 * v_sigma / f64x8::splat(6.0) + f64x8::splat(11.0) / f64x8::splat(27.0) * t87 * t73 * t134 - f64x8::splat(11.0) / f64x8::splat(27.0) * t91 * t92 * t138 + t145 * t128 * t177 / f64x8::splat(12.0) - t256 * t277 * t122 * t47 / f64x8::splat(81.0);
            let t304 = ((t2).select(f64x8::splat(0.0), t6 * t106 * t97 / f64x8::splat(12.0) - t6 * t56 * t181 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t299));
            let tv3rho2sigma0 = f64x8::splat(2.0) * v_rho * t304 + f64x8::splat(4.0) * t186;
            acc_v3rho2sigma = tv3rho2sigma0;
            let t310 = t228 * v_rho;
            let t311 = f64x8::splat(1.0) / t310;
            let t312 = t225 * t311;
            let t327 = -t223 * t312 * v_sigma / f64x8::splat(36.0) + t189 * t144 * t176 / f64x8::splat(27.0) - t197 * t162 * t165 * t47 / f64x8::splat(54.0) + t256 * t311 * v_sigma * t47 / f64x8::splat(216.0);
            let t332 = ((t2).select(f64x8::splat(0.0), -t6 * t56 * t202 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t327));
            let tv3rhosigma20 = f64x8::splat(2.0) * v_rho * t332 + f64x8::splat(2.0) * t206;
            acc_v3rhosigma2 = tv3rhosigma20;
            let t335 = t222 * t225;
            let t336 = f64x8::splat(1.0) / t228;
            let t344 = t221 * t335 * t336 / f64x8::splat(96.0) - t255 * t225 * t336 * t47 / f64x8::splat(576.0);
            let t348 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t344));
            let tv3sigma30 = f64x8::splat(2.0) * v_rho * t348;
            acc_v3sigma3 = tv3sigma30;
            let t367 = t116 * t116;
            let t369 = t60 / t219 / t37 * t367 * t225;
            let t370 = t122 * t122;
            let t373 = f64x8::splat(1.0) / t31 / t228 / t163;
            let t375 = t73 * t28;
            let t379 = t228 * t124;
            let t380 = f64x8::splat(1.0) / t379;
            let t385 = f64x8::splat(1.0) / t18 / t310;
            let t391 = f64x8::splat(1.0) / t31 / t163;
            let t408 = t142 * t142;
            let t409 = t42 * t408;
            let t413 = t92 * t47;
            let t422 = ((t2).select(f64x8::splat(0.0), f64x8::splat(10.0) / f64x8::splat(27.0) * t6 * t17 * t68 * t50 - f64x8::splat(5.0) / f64x8::splat(9.0) * t6 * t209 * t79 + t6 * t106 * t151 / f64x8::splat(2.0) - t6 * t56 * t261 / f64x8::splat(2.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * (-f64x8::splat(64.0) / f64x8::splat(729.0) * t369 * t370 * t373 * t375 + f64x8::splat(352.0) / f64x8::splat(81.0) * t223 * t227 * t380 - f64x8::splat(3916.0) / f64x8::splat(729.0) * t119 * t123 * t27 * t385 + f64x8::splat(2618.0) / f64x8::splat(243.0) * t64 * t65 * t28 * t391 - f64x8::splat(2618.0) / f64x8::splat(243.0) * t74 * t29 * t391 * t47 + f64x8::splat(1958.0) / f64x8::splat(729.0) * t145 * t146 * t385 * t47 - f64x8::splat(176.0) / f64x8::splat(243.0) * t256 * t226 * t380 * t47 + f64x8::splat(8.0) / f64x8::splat(2187.0) * t409 * t225 * t370 * t373 * t20 * t413)));
            let tv4rho40 = f64x8::splat(2.0) * v_rho * t422 + f64x8::splat(8.0) * t266;
            acc_v4rho4 = tv4rho40;
            let t437 = f64x8::splat(1.0) / t31 / t228 / t190;
            let t474 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(5.0) / f64x8::splat(36.0) * t6 * t209 * t97 + t6 * t106 * t181 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t56 * t299 - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * (f64x8::splat(8.0) / f64x8::splat(243.0) * t369 * t437 * t226 * t375 - f64x8::splat(38.0) / f64x8::splat(27.0) * t223 * t225 * t230 * t122 + f64x8::splat(341.0) / f64x8::splat(243.0) * t119 * t162 * t235 * v_sigma - f64x8::splat(154.0) / f64x8::splat(81.0) * t87 * t73 * t242 + f64x8::splat(154.0) / f64x8::splat(81.0) * t91 * t92 * t246 - f64x8::splat(341.0) / f64x8::splat(486.0) * t145 * t236 * t177 + f64x8::splat(19.0) / f64x8::splat(81.0) * t256 * t230 * t122 * t47 - t409 * t225 * t437 * t226 * t20 * t413 / f64x8::splat(729.0))));
            let tv4rho3sigma0 = f64x8::splat(2.0) * v_rho * t474 + f64x8::splat(6.0) * t304;
            acc_v4rho3sigma = tv4rho3sigma0;
            let t485 = f64x8::splat(1.0) / t31 / t379;
            let t514 = ((t2).select(f64x8::splat(0.0), t6 * t106 * t202 / f64x8::splat(12.0) - t6 * t56 * t327 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * (-t369 * t485 * t122 * t375 / f64x8::splat(81.0) + f64x8::splat(43.0) / f64x8::splat(108.0) * t223 * t278 * v_sigma - f64x8::splat(19.0) / f64x8::splat(81.0) * t189 * t144 * t128 + f64x8::splat(19.0) / f64x8::splat(162.0) * t197 * t162 * t147 - f64x8::splat(43.0) / f64x8::splat(648.0) * t256 * t277 * v_sigma * t47 + t409 * t225 * t485 * t122 * t20 * t413 / f64x8::splat(1944.0))));
            let tv4rho2sigma20 = f64x8::splat(2.0) * v_rho * t514 + f64x8::splat(4.0) * t332;
            acc_v4rho2sigma2 = tv4rho2sigma20;
            let t521 = f64x8::splat(1.0) / t31 / t229;
            let t544 = ((t2).select(f64x8::splat(0.0), -t6 * t56 * t344 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * (t369 * t521 * t20 * t65 * t28 / f64x8::splat(216.0) - t221 * t335 * t311 / f64x8::splat(12.0) + t255 * t312 * t47 / f64x8::splat(72.0) - t409 * t225 * t521 * t73 * t29 * t47 / f64x8::splat(5184.0))));
            let tv4rhosigma30 = f64x8::splat(2.0) * v_rho * t544 + f64x8::splat(2.0) * t348;
            acc_v4rhosigma3 = tv4rhosigma30;
            let t548 = f64x8::splat(1.0) / t31 / t276;
            let t563 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * (-t369 * t548 * t20 * t92 / f64x8::splat(576.0) + t409 * t225 * t548 * t73 * t28 * t47 / f64x8::splat(13824.0))));
            let tv4sigma40 = f64x8::splat(2.0) * v_rho * t563;
            acc_v4sigma4 = tv4sigma40;
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
        store_add(v4rho4, ip, m, acc_v4rho4);
        store_add(v4rho3sigma, ip, m, acc_v4rho3sigma);
        store_add(v4rho2sigma2, ip, m, acc_v4rho2sigma2);
        store_add(v4rhosigma3, ip, m, acc_v4rhosigma3);
        store_add(v4sigma4, ip, m, acc_v4sigma4);
        ip += 8;
    }
}

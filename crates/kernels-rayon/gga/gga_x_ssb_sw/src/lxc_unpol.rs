//! GGA_X_SSB_SW lxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_ssb_sw.c`
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
pub fn gga_x_ssb_sw_lxc_unpol(
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
    param_A: f64,
    param_B: f64,
    param_C: f64,
    param_D: f64,
    param_E: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_A = f64x8::splat(param_A);
    let param_B = f64x8::splat(param_B);
    let param_C = f64x8::splat(param_C);
    let param_D = f64x8::splat(param_D);
    let param_E = f64x8::splat(param_E);
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
            let t22 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t23 = (simd::cbrt(t22));
            let t24 = t23 * t23;
            let t25 = f64x8::splat(1.0) / t24;
            let t26 = param_B * t20 * t25;
            let t27 = f64x8::splat(M_CBRT2);
            let t28 = t27 * t27;
            let t29 = v_sigma * t28;
            let t30 = v_rho * v_rho;
            let t31 = t18 * t18;
            let t33 = f64x8::splat(1.0) / t31 / t30;
            let t39 = f64x8::splat(1.0) + param_C * t20 * t25 * t29 * t33 / f64x8::splat(24.0);
            let t40 = f64x8::splat(1.0) / t39;
            let t46 = param_D * t20 * t25;
            let t47 = t20 * t20;
            let t50 = f64x8::splat(1.0) / t23 / t22;
            let t52 = v_sigma * v_sigma;
            let t54 = t30 * t30;
            let t55 = t54 * v_rho;
            let t57 = f64x8::splat(1.0) / t18 / t55;
            let t61 = f64x8::splat(1.0) + param_E * t47 * t50 * t52 * t27 * t57 / f64x8::splat(288.0);
            let t62 = f64x8::splat(1.0) / t61;
            let t67 = param_A + t26 * t29 * t33 * t40 / f64x8::splat(24.0) - t46 * t29 * t33 * t62 / f64x8::splat(24.0);
            let t71 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t67));
            let tzk0 = f64x8::splat(2.0) * t71;
            acc_zk = tzk0;
            let t73 = t17 / t31;
            let t77 = t30 * v_rho;
            let t79 = f64x8::splat(1.0) / t31 / t77;
            let t84 = param_B * t47;
            let t86 = t84 * t50 * t52;
            let t87 = t54 * t30;
            let t89 = f64x8::splat(1.0) / t18 / t87;
            let t91 = t39 * t39;
            let t92 = f64x8::splat(1.0) / t91;
            let t93 = t92 * param_C;
            let t94 = t27 * t89 * t93;
            let t101 = t22 * t22;
            let t102 = f64x8::splat(1.0) / t101;
            let t103 = param_D * t102;
            let t104 = t52 * v_sigma;
            let t105 = t103 * t104;
            let t106 = t54 * t54;
            let t107 = t106 * v_rho;
            let t108 = f64x8::splat(1.0) / t107;
            let t109 = t61 * t61;
            let t110 = f64x8::splat(1.0) / t109;
            let t112 = t108 * t110 * param_E;
            let t115 = -t26 * t29 * t79 * t40 / f64x8::splat(9.0) + t86 * t94 / f64x8::splat(108.0) + t46 * t29 * t79 * t62 / f64x8::splat(9.0) - t105 * t112 / f64x8::splat(108.0);
            let t120 = ((t2).select(f64x8::splat(0.0), -t6 * t73 * t67 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t115));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t120 + f64x8::splat(2.0) * t71;
            acc_vrho = tvrho0;
            let t123 = t28 * t33;
            let t130 = t27 * t57 * t93;
            let t137 = f64x8::splat(1.0) / t106;
            let t139 = t137 * t110 * param_E;
            let t142 = t26 * t123 * t40 / f64x8::splat(24.0) - t84 * t50 * v_sigma * t130 / f64x8::splat(288.0) - t46 * t123 * t62 / f64x8::splat(24.0) + t103 * t52 * t139 / f64x8::splat(288.0);
            let t146 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t142));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t146;
            acc_vsigma = tvsigma0;
            let t151 = t17 / t31 / v_rho;
            let t159 = f64x8::splat(1.0) / t31 / t54;
            let t164 = t54 * t77;
            let t166 = f64x8::splat(1.0) / t18 / t164;
            let t168 = t27 * t166 * t93;
            let t171 = param_B * t102;
            let t172 = t171 * t104;
            let t173 = t106 * t30;
            let t174 = f64x8::splat(1.0) / t173;
            let t176 = f64x8::splat(1.0) / t91 / t39;
            let t178 = param_C * param_C;
            let t190 = t52 * t52;
            let t191 = t190 * v_sigma;
            let t194 = f64x8::splat(1.0) / t18 / t106 / t164;
            let t198 = f64x8::splat(1.0) / t109 / t61;
            let t199 = param_E * param_E;
            let t202 = t47 * t50 * t27;
            let t203 = t198 * t199 * t202;
            let t206 = f64x8::splat(11.0) / f64x8::splat(27.0) * t26 * t29 * t159 * t40 - t86 * t168 / f64x8::splat(12.0) + f64x8::splat(2.0) / f64x8::splat(81.0) * t172 * t174 * t176 * t178 - f64x8::splat(11.0) / f64x8::splat(27.0) * t46 * t29 * t159 * t62 + f64x8::splat(35.0) / f64x8::splat(324.0) * t105 * t174 * t110 * param_E - t103 * t191 * t194 * t203 / f64x8::splat(2916.0);
            let t211 = ((t2).select(f64x8::splat(0.0), t6 * t151 * t67 / f64x8::splat(12.0) - t6 * t73 * t115 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t206));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t211 + f64x8::splat(4.0) * t120;
            acc_v2rho2 = tv2rho20;
            let t217 = t28 * t79;
            let t222 = t84 * t50 * t27;
            let t224 = param_C * v_sigma;
            let t230 = t108 * t176 * t178;
            let t236 = t103 * t108;
            let t237 = t110 * param_E;
            let t238 = t237 * t52;
            let t241 = t106 * t87;
            let t243 = f64x8::splat(1.0) / t18 / t241;
            let t248 = -t26 * t217 * t40 / f64x8::splat(9.0) + t222 * t89 * t92 * t224 / f64x8::splat(36.0) - t171 * t52 * t230 / f64x8::splat(108.0) + t46 * t217 * t62 / f64x8::splat(9.0) - t236 * t238 / f64x8::splat(27.0) + t103 * t190 * t243 * t203 / f64x8::splat(7776.0);
            let t253 = ((t2).select(f64x8::splat(0.0), -t6 * t73 * t142 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t248));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t253 + f64x8::splat(2.0) * t146;
            acc_v2rhosigma = tv2rhosigma0;
            let t256 = t84 * t50;
            let t261 = t137 * t176 * t178;
            let t265 = t237 * v_sigma;
            let t268 = t106 * t55;
            let t270 = f64x8::splat(1.0) / t18 / t268;
            let t275 = -t256 * t130 / f64x8::splat(144.0) + t171 * v_sigma * t261 / f64x8::splat(288.0) + t103 * t137 * t265 / f64x8::splat(96.0) - t103 * t104 * t270 * t203 / f64x8::splat(20736.0);
            let t279 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t275));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t279;
            acc_v2sigma2 = tv2sigma20;
            let t282 = t17 * t33;
            let t293 = f64x8::splat(1.0) / t31 / t55;
            let t299 = f64x8::splat(1.0) / t18 / t106;
            let t304 = t106 * t77;
            let t305 = f64x8::splat(1.0) / t304;
            let t311 = f64x8::splat(1.0) / t31 / t268;
            let t314 = t91 * t91;
            let t315 = f64x8::splat(1.0) / t314;
            let t316 = t178 * param_C;
            let t319 = t20 * t25 * t28;
            let t320 = t315 * t316 * t319;
            let t331 = t106 * t106;
            let t333 = f64x8::splat(1.0) / t18 / t331;
            let t338 = t190 * t104;
            let t341 = f64x8::splat(1.0) / t31 / t331 / t55;
            let t344 = t109 * t109;
            let t345 = f64x8::splat(1.0) / t344;
            let t346 = t199 * param_E;
            let t351 = t20 / t24 / t101 * t28;
            let t352 = t345 * t346 * t351;
            let t355 = -f64x8::splat(154.0) / f64x8::splat(81.0) * t26 * t29 * t293 * t40 + f64x8::splat(341.0) / f64x8::splat(486.0) * t86 * t27 * t299 * t93 - f64x8::splat(38.0) / f64x8::splat(81.0) * t172 * t305 * t176 * t178 + f64x8::splat(2.0) / f64x8::splat(243.0) * t171 * t190 * t311 * t320 + f64x8::splat(154.0) / f64x8::splat(81.0) * t46 * t29 * t293 * t62 - f64x8::splat(569.0) / f64x8::splat(486.0) * t105 * t305 * t110 * param_E + t103 * t191 * t333 * t203 / f64x8::splat(108.0) - t103 * t338 * t341 * t352 / f64x8::splat(8748.0);
            let t360 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(5.0) / f64x8::splat(36.0) * t6 * t282 * t67 + t6 * t151 * t115 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t73 * t206 - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t355));
            let tv3rho30 = f64x8::splat(2.0) * v_rho * t360 + f64x8::splat(6.0) * t211;
            acc_v3rho3 = tv3rho30;
            let t370 = t28 * t159;
            let t378 = t171 * t174;
            let t379 = t176 * t178;
            let t380 = t379 * t52;
            let t383 = t106 * t54;
            let t385 = f64x8::splat(1.0) / t31 / t383;
            let t393 = t103 * t174;
            let t397 = t103 * t194 * t198;
            let t399 = t199 * t190 * t202;
            let t402 = t190 * t52;
            let t405 = f64x8::splat(1.0) / t31 / t331 / t54;
            let t410 = f64x8::splat(11.0) / f64x8::splat(27.0) * t26 * t370 * t40 - f64x8::splat(65.0) / f64x8::splat(324.0) * t222 * t166 * t92 * t224 + f64x8::splat(17.0) / f64x8::splat(108.0) * t378 * t380 - t171 * t104 * t385 * t320 / f64x8::splat(324.0) - f64x8::splat(11.0) / f64x8::splat(27.0) * t46 * t370 * t62 + f64x8::splat(29.0) / f64x8::splat(81.0) * t393 * t238 - f64x8::splat(25.0) / f64x8::splat(7776.0) * t397 * t399 + t103 * t402 * t405 * t352 / f64x8::splat(23328.0);
            let t415 = ((t2).select(f64x8::splat(0.0), t6 * t151 * t142 / f64x8::splat(12.0) - t6 * t73 * t248 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t410));
            let tv3rho2sigma0 = f64x8::splat(2.0) * v_rho * t415 + f64x8::splat(4.0) * t253;
            acc_v3rho2sigma = tv3rho2sigma0;
            let t424 = t379 * v_sigma;
            let t428 = f64x8::splat(1.0) / t31 / t304;
            let t436 = t103 * t243 * t198;
            let t438 = t199 * t104 * t202;
            let t443 = f64x8::splat(1.0) / t31 / t331 / t77;
            let t448 = t256 * t94 / f64x8::splat(27.0) - f64x8::splat(5.0) / f64x8::splat(108.0) * t171 * t108 * t424 + t171 * t52 * t428 * t320 / f64x8::splat(864.0) - t236 * t265 / f64x8::splat(12.0) + t436 * t438 / f64x8::splat(972.0) - t103 * t191 * t443 * t352 / f64x8::splat(62208.0);
            let t453 = ((t2).select(f64x8::splat(0.0), -t6 * t73 * t275 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t448));
            let tv3rhosigma20 = f64x8::splat(2.0) * v_rho * t453 + f64x8::splat(2.0) * t279;
            acc_v3rhosigma2 = tv3rhosigma20;
            let t459 = f64x8::splat(1.0) / t31 / t173;
            let t465 = t103 * t270 * t198;
            let t467 = t199 * t52 * t202;
            let t474 = f64x8::splat(1.0) / t31 / t331 / t30;
            let t479 = t171 * t261 / f64x8::splat(96.0) - t171 * v_sigma * t459 * t320 / f64x8::splat(2304.0) - t465 * t467 / f64x8::splat(3456.0) + t103 * t139 / f64x8::splat(96.0) + t103 * t190 * t474 * t352 / f64x8::splat(165888.0);
            let t483 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t479));
            let tv3sigma30 = f64x8::splat(2.0) * v_rho * t483;
            acc_v3sigma3 = tv3sigma30;
            let t500 = f64x8::splat(1.0) / t31 / t87;
            let t511 = f64x8::splat(1.0) / t383;
            let t525 = t191 / t18 / t331 / v_rho;
            let t529 = t178 * t178;
            let t531 = f64x8::splat(1.0) / t314 / t39 * t529 * t202;
            let t552 = t101 * t101;
            let t555 = param_D / t552 / t101;
            let t556 = t190 * t190;
            let t562 = f64x8::splat(1.0) / t344 / t61;
            let t564 = t199 * t199;
            let t573 = ((t2).select(f64x8::splat(0.0), f64x8::splat(10.0) / f64x8::splat(27.0) * t6 * t17 * t79 * t67 - f64x8::splat(5.0) / f64x8::splat(9.0) * t6 * t282 * t115 + t6 * t151 * t206 / f64x8::splat(2.0) - t6 * t73 * t355 / f64x8::splat(2.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * (f64x8::splat(2618.0) / f64x8::splat(243.0) * t26 * t29 * t500 * t40 - f64x8::splat(3047.0) / f64x8::splat(486.0) * t86 * t27 / t18 / t107 * t93 + f64x8::splat(5126.0) / f64x8::splat(729.0) * t172 * t511 * t176 * t178 - f64x8::splat(196.0) / f64x8::splat(729.0) * t171 * t190 / t31 / t241 * t320 + f64x8::splat(16.0) / f64x8::splat(2187.0) * t171 * t525 * t531 - f64x8::splat(2618.0) / f64x8::splat(243.0) * t46 * t29 * t500 * t62 + f64x8::splat(19393.0) / f64x8::splat(1458.0) * t105 * t511 * t110 * param_E - f64x8::splat(5107.0) / f64x8::splat(26244.0) * t103 * t525 * t203 + f64x8::splat(73.0) / f64x8::splat(13122.0) * t103 * t338 / t31 / t331 / t87 * t352 - f64x8::splat(2.0) / f64x8::splat(19683.0) * t555 * t556 * v_sigma / t331 / t383 * t562 * t564)));
            let tv4rho40 = f64x8::splat(2.0) * v_rho * t573 + f64x8::splat(8.0) * t360;
            acc_v4rho4 = tv4rho40;
            let t586 = t28 * t293;
            let t635 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(5.0) / f64x8::splat(36.0) * t6 * t282 * t142 + t6 * t151 * t248 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t73 * t410 - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * (-f64x8::splat(154.0) / f64x8::splat(81.0) * t26 * t586 * t40 + f64x8::splat(253.0) / f64x8::splat(162.0) * t222 * t299 * t92 * t224 - f64x8::splat(1025.0) / f64x8::splat(486.0) * t171 * t305 * t380 + f64x8::splat(89.0) / f64x8::splat(972.0) * t171 * t311 * t315 * t316 * t104 * t319 - f64x8::splat(2.0) / f64x8::splat(729.0) * t171 * t190 * t333 * t531 + f64x8::splat(154.0) / f64x8::splat(81.0) * t46 * t586 * t62 - f64x8::splat(892.0) / f64x8::splat(243.0) * t103 * t305 * t238 + f64x8::splat(2189.0) / f64x8::splat(34992.0) * t103 * t333 * t198 * t399 - f64x8::splat(137.0) / f64x8::splat(69984.0) * t103 * t341 * t345 * t346 * t402 * t351 + t555 * t556 / t331 / t304 * t562 * t564 / f64x8::splat(26244.0))));
            let tv4rho3sigma0 = f64x8::splat(2.0) * v_rho * t635 + f64x8::splat(6.0) * t415;
            acc_v4rho3sigma = tv4rho3sigma0;
            let t681 = ((t2).select(f64x8::splat(0.0), t6 * t151 * t275 / f64x8::splat(12.0) - t6 * t73 * t448 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * (-f64x8::splat(19.0) / f64x8::splat(81.0) * t256 * t168 + f64x8::splat(167.0) / f64x8::splat(324.0) * t378 * t424 - f64x8::splat(25.0) / f64x8::splat(864.0) * t171 * t385 * t315 * t316 * t52 * t319 + t171 * t104 * t194 * t531 / f64x8::splat(972.0) + f64x8::splat(3.0) / f64x8::splat(4.0) * t393 * t265 - f64x8::splat(13.0) / f64x8::splat(729.0) * t397 * t438 + f64x8::splat(41.0) / f64x8::splat(62208.0) * t103 * t405 * t345 * t346 * t191 * t351 - t555 * t338 / t331 / t173 * t562 * t564 / f64x8::splat(69984.0))));
            let tv4rho2sigma20 = f64x8::splat(2.0) * v_rho * t681 + f64x8::splat(4.0) * t453;
            acc_v4rho2sigma2 = tv4rho2sigma20;
            let t691 = t316 * t20;
            let t723 = ((t2).select(f64x8::splat(0.0), -t6 * t73 * t479 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * (-t171 * t230 / f64x8::splat(12.0) + f64x8::splat(7.0) / f64x8::splat(864.0) * t171 * t428 * t315 * t691 * t25 * v_sigma * t28 - t171 * t52 * t243 * t531 / f64x8::splat(2592.0) + f64x8::splat(11.0) / f64x8::splat(2592.0) * t436 * t467 - f64x8::splat(13.0) / f64x8::splat(62208.0) * t103 * t443 * t345 * t346 * t190 * t351 - t103 * t112 / f64x8::splat(12.0) + t555 * t402 / t331 / t107 * t562 * t564 / f64x8::splat(186624.0))));
            let tv4rhosigma30 = f64x8::splat(2.0) * v_rho * t723 + f64x8::splat(2.0) * t483;
            acc_v4rhosigma3 = tv4rhosigma30;
            let t757 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * (-t171 * t459 * t315 * t691 * t25 * t28 / f64x8::splat(576.0) + t171 * v_sigma * t270 * t531 / f64x8::splat(6912.0) + f64x8::splat(5.0) / f64x8::splat(82944.0) * t103 * t474 * t345 * t346 * t104 * t351 - f64x8::splat(5.0) / f64x8::splat(6912.0) * t465 * t199 * v_sigma * t202 - t555 * t191 / t331 / t106 * t562 * t564 / f64x8::splat(497664.0))));
            let tv4sigma40 = f64x8::splat(2.0) * v_rho * t757;
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

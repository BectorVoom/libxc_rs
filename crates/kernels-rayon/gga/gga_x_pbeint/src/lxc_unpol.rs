//! GGA_X_PBEINT lxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_pbeint.c`
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
pub fn gga_x_pbeint_lxc_unpol(
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
    param_muPBE: f64,
    param_muGE: f64,
    param_alpha: f64,
    param_kappa: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_muPBE = f64x8::splat(param_muPBE);
    let param_muGE = f64x8::splat(param_muGE);
    let param_alpha = f64x8::splat(param_alpha);
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
            let t20 = param_muPBE - param_muGE;
            let t21 = t20 * param_alpha;
            let t22 = f64x8::splat(M_CBRT6);
            let t23 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t24 = (simd::cbrt(t23));
            let t25 = t24 * t24;
            let t26 = f64x8::splat(1.0) / t25;
            let t27 = t22 * t26;
            let t28 = t21 * t27;
            let t29 = f64x8::splat(M_CBRT2);
            let t30 = t29 * t29;
            let t31 = v_sigma * t30;
            let t32 = v_rho * v_rho;
            let t33 = t18 * t18;
            let t35 = f64x8::splat(1.0) / t33 / t32;
            let t38 = t31 * t35;
            let t41 = f64x8::splat(1.0) + param_alpha * t22 * t26 * t38 / f64x8::splat(24.0);
            let t42 = f64x8::splat(1.0) / t41;
            let t43 = t35 * t42;
            let t48 = (param_muGE + t28 * t31 * t43 / f64x8::splat(24.0)) * t22;
            let t49 = t48 * t26;
            let t52 = param_kappa + t49 * t38 / f64x8::splat(24.0);
            let t57 = f64x8::splat(1.0) + param_kappa * (f64x8::splat(1.0) - param_kappa / t52);
            let t61 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t57));
            let tzk0 = f64x8::splat(2.0) * t61;
            acc_zk = tzk0;
            let t62 = f64x8::splat(1.0) / t33;
            let t63 = t17 * t62;
            let t67 = t6 * t17;
            let t68 = param_kappa * param_kappa;
            let t69 = t18 * t68;
            let t70 = t52 * t52;
            let t71 = f64x8::splat(1.0) / t70;
            let t72 = t32 * v_rho;
            let t74 = f64x8::splat(1.0) / t33 / t72;
            let t75 = t74 * t42;
            let t79 = param_alpha * param_alpha;
            let t80 = t20 * t79;
            let t81 = t22 * t22;
            let t83 = f64x8::splat(1.0) / t24 / t23;
            let t84 = t81 * t83;
            let t85 = t80 * t84;
            let t86 = v_sigma * v_sigma;
            let t87 = t86 * t29;
            let t88 = t32 * t32;
            let t89 = t88 * t32;
            let t91 = f64x8::splat(1.0) / t18 / t89;
            let t92 = t41 * t41;
            let t93 = f64x8::splat(1.0) / t92;
            let t94 = t91 * t93;
            let t99 = (-t28 * t31 * t75 / f64x8::splat(9.0) + t85 * t87 * t94 / f64x8::splat(108.0)) * t22;
            let t100 = t99 * t26;
            let t103 = t31 * t74;
            let t106 = t100 * t38 / f64x8::splat(24.0) - t49 * t103 / f64x8::splat(9.0);
            let t107 = t71 * t106;
            let t112 = ((t2).select(f64x8::splat(0.0), -t6 * t63 * t57 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t67 * t69 * t107));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t112 + f64x8::splat(2.0) * t61;
            acc_vrho = tvrho0;
            let t115 = t21 * t22;
            let t116 = t26 * t30;
            let t121 = t88 * v_rho;
            let t124 = f64x8::splat(1.0) / t18 / t121 * t93;
            let t129 = (t115 * t116 * t43 / f64x8::splat(24.0) - t85 * v_sigma * t29 * t124 / f64x8::splat(288.0)) * t22;
            let t130 = t129 * t26;
            let t132 = t116 * t35;
            let t135 = t130 * t38 / f64x8::splat(24.0) + t48 * t132 / f64x8::splat(24.0);
            let t136 = t71 * t135;
            let t140 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t67 * t69 * t136));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t140;
            acc_vsigma = tvsigma0;
            let t144 = f64x8::splat(1.0) / t33 / v_rho;
            let t145 = t17 * t144;
            let t149 = t62 * t68;
            let t154 = f64x8::splat(1.0) / t70 / t52;
            let t155 = t106 * t106;
            let t156 = t154 * t155;
            let t161 = f64x8::splat(1.0) / t33 / t88;
            let t162 = t161 * t42;
            let t166 = t88 * t72;
            let t168 = f64x8::splat(1.0) / t18 / t166;
            let t169 = t168 * t93;
            let t174 = t20 * t79 * param_alpha;
            let t175 = t23 * t23;
            let t176 = f64x8::splat(1.0) / t175;
            let t177 = t174 * t176;
            let t178 = t86 * v_sigma;
            let t179 = t88 * t88;
            let t180 = t179 * t32;
            let t181 = f64x8::splat(1.0) / t180;
            let t184 = f64x8::splat(1.0) / t92 / t41;
            let t189 = (f64x8::splat(11.0) / f64x8::splat(27.0) * t28 * t31 * t162 - t85 * t87 * t169 / f64x8::splat(12.0) + f64x8::splat(2.0) / f64x8::splat(81.0) * t177 * t178 * t181 * t184) * t22;
            let t190 = t189 * t26;
            let t195 = t31 * t161;
            let t198 = t190 * t38 / f64x8::splat(24.0) - f64x8::splat(2.0) / f64x8::splat(9.0) * t100 * t103 + f64x8::splat(11.0) / f64x8::splat(27.0) * t49 * t195;
            let t199 = t71 * t198;
            let t204 = ((t2).select(f64x8::splat(0.0), t6 * t145 * t57 / f64x8::splat(12.0) - t67 * t149 * t107 / f64x8::splat(4.0) + f64x8::splat(3.0) / f64x8::splat(4.0) * t67 * t69 * t156 - f64x8::splat(3.0) / f64x8::splat(8.0) * t67 * t69 * t199));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t204 + f64x8::splat(4.0) * t112;
            acc_v2rho2 = tv2rho20;
            let t210 = t6 * t19;
            let t211 = t68 * t154;
            let t212 = t135 * t106;
            let t213 = t211 * t212;
            let t220 = t93 * v_sigma;
            let t224 = t179 * v_rho;
            let t225 = f64x8::splat(1.0) / t224;
            let t231 = (-t115 * t116 * t75 / f64x8::splat(9.0) + t85 * t29 * t91 * t220 / f64x8::splat(36.0) - t177 * t86 * t225 * t184 / f64x8::splat(108.0)) * t22;
            let t232 = t231 * t26;
            let t239 = t116 * t74;
            let t242 = t232 * t38 / f64x8::splat(24.0) - t130 * t103 / f64x8::splat(9.0) + t99 * t132 / f64x8::splat(24.0) - t48 * t239 / f64x8::splat(9.0);
            let t243 = t71 * t242;
            let t248 = ((t2).select(f64x8::splat(0.0), -t67 * t149 * t136 / f64x8::splat(8.0) + f64x8::splat(3.0) / f64x8::splat(4.0) * t210 * t213 - f64x8::splat(3.0) / f64x8::splat(8.0) * t67 * t69 * t243));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t248 + f64x8::splat(2.0) * t140;
            acc_v2rhosigma = tv2rhosigma0;
            let t251 = t135 * t135;
            let t252 = t154 * t251;
            let t256 = t80 * t81;
            let t257 = t83 * t29;
            let t261 = f64x8::splat(1.0) / t179;
            let t267 = (-t256 * t257 * t124 / f64x8::splat(144.0) + t177 * v_sigma * t261 * t184 / f64x8::splat(288.0)) * t22;
            let t268 = t267 * t26;
            let t273 = t268 * t38 / f64x8::splat(24.0) + t129 * t132 / f64x8::splat(12.0);
            let t274 = t71 * t273;
            let t279 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(4.0) * t67 * t69 * t252 - f64x8::splat(3.0) / f64x8::splat(8.0) * t67 * t69 * t274));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t279;
            acc_v2sigma2 = tv2sigma20;
            let t286 = t144 * t68;
            let t296 = t70 * t70;
            let t297 = f64x8::splat(1.0) / t296;
            let t298 = t155 * t106;
            let t299 = t297 * t298;
            let t304 = t211 * t106 * t198;
            let t308 = f64x8::splat(1.0) / t33 / t121;
            let t309 = t308 * t42;
            let t314 = f64x8::splat(1.0) / t18 / t179;
            let t319 = t179 * t72;
            let t320 = f64x8::splat(1.0) / t319;
            let t325 = t79 * t79;
            let t326 = t20 * t325;
            let t327 = t86 * t86;
            let t328 = t176 * t327;
            let t329 = t326 * t328;
            let t330 = t179 * t121;
            let t332 = f64x8::splat(1.0) / t33 / t330;
            let t333 = t92 * t92;
            let t334 = f64x8::splat(1.0) / t333;
            let t336 = t27 * t30;
            let t341 = (-f64x8::splat(154.0) / f64x8::splat(81.0) * t28 * t31 * t309 + f64x8::splat(341.0) / f64x8::splat(486.0) * t85 * t87 * t314 * t93 - f64x8::splat(38.0) / f64x8::splat(81.0) * t177 * t178 * t320 * t184 + f64x8::splat(2.0) / f64x8::splat(243.0) * t329 * t332 * t334 * t336) * t22;
            let t342 = t341 * t26;
            let t349 = t31 * t308;
            let t352 = t342 * t38 / f64x8::splat(24.0) - t190 * t103 / f64x8::splat(3.0) + f64x8::splat(11.0) / f64x8::splat(9.0) * t100 * t195 - f64x8::splat(154.0) / f64x8::splat(81.0) * t49 * t349;
            let t353 = t71 * t352;
            let t358 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(5.0) / f64x8::splat(36.0) * t6 * t17 * t35 * t57 + t67 * t286 * t107 / f64x8::splat(4.0) + f64x8::splat(3.0) / f64x8::splat(4.0) * t67 * t149 * t156 - f64x8::splat(3.0) / f64x8::splat(8.0) * t67 * t149 * t199 - f64x8::splat(9.0) / f64x8::splat(4.0) * t67 * t69 * t299 + f64x8::splat(9.0) / f64x8::splat(4.0) * t210 * t304 - f64x8::splat(3.0) / f64x8::splat(8.0) * t67 * t69 * t353));
            let tv3rho30 = f64x8::splat(2.0) * v_rho * t358 + f64x8::splat(6.0) * t204;
            acc_v3rho3 = tv3rho30;
            let t365 = t6 * t63;
            let t371 = t68 * t297;
            let t373 = t371 * t135 * t155;
            let t377 = t211 * t242 * t106;
            let t381 = t211 * t135 * t198;
            let t391 = t181 * t184;
            let t395 = t176 * t178;
            let t397 = t179 * t88;
            let t399 = f64x8::splat(1.0) / t33 / t397;
            let t405 = (f64x8::splat(11.0) / f64x8::splat(27.0) * t115 * t116 * t162 - f64x8::splat(65.0) / f64x8::splat(324.0) * t85 * t29 * t168 * t220 + f64x8::splat(17.0) / f64x8::splat(108.0) * t177 * t391 * t86 - t326 * t395 * t399 * t334 * t336 / f64x8::splat(324.0)) * t22;
            let t406 = t405 * t26;
            let t417 = t116 * t161;
            let t420 = t406 * t38 / f64x8::splat(24.0) - f64x8::splat(2.0) / f64x8::splat(9.0) * t232 * t103 + f64x8::splat(11.0) / f64x8::splat(27.0) * t130 * t195 + t189 * t132 / f64x8::splat(24.0) - f64x8::splat(2.0) / f64x8::splat(9.0) * t99 * t239 + f64x8::splat(11.0) / f64x8::splat(27.0) * t48 * t417;
            let t421 = t71 * t420;
            let t426 = ((t2).select(f64x8::splat(0.0), t67 * t286 * t136 / f64x8::splat(12.0) + t365 * t213 / f64x8::splat(2.0) - t67 * t149 * t243 / f64x8::splat(4.0) - f64x8::splat(9.0) / f64x8::splat(4.0) * t210 * t373 + f64x8::splat(3.0) / f64x8::splat(2.0) * t210 * t377 + f64x8::splat(3.0) / f64x8::splat(4.0) * t210 * t381 - f64x8::splat(3.0) / f64x8::splat(8.0) * t67 * t69 * t421));
            let tv3rho2sigma0 = f64x8::splat(2.0) * v_rho * t426 + f64x8::splat(4.0) * t248;
            acc_v3rho2sigma = tv3rho2sigma0;
            let t433 = t371 * t251 * t106;
            let t437 = t211 * t135 * t242;
            let t444 = t211 * t273 * t106;
            let t454 = t176 * t86;
            let t457 = f64x8::splat(1.0) / t33 / t319;
            let t463 = (t256 * t257 * t94 / f64x8::splat(27.0) - f64x8::splat(5.0) / f64x8::splat(108.0) * t177 * t225 * t184 * v_sigma + t326 * t454 * t457 * t334 * t336 / f64x8::splat(864.0)) * t22;
            let t464 = t463 * t26;
            let t473 = t464 * t38 / f64x8::splat(24.0) - t268 * t103 / f64x8::splat(9.0) + t231 * t132 / f64x8::splat(12.0) - f64x8::splat(2.0) / f64x8::splat(9.0) * t129 * t239;
            let t474 = t71 * t473;
            let t479 = ((t2).select(f64x8::splat(0.0), t67 * t149 * t252 / f64x8::splat(4.0) - f64x8::splat(9.0) / f64x8::splat(4.0) * t210 * t433 + f64x8::splat(3.0) / f64x8::splat(2.0) * t210 * t437 - t67 * t149 * t274 / f64x8::splat(8.0) + f64x8::splat(3.0) / f64x8::splat(4.0) * t210 * t444 - f64x8::splat(3.0) / f64x8::splat(8.0) * t67 * t69 * t474));
            let tv3rhosigma20 = f64x8::splat(2.0) * v_rho * t479 + f64x8::splat(2.0) * t279;
            acc_v3rhosigma2 = tv3rhosigma20;
            let t482 = t251 * t135;
            let t483 = t297 * t482;
            let t487 = t135 * t273;
            let t488 = t211 * t487;
            let t495 = t176 * v_sigma;
            let t498 = f64x8::splat(1.0) / t33 / t180;
            let t504 = (t174 * t176 * t261 * t184 / f64x8::splat(96.0) - t326 * t495 * t498 * t334 * t336 / f64x8::splat(2304.0)) * t22;
            let t505 = t504 * t26;
            let t510 = t505 * t38 / f64x8::splat(24.0) + t267 * t132 / f64x8::splat(8.0);
            let t511 = t71 * t510;
            let t516 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(9.0) / f64x8::splat(4.0) * t67 * t69 * t483 + f64x8::splat(9.0) / f64x8::splat(4.0) * t210 * t488 - f64x8::splat(3.0) / f64x8::splat(8.0) * t67 * t69 * t511));
            let tv3sigma30 = f64x8::splat(2.0) * v_rho * t516;
            acc_v3sigma3 = tv3sigma30;
            let t523 = t35 * t68;
            let t541 = f64x8::splat(1.0) / t296 / t52;
            let t542 = t155 * t155;
            let t551 = t198 * t198;
            let t561 = f64x8::splat(1.0) / t33 / t89;
            let t577 = t179 * t89;
            let t585 = t20 * t325 * param_alpha;
            let t589 = t179 * t179;
            let t594 = f64x8::splat(1.0) / t333 / t41;
            let t596 = t84 * t29;
            let t619 = f64x8::splat(10.0) / f64x8::splat(27.0) * t6 * t17 * t74 * t57 - f64x8::splat(5.0) / f64x8::splat(9.0) * t67 * t523 * t107 - t67 * t286 * t156 + t67 * t286 * t199 / f64x8::splat(2.0) - f64x8::splat(3.0) * t67 * t149 * t299 + f64x8::splat(3.0) * t365 * t304 - t67 * t149 * t353 / f64x8::splat(2.0) + f64x8::splat(9.0) * t67 * t69 * t541 * t542 - f64x8::splat(27.0) / f64x8::splat(2.0) * t210 * t371 * t155 * t198 + f64x8::splat(9.0) / f64x8::splat(4.0) * t67 * t69 * t154 * t551 + f64x8::splat(3.0) * t210 * t211 * t106 * t352 - f64x8::splat(3.0) / f64x8::splat(8.0) * t67 * t69 * t71 * ((f64x8::splat(2618.0) / f64x8::splat(243.0) * t28 * t31 * t561 * t42 - f64x8::splat(3047.0) / f64x8::splat(486.0) * t85 * t87 / t18 / t224 * t93 + f64x8::splat(5126.0) / f64x8::splat(729.0) * t177 * t178 / t397 * t184 - f64x8::splat(196.0) / f64x8::splat(729.0) * t329 / t33 / t577 * t334 * t336 + f64x8::splat(16.0) / f64x8::splat(2187.0) * t585 * t176 * t327 * v_sigma / t18 / t589 / v_rho * t594 * t596) * t22 * t26 * t38 / f64x8::splat(24.0) - f64x8::splat(4.0) / f64x8::splat(9.0) * t342 * t103 + f64x8::splat(22.0) / f64x8::splat(9.0) * t190 * t195 - f64x8::splat(616.0) / f64x8::splat(81.0) * t100 * t349 + f64x8::splat(2618.0) / f64x8::splat(243.0) * t49 * t31 * t561);
            let t620 = ((t2).select(f64x8::splat(0.0), t619));
            let tv4rho40 = f64x8::splat(2.0) * v_rho * t620 + f64x8::splat(8.0) * t358;
            acc_v4rho4 = tv4rho40;
            let t707 = t68 * t541;
            let t716 = -f64x8::splat(27.0) / f64x8::splat(4.0) * t210 * t371 * t242 * t155 + f64x8::splat(9.0) / f64x8::splat(4.0) * t210 * t211 * t420 * t106 + f64x8::splat(9.0) / f64x8::splat(4.0) * t210 * t211 * t242 * t198 + f64x8::splat(3.0) / f64x8::splat(4.0) * t210 * t211 * t135 * t352 - f64x8::splat(5.0) / f64x8::splat(36.0) * t67 * t523 * t136 - t6 * t145 * t213 / f64x8::splat(2.0) + f64x8::splat(3.0) / f64x8::splat(2.0) * t365 * t377 + f64x8::splat(3.0) / f64x8::splat(4.0) * t365 * t381 + t67 * t286 * t243 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t67 * t149 * t421 - f64x8::splat(3.0) / f64x8::splat(8.0) * t67 * t69 * t71 * ((-f64x8::splat(154.0) / f64x8::splat(81.0) * t115 * t116 * t309 + f64x8::splat(253.0) / f64x8::splat(162.0) * t85 * t29 * t314 * t220 - f64x8::splat(1025.0) / f64x8::splat(486.0) * t177 * t320 * t184 * t86 + f64x8::splat(89.0) / f64x8::splat(972.0) * t326 * t176 * t332 * t334 * t178 * t336 - f64x8::splat(2.0) / f64x8::splat(729.0) * t585 * t328 / t18 / t589 * t594 * t596) * t22 * t26 * t38 / f64x8::splat(24.0) - t406 * t103 / f64x8::splat(3.0) + f64x8::splat(11.0) / f64x8::splat(9.0) * t232 * t195 - f64x8::splat(154.0) / f64x8::splat(81.0) * t130 * t349 + t341 * t132 / f64x8::splat(24.0) - t189 * t239 / f64x8::splat(3.0) + f64x8::splat(11.0) / f64x8::splat(9.0) * t99 * t417 - f64x8::splat(154.0) / f64x8::splat(81.0) * t48 * t116 * t308) - f64x8::splat(9.0) / f64x8::splat(4.0) * t365 * t373 + f64x8::splat(9.0) * t210 * t707 * t135 * t298 - f64x8::splat(27.0) / f64x8::splat(4.0) * t210 * t371 * t212 * t198;
            let t717 = ((t2).select(f64x8::splat(0.0), t716));
            let tv4rho3sigma0 = f64x8::splat(2.0) * v_rho * t717 + f64x8::splat(6.0) * t426;
            acc_v4rho3sigma = tv4rho3sigma0;
            let t739 = t242 * t242;
            let t808 = -t67 * t286 * t252 / f64x8::splat(6.0) - f64x8::splat(3.0) / f64x8::splat(2.0) * t365 * t433 + t365 * t437 + f64x8::splat(9.0) * t210 * t707 * t251 * t155 - f64x8::splat(9.0) * t210 * t371 * t212 * t242 - f64x8::splat(9.0) / f64x8::splat(4.0) * t210 * t371 * t251 * t198 + f64x8::splat(3.0) / f64x8::splat(2.0) * t67 * t69 * t154 * t739 + f64x8::splat(3.0) / f64x8::splat(2.0) * t210 * t211 * t135 * t420 + t67 * t286 * t274 / f64x8::splat(12.0) + t365 * t444 / f64x8::splat(2.0) - t67 * t149 * t474 / f64x8::splat(4.0) - f64x8::splat(9.0) / f64x8::splat(4.0) * t210 * t371 * t273 * t155 + f64x8::splat(3.0) / f64x8::splat(2.0) * t210 * t211 * t473 * t106 + f64x8::splat(3.0) / f64x8::splat(4.0) * t210 * t211 * t273 * t198 - f64x8::splat(3.0) / f64x8::splat(8.0) * t67 * t69 * t71 * ((-f64x8::splat(19.0) / f64x8::splat(81.0) * t256 * t257 * t169 + f64x8::splat(167.0) / f64x8::splat(324.0) * t177 * t391 * v_sigma - f64x8::splat(25.0) / f64x8::splat(864.0) * t326 * t176 * t399 * t334 * t86 * t336 + t585 * t395 / t18 / t179 / t166 * t594 * t596 / f64x8::splat(972.0)) * t22 * t26 * t38 / f64x8::splat(24.0) - f64x8::splat(2.0) / f64x8::splat(9.0) * t464 * t103 + f64x8::splat(11.0) / f64x8::splat(27.0) * t268 * t195 + t405 * t132 / f64x8::splat(12.0) - f64x8::splat(4.0) / f64x8::splat(9.0) * t231 * t239 + f64x8::splat(22.0) / f64x8::splat(27.0) * t129 * t417);
            let t809 = ((t2).select(f64x8::splat(0.0), t808));
            let tv4rho2sigma20 = f64x8::splat(2.0) * v_rho * t809 + f64x8::splat(4.0) * t479;
            acc_v4rho2sigma2 = tv4rho2sigma20;
            let t850 = t334 * t22;
            let t880 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(4.0) * t67 * t149 * t483 + f64x8::splat(9.0) * t210 * t707 * t482 * t106 - f64x8::splat(27.0) / f64x8::splat(4.0) * t210 * t371 * t251 * t242 + f64x8::splat(3.0) / f64x8::splat(4.0) * t365 * t488 - f64x8::splat(27.0) / f64x8::splat(4.0) * t210 * t371 * t487 * t106 + f64x8::splat(9.0) / f64x8::splat(4.0) * t210 * t211 * t242 * t273 + f64x8::splat(9.0) / f64x8::splat(4.0) * t210 * t211 * t135 * t473 - t67 * t149 * t511 / f64x8::splat(8.0) + f64x8::splat(3.0) / f64x8::splat(4.0) * t210 * t211 * t510 * t106 - f64x8::splat(3.0) / f64x8::splat(8.0) * t67 * t69 * t71 * ((-t174 * t176 * t225 * t184 / f64x8::splat(12.0) + f64x8::splat(7.0) / f64x8::splat(864.0) * t326 * t176 * t457 * t850 * t26 * v_sigma * t30 - t585 * t454 / t18 / t577 * t594 * t596 / f64x8::splat(2592.0)) * t22 * t26 * t38 / f64x8::splat(24.0) - t505 * t103 / f64x8::splat(9.0) + t463 * t132 / f64x8::splat(8.0) - t267 * t239 / f64x8::splat(3.0))));
            let tv4rhosigma30 = f64x8::splat(2.0) * v_rho * t880 + f64x8::splat(2.0) * t516;
            acc_v4rhosigma3 = tv4rhosigma30;
            let t883 = t251 * t251;
            let t892 = t273 * t273;
            let t926 = ((t2).select(f64x8::splat(0.0), f64x8::splat(9.0) * t67 * t69 * t541 * t883 - f64x8::splat(27.0) / f64x8::splat(2.0) * t210 * t371 * t251 * t273 + f64x8::splat(9.0) / f64x8::splat(4.0) * t67 * t69 * t154 * t892 + f64x8::splat(3.0) * t210 * t211 * t135 * t510 - f64x8::splat(3.0) / f64x8::splat(8.0) * t67 * t69 * t71 * ((-t326 * t176 * t498 * t850 * t116 / f64x8::splat(576.0) + t585 * t495 / t18 / t330 * t594 * t596 / f64x8::splat(6912.0)) * t22 * t26 * t38 / f64x8::splat(24.0) + t504 * t132 / f64x8::splat(6.0))));
            let tv4sigma40 = f64x8::splat(2.0) * v_rho * t926;
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

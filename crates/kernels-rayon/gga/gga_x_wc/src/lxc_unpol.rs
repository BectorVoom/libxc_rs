//! GGA_X_WC lxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_wc.c`
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
pub fn gga_x_wc_lxc_unpol(
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
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
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
            let t20 = f64x8::splat(M_CBRT6);
            let t21 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t22 = (simd::cbrt(t21));
            let t23 = t22 * t22;
            let t24 = f64x8::splat(1.0) / t23;
            let t25 = t20 * t24;
            let t26 = f64x8::splat(M_CBRT2);
            let t27 = t26 * t26;
            let t28 = v_sigma * t27;
            let t29 = v_rho * v_rho;
            let t30 = t18 * t18;
            let t32 = f64x8::splat(1.0) / t30 / t29;
            let t34 = t25 * t28 * t32;
            let t36 = t25 * v_sigma;
            let t37 = t27 * t32;
            let t39 = (simd::exp(-t34 / f64x8::splat(24.0)));
            let t40 = t37 * t39;
            let t43 = t20 * t20;
            let t46 = t43 / t22 / t21;
            let t47 = v_sigma * v_sigma;
            let t49 = t29 * t29;
            let t50 = t49 * v_rho;
            let t52 = f64x8::splat(1.0) / t18 / t50;
            let t56 = f64x8::splat(1.0) + f64x8::splat(2.7560657413756314e-05) * t46 * t47 * t26 * t52;
            let t57 = (simd::ln(t56));
            let t58 = f64x8::splat(0.804) + f64x8::splat(5.0) / f64x8::splat(972.0) * t34 + f64x8::splat(0.004002424276710846) * t36 * t40 + t57;
            let t61 = f64x8::splat(1.804) - f64x8::splat(0.646416) / t58;
            let t65 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t17 * t18 * t61));
            let tzk0 = f64x8::splat(2.0) * t65;
            acc_zk = tzk0;
            let t66 = f64x8::splat(1.0) / t30;
            let t71 = t3 * t17;
            let t72 = t58 * t58;
            let t73 = f64x8::splat(1.0) / t72;
            let t74 = t18 * t73;
            let t75 = t29 * v_rho;
            let t77 = f64x8::splat(1.0) / t30 / t75;
            let t81 = t27 * t77;
            let t82 = t81 * t39;
            let t85 = t46 * t47;
            let t86 = t49 * t29;
            let t88 = f64x8::splat(1.0) / t18 / t86;
            let t89 = t26 * t88;
            let t90 = t89 * t39;
            let t93 = f64x8::splat(1.0) / t56;
            let t94 = t89 * t93;
            let t97 = -f64x8::splat(10.0) / f64x8::splat(729.0) * t25 * t28 * t77 - f64x8::splat(0.010673131404562256) * t36 * t82 + f64x8::splat(0.0008894276170468547) * t85 * t90 - f64x8::splat(0.00014699017287336702) * t85 * t94;
            let t102 = ((t2).select(f64x8::splat(0.0), -t6 * t17 * t66 * t61 / f64x8::splat(8.0) - f64x8::splat(0.1655109536374632) * t71 * t74 * t97));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t102 + f64x8::splat(2.0) * t65;
            acc_vrho = tvrho0;
            let t109 = t46 * v_sigma;
            let t110 = t26 * t52;
            let t111 = t110 * t39;
            let t114 = t110 * t93;
            let t117 = f64x8::splat(5.0) / f64x8::splat(972.0) * t25 * t37 + f64x8::splat(0.004002424276710846) * t25 * t40 - f64x8::splat(0.0003335353563925705) * t109 * t111 + f64x8::splat(5.512131482751263e-05) * t109 * t114;
            let t121 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(0.1655109536374632) * t71 * t74 * t117));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t121;
            acc_vsigma = tvsigma0;
            let t125 = f64x8::splat(1.0) / t30 / v_rho;
            let t130 = t66 * t73;
            let t135 = f64x8::splat(1.0) / t72 / t58;
            let t136 = t18 * t135;
            let t137 = t97 * t97;
            let t142 = f64x8::splat(1.0) / t30 / t49;
            let t146 = t27 * t142;
            let t147 = t146 * t39;
            let t150 = t49 * t75;
            let t152 = f64x8::splat(1.0) / t18 / t150;
            let t153 = t26 * t152;
            let t154 = t153 * t39;
            let t157 = t21 * t21;
            let t158 = f64x8::splat(1.0) / t157;
            let t159 = t47 * v_sigma;
            let t160 = t158 * t159;
            let t161 = t49 * t49;
            let t162 = t161 * t29;
            let t163 = f64x8::splat(1.0) / t162;
            let t167 = t153 * t93;
            let t172 = t20 / t23 / t157;
            let t173 = t47 * t47;
            let t174 = t172 * t173;
            let t175 = t161 * t49;
            let t177 = f64x8::splat(1.0) / t30 / t175;
            let t179 = t56 * t56;
            let t180 = f64x8::splat(1.0) / t179;
            let t181 = t27 * t177 * t180;
            let t184 = f64x8::splat(110.0) / f64x8::splat(2187.0) * t25 * t28 * t142 + f64x8::splat(0.039134815150061605) * t36 * t147 - f64x8::splat(0.008004848553421692) * t85 * t154 + f64x8::splat(0.0011859034893958063) * t160 * t163 * t39 + f64x8::splat(0.0009309377615313244) * t85 * t167 - f64x8::splat(1.2963666552805393e-07) * t174 * t181;
            let t189 = ((t2).select(f64x8::splat(0.0), t6 * t17 * t125 * t61 / f64x8::splat(12.0) - f64x8::splat(0.1103406357583088) * t71 * t130 * t97 + f64x8::splat(0.3310219072749264) * t71 * t136 * t137 - f64x8::splat(0.1655109536374632) * t71 * t74 * t184));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t189 + f64x8::splat(4.0) * t102;
            acc_v2rho2 = tv2rho20;
            let t195 = t71 * t18;
            let t196 = t135 * t117;
            let t197 = t196 * t97;
            let t204 = t46 * t26;
            let t209 = t158 * t47;
            let t210 = t161 * v_rho;
            let t211 = f64x8::splat(1.0) / t210;
            let t217 = t172 * t159;
            let t218 = t161 * t75;
            let t220 = f64x8::splat(1.0) / t30 / t218;
            let t225 = -f64x8::splat(10.0) / f64x8::splat(729.0) * t25 * t81 - f64x8::splat(0.010673131404562256) * t25 * t82 + f64x8::splat(0.002668282851140564) * t204 * t88 * v_sigma * t39 - f64x8::splat(0.00044471380852342736) * t209 * t211 * t39 - f64x8::splat(0.00029398034574673403) * t109 * t94 + f64x8::splat(4.861374957302022e-08) * t217 * t27 * t220 * t180;
            let t230 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(0.0551703178791544) * t71 * t130 * t117 + f64x8::splat(0.3310219072749264) * t195 * t197 - f64x8::splat(0.1655109536374632) * t71 * t74 * t225));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t230 + f64x8::splat(2.0) * t121;
            acc_v2rhosigma = tv2rhosigma0;
            let t233 = t117 * t117;
            let t239 = t158 * v_sigma;
            let t240 = f64x8::splat(1.0) / t161;
            let t248 = f64x8::splat(1.0) / t30 / t162;
            let t250 = t27 * t248 * t180;
            let t253 = -f64x8::splat(0.000667070712785141) * t46 * t111 + f64x8::splat(0.00016676767819628525) * t239 * t240 * t39 + f64x8::splat(5.512131482751263e-05) * t46 * t114 - f64x8::splat(1.8230156089882582e-08) * t172 * t47 * t250;
            let t258 = ((t2).select(f64x8::splat(0.0), f64x8::splat(0.3310219072749264) * t71 * t136 * t233 - f64x8::splat(0.1655109536374632) * t71 * t74 * t253));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t258;
            acc_v2sigma2 = tv2sigma20;
            let t265 = t125 * t73;
            let t269 = t66 * t135;
            let t276 = t72 * t72;
            let t277 = f64x8::splat(1.0) / t276;
            let t278 = t18 * t277;
            let t279 = t137 * t97;
            let t283 = t135 * t97;
            let t284 = t283 * t184;
            let t288 = f64x8::splat(1.0) / t30 / t50;
            let t292 = t27 * t288;
            let t293 = t292 * t39;
            let t297 = f64x8::splat(1.0) / t18 / t161;
            let t298 = t26 * t297;
            let t302 = f64x8::splat(1.0) / t218;
            let t306 = t158 * t173;
            let t307 = t161 * t50;
            let t309 = f64x8::splat(1.0) / t30 / t307;
            let t311 = t27 * t39;
            let t312 = t25 * t311;
            let t315 = t298 * t93;
            let t319 = t27 * t309 * t180;
            let t322 = t173 * t47;
            let t323 = t161 * t161;
            let t325 = f64x8::splat(1.0) / t323 / t75;
            let t328 = f64x8::splat(1.0) / t179 / t56;
            let t331 = -f64x8::splat(1540.0) / f64x8::splat(6561.0) * t25 * t28 * t288 - f64x8::splat(0.18262913736695416) * t36 * t293 + f64x8::splat(0.067398848313995) * t85 * t298 * t39 - f64x8::splat(0.02253216629852032) * t160 * t302 * t39 + f64x8::splat(0.00013176705437731182) * t306 * t309 * t312 - f64x8::splat(0.0068268769178963795) * t85 * t315 + f64x8::splat(2.4630966450330243e-06) * t174 * t319 - f64x8::splat(4.819793287982341e-14) * t322 * t325 * t328;
            let t336 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(5.0) / f64x8::splat(36.0) * t6 * t17 * t32 * t61 + f64x8::splat(0.1103406357583088) * t71 * t265 * t97 + f64x8::splat(0.3310219072749264) * t71 * t269 * t137 - f64x8::splat(0.1655109536374632) * t71 * t130 * t184 - f64x8::splat(0.9930657218247793) * t71 * t278 * t279 + f64x8::splat(0.9930657218247793) * t195 * t284 - f64x8::splat(0.1655109536374632) * t71 * t74 * t331));
            let tv3rho30 = f64x8::splat(2.0) * v_rho * t336 + f64x8::splat(6.0) * t189;
            acc_v3rho3 = tv3rho30;
            let t343 = t71 * t66;
            let t349 = t277 * t117;
            let t350 = t349 * t137;
            let t353 = t135 * t225;
            let t354 = t353 * t97;
            let t357 = t196 * t184;
            let t368 = t158 * t163;
            let t369 = t47 * t39;
            let t379 = t173 * v_sigma;
            let t381 = f64x8::splat(1.0) / t323 / t29;
            let t385 = f64x8::splat(110.0) / f64x8::splat(2187.0) * t25 * t146 + f64x8::splat(0.039134815150061605) * t25 * t147 - f64x8::splat(0.019270931702681852) * t204 * t152 * v_sigma * t39 + f64x8::splat(0.007560134744898265) * t368 * t369 - f64x8::splat(4.941264539149193e-05) * t160 * t177 * t312 + f64x8::splat(0.0018618755230626488) * t109 * t167 - f64x8::splat(8.264337427413437e-07) * t217 * t181 + f64x8::splat(1.8074224829933777e-14) * t379 * t381 * t328;
            let t390 = ((t2).select(f64x8::splat(0.0), f64x8::splat(0.03678021191943627) * t71 * t265 * t117 + f64x8::splat(0.2206812715166176) * t343 * t197 - f64x8::splat(0.1103406357583088) * t71 * t130 * t225 - f64x8::splat(0.9930657218247793) * t195 * t350 + f64x8::splat(0.6620438145498528) * t195 * t354 + f64x8::splat(0.3310219072749264) * t195 * t357 - f64x8::splat(0.1655109536374632) * t71 * t74 * t385));
            let tv3rho2sigma0 = f64x8::splat(2.0) * v_rho * t390 + f64x8::splat(4.0) * t230;
            acc_v3rho2sigma = tv3rho2sigma0;
            let t396 = t277 * t233;
            let t397 = t396 * t97;
            let t400 = t196 * t225;
            let t406 = t135 * t253;
            let t407 = t406 * t97;
            let t412 = t158 * t211;
            let t413 = v_sigma * t39;
            let t421 = t172 * t27;
            let t422 = t220 * t180;
            let t426 = t323 * v_rho;
            let t427 = f64x8::splat(1.0) / t426;
            let t431 = f64x8::splat(0.003557710468187419) * t46 * t90 - f64x8::splat(0.0022235690426171367) * t412 * t413 + f64x8::splat(1.8529742021809473e-05) * t209 * t220 * t312 - f64x8::splat(0.00029398034574673403) * t46 * t94 + f64x8::splat(2.430687478651011e-07) * t421 * t422 * t47 - f64x8::splat(6.7778343112251664e-15) * t173 * t427 * t328;
            let t436 = ((t2).select(f64x8::splat(0.0), f64x8::splat(0.1103406357583088) * t71 * t269 * t233 - f64x8::splat(0.9930657218247793) * t195 * t397 + f64x8::splat(0.6620438145498528) * t195 * t400 - f64x8::splat(0.0551703178791544) * t71 * t130 * t253 + f64x8::splat(0.3310219072749264) * t195 * t407 - f64x8::splat(0.1655109536374632) * t71 * t74 * t431));
            let tv3rhosigma20 = f64x8::splat(2.0) * v_rho * t436 + f64x8::splat(2.0) * t258;
            acc_v3rhosigma2 = tv3rhosigma20;
            let t439 = t233 * t117;
            let t443 = t196 * t253;
            let t456 = f64x8::splat(1.0) / t323;
            let t460 = f64x8::splat(0.0005003030345888558) * t158 * t240 * t39 - f64x8::splat(6.9486532581785526e-06) * t239 * t248 * t312 - f64x8::splat(5.469046826964775e-08) * t421 * t248 * t180 * v_sigma + f64x8::splat(2.5416878667094372e-15) * t159 * t456 * t328;
            let t465 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(0.9930657218247793) * t71 * t278 * t439 + f64x8::splat(0.9930657218247793) * t195 * t443 - f64x8::splat(0.1655109536374632) * t71 * t74 * t460));
            let tv3sigma30 = f64x8::splat(2.0) * v_rho * t465;
            acc_v3sigma3 = tv3sigma30;
            let t472 = t32 * t73;
            let t476 = t125 * t135;
            let t483 = t66 * t277;
            let t493 = f64x8::splat(1.0) / t276 / t58;
            let t494 = t18 * t493;
            let t495 = t137 * t137;
            let t503 = t184 * t184;
            let t511 = f64x8::splat(1.0) / t30 / t86;
            let t521 = t26 / t18 / t210;
            let t529 = t161 * t86;
            let t531 = f64x8::splat(1.0) / t30 / t529;
            let t540 = t46 * t26 * t39;
            let t555 = t173 * t173;
            let t560 = t179 * t179;
            let t561 = f64x8::splat(1.0) / t560;
            let t569 = f64x8::splat(10.0) / f64x8::splat(27.0) * t6 * t17 * t77 * t61 - f64x8::splat(0.24520141279624177) * t71 * t472 * t97 - f64x8::splat(0.4413625430332352) * t71 * t476 * t137 + f64x8::splat(0.2206812715166176) * t71 * t265 * t184 - f64x8::splat(1.3240876290997057) * t71 * t483 * t279 + f64x8::splat(1.3240876290997057) * t343 * t284 - f64x8::splat(0.2206812715166176) * t71 * t130 * t331 + f64x8::splat(3.972262887299117) * t71 * t494 * t495 - f64x8::splat(5.958394330948676) * t195 * t277 * t137 * t184 + f64x8::splat(0.9930657218247793) * t71 * t136 * t503 + f64x8::splat(1.3240876290997057) * t195 * t283 * t331 - f64x8::splat(0.1655109536374632) * t71 * t74 * (f64x8::splat(26180.0) / f64x8::splat(19683.0) * t25 * t28 * t511 + f64x8::splat(1.034898445079407) * t36 * t27 * t511 * t39 - f64x8::splat(0.6022413220315036) * t85 * t521 * t39 + f64x8::splat(0.3377189603690502) * t160 / t175 * t39 - f64x8::splat(0.004304390442992185) * t306 * t531 * t312 + f64x8::splat(2.9281567639402625e-05) * t158 * t379 / t18 / t426 * t540 + f64x8::splat(0.05689064098246983) * t85 * t521 * t93 - f64x8::splat(3.9683223725532064e-05) * t174 * t27 * t531 * t180 + f64x8::splat(1.8315214494332895e-12) * t322 / t323 / t49 * t328 - f64x8::splat(2.1253867458432546e-17) * t555 / t18 / t323 / t210 * t561 * t204);
            let t570 = ((t2).select(f64x8::splat(0.0), t569));
            let tv4rho40 = f64x8::splat(2.0) * v_rho * t570 + f64x8::splat(8.0) * t336;
            acc_v4rho4 = tv4rho40;
            let t653 = f64x8::splat(3.972262887299117) * t195 * t493 * t117 * t279 - f64x8::splat(2.979197165474338) * t195 * t349 * t97 * t184 + f64x8::splat(0.3310219072749264) * t195 * t196 * t331 + f64x8::splat(0.9930657218247793) * t195 * t353 * t184 + f64x8::splat(0.9930657218247793) * t195 * t135 * t385 * t97 - f64x8::splat(2.979197165474338) * t195 * t277 * t225 * t137 + f64x8::splat(0.6620438145498528) * t343 * t354 + f64x8::splat(0.3310219072749264) * t343 * t357 - f64x8::splat(0.061300353199060444) * t71 * t472 * t117 - f64x8::splat(0.2206812715166176) * t71 * t125 * t197 - f64x8::splat(0.9930657218247793) * t343 * t350 - f64x8::splat(0.1655109536374632) * t71 * t74 * (-f64x8::splat(1540.0) / f64x8::splat(6561.0) * t25 * t292 - f64x8::splat(0.18262913736695416) * t25 * t293 + f64x8::splat(0.1500167914085695) * t204 * t297 * v_sigma * t39 - f64x8::splat(0.10129592305255845) * t158 * t302 * t369 + f64x8::splat(0.001465908479947594) * t158 * t309 * t159 * t312 - f64x8::splat(1.0980587864775984e-05) * t306 / t18 / t323 * t540 - f64x8::splat(0.013653753835792759) * t109 * t315 + f64x8::splat(1.211022517141237e-05) * t217 * t319 - f64x8::splat(6.325978690476822e-13) * t379 * t325 * t328 + f64x8::splat(7.970200296912205e-18) * t173 * t159 / t18 / t323 / t161 * t561 * t204) + f64x8::splat(0.1103406357583088) * t71 * t265 * t225 - f64x8::splat(0.1655109536374632) * t71 * t130 * t385;
            let t654 = ((t2).select(f64x8::splat(0.0), t653));
            let tv4rho3sigma0 = f64x8::splat(2.0) * v_rho * t654 + f64x8::splat(6.0) * t390;
            acc_v4rho3sigma = tv4rho3sigma0;
            let t676 = t225 * t225;
            let t736 = -f64x8::splat(0.07356042383887254) * t71 * t476 * t233 - f64x8::splat(0.6620438145498528) * t343 * t397 + f64x8::splat(0.4413625430332352) * t343 * t400 + f64x8::splat(3.972262887299117) * t195 * t493 * t233 * t137 - f64x8::splat(3.972262887299117) * t195 * t349 * t97 * t225 - f64x8::splat(0.9930657218247793) * t195 * t396 * t184 + f64x8::splat(0.6620438145498528) * t71 * t136 * t676 + f64x8::splat(0.6620438145498528) * t195 * t196 * t385 + f64x8::splat(0.03678021191943627) * t71 * t265 * t253 + f64x8::splat(0.2206812715166176) * t343 * t407 - f64x8::splat(0.1103406357583088) * t71 * t130 * t431 - f64x8::splat(0.9930657218247793) * t195 * t277 * t253 * t137 + f64x8::splat(0.6620438145498528) * t195 * t135 * t431 * t97 + f64x8::splat(0.3310219072749264) * t195 * t406 * t184 - f64x8::splat(0.1655109536374632) * t71 * t74 * (-f64x8::splat(0.02253216629852032) * t46 * t154 + f64x8::splat(0.024755735341137456) * t368 * t413 - f64x8::splat(0.00046324355054523685) * t158 * t177 * t47 * t312 + f64x8::splat(4.117720449290994e-06) * t160 / t18 / t161 / t150 * t540 + f64x8::splat(0.0018618755230626488) * t46 * t167 - f64x8::splat(3.0950753894822872e-06) * t421 * t177 * t180 * t47 + f64x8::splat(2.0559430744049672e-13) * t381 * t328 * t173 - f64x8::splat(2.988825111342077e-18) * t322 / t18 / t323 / t150 * t561 * t204);
            let t737 = ((t2).select(f64x8::splat(0.0), t736));
            let tv4rho2sigma20 = f64x8::splat(2.0) * v_rho * t737 + f64x8::splat(4.0) * t436;
            acc_v4rho2sigma2 = tv4rho2sigma20;
            let t800 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(0.3310219072749264) * t71 * t483 * t439 + f64x8::splat(3.972262887299117) * t195 * t493 * t439 * t97 - f64x8::splat(2.979197165474338) * t195 * t396 * t225 + f64x8::splat(0.3310219072749264) * t343 * t443 - f64x8::splat(2.979197165474338) * t195 * t349 * t253 * t97 + f64x8::splat(0.9930657218247793) * t195 * t353 * t253 + f64x8::splat(0.9930657218247793) * t195 * t196 * t431 - f64x8::splat(0.0551703178791544) * t71 * t130 * t460 + f64x8::splat(0.3310219072749264) * t195 * t135 * t460 * t97 - f64x8::splat(0.1655109536374632) * t71 * t74 * (-f64x8::splat(0.004002424276710846) * t412 * t39 + f64x8::splat(0.0001297081941526663) * t158 * t220 * t20 * t24 * v_sigma * t311 - f64x8::splat(1.5441451684841229e-06) * t209 / t18 / t529 * t540 + f64x8::splat(5.833649948762426e-07) * t421 * t422 * v_sigma - f64x8::splat(6.100050880102649e-14) * t427 * t328 * t159 + f64x8::splat(1.1208094167532788e-18) * t379 / t18 / t323 / t86 * t561 * t204)));
            let tv4rhosigma30 = f64x8::splat(2.0) * v_rho * t800 + f64x8::splat(2.0) * t465;
            acc_v4rhosigma3 = tv4rhosigma30;
            let t803 = t233 * t233;
            let t810 = t253 * t253;
            let t845 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.972262887299117) * t71 * t494 * t803 - f64x8::splat(5.958394330948676) * t195 * t396 * t253 + f64x8::splat(0.9930657218247793) * t71 * t136 * t810 + f64x8::splat(1.3240876290997057) * t195 * t196 * t460 - f64x8::splat(0.1655109536374632) * t71 * t74 * (-f64x8::splat(2.779461303271421e-05) * t158 * t248 * t20 * t24 * t27 * t39 + f64x8::splat(5.79054438181546e-07) * t239 / t18 / t307 * t540 + f64x8::splat(1.5250127200256623e-14) * t456 * t328 * t47 - f64x8::splat(5.469046826964775e-08) * t172 * t250 - f64x8::splat(4.2030353128247956e-19) * t173 / t18 / t323 / t50 * t561 * t204)));
            let tv4sigma40 = f64x8::splat(2.0) * v_rho * t845;
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

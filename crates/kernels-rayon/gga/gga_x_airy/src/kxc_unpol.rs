//! GGA_X_AIRY kxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_airy.c`
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
pub fn gga_x_airy_kxc_unpol(
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
            let t21 = t20 * t20;
            let t22 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t23 = (simd::cbrt(t22));
            let t24 = f64x8::splat(1.0) / t23;
            let t25 = t21 * t24;
            let t26 = ((v_sigma).sqrt());
            let t27 = f64x8::splat(M_CBRT2);
            let t28 = t26 * t27;
            let t30 = f64x8::splat(1.0) / t18 / v_rho;
            let t32 = t25 * t28 * t30;
            let t33 = (simd::pow(t32, f64x8::splat(2.626712)));
            let t35 = f64x8::splat(1.0) + f64x8::splat(0.00013471619689594795) * t33;
            let t36 = (simd::pow(t35, -f64x8::splat(0.657946)));
            let t39 = (simd::pow(t32, f64x8::splat(3.217063)));
            let t41 = (simd::pow(t32, f64x8::splat(3.223476)));
            let t43 = f64x8::splat(1.0) - f64x8::splat(0.04521241301076986) * t39 + f64x8::splat(0.04540222195662038) * t41;
            let t44 = (simd::pow(t32, f64x8::splat(3.473804)));
            let t46 = f64x8::splat(1.0) + f64x8::splat(0.0004770218022490335) * t44;
            let t47 = f64x8::splat(1.0) / t46;
            let t49 = f64x8::splat(6.014601922021111e-05) * t33 * t36 + t43 * t47;
            let t53 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t49));
            let tzk0 = f64x8::splat(2.0) * t53;
            acc_zk = tzk0;
            let t54 = t18 * t18;
            let t56 = t17 / t54;
            let t60 = (simd::pow(t32, f64x8::splat(1.626712)));
            let t62 = t60 * t36 * t21;
            let t63 = t24 * t26;
            let t64 = v_rho * v_rho;
            let t66 = f64x8::splat(1.0) / t18 / t64;
            let t67 = t27 * t66;
            let t68 = t63 * t67;
            let t71 = (simd::pow(t32, f64x8::splat(4.253424)));
            let t72 = (simd::pow(t35, -f64x8::splat(1.657946)));
            let t74 = t71 * t72 * t21;
            let t77 = (simd::pow(t32, f64x8::splat(2.217063)));
            let t79 = t77 * t21 * t24;
            let t80 = t28 * t66;
            let t83 = (simd::pow(t32, f64x8::splat(2.223476)));
            let t85 = t83 * t21 * t24;
            let t88 = f64x8::splat(0.19393490805022173) * t79 * t80 - f64x8::splat(0.19513729709845176) * t85 * t80;
            let t90 = t46 * t46;
            let t91 = f64x8::splat(1.0) / t90;
            let t92 = t43 * t91;
            let t93 = (simd::pow(t32, f64x8::splat(2.473804)));
            let t94 = t93 * t21;
            let t95 = t92 * t94;
            let t98 = -f64x8::splat(0.00021064836058394556) * t62 * t68 + f64x8::splat(1.8671024483029836e-08) * t74 * t68 + t88 * t47 + f64x8::splat(0.0022094403263198687) * t95 * t68;
            let t103 = ((t2).select(f64x8::splat(0.0), -t6 * t56 * t49 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t98));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t103 + f64x8::splat(2.0) * t53;
            acc_vrho = tvrho0;
            let t106 = f64x8::splat(1.0) / t26;
            let t107 = t24 * t106;
            let t108 = t27 * t30;
            let t109 = t107 * t108;
            let t114 = t106 * t27;
            let t115 = t114 * t30;
            let t120 = -f64x8::splat(0.07272559051883315) * t79 * t115 + f64x8::splat(0.07317648641191941) * t85 * t115;
            let t124 = f64x8::splat(7.899313521897959e-05) * t62 * t109 - f64x8::splat(7.001634181136188e-09) * t74 * t109 + t120 * t47 - f64x8::splat(0.0008285401223699508) * t95 * t109;
            let t128 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t124));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t128;
            acc_vsigma = tvsigma0;
            let t133 = t17 / t54 / v_rho;
            let t140 = (simd::pow(t32, f64x8::splat(0.626712)));
            let t142 = t140 * t36 * t20;
            let t143 = t23 * t23;
            let t144 = f64x8::splat(1.0) / t143;
            let t145 = t144 * v_sigma;
            let t146 = t27 * t27;
            let t147 = t64 * t64;
            let t149 = f64x8::splat(1.0) / t54 / t147;
            let t150 = t146 * t149;
            let t151 = t145 * t150;
            let t154 = (simd::pow(t32, f64x8::splat(3.253424)));
            let t156 = t154 * t72 * t20;
            let t159 = t64 * v_rho;
            let t161 = f64x8::splat(1.0) / t18 / t159;
            let t162 = t27 * t161;
            let t163 = t63 * t162;
            let t166 = (simd::pow(t32, f64x8::splat(5.880136)));
            let t167 = (simd::pow(t35, -f64x8::splat(2.657946)));
            let t169 = t166 * t167 * t20;
            let t174 = (simd::pow(t32, f64x8::splat(1.217063)));
            let t175 = t174 * t20;
            let t176 = t175 * t144;
            let t177 = v_sigma * t146;
            let t178 = t177 * t149;
            let t181 = t28 * t161;
            let t184 = (simd::pow(t32, f64x8::splat(1.223476)));
            let t185 = t184 * t20;
            let t186 = t185 * t144;
            let t191 = -f64x8::splat(3.4397272723723904) * t176 * t178 - f64x8::splat(0.45251478545051743) * t79 * t181 + f64x8::splat(3.471064774426217) * t186 * t178 + f64x8::splat(0.45532035989638747) * t85 * t181;
            let t193 = t88 * t91;
            let t194 = t193 * t94;
            let t198 = f64x8::splat(1.0) / t90 / t46;
            let t199 = t43 * t198;
            let t200 = (simd::pow(t32, f64x8::splat(4.947608)));
            let t201 = t200 * t20;
            let t202 = t199 * t201;
            let t205 = (simd::pow(t32, f64x8::splat(1.473804)));
            let t206 = t205 * t20;
            let t207 = t92 * t206;
            let t212 = f64x8::splat(0.00274131372753785) * t142 * t151 - f64x8::splat(1.0276735016205997e-06) * t156 * t151 + f64x8::splat(0.0004915128413625396) * t62 * t163 + f64x8::splat(8.763160960794521e-11) * t169 * t151 - f64x8::splat(4.356572379373628e-08) * t74 * t163 + t191 * t47 + f64x8::splat(0.004418880652639737) * t194 * t68 + f64x8::splat(5.8579518666821375e-05) * t202 * t151 - f64x8::splat(0.04372577853609117) * t207 * t151 - f64x8::splat(0.005155360761413027) * t95 * t163;
            let t217 = ((t2).select(f64x8::splat(0.0), t6 * t133 * t49 / f64x8::splat(12.0) - t6 * t56 * t98 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t212));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t217 + f64x8::splat(4.0) * t103;
            acc_v2rho2 = tv2rho20;
            let t223 = t144 * t146;
            let t225 = f64x8::splat(1.0) / t54 / t159;
            let t226 = t223 * t225;
            let t231 = t107 * t67;
            let t240 = t114 * t66;
            let t247 = f64x8::splat(1.2898977271396463) * t175 * t226 + f64x8::splat(0.09696745402511087) * t79 * t240 - f64x8::splat(1.3016492904098316) * t185 * t226 - f64x8::splat(0.09756864854922588) * t85 * t240;
            let t249 = t120 * t91;
            let t250 = t249 * t94;
            let t255 = t199 * t200;
            let t256 = t20 * t144;
            let t257 = t146 * t225;
            let t258 = t256 * t257;
            let t261 = t92 * t205;
            let t266 = -f64x8::splat(0.0010279926478266937) * t142 * t226 + f64x8::splat(3.853775631077249e-07) * t156 * t226 - f64x8::splat(0.00010532418029197278) * t62 * t231 - f64x8::splat(3.2861853602979454e-11) * t169 * t226 + f64x8::splat(9.335512241514918e-09) * t74 * t231 + t247 * t47 + f64x8::splat(0.0022094403263198687) * t250 * t68 - f64x8::splat(0.0008285401223699508) * t194 * t109 - f64x8::splat(2.1967319500058017e-05) * t255 * t258 + f64x8::splat(0.01639716695103419) * t261 * t258 + f64x8::splat(0.0011047201631599344) * t95 * t231;
            let t271 = ((t2).select(f64x8::splat(0.0), -t6 * t56 * t124 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t266));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t271 + f64x8::splat(2.0) * t128;
            acc_v2rhosigma = tv2rhosigma0;
            let t274 = f64x8::splat(1.0) / v_sigma;
            let t275 = t144 * t274;
            let t277 = f64x8::splat(1.0) / t54 / t64;
            let t278 = t146 * t277;
            let t279 = t275 * t278;
            let t284 = t26 * v_sigma;
            let t285 = f64x8::splat(1.0) / t284;
            let t286 = t24 * t285;
            let t287 = t286 * t108;
            let t294 = t274 * t146;
            let t295 = t294 * t277;
            let t298 = t285 * t27;
            let t299 = t298 * t30;
            let t306 = -f64x8::splat(0.48371164767736735) * t176 * t295 + f64x8::splat(0.036362795259416575) * t79 * t299 + f64x8::splat(0.4881184839036868) * t186 * t295 - f64x8::splat(0.03658824320595971) * t85 * t299;
            let t316 = f64x8::splat(0.00038549724293501016) * t142 * t279 - f64x8::splat(1.4451658616539682e-07) * t156 * t279 - f64x8::splat(3.9496567609489795e-05) * t62 * t287 + f64x8::splat(1.2323195101117295e-11) * t169 * t279 + f64x8::splat(3.500817090568094e-09) * t74 * t287 + t306 * t47 - f64x8::splat(0.0016570802447399015) * t250 * t109 + f64x8::splat(8.237744812521756e-06) * t202 * t279 - f64x8::splat(0.006148937606637821) * t207 * t279 + f64x8::splat(0.0004142700611849754) * t95 * t287;
            let t320 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t316));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t320;
            acc_v2sigma2 = tv2sigma20;
            let t323 = t17 * t277;
            let t333 = (simd::pow(t32, f64x8::splat(0.473804)));
            let t334 = t92 * t333;
            let t335 = f64x8::splat(1.0) / t22;
            let t336 = t335 * t284;
            let t337 = t147 * t159;
            let t338 = f64x8::splat(1.0) / t337;
            let t339 = t336 * t338;
            let t342 = (simd::pow(t32, f64x8::splat(3.947608)));
            let t343 = t199 * t342;
            let t346 = t342 * t284;
            let t347 = t346 * t338;
            let t350 = (simd::pow(t32, -f64x8::splat(0.373288)));
            let t351 = t350 * t36;
            let t354 = (simd::pow(t32, f64x8::splat(2.253424)));
            let t355 = t354 * t72;
            let t358 = (simd::pow(t32, f64x8::splat(4.880136)));
            let t359 = t358 * t167;
            let t362 = t90 * t90;
            let t363 = f64x8::splat(1.0) / t362;
            let t364 = t43 * t363;
            let t365 = (simd::pow(t32, f64x8::splat(7.421412)));
            let t366 = t365 * t284;
            let t367 = t366 * t338;
            let t370 = t88 * t198;
            let t371 = t370 * t201;
            let t374 = t193 * t206;
            let t377 = t147 * v_rho;
            let t379 = f64x8::splat(1.0) / t54 / t377;
            let t380 = t146 * t379;
            let t381 = t145 * t380;
            let t385 = f64x8::splat(1.0) / t18 / t147;
            let t386 = t27 * t385;
            let t387 = t63 * t386;
            let t390 = f64x8::splat(1.031091636953685) * t334 * t339 - f64x8::splat(0.004637255923073837) * t343 * t339 - f64x8::splat(0.00023492612948914138) * t199 * t347 - f64x8::splat(0.027488227341003216) * t351 * t339 + f64x8::splat(5.349532214938397e-05) * t355 * t339 - f64x8::splat(8.244572518297991e-09) * t359 * t339 + f64x8::splat(4.7209655431432423e-07) * t364 * t367 + f64x8::splat(0.00017573855600046414) * t371 * t151 - f64x8::splat(0.13117733560827352) * t374 * t151 - f64x8::splat(6.134212672556165e-10) * t169 * t381 + f64x8::splat(1.4521907931245427e-07) * t74 * t387;
            let t391 = t191 * t91;
            let t392 = t391 * t94;
            let t401 = (simd::pow(t32, f64x8::splat(0.217063)));
            let t402 = t401 * t335;
            let t403 = t284 * t338;
            let t406 = t177 * t379;
            let t409 = t28 * t385;
            let t412 = (simd::pow(t32, f64x8::splat(0.223476)));
            let t413 = t412 * t335;
            let t420 = f64x8::splat(66.98183669272574) * t402 * t403 + f64x8::splat(24.078090906606732) * t176 * t406 + f64x8::splat(1.5083826181683913) * t79 * t409 - f64x8::splat(67.94823113529425) * t413 * t403 - f64x8::splat(24.297453420983523) * t186 * t406 - f64x8::splat(1.5177345329879584) * t85 * t409;
            let t426 = (simd::pow(t32, f64x8::splat(7.506848)));
            let t427 = (simd::pow(t35, -f64x8::splat(3.657946)));
            let t428 = t426 * t427;
            let t439 = f64x8::splat(0.006628320978959606) * t392 * t68 - f64x8::splat(0.01918919609276495) * t142 * t381 + f64x8::splat(7.193714511344198e-06) * t156 * t381 - f64x8::splat(0.001638376137875132) * t62 * t387 + t420 * t47 + f64x8::splat(1.0346691524063492e-06) * t355 * t403 - f64x8::splat(9.774128607384063e-10) * t359 * t403 + f64x8::splat(1.3361630313149017e-13) * t428 * t403 - f64x8::splat(0.015466082284239081) * t194 * t163 - f64x8::splat(0.00041005663066774964) * t202 * t381 + f64x8::splat(0.3060804497526382) * t207 * t381 + f64x8::splat(0.017184535871376756) * t95 * t387;
            let t440 = t390 + t439;
            let t445 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(5.0) / f64x8::splat(36.0) * t6 * t323 * t49 + t6 * t133 * t98 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t56 * t212 - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t440));
            let tv3rho30 = f64x8::splat(2.0) * v_rho * t445 + f64x8::splat(6.0) * t217;
            acc_v3rho3 = tv3rho30;
            let t455 = t256 * t150;
            let t460 = t107 * t162;
            let t465 = t147 * t64;
            let t466 = f64x8::splat(1.0) / t465;
            let t468 = t365 * t466 * t26;
            let t472 = t342 * t466 * t26;
            let t476 = t335 * t466 * t26;
            let t483 = t466 * t26;
            let t486 = t223 * t149;
            let t489 = t114 * t161;
            let t498 = -f64x8::splat(25.11818875977215) * t402 * t483 - f64x8::splat(6.4494886356982315) * t175 * t486 - f64x8::splat(0.22625739272525872) * t79 * t489 + f64x8::splat(25.480586675735342) * t413 * t483 + f64x8::splat(6.508246452049157) * t185 * t486 + f64x8::splat(0.22766017994819374) * t85 * t489;
            let t506 = -f64x8::splat(0.08198583475517095) * t261 * t455 + f64x8::splat(0.00010983659750029009) * t255 * t455 + f64x8::splat(0.0002457564206812698) * t62 * t460 - f64x8::splat(2.178286189686814e-08) * t74 * t460 - f64x8::splat(1.7703620786787158e-07) * t364 * t468 + f64x8::splat(8.809729855842801e-05) * t199 * t472 + f64x8::splat(0.010308085252876206) * t351 * t476 - f64x8::splat(2.0060745806018988e-05) * t355 * t476 + f64x8::splat(3.091714694361747e-09) * t359 * t476 + t498 * t47 - f64x8::splat(3.8800093215238094e-07) * t355 * t483 + f64x8::splat(3.6652982277690234e-10) * t359 * t483 - f64x8::splat(5.0106113674308816e-14) * t428 * t483;
            let t507 = t247 * t91;
            let t508 = t507 * t94;
            let t517 = t120 * t198;
            let t518 = t517 * t201;
            let t521 = t249 * t206;
            let t536 = t370 * t200;
            let t539 = t193 * t205;
            let t542 = f64x8::splat(0.004418880652639737) * t508 * t68 - f64x8::splat(0.005155360761413027) * t250 * t163 + f64x8::splat(0.0022094403263198687) * t194 * t231 - f64x8::splat(0.0025776803807065134) * t95 * t460 + f64x8::splat(5.8579518666821375e-05) * t518 * t151 - f64x8::splat(0.04372577853609117) * t521 * t151 - f64x8::splat(0.0008285401223699508) * t392 * t109 + f64x8::splat(0.0017389709711526887) * t343 * t476 - f64x8::splat(0.3866593638576319) * t334 * t476 + f64x8::splat(1.6430926801489725e-10) * t169 * t486 + f64x8::splat(0.005139963239133468) * t142 * t486 - f64x8::splat(1.9268878155386245e-06) * t156 * t486 - f64x8::splat(4.3934639000116035e-05) * t536 * t258 + f64x8::splat(0.03279433390206838) * t539 * t258;
            let t543 = t506 + t542;
            let t548 = ((t2).select(f64x8::splat(0.0), t6 * t133 * t124 / f64x8::splat(12.0) - t6 * t56 * t266 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t543));
            let tv3rho2sigma0 = f64x8::splat(2.0) * v_rho * t548 + f64x8::splat(4.0) * t271;
            acc_v3rho2sigma = tv3rho2sigma0;
            let t554 = t249 * t205;
            let t557 = t286 * t67;
            let t560 = t275 * t257;
            let t569 = t517 * t200;
            let t572 = t335 * t106;
            let t573 = f64x8::splat(1.0) / t377;
            let t574 = t572 * t573;
            let t577 = t365 * t106;
            let t578 = t577 * t573;
            let t581 = t342 * t106;
            let t582 = t581 * t573;
            let t589 = t106 * t573;
            let t592 = t294 * t225;
            let t595 = t298 * t66;
            let t604 = f64x8::splat(9.419320784914555) * t402 * t589 + f64x8::splat(0.6449488635698232) * t176 * t592 - f64x8::splat(0.04848372701255543) * t79 * t595 - f64x8::splat(9.555220003400754) * t413 * t589 - f64x8::splat(0.6508246452049158) * t186 * t592 + f64x8::splat(0.04878432427461294) * t85 * t595;
            let t606 = f64x8::splat(0.03279433390206838) * t554 * t258 + f64x8::splat(5.266209014598639e-05) * t62 * t557 - f64x8::splat(1.6430926801489727e-11) * t169 * t560 - f64x8::splat(4.667756120757459e-09) * t74 * t557 - f64x8::splat(0.0005139963239133468) * t142 * t560 + f64x8::splat(1.9268878155386244e-07) * t156 * t560 - f64x8::splat(4.3934639000116035e-05) * t569 * t258 - f64x8::splat(1.159393010385655e-09) * t359 * t574 + f64x8::splat(6.638857795045185e-08) * t364 * t578 - f64x8::splat(3.3036486959410505e-05) * t199 * t582 - f64x8::splat(0.0038655319698285774) * t351 * t574 + f64x8::splat(7.52277967725712e-06) * t355 * t574 + t604 * t47;
            let t617 = t306 * t91;
            let t618 = t617 * t94;
            let t637 = f64x8::splat(1.4550034955714285e-07) * t355 * t589 - f64x8::splat(1.3744868354133837e-10) * t359 * t589 + f64x8::splat(1.8789792627865805e-14) * t428 * t589 + f64x8::splat(0.008198583475517095) * t207 * t560 - f64x8::splat(0.0005523600815799672) * t95 * t557 + f64x8::splat(0.0022094403263198687) * t618 * t68 - f64x8::splat(0.0016570802447399015) * t508 * t109 + f64x8::splat(8.237744812521756e-06) * t371 * t279 - f64x8::splat(0.006148937606637821) * t374 * t279 + f64x8::splat(0.0004142700611849754) * t194 * t287 + f64x8::splat(0.0022094403263198687) * t250 * t231 - f64x8::splat(1.0983659750029009e-05) * t202 * t560 + f64x8::splat(0.14499726144661196) * t334 * t574 - f64x8::splat(0.0006521141141822582) * t343 * t574;
            let t638 = t606 + t637;
            let t643 = ((t2).select(f64x8::splat(0.0), -t6 * t56 * t316 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t638));
            let tv3rhosigma20 = f64x8::splat(2.0) * v_rho * t643 + f64x8::splat(2.0) * t320;
            acc_v3rhosigma2 = tv3rhosigma20;
            let t650 = v_sigma * v_sigma;
            let t651 = f64x8::splat(1.0) / t650;
            let t652 = t144 * t651;
            let t653 = t652 * t278;
            let t657 = f64x8::splat(1.0) / t26 / t650;
            let t658 = t24 * t657;
            let t659 = t658 * t108;
            let t670 = f64x8::splat(1.0) / t147;
            let t671 = t285 * t670;
            let t674 = t651 * t146;
            let t675 = t674 * t277;
            let t678 = t657 * t27;
            let t679 = t678 * t30;
            let t688 = -f64x8::splat(3.5322452943429585) * t402 * t671 + f64x8::splat(0.725567471516051) * t176 * t675 - f64x8::splat(0.05454419288912486) * t79 * t679 + f64x8::splat(3.583207501275283) * t413 * t671 - f64x8::splat(0.7321777258555302) * t186 * t675 + f64x8::splat(0.054882364808939564) * t85 * t679;
            let t690 = t335 * t285;
            let t691 = t690 * t670;
            let t696 = f64x8::splat(2.471323443756527e-05) * t518 * t279 - f64x8::splat(0.018446812819913463) * t521 * t279 - f64x8::splat(1.8484792651675942e-11) * t169 * t653 - f64x8::splat(5.251225635852141e-09) * t74 * t659 - f64x8::splat(0.0024856203671098525) * t618 * t109 - f64x8::splat(0.0005782458644025152) * t142 * t653 + f64x8::splat(2.1677487924809525e-07) * t156 * t653 + f64x8::splat(5.9244851414234685e-05) * t62 * t659 + t688 * t47 - f64x8::splat(0.054373973042479484) * t334 * t691 + f64x8::splat(0.0002445427928183468) * t343 * t691;
            let t711 = t365 * t285;
            let t712 = t711 * t670;
            let t716 = t342 * t285 * t670;
            let t725 = f64x8::splat(0.0012428101835549263) * t250 * t287 - f64x8::splat(1.2356617218782635e-05) * t202 * t653 + f64x8::splat(0.009223406409956732) * t207 * t653 - f64x8::splat(0.0006214050917774631) * t95 * t659 + f64x8::splat(0.0014495744886857164) * t351 * t691 - f64x8::splat(2.82104237897142e-06) * t355 * t691 + f64x8::splat(4.3477237889462065e-10) * t359 * t691 - f64x8::splat(2.4895716731419444e-08) * t364 * t712 + f64x8::splat(1.238868260977894e-05) * t199 * t716 - f64x8::splat(5.456263108392857e-08) * t355 * t671 + f64x8::splat(5.154325632800189e-11) * t359 * t671 - f64x8::splat(7.046172235449677e-15) * t428 * t671;
            let t726 = t696 + t725;
            let t730 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t726));
            let tv3sigma30 = f64x8::splat(2.0) * v_rho * t730;
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

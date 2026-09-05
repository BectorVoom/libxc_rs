//! GGA_C_LYP lxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_lyp.c`
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

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_lyp_lxc_unpol(
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
    param_a: f64,
    param_b: f64,
    param_c: f64,
    param_d: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_a = f64x8::splat(param_a);
    let param_b = f64x8::splat(param_b);
    let param_c = f64x8::splat(param_c);
    let param_d = f64x8::splat(param_d);
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
            let t1 = (simd::cbrt(v_rho));
            let t2 = f64x8::splat(1.0) / t1;
            let t4 = param_d * t2 + f64x8::splat(1.0);
            let t5 = f64x8::splat(1.0) / t4;
            let t7 = (simd::exp(-param_c * t2));
            let t8 = param_b * t7;
            let t9 = v_rho * v_rho;
            let t10 = t1 * t1;
            let t12 = f64x8::splat(1.0) / t10 / t9;
            let t13 = v_sigma * t12;
            let t15 = param_d * t5 + param_c;
            let t16 = t15 * t2;
            let t18 = -f64x8::splat(1.0) / f64x8::splat(72.0) - f64x8::splat(7.0) / f64x8::splat(72.0) * t16;
            let t20 = f64x8::splat(M_CBRT3);
            let t21 = t20 * t20;
            let t22 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t23 = (simd::cbrt(t22));
            let t24 = t23 * t23;
            let t26 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t27 = zeta_threshold * zeta_threshold;
            let t28 = (simd::cbrt(zeta_threshold));
            let t29 = t28 * t28;
            let t31 = ((t26).select(t29 * t27, f64x8::splat(1.0)));
            let t35 = f64x8::splat(5.0) / f64x8::splat(2.0) - t16 / f64x8::splat(18.0);
            let t36 = t35 * v_sigma;
            let t37 = t12 * t31;
            let t40 = t16 - f64x8::splat(11.0);
            let t41 = t40 * v_sigma;
            let t44 = ((t26).select(t29 * t27 * zeta_threshold, f64x8::splat(1.0)));
            let t45 = t12 * t44;
            let t48 = f64x8::splat(M_CBRT2);
            let t49 = t48 * t48;
            let t50 = v_sigma * t49;
            let t53 = ((t26).select(t27, f64x8::splat(1.0)));
            let t54 = t53 * v_sigma;
            let t56 = t49 * t12 * t31;
            let t62 = -t13 * t18 - f64x8::splat(3.0) / f64x8::splat(10.0) * t21 * t24 * t31 + t36 * t37 / f64x8::splat(8.0) + t41 * t45 / f64x8::splat(144.0) - t48 * (f64x8::splat(4.0) / f64x8::splat(3.0) * t50 * t37 - t54 * t56 / f64x8::splat(2.0)) / f64x8::splat(8.0);
            let tzk0 = param_a * (t8 * t5 * t62 - t5);
            acc_zk = tzk0;
            let t66 = v_rho * param_a;
            let t67 = t4 * t4;
            let t68 = f64x8::splat(1.0) / t67;
            let t69 = t68 * param_d;
            let t71 = f64x8::splat(1.0) / t1 / v_rho;
            let t74 = param_b * param_c;
            let t75 = t74 * t71;
            let t76 = t7 * t5;
            let t77 = t76 * t62;
            let t80 = t8 * t68;
            let t81 = t62 * param_d;
            let t85 = t9 * v_rho;
            let t87 = f64x8::splat(1.0) / t10 / t85;
            let t88 = v_sigma * t87;
            let t91 = param_d * param_d;
            let t92 = t91 * t68;
            let t94 = f64x8::splat(1.0) / t10 / v_rho;
            let t97 = t15 * t71 - t92 * t94;
            let t98 = f64x8::splat(7.0) / f64x8::splat(216.0) * t97;
            let t100 = t97 / f64x8::splat(54.0);
            let t101 = t100 * v_sigma;
            let t104 = t87 * t31;
            let t108 = -t97 / f64x8::splat(3.0);
            let t109 = t108 * v_sigma;
            let t112 = t87 * t44;
            let t118 = t49 * t87 * t31;
            let t124 = f64x8::splat(8.0) / f64x8::splat(3.0) * t88 * t18 - t13 * t98 + t101 * t37 / f64x8::splat(8.0) - t36 * t104 / f64x8::splat(3.0) + t109 * t45 / f64x8::splat(144.0) - t41 * t112 / f64x8::splat(54.0) - t48 * (-f64x8::splat(32.0) / f64x8::splat(9.0) * t50 * t104 + f64x8::splat(4.0) / f64x8::splat(3.0) * t54 * t118) / f64x8::splat(8.0);
            let t127 = -t69 * t71 / f64x8::splat(3.0) + t75 * t77 / f64x8::splat(3.0) + t80 * t81 * t71 / f64x8::splat(3.0) + t8 * t5 * t124;
            let tvrho0 = t66 * t127 + tzk0;
            acc_vrho = tvrho0;
            let t129 = t66 * param_b;
            let t138 = t53 * t49;
            let t144 = -t12 * t18 + t35 * t12 * t31 / f64x8::splat(8.0) + t40 * t12 * t44 / f64x8::splat(144.0) - t48 * (f64x8::splat(4.0) / f64x8::splat(3.0) * t56 - t138 * t37 / f64x8::splat(2.0)) / f64x8::splat(8.0);
            let t145 = t76 * t144;
            let tvsigma0 = t129 * t145;
            acc_vsigma = tvsigma0;
            let t149 = f64x8::splat(1.0) / t67 / t4;
            let t150 = t149 * t91;
            let t154 = f64x8::splat(1.0) / t1 / t9;
            let t157 = t74 * t154;
            let t160 = param_c * param_c;
            let t161 = param_b * t160;
            let t162 = t161 * t12;
            let t165 = t74 * t12;
            let t166 = t7 * t68;
            let t167 = t166 * t81;
            let t170 = t76 * t124;
            let t173 = t8 * t149;
            let t174 = t62 * t91;
            let t178 = t124 * param_d;
            let t185 = t9 * t9;
            let t187 = f64x8::splat(1.0) / t10 / t185;
            let t188 = v_sigma * t187;
            let t193 = t91 * param_d;
            let t194 = t193 * t149;
            let t195 = f64x8::splat(1.0) / t85;
            let t196 = t194 * t195;
            let t198 = t92 * t12;
            let t200 = t15 * t154;
            let t202 = -f64x8::splat(7.0) / f64x8::splat(324.0) * t196 + f64x8::splat(7.0) / f64x8::splat(108.0) * t198 - f64x8::splat(7.0) / f64x8::splat(162.0) * t200;
            let t207 = -t196 / f64x8::splat(81.0) + t198 / f64x8::splat(27.0) - f64x8::splat(2.0) / f64x8::splat(81.0) * t200;
            let t208 = t207 * v_sigma;
            let t213 = t187 * t31;
            let t219 = f64x8::splat(2.0) / f64x8::splat(9.0) * t196 - f64x8::splat(2.0) / f64x8::splat(3.0) * t198 + f64x8::splat(4.0) / f64x8::splat(9.0) * t200;
            let t220 = t219 * v_sigma;
            let t225 = t187 * t44;
            let t231 = t49 * t187 * t31;
            let t237 = -f64x8::splat(88.0) / f64x8::splat(9.0) * t188 * t18 + f64x8::splat(16.0) / f64x8::splat(3.0) * t88 * t98 - t13 * t202 + t208 * t37 / f64x8::splat(8.0) - f64x8::splat(2.0) / f64x8::splat(3.0) * t101 * t104 + f64x8::splat(11.0) / f64x8::splat(9.0) * t36 * t213 + t220 * t45 / f64x8::splat(144.0) - t109 * t112 / f64x8::splat(27.0) + f64x8::splat(11.0) / f64x8::splat(162.0) * t41 * t225 - t48 * (f64x8::splat(352.0) / f64x8::splat(27.0) * t50 * t213 - f64x8::splat(44.0) / f64x8::splat(9.0) * t54 * t231) / f64x8::splat(8.0);
            let t240 = -f64x8::splat(2.0) / f64x8::splat(9.0) * t150 * t12 + f64x8::splat(4.0) / f64x8::splat(9.0) * t69 * t154 - f64x8::splat(4.0) / f64x8::splat(9.0) * t157 * t77 + t162 * t77 / f64x8::splat(9.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t165 * t167 + f64x8::splat(2.0) / f64x8::splat(3.0) * t75 * t170 + f64x8::splat(2.0) / f64x8::splat(9.0) * t173 * t174 * t12 + f64x8::splat(2.0) / f64x8::splat(3.0) * t80 * t178 * t71 - f64x8::splat(4.0) / f64x8::splat(9.0) * t80 * t81 * t154 + t8 * t5 * t237;
            let tv2rho20 = f64x8::splat(2.0) * param_a * t127 + t66 * t240;
            acc_v2rho2 = tv2rho20;
            let t242 = param_a * param_b;
            let t245 = t2 * param_a * param_b;
            let t246 = param_c * t7;
            let t247 = t5 * t144;
            let t252 = t166 * t144 * param_d;
            let t276 = f64x8::splat(8.0) / f64x8::splat(3.0) * t87 * t18 - t12 * t98 + t100 * t12 * t31 / f64x8::splat(8.0) - t35 * t87 * t31 / f64x8::splat(3.0) + t108 * t12 * t44 / f64x8::splat(144.0) - t40 * t87 * t44 / f64x8::splat(54.0) - t48 * (-f64x8::splat(32.0) / f64x8::splat(9.0) * t118 + f64x8::splat(4.0) / f64x8::splat(3.0) * t138 * t104) / f64x8::splat(8.0);
            let t277 = t76 * t276;
            let tv2rhosigma0 = t242 * t145 + t245 * t246 * t247 / f64x8::splat(3.0) + t245 * t252 / f64x8::splat(3.0) + t129 * t277;
            acc_v2rhosigma = tv2rhosigma0;
            let tv2sigma20 = f64x8::splat(0.0);
            acc_v2sigma2 = tv2sigma20;
            let t284 = f64x8::splat(1.0) / t1 / t85;
            let t287 = t185 * v_rho;
            let t289 = f64x8::splat(1.0) / t10 / t287;
            let t290 = v_sigma * t289;
            let t297 = t91 * t91;
            let t298 = t67 * t67;
            let t299 = f64x8::splat(1.0) / t298;
            let t300 = t297 * t299;
            let t302 = f64x8::splat(1.0) / t1 / t185;
            let t303 = t300 * t302;
            let t305 = f64x8::splat(1.0) / t185;
            let t306 = t194 * t305;
            let t308 = t92 * t87;
            let t310 = t15 * t284;
            let t312 = -f64x8::splat(7.0) / f64x8::splat(324.0) * t303 + f64x8::splat(35.0) / f64x8::splat(324.0) * t306 - f64x8::splat(91.0) / f64x8::splat(486.0) * t308 + f64x8::splat(49.0) / f64x8::splat(486.0) * t310;
            let t318 = -t303 / f64x8::splat(81.0) + f64x8::splat(5.0) / f64x8::splat(81.0) * t306 - f64x8::splat(26.0) / f64x8::splat(243.0) * t308 + f64x8::splat(14.0) / f64x8::splat(243.0) * t310;
            let t319 = t318 * v_sigma;
            let t325 = t289 * t31;
            let t332 = f64x8::splat(2.0) / f64x8::splat(9.0) * t303 - f64x8::splat(10.0) / f64x8::splat(9.0) * t306 + f64x8::splat(52.0) / f64x8::splat(27.0) * t308 - f64x8::splat(28.0) / f64x8::splat(27.0) * t310;
            let t333 = t332 * v_sigma;
            let t340 = t289 * t44;
            let t346 = t49 * t289 * t31;
            let t352 = f64x8::splat(1232.0) / f64x8::splat(27.0) * t290 * t18 - f64x8::splat(88.0) / f64x8::splat(3.0) * t188 * t98 + f64x8::splat(8.0) * t88 * t202 - t13 * t312 + t319 * t37 / f64x8::splat(8.0) - t208 * t104 + f64x8::splat(11.0) / f64x8::splat(3.0) * t101 * t213 - f64x8::splat(154.0) / f64x8::splat(27.0) * t36 * t325 + t333 * t45 / f64x8::splat(144.0) - t220 * t112 / f64x8::splat(18.0) + f64x8::splat(11.0) / f64x8::splat(54.0) * t109 * t225 - f64x8::splat(77.0) / f64x8::splat(243.0) * t41 * t340 - t48 * (-f64x8::splat(4928.0) / f64x8::splat(81.0) * t50 * t325 + f64x8::splat(616.0) / f64x8::splat(27.0) * t54 * t346) / f64x8::splat(8.0);
            let t355 = t299 * t193;
            let t358 = t74 * t284;
            let t361 = t74 * t87;
            let t364 = t161 * t305;
            let t367 = t166 * t178;
            let t370 = t74 * t305;
            let t371 = t7 * t149;
            let t372 = t371 * t174;
            let t386 = t161 * t87;
            let t391 = t160 * param_c;
            let t392 = param_b * t391;
            let t393 = t392 * t305;
            let t396 = t76 * t237;
            let t398 = t124 * t91;
            let t402 = t8 * t299;
            let t403 = t62 * t193;
            let t407 = t237 * param_d;
            let t410 = f64x8::splat(8.0) / f64x8::splat(9.0) * t150 * t87 - f64x8::splat(28.0) / f64x8::splat(27.0) * t69 * t284 + t8 * t5 * t352 - f64x8::splat(2.0) / f64x8::splat(9.0) * t355 * t305 + f64x8::splat(28.0) / f64x8::splat(27.0) * t358 * t77 - f64x8::splat(8.0) / f64x8::splat(9.0) * t361 * t167 + t364 * t167 / f64x8::splat(9.0) + f64x8::splat(2.0) / f64x8::splat(3.0) * t165 * t367 + f64x8::splat(2.0) / f64x8::splat(9.0) * t370 * t372 - f64x8::splat(8.0) / f64x8::splat(9.0) * t173 * t174 * t87 - f64x8::splat(4.0) / f64x8::splat(3.0) * t80 * t178 * t154 + f64x8::splat(28.0) / f64x8::splat(27.0) * t80 * t81 * t284 - f64x8::splat(4.0) / f64x8::splat(3.0) * t157 * t170 - f64x8::splat(4.0) / f64x8::splat(9.0) * t386 * t77 + t162 * t170 / f64x8::splat(3.0) + t393 * t77 / f64x8::splat(27.0) + t75 * t396 + f64x8::splat(2.0) / f64x8::splat(3.0) * t173 * t398 * t12 + f64x8::splat(2.0) / f64x8::splat(9.0) * t402 * t403 * t305 + t80 * t407 * t71;
            let tv3rho30 = f64x8::splat(3.0) * param_a * t240 + t66 * t410;
            acc_v3rho3 = tv3rho30;
            let t412 = t242 * param_c;
            let t413 = t71 * t7;
            let t417 = t242 * t7;
            let t418 = t68 * t144;
            let t419 = param_d * t71;
            let t425 = t94 * param_a;
            let t426 = t425 * param_b;
            let t427 = t160 * t7;
            let t431 = t425 * t74;
            let t434 = t5 * t276;
            let t439 = t371 * t144 * t91;
            let t443 = t166 * t276 * param_d;
            let t475 = -f64x8::splat(88.0) / f64x8::splat(9.0) * t187 * t18 + f64x8::splat(16.0) / f64x8::splat(3.0) * t87 * t98 - t12 * t202 + t207 * t12 * t31 / f64x8::splat(8.0) - f64x8::splat(2.0) / f64x8::splat(3.0) * t100 * t87 * t31 + f64x8::splat(11.0) / f64x8::splat(9.0) * t35 * t187 * t31 + t219 * t12 * t44 / f64x8::splat(144.0) - t108 * t87 * t44 / f64x8::splat(27.0) + f64x8::splat(11.0) / f64x8::splat(162.0) * t40 * t187 * t44 - t48 * (f64x8::splat(352.0) / f64x8::splat(27.0) * t231 - f64x8::splat(44.0) / f64x8::splat(9.0) * t138 * t213) / f64x8::splat(8.0);
            let t476 = t76 * t475;
            let tv3rho2sigma0 = f64x8::splat(2.0) / f64x8::splat(9.0) * t412 * t413 * t247 + f64x8::splat(2.0) / f64x8::splat(9.0) * t417 * t418 * t419 + f64x8::splat(2.0) * t242 * t277 + t426 * t427 * t247 / f64x8::splat(9.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t431 * t252 + f64x8::splat(2.0) / f64x8::splat(3.0) * t245 * t246 * t434 + f64x8::splat(2.0) / f64x8::splat(9.0) * t426 * t439 + f64x8::splat(2.0) / f64x8::splat(3.0) * t245 * t443 + t129 * t476;
            acc_v3rho2sigma = tv3rho2sigma0;
            let tv3rhosigma20 = f64x8::splat(0.0);
            acc_v3rhosigma2 = tv3rhosigma20;
            let tv3sigma30 = f64x8::splat(0.0);
            acc_v3sigma3 = tv3sigma30;
            let t481 = f64x8::splat(1.0) / t298 / t4;
            let t484 = f64x8::splat(1.0) / t1 / t287;
            let t491 = f64x8::splat(1.0) / t287;
            let t502 = f64x8::splat(1.0) / t10 / t185 / t9;
            let t503 = t502 * t31;
            let t523 = t297 * param_d * t481 * t289;
            let t525 = t300 * t484;
            let t527 = t194 * t491;
            let t529 = t92 * t187;
            let t531 = t15 * t302;
            let t565 = -f64x8::splat(4.0) / f64x8::splat(3.0) * t319 * t104 + f64x8::splat(22.0) / f64x8::splat(3.0) * t208 * t213 - f64x8::splat(616.0) / f64x8::splat(27.0) * t101 * t325 + f64x8::splat(2618.0) / f64x8::splat(81.0) * t36 * t503 - f64x8::splat(2.0) / f64x8::splat(27.0) * t333 * t112 + f64x8::splat(11.0) / f64x8::splat(27.0) * t220 * t225 - f64x8::splat(308.0) / f64x8::splat(243.0) * t109 * t340 + f64x8::splat(1309.0) / f64x8::splat(729.0) * t41 * t502 * t44 + f64x8::splat(4928.0) / f64x8::splat(27.0) * t290 * t98 - f64x8::splat(176.0) / f64x8::splat(3.0) * t188 * t202 + f64x8::splat(32.0) / f64x8::splat(3.0) * t88 * t312 - t13 * (-f64x8::splat(7.0) / f64x8::splat(243.0) * t523 + f64x8::splat(49.0) / f64x8::splat(243.0) * t525 - f64x8::splat(406.0) / f64x8::splat(729.0) * t527 + f64x8::splat(175.0) / f64x8::splat(243.0) * t529 - f64x8::splat(245.0) / f64x8::splat(729.0) * t531) - f64x8::splat(20944.0) / f64x8::splat(81.0) * v_sigma * t502 * t18 + (-f64x8::splat(4.0) / f64x8::splat(243.0) * t523 + f64x8::splat(28.0) / f64x8::splat(243.0) * t525 - f64x8::splat(232.0) / f64x8::splat(729.0) * t527 + f64x8::splat(100.0) / f64x8::splat(243.0) * t529 - f64x8::splat(140.0) / f64x8::splat(729.0) * t531) * v_sigma * t37 / f64x8::splat(8.0) + (f64x8::splat(8.0) / f64x8::splat(27.0) * t523 - f64x8::splat(56.0) / f64x8::splat(27.0) * t525 + f64x8::splat(464.0) / f64x8::splat(81.0) * t527 - f64x8::splat(200.0) / f64x8::splat(27.0) * t529 + f64x8::splat(280.0) / f64x8::splat(81.0) * t531) * v_sigma * t45 / f64x8::splat(144.0) - t48 * (f64x8::splat(83776.0) / f64x8::splat(243.0) * t50 * t503 - f64x8::splat(10472.0) / f64x8::splat(81.0) * t54 * t49 * t502 * t31) / f64x8::splat(8.0);
            let t594 = t7 * t299;
            let t604 = -f64x8::splat(8.0) / f64x8::splat(27.0) * t481 * t297 * t484 - f64x8::splat(320.0) / f64x8::splat(81.0) * t150 * t187 + f64x8::splat(280.0) / f64x8::splat(81.0) * t69 * t302 + f64x8::splat(16.0) / f64x8::splat(9.0) * t355 * t491 + t8 * t5 * t565 + f64x8::splat(320.0) / f64x8::splat(81.0) * t74 * t187 * t167 - f64x8::splat(32.0) / f64x8::splat(9.0) * t361 * t367 - f64x8::splat(8.0) / f64x8::splat(9.0) * t161 * t491 * t167 - f64x8::splat(16.0) / f64x8::splat(9.0) * t74 * t491 * t372 + f64x8::splat(4.0) / f64x8::splat(9.0) * t364 * t367 + f64x8::splat(4.0) / f64x8::splat(81.0) * t392 * t484 * t167 + f64x8::splat(4.0) / f64x8::splat(27.0) * t161 * t484 * t372 + f64x8::splat(4.0) / f64x8::splat(3.0) * t165 * t166 * t407 + f64x8::splat(8.0) / f64x8::splat(9.0) * t370 * t371 * t398 + f64x8::splat(8.0) / f64x8::splat(27.0) * t74 * t484 * t594 * t403 - f64x8::splat(280.0) / f64x8::splat(81.0) * t74 * t302 * t77 + f64x8::splat(320.0) / f64x8::splat(81.0) * t173 * t174 * t187;
            let t643 = t160 * t160;
            let t661 = f64x8::splat(112.0) / f64x8::splat(27.0) * t80 * t178 * t284 - f64x8::splat(280.0) / f64x8::splat(81.0) * t80 * t81 * t302 + f64x8::splat(4.0) / f64x8::splat(3.0) * t75 * t76 * t352 + f64x8::splat(4.0) / f64x8::splat(3.0) * t80 * t352 * param_d * t71 + f64x8::splat(112.0) / f64x8::splat(27.0) * t358 * t170 + f64x8::splat(160.0) / f64x8::splat(81.0) * t161 * t187 * t77 - f64x8::splat(32.0) / f64x8::splat(9.0) * t173 * t398 * t87 - f64x8::splat(16.0) / f64x8::splat(9.0) * t402 * t403 * t491 - f64x8::splat(8.0) / f64x8::splat(3.0) * t80 * t407 * t154 - f64x8::splat(8.0) / f64x8::splat(3.0) * t157 * t396 - f64x8::splat(16.0) / f64x8::splat(9.0) * t386 * t170 - f64x8::splat(8.0) / f64x8::splat(27.0) * t392 * t491 * t77 + f64x8::splat(2.0) / f64x8::splat(3.0) * t162 * t396 + f64x8::splat(4.0) / f64x8::splat(27.0) * t393 * t170 + param_b * t643 * t484 * t77 / f64x8::splat(81.0) + f64x8::splat(4.0) / f64x8::splat(3.0) * t173 * t237 * t91 * t12 + f64x8::splat(8.0) / f64x8::splat(9.0) * t402 * t124 * t193 * t305 + f64x8::splat(8.0) / f64x8::splat(27.0) * t8 * t481 * t62 * t297 * t484;
            let tv4rho40 = f64x8::splat(4.0) * param_a * t410 + t66 * (t604 + t661);
            acc_v4rho4 = tv4rho40;
            let t684 = t195 * param_a;
            let t685 = t684 * param_b;
            let t762 = f64x8::splat(1232.0) / f64x8::splat(27.0) * t289 * t18 - f64x8::splat(88.0) / f64x8::splat(3.0) * t187 * t98 + f64x8::splat(8.0) * t87 * t202 - t12 * t312 + t318 * t12 * t31 / f64x8::splat(8.0) - t207 * t87 * t31 + f64x8::splat(11.0) / f64x8::splat(3.0) * t100 * t187 * t31 - f64x8::splat(154.0) / f64x8::splat(27.0) * t35 * t289 * t31 + t332 * t12 * t44 / f64x8::splat(144.0) - t219 * t87 * t44 / f64x8::splat(18.0) + f64x8::splat(11.0) / f64x8::splat(54.0) * t108 * t187 * t44 - f64x8::splat(77.0) / f64x8::splat(243.0) * t40 * t289 * t44 - t48 * (-f64x8::splat(4928.0) / f64x8::splat(81.0) * t346 + f64x8::splat(616.0) / f64x8::splat(27.0) * t138 * t325) / f64x8::splat(8.0);
            let tv4rho3sigma0 = f64x8::splat(2.0) / f64x8::splat(3.0) * t412 * t413 * t434 - t242 * t160 * t12 * t7 * t247 / f64x8::splat(9.0) + f64x8::splat(2.0) / f64x8::splat(3.0) * t417 * t68 * t276 * t419 - f64x8::splat(2.0) / f64x8::splat(9.0) * t417 * t149 * t144 * t91 * t12 + t426 * t427 * t434 / f64x8::splat(3.0) + t685 * t391 * t7 * t247 / f64x8::splat(27.0) + t245 * t246 * t5 * t475 + f64x8::splat(2.0) / f64x8::splat(3.0) * t426 * t371 * t276 * t91 + f64x8::splat(2.0) / f64x8::splat(9.0) * t685 * t594 * t144 * t193 + t245 * t166 * t475 * param_d - f64x8::splat(8.0) / f64x8::splat(27.0) * t412 * t154 * t7 * t247 - f64x8::splat(2.0) / f64x8::splat(9.0) * t242 * param_c * t12 * t252 - f64x8::splat(8.0) / f64x8::splat(27.0) * t417 * t418 * param_d * t154 + t684 * t161 * t252 / f64x8::splat(9.0) + f64x8::splat(2.0) / f64x8::splat(3.0) * t431 * t443 + f64x8::splat(2.0) / f64x8::splat(9.0) * t684 * t74 * t439 + f64x8::splat(3.0) * t242 * t476 + t129 * t76 * t762;
            acc_v4rho3sigma = tv4rho3sigma0;
            let tv4rho2sigma20 = f64x8::splat(0.0);
            acc_v4rho2sigma2 = tv4rho2sigma20;
            let tv4rhosigma30 = f64x8::splat(0.0);
            acc_v4rhosigma3 = tv4rhosigma30;
            let tv4sigma40 = f64x8::splat(0.0);
            acc_v4sigma4 = tv4sigma40;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rho2.into(); v2rho2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rhosigma.into(); v2rhosigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2sigma2.into(); v2sigma2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v3rho3.into(); v3rho3[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v3rho2sigma.into(); v3rho2sigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v3rhosigma2.into(); v3rhosigma2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v3sigma3.into(); v3sigma3[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v4rho4.into(); v4rho4[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v4rho3sigma.into(); v4rho3sigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v4rho2sigma2.into(); v4rho2sigma2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v4rhosigma3.into(); v4rhosigma3[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v4sigma4.into(); v4sigma4[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}

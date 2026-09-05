//! GGA_X_HJS_B88_V2 vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_hjs_b88_v2.c`
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
pub fn gga_x_hjs_b88_v2_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_a_0: f64,
    param_a_1: f64,
    param_a_2: f64,
    param_a_3: f64,
    param_a_4: f64,
    param_a_5: f64,
    param_b_0: f64,
    param_b_1: f64,
    param_b_2: f64,
    param_b_3: f64,
    param_b_4: f64,
    param_b_5: f64,
    param_b_6: f64,
    param_b_7: f64,
    param_b_8: f64,
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_a_0 = f64x8::splat(param_a_0);
    let param_a_1 = f64x8::splat(param_a_1);
    let param_a_2 = f64x8::splat(param_a_2);
    let param_a_3 = f64x8::splat(param_a_3);
    let param_a_4 = f64x8::splat(param_a_4);
    let param_a_5 = f64x8::splat(param_a_5);
    let param_b_0 = f64x8::splat(param_b_0);
    let param_b_1 = f64x8::splat(param_b_1);
    let param_b_2 = f64x8::splat(param_b_2);
    let param_b_3 = f64x8::splat(param_b_3);
    let param_b_4 = f64x8::splat(param_b_4);
    let param_b_5 = f64x8::splat(param_b_5);
    let param_b_6 = f64x8::splat(param_b_6);
    let param_b_7 = f64x8::splat(param_b_7);
    let param_b_8 = f64x8::splat(param_b_8);
    let param_hyb_omega_0 = f64x8::splat(param_hyb_omega_0);
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
        {
            let t2 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = f64x8::splat(M_CBRTPI);
            let t6 = t3 / t4;
            let t7 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t8 = zeta_threshold - f64x8::splat(1.0);
            let t10 = ((t7).select(t8, (t7).select(-t8, f64x8::splat(0.0))));
            let t11 = f64x8::splat(1.0) + t10;
            let t12 = (t11).simd_le(zeta_threshold);
            let t13 = (simd::cbrt(zeta_threshold));
            let t15 = (simd::cbrt(t11));
            let t17 = ((t12).select(t13 * zeta_threshold, t15 * t11));
            let t18 = (simd::cbrt(v_rho));
            let t19 = t17 * t18;
            let t20 = t3 * t3;
            let t21 = param_hyb_omega_0 * t20;
            let t22 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t23 = (simd::cbrt(t22));
            let t24 = f64x8::splat(1.0) / t23;
            let t25 = t21 * t24;
            let t26 = ((t12).select(t13, t15));
            let t27 = f64x8::splat(1.0) / t26;
            let t28 = f64x8::splat(1.0) / t18;
            let t29 = t27 * t28;
            let t30 = f64x8::splat(M_CBRT6);
            let t31 = t30 * t30;
            let t32 = t31 * t24;
            let t33 = ((v_sigma).sqrt());
            let t34 = f64x8::splat(M_CBRT2);
            let t35 = t33 * t34;
            let t37 = f64x8::splat(1.0) / t18 / v_rho;
            let t41 = (simd::exp(-t32 * t35 * t37 / f64x8::splat(12.0)));
            let t42 = (simd::exp(f64x8::splat(20.0)));
            let t44 = f64x8::splat(1.0) / (t42 - f64x8::splat(1.0));
            let t45 = t41 + t44;
            let t49 = (simd::ln(t45 / (f64x8::splat(1.0) + t44)));
            let t50 = t49 * t49;
            let t51 = param_a_0;
            let t53 = param_a_1;
            let t54 = t50 * t49;
            let t56 = param_a_2;
            let t57 = t50 * t50;
            let t59 = param_a_3;
            let t60 = t57 * t49;
            let t62 = param_a_4;
            let t63 = t57 * t50;
            let t65 = param_a_5;
            let t66 = t57 * t54;
            let t68 = t50 * t51 - t53 * t54 + t56 * t57 - t59 * t60 + t62 * t63 - t65 * t66;
            let t69 = t50 * t68;
            let t70 = param_b_0;
            let t72 = param_b_1;
            let t74 = param_b_2;
            let t76 = param_b_3;
            let t78 = param_b_4;
            let t80 = param_b_5;
            let t82 = param_b_6;
            let t84 = param_b_7;
            let t85 = t57 * t57;
            let t87 = param_b_8;
            let t90 = -t49 * t85 * t87 - t49 * t70 + t50 * t72 - t54 * t74 + t57 * t76 - t60 * t78 + t63 * t80 - t66 * t82 + t84 * t85 + f64x8::splat(1.0);
            let t91 = f64x8::splat(1.0) / t90;
            let t92 = t69 * t91;
            let t93 = (f64x8::splat(1e-10)).simd_lt(t92);
            let t94 = ((t93).select(t92, f64x8::splat(1e-10)));
            let t95 = param_hyb_omega_0 * param_hyb_omega_0;
            let t96 = t95 * t3;
            let t97 = t23 * t23;
            let t98 = f64x8::splat(1.0) / t97;
            let t99 = t26 * t26;
            let t101 = t98 / t99;
            let t102 = t18 * t18;
            let t103 = f64x8::splat(1.0) / t102;
            let t105 = t96 * t101 * t103;
            let t107 = f64x8::splat(0.60965) + t94 + t105 / f64x8::splat(3.0);
            let t108 = ((t107).sqrt());
            let t109 = f64x8::splat(1.0) / t108;
            let t111 = t25 * t29 * t109;
            let t113 = f64x8::splat(1.0) - t111 / f64x8::splat(3.0);
            let t114 = f64x8::splat(0.60965) + t94;
            let t115 = f64x8::splat(1.0) / t114;
            let t119 = f64x8::splat(1.0) + t50 / f64x8::splat(4.0);
            let t120 = f64x8::splat(1.0) / t119;
            let t124 = f64x8::splat(1.0) + f64x8::splat(0.3121563353845126) * t50 * t120 + f64x8::splat(4.21411052769092) * t94;
            let t126 = f64x8::splat(1.0) / t22;
            let t127 = t95 * param_hyb_omega_0 * t126;
            let t129 = f64x8::splat(1.0) / t99 / t26;
            let t130 = f64x8::splat(1.0) / v_rho;
            let t131 = t129 * t130;
            let t133 = f64x8::splat(1.0) / t108 / t107;
            let t135 = t127 * t131 * t133;
            let t137 = f64x8::splat(2.0) - t111 + t135 / f64x8::splat(3.0);
            let t138 = t124 * t137;
            let t139 = t114 * t114;
            let t140 = f64x8::splat(1.0) / t139;
            let t146 = t139 * t114;
            let t148 = ((t114).sqrt());
            let t149 = t148 * t146;
            let t150 = ((f64x8::splat(M_PI)).sqrt());
            let t152 = ((t94).sqrt());
            let t155 = (f64x8::splat(0.0)).simd_lt(f64x8::splat(0.7572109999) + t94);
            let t157 = ((t155).select(f64x8::splat(0.757211) + t94, f64x8::splat(1e-10)));
            let t158 = ((t157).sqrt());
            let t160 = f64x8::splat(4.0) / f64x8::splat(5.0) * t150 + f64x8::splat(12.0) / f64x8::splat(5.0) * t152 - f64x8::splat(12.0) / f64x8::splat(5.0) * t158;
            let t162 = f64x8::splat(0.0474596) * t124 * t114 + f64x8::splat(0.028363733333333332) * t139 - f64x8::splat(0.9086532) * t146 - t149 * t160;
            let t165 = t95 * t95;
            let t167 = t165 * param_hyb_omega_0 * t3;
            let t169 = f64x8::splat(1.0) / t97 / t22;
            let t170 = t167 * t169;
            let t171 = t99 * t99;
            let t173 = f64x8::splat(1.0) / t171 / t26;
            let t175 = f64x8::splat(1.0) / t102 / v_rho;
            let t176 = t173 * t175;
            let t177 = t107 * t107;
            let t179 = f64x8::splat(1.0) / t108 / t177;
            let t183 = f64x8::splat(8.0) - f64x8::splat(5.0) * t111 + f64x8::splat(10.0) / f64x8::splat(3.0) * t135 - t170 * t176 * t179 / f64x8::splat(3.0);
            let t184 = t162 * t183;
            let t185 = f64x8::splat(1.0) / t146;
            let t189 = f64x8::splat(3.0) * t105;
            let t190 = f64x8::splat(9.0) * t94 + t189;
            let t191 = ((t190).sqrt());
            let t193 = f64x8::splat(9.0) * t157 + t189;
            let t194 = ((t193).sqrt());
            let t196 = t191 / f64x8::splat(3.0) - t194 / f64x8::splat(3.0);
            let t200 = t24 * t27;
            let t202 = t21 * t200 * t28;
            let t204 = t202 / f64x8::splat(3.0) + t191 / f64x8::splat(3.0);
            let t206 = t202 / f64x8::splat(3.0) + t108;
            let t207 = f64x8::splat(1.0) / t206;
            let t209 = (simd::ln(t204 * t207));
            let t213 = t202 / f64x8::splat(3.0) + t194 / f64x8::splat(3.0);
            let t215 = (simd::ln(t213 * t207));
            let t218 = f64x8::splat(0.757211) + f64x8::splat(0.04727288888888889) * t113 * t115 + f64x8::splat(0.026366444444444446) * t138 * t140 - t184 * t185 / f64x8::splat(9.0) + f64x8::splat(2.0) / f64x8::splat(3.0) * t25 * t29 * t196 + f64x8::splat(2.0) * t94 * t209 - f64x8::splat(2.0) * t157 * t215;
            let t222 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t218));
            let tzk0 = f64x8::splat(2.0) * t222;
            acc_zk = tzk0;
            let t223 = t17 * t103;
            let t227 = t27 * t37;
            let t229 = t25 * t227 * t109;
            let t231 = t49 * t68;
            let t233 = t91 * t31 * t24;
            let t234 = t231 * t233;
            let t235 = v_rho * v_rho;
            let t237 = f64x8::splat(1.0) / t18 / t235;
            let t239 = f64x8::splat(1.0) / t45;
            let t240 = t237 * t41 * t239;
            let t241 = t35 * t240;
            let t244 = t51 * t49;
            let t245 = t244 * t32;
            let t248 = t53 * t50;
            let t249 = t248 * t32;
            let t252 = t56 * t54;
            let t253 = t252 * t32;
            let t256 = t59 * t57;
            let t257 = t256 * t32;
            let t260 = t62 * t60;
            let t261 = t260 * t32;
            let t264 = t65 * t63;
            let t265 = t264 * t32;
            let t268 = f64x8::splat(2.0) / f64x8::splat(9.0) * t245 * t241 - t249 * t241 / f64x8::splat(3.0) + f64x8::splat(4.0) / f64x8::splat(9.0) * t253 * t241 - f64x8::splat(5.0) / f64x8::splat(9.0) * t257 * t241 + f64x8::splat(2.0) / f64x8::splat(3.0) * t261 * t241 - f64x8::splat(7.0) / f64x8::splat(9.0) * t265 * t241;
            let t269 = t50 * t268;
            let t271 = t90 * t90;
            let t272 = f64x8::splat(1.0) / t271;
            let t273 = t70 * t31;
            let t274 = t24 * t33;
            let t275 = t273 * t274;
            let t276 = t34 * t237;
            let t277 = t41 * t239;
            let t278 = t276 * t277;
            let t281 = t72 * t49;
            let t282 = t281 * t32;
            let t285 = t74 * t50;
            let t286 = t285 * t32;
            let t289 = t76 * t54;
            let t290 = t289 * t32;
            let t293 = t78 * t57;
            let t294 = t293 * t32;
            let t297 = t80 * t60;
            let t298 = t297 * t32;
            let t301 = t82 * t63;
            let t302 = t301 * t32;
            let t305 = t84 * t66;
            let t306 = t305 * t32;
            let t309 = t87 * t85;
            let t310 = t309 * t32;
            let t312 = -t275 * t278 / f64x8::splat(9.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t282 * t241 - t286 * t241 / f64x8::splat(3.0) + f64x8::splat(4.0) / f64x8::splat(9.0) * t290 * t241 - f64x8::splat(5.0) / f64x8::splat(9.0) * t294 * t241 + f64x8::splat(2.0) / f64x8::splat(3.0) * t298 * t241 - f64x8::splat(7.0) / f64x8::splat(9.0) * t302 * t241 + f64x8::splat(8.0) / f64x8::splat(9.0) * t306 * t241 - t310 * t241;
            let t313 = t272 * t312;
            let t316 = ((t93).select(f64x8::splat(2.0) / f64x8::splat(9.0) * t234 * t241 + t269 * t91 - t69 * t313, f64x8::splat(0.0)));
            let t318 = t96 * t101 * t175;
            let t320 = t316 - f64x8::splat(2.0) / f64x8::splat(9.0) * t318;
            let t321 = t133 * t320;
            let t323 = t25 * t29 * t321;
            let t325 = t229 / f64x8::splat(9.0) + t323 / f64x8::splat(6.0);
            let t328 = t113 * t140;
            let t331 = t49 * t120;
            let t332 = t331 * t32;
            let t335 = t119 * t119;
            let t336 = f64x8::splat(1.0) / t335;
            let t337 = t54 * t336;
            let t338 = t337 * t32;
            let t342 = f64x8::splat(0.06936807452989169) * t332 * t241 - f64x8::splat(0.017342018632472922) * t338 * t241 + f64x8::splat(4.21411052769092) * t316;
            let t343 = t342 * t137;
            let t348 = f64x8::splat(1.0) / t235;
            let t351 = t127 * t129 * t348 * t133;
            let t353 = t127 * t129;
            let t354 = t130 * t179;
            let t356 = t353 * t354 * t320;
            let t358 = t229 / f64x8::splat(3.0) + t323 / f64x8::splat(2.0) - t351 / f64x8::splat(3.0) - t356 / f64x8::splat(2.0);
            let t359 = t124 * t358;
            let t362 = t185 * t316;
            let t369 = t114 * t316;
            let t373 = t148 * t139;
            let t374 = t373 * t160;
            let t377 = f64x8::splat(1.0) / t152;
            let t379 = f64x8::splat(1.0) / t158;
            let t380 = ((t155).select(t316, f64x8::splat(0.0)));
            let t383 = f64x8::splat(6.0) / f64x8::splat(5.0) * t377 * t316 - f64x8::splat(6.0) / f64x8::splat(5.0) * t379 * t380;
            let t385 = f64x8::splat(0.0474596) * t342 * t114 + f64x8::splat(0.0474596) * t124 * t316 + f64x8::splat(0.056727466666666664) * t369 - f64x8::splat(2.7259596) * t139 * t316 - f64x8::splat(7.0) / f64x8::splat(2.0) * t374 * t316 - t149 * t383;
            let t386 = t385 * t183;
            let t394 = f64x8::splat(1.0) / t102 / t235;
            let t395 = t173 * t394;
            let t399 = t177 * t107;
            let t401 = f64x8::splat(1.0) / t108 / t399;
            let t402 = t401 * t320;
            let t406 = f64x8::splat(5.0) / f64x8::splat(3.0) * t229 + f64x8::splat(5.0) / f64x8::splat(2.0) * t323 - f64x8::splat(10.0) / f64x8::splat(3.0) * t351 - f64x8::splat(5.0) * t356 + f64x8::splat(5.0) / f64x8::splat(9.0) * t170 * t395 * t179 + f64x8::splat(5.0) / f64x8::splat(6.0) * t170 * t176 * t402;
            let t407 = t162 * t406;
            let t410 = t139 * t139;
            let t411 = f64x8::splat(1.0) / t410;
            let t412 = t411 * t316;
            let t418 = f64x8::splat(1.0) / t191;
            let t420 = f64x8::splat(2.0) * t318;
            let t421 = f64x8::splat(9.0) * t316 - t420;
            let t422 = t418 * t421;
            let t423 = f64x8::splat(1.0) / t194;
            let t425 = f64x8::splat(9.0) * t380 - t420;
            let t426 = t423 * t425;
            let t428 = t422 / f64x8::splat(6.0) - t426 / f64x8::splat(6.0);
            let t435 = t21 * t200 * t37;
            let t436 = t435 / f64x8::splat(9.0);
            let t438 = -t436 + t422 / f64x8::splat(6.0);
            let t440 = t206 * t206;
            let t441 = f64x8::splat(1.0) / t440;
            let t442 = t204 * t441;
            let t445 = -t436 + t109 * t320 / f64x8::splat(2.0);
            let t447 = t207 * t438 - t442 * t445;
            let t448 = t94 * t447;
            let t449 = f64x8::splat(1.0) / t204;
            let t450 = t449 * t206;
            let t456 = -t436 + t426 / f64x8::splat(6.0);
            let t458 = t213 * t441;
            let t460 = t207 * t456 - t445 * t458;
            let t461 = t157 * t460;
            let t462 = f64x8::splat(1.0) / t213;
            let t463 = t462 * t206;
            let t466 = f64x8::splat(0.04727288888888889) * t325 * t115 - f64x8::splat(0.04727288888888889) * t328 * t316 + f64x8::splat(0.026366444444444446) * t343 * t140 + f64x8::splat(0.026366444444444446) * t359 * t140 - f64x8::splat(0.05273288888888889) * t138 * t362 - t386 * t185 / f64x8::splat(9.0) - t407 * t185 / f64x8::splat(9.0) + t184 * t412 / f64x8::splat(3.0) - f64x8::splat(2.0) / f64x8::splat(9.0) * t25 * t227 * t196 + f64x8::splat(2.0) / f64x8::splat(3.0) * t25 * t29 * t428 + f64x8::splat(2.0) * t316 * t209 + f64x8::splat(2.0) * t448 * t450 - f64x8::splat(2.0) * t380 * t215 - f64x8::splat(2.0) * t461 * t463;
            let t471 = ((t2).select(f64x8::splat(0.0), -t6 * t223 * t218 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t466));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t471 + f64x8::splat(2.0) * t222;
            acc_vrho = tvrho0;
            let t474 = t21 * t200;
            let t475 = t28 * t133;
            let t476 = f64x8::splat(1.0) / t33;
            let t477 = t476 * t34;
            let t479 = t37 * t41 * t239;
            let t480 = t477 * t479;
            let t495 = -t245 * t480 / f64x8::splat(12.0) + t249 * t480 / f64x8::splat(8.0) - t253 * t480 / f64x8::splat(6.0) + f64x8::splat(5.0) / f64x8::splat(24.0) * t257 * t480 - t261 * t480 / f64x8::splat(4.0) + f64x8::splat(7.0) / f64x8::splat(24.0) * t265 * t480;
            let t496 = t50 * t495;
            let t499 = t273 * t24 * t476;
            let t500 = t34 * t37;
            let t501 = t500 * t277;
            let t520 = t499 * t501 / f64x8::splat(24.0) - t282 * t480 / f64x8::splat(12.0) + t286 * t480 / f64x8::splat(8.0) - t290 * t480 / f64x8::splat(6.0) + f64x8::splat(5.0) / f64x8::splat(24.0) * t294 * t480 - t298 * t480 / f64x8::splat(4.0) + f64x8::splat(7.0) / f64x8::splat(24.0) * t302 * t480 - t306 * t480 / f64x8::splat(3.0) + f64x8::splat(3.0) / f64x8::splat(8.0) * t310 * t480;
            let t521 = t272 * t520;
            let t524 = ((t93).select(-t234 * t480 / f64x8::splat(12.0) + t496 * t91 - t69 * t521, f64x8::splat(0.0)));
            let t525 = t524 * t115;
            let t536 = -f64x8::splat(0.026013027948709383) * t332 * t480 + f64x8::splat(0.006503256987177346) * t338 * t480 + f64x8::splat(4.21411052769092) * t524;
            let t537 = t536 * t137;
            let t540 = t133 * t524;
            let t542 = t25 * t29 * t540;
            let t544 = t353 * t354 * t524;
            let t546 = t542 / f64x8::splat(2.0) - t544 / f64x8::splat(2.0);
            let t547 = t124 * t546;
            let t550 = t185 * t524;
            let t557 = t114 * t524;
            let t564 = ((t155).select(t524, f64x8::splat(0.0)));
            let t567 = f64x8::splat(6.0) / f64x8::splat(5.0) * t377 * t524 - f64x8::splat(6.0) / f64x8::splat(5.0) * t379 * t564;
            let t569 = f64x8::splat(0.0474596) * t536 * t114 + f64x8::splat(0.0474596) * t124 * t524 + f64x8::splat(0.056727466666666664) * t557 - f64x8::splat(2.7259596) * t139 * t524 - f64x8::splat(7.0) / f64x8::splat(2.0) * t374 * t524 - t149 * t567;
            let t570 = t569 * t183;
            let t575 = t401 * t524;
            let t579 = f64x8::splat(5.0) / f64x8::splat(2.0) * t542 - f64x8::splat(5.0) * t544 + f64x8::splat(5.0) / f64x8::splat(6.0) * t170 * t176 * t575;
            let t580 = t162 * t579;
            let t583 = t411 * t524;
            let t586 = t418 * t524;
            let t587 = t423 * t564;
            let t589 = f64x8::splat(3.0) / f64x8::splat(2.0) * t586 - f64x8::splat(3.0) / f64x8::splat(2.0) * t587;
            let t597 = t109 * t524;
            let t600 = f64x8::splat(3.0) / f64x8::splat(2.0) * t586 * t207 - t442 * t597 / f64x8::splat(2.0);
            let t601 = t94 * t600;
            let t610 = f64x8::splat(3.0) / f64x8::splat(2.0) * t587 * t207 - t458 * t597 / f64x8::splat(2.0);
            let t611 = t157 * t610;
            let t614 = f64x8::splat(0.007878814814814814) * t474 * t475 * t525 - f64x8::splat(0.04727288888888889) * t328 * t524 + f64x8::splat(0.026366444444444446) * t537 * t140 + f64x8::splat(0.026366444444444446) * t547 * t140 - f64x8::splat(0.05273288888888889) * t138 * t550 - t570 * t185 / f64x8::splat(9.0) - t580 * t185 / f64x8::splat(9.0) + t184 * t583 / f64x8::splat(3.0) + f64x8::splat(2.0) / f64x8::splat(3.0) * t25 * t29 * t589 + f64x8::splat(2.0) * t524 * t209 + f64x8::splat(2.0) * t601 * t450 - f64x8::splat(2.0) * t564 * t215 - f64x8::splat(2.0) * t611 * t463;
            let t618 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t614));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t618;
            acc_vsigma = tvsigma0;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        ip += 8;
    }
}

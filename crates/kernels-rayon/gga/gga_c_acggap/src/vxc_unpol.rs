//! GGA_C_ACGGAP vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_acggap.c`
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
pub fn gga_c_acggap_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
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
        {
            let t1 = f64x8::splat(M_CBRT3);
            let t2 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t3 = (simd::cbrt(t2));
            let t4 = t1 * t3;
            let t5 = f64x8::splat(M_CBRT4);
            let t6 = t5 * t5;
            let t7 = (simd::cbrt(v_rho));
            let t9 = t6 / t7;
            let t10 = t4 * t9;
            let t12 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t10;
            let t13 = ((t10).sqrt());
            let t16 = ((t10) * (t10).sqrt());
            let t18 = t1 * t1;
            let t19 = t3 * t3;
            let t20 = t18 * t19;
            let t21 = t7 * t7;
            let t24 = t20 * t5 / t21;
            let t26 = f64x8::splat(3.79785) * t13 + f64x8::splat(0.8969) * t10 + f64x8::splat(0.204775) * t16 + f64x8::splat(0.123235) * t24;
            let t29 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t26;
            let t30 = (simd::ln(t29));
            let t32 = f64x8::splat(0.0621814) * t12 * t30;
            let t33 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t34 = (simd::cbrt(zeta_threshold));
            let t36 = ((t33).select(t34 * zeta_threshold, f64x8::splat(1.0)));
            let t39 = f64x8::splat(M_CBRT2);
            let t43 = (f64x8::splat(2.0) * t36 - f64x8::splat(2.0)) / (f64x8::splat(2.0) * t39 - f64x8::splat(2.0));
            let t45 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t10;
            let t50 = f64x8::splat(5.1785) * t13 + f64x8::splat(0.905775) * t10 + f64x8::splat(0.1100325) * t16 + f64x8::splat(0.1241775) * t24;
            let t53 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t50;
            let t54 = (simd::ln(t53));
            let t57 = f64x8::splat(0.0197516734986138) * t43 * t45 * t54;
            let t58 = (simd::ln(f64x8::splat(2.0)));
            let t59 = f64x8::splat(1.0) - t58;
            let t60 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t61 = f64x8::splat(1.0) / t60;
            let t62 = t59 * t61;
            let t63 = t34 * t34;
            let t64 = ((t33).select(t63, f64x8::splat(1.0)));
            let t65 = t64 * t64;
            let t66 = t65 * t64;
            let t68 = f64x8::splat(1.0) + f64x8::splat(0.0416675) * t10;
            let t72 = f64x8::splat(1.0) + f64x8::splat(0.125) * t4 * t9 * t68;
            let t74 = f64x8::splat(1.0) + f64x8::splat(0.0740825) * t10;
            let t78 = f64x8::splat(1.0) + f64x8::splat(0.125) * t4 * t9 * t74;
            let t79 = f64x8::splat(1.0) / t78;
            let t80 = t72 * t79;
            let t81 = v_rho * v_rho;
            let t83 = f64x8::splat(1.0) / t7 / t81;
            let t84 = v_sigma * t83;
            let t85 = f64x8::splat(1.0) / t65;
            let t86 = t39 * t85;
            let t87 = t84 * t86;
            let t88 = f64x8::splat(1.0) / t3;
            let t89 = t18 * t88;
            let t90 = ((v_sigma).sqrt());
            let t92 = f64x8::splat(1.0) / t7 / v_rho;
            let t94 = t39 * t39;
            let t95 = f64x8::splat(1.0) / t64;
            let t96 = t94 * t95;
            let t97 = f64x8::splat(1.0) / t13;
            let t98 = t96 * t97;
            let t99 = t90 * t92 * t98;
            let t101 = f64x8::splat(4.5) + t99 / f64x8::splat(4.0);
            let t102 = t5 * t101;
            let t104 = f64x8::splat(4.5) + f64x8::splat(0.36675) * t99;
            let t105 = f64x8::splat(1.0) / t104;
            let t107 = t89 * t102 * t105;
            let t110 = f64x8::splat(1.0) / t59;
            let t111 = t80 * t110;
            let t114 = f64x8::splat(1.0) / t66;
            let t115 = t60 * t114;
            let t117 = (simd::exp(-(-t32 + t57) * t110 * t115));
            let t118 = t117 - f64x8::splat(1.0);
            let t119 = f64x8::splat(1.0) / t118;
            let t120 = v_sigma * v_sigma;
            let t121 = t119 * t120;
            let t122 = t81 * t81;
            let t124 = f64x8::splat(1.0) / t21 / t122;
            let t125 = t121 * t124;
            let t126 = t111 * t125;
            let t127 = t65 * t65;
            let t128 = f64x8::splat(1.0) / t127;
            let t129 = t94 * t128;
            let t130 = t129 * t1;
            let t131 = f64x8::splat(1.0) / t19;
            let t132 = t131 * t6;
            let t133 = t101 * t101;
            let t134 = t104 * t104;
            let t135 = f64x8::splat(1.0) / t134;
            let t136 = t133 * t135;
            let t137 = t132 * t136;
            let t138 = t130 * t137;
            let t141 = t87 * t107 / f64x8::splat(96.0) + f64x8::splat(0.0002143700905903487) * t126 * t138;
            let t142 = t141 * t110;
            let t143 = t110 * t119;
            let t144 = t143 * t141;
            let t147 = f64x8::splat(1.0) + f64x8::splat(0.6585449182935511) * t80 * t144;
            let t148 = f64x8::splat(1.0) / t147;
            let t149 = t142 * t148;
            let t152 = f64x8::splat(1.0) + f64x8::splat(0.6585449182935511) * t80 * t149;
            let t153 = (simd::ln(t152));
            let t155 = t62 * t66 * t153;
            let tzk0 = -t32 + t57 + t155;
            acc_zk = tzk0;
            let t156 = t6 * t92;
            let t158 = t4 * t156 * t30;
            let t159 = f64x8::splat(0.0011073470983333333) * t158;
            let t160 = t26 * t26;
            let t161 = f64x8::splat(1.0) / t160;
            let t162 = t12 * t161;
            let t163 = t97 * t1;
            let t164 = t3 * t6;
            let t165 = t164 * t92;
            let t166 = t163 * t165;
            let t168 = t4 * t156;
            let t170 = ((t10).sqrt());
            let t171 = t170 * t1;
            let t172 = t171 * t165;
            let t177 = t20 * t5 / t21 / v_rho;
            let t179 = -f64x8::splat(0.632975) * t166 - f64x8::splat(0.29896666666666666) * t168 - f64x8::splat(0.1023875) * t172 - f64x8::splat(0.08215666666666667) * t177;
            let t180 = f64x8::splat(1.0) / t29;
            let t181 = t179 * t180;
            let t182 = t162 * t181;
            let t183 = f64x8::splat(1.0) * t182;
            let t184 = t43 * t1;
            let t187 = t184 * t164 * t92 * t54;
            let t188 = f64x8::splat(0.00018311447306006544) * t187;
            let t189 = t43 * t45;
            let t190 = t50 * t50;
            let t191 = f64x8::splat(1.0) / t190;
            let t196 = -f64x8::splat(0.8630833333333333) * t166 - f64x8::splat(0.301925) * t168 - f64x8::splat(0.05501625) * t172 - f64x8::splat(0.082785) * t177;
            let t198 = f64x8::splat(1.0) / t53;
            let t199 = t191 * t196 * t198;
            let t200 = t189 * t199;
            let t201 = f64x8::splat(0.5848223622634646) * t200;
            let t206 = -f64x8::splat(0.041666666666666664) * t4 * t156 * t68 - f64x8::splat(0.006944583333333333) * t177;
            let t207 = t206 * t79;
            let t210 = t78 * t78;
            let t211 = f64x8::splat(1.0) / t210;
            let t212 = t72 * t211;
            let t213 = t212 * t141;
            let t214 = t110 * t148;
            let t219 = -f64x8::splat(0.041666666666666664) * t4 * t156 * t74 - f64x8::splat(0.012347083333333333) * t177;
            let t220 = t214 * t219;
            let t223 = t81 * v_rho;
            let t225 = f64x8::splat(1.0) / t7 / t223;
            let t226 = v_sigma * t225;
            let t227 = t226 * t86;
            let t231 = t90 * t83 * t98;
            let t234 = f64x8::splat(1.0) / t21 / t81;
            let t238 = f64x8::splat(1.0) / t13 / t10;
            let t240 = t238 * t1 * t164;
            let t241 = t90 * t234 * t96 * t240;
            let t243 = -t231 / f64x8::splat(3.0) + t241 / f64x8::splat(24.0);
            let t244 = t5 * t243;
            let t246 = t89 * t244 * t105;
            let t249 = t86 * t18;
            let t250 = t84 * t249;
            let t251 = t88 * t5;
            let t252 = t101 * t135;
            let t255 = -f64x8::splat(0.489) * t231 + f64x8::splat(0.061125) * t241;
            let t257 = t251 * t252 * t255;
            let t260 = t207 * t110;
            let t261 = t260 * t125;
            let t264 = t212 * t110;
            let t265 = t124 * t94;
            let t266 = t121 * t265;
            let t267 = t264 * t266;
            let t268 = t128 * t1;
            let t269 = t268 * t131;
            let t270 = t6 * t133;
            let t271 = t135 * t219;
            let t273 = t269 * t270 * t271;
            let t276 = t59 * t59;
            let t277 = f64x8::splat(1.0) / t276;
            let t278 = t118 * t118;
            let t279 = f64x8::splat(1.0) / t278;
            let t280 = t277 * t279;
            let t281 = t80 * t280;
            let t282 = t120 * t124;
            let t284 = f64x8::splat(1.0) / t127 / t66;
            let t285 = t94 * t284;
            let t286 = t282 * t285;
            let t287 = t281 * t286;
            let t288 = t1 * t131;
            let t289 = t288 * t270;
            let t290 = t159 + t183 - t188 - t201;
            let t291 = t135 * t290;
            let t292 = t60 * t117;
            let t293 = t291 * t292;
            let t294 = t289 * t293;
            let t297 = t122 * v_rho;
            let t299 = f64x8::splat(1.0) / t21 / t297;
            let t300 = t121 * t299;
            let t301 = t111 * t300;
            let t304 = t111 * t266;
            let t305 = t6 * t101;
            let t306 = t135 * t243;
            let t308 = t269 * t305 * t306;
            let t312 = f64x8::splat(1.0) / t134 / t104;
            let t313 = t312 * t255;
            let t315 = t269 * t270 * t313;
            let t318 = -f64x8::splat(7.0) / f64x8::splat(288.0) * t227 * t107 + t87 * t246 / f64x8::splat(96.0) - t250 * t257 / f64x8::splat(96.0) + f64x8::splat(0.0002143700905903487) * t261 * t138 - f64x8::splat(0.0002143700905903487) * t267 * t273 + f64x8::splat(0.0002143700905903487) * t287 * t294 - f64x8::splat(0.0010003937560882938) * t301 * t138 + f64x8::splat(0.0004287401811806974) * t304 * t308 - f64x8::splat(0.0004287401811806974) * t304 * t315;
            let t320 = t318 * t110 * t148;
            let t323 = t80 * t141;
            let t324 = t147 * t147;
            let t325 = f64x8::splat(1.0) / t324;
            let t326 = t110 * t325;
            let t329 = t119 * t141;
            let t330 = t329 * t219;
            let t334 = t115 * t117;
            let t335 = t141 * t290 * t334;
            let t338 = t143 * t318;
            let t341 = f64x8::splat(0.6585449182935511) * t207 * t144 - f64x8::splat(0.6585449182935511) * t264 * t330 + f64x8::splat(0.6585449182935511) * t281 * t335 + f64x8::splat(0.6585449182935511) * t80 * t338;
            let t342 = t326 * t341;
            let t345 = f64x8::splat(0.6585449182935511) * t207 * t149 - f64x8::splat(0.6585449182935511) * t213 * t220 + f64x8::splat(0.6585449182935511) * t80 * t320 - f64x8::splat(0.6585449182935511) * t323 * t342;
            let t347 = f64x8::splat(1.0) / t152;
            let t349 = t62 * t66 * t345 * t347;
            let tvrho0 = -t32 + t57 + t155 + v_rho * (t159 + t183 - t188 - t201 + t349);
            acc_vrho = tvrho0;
            let t352 = v_rho * t59;
            let t353 = t352 * t61;
            let t354 = t83 * t39;
            let t355 = t85 * t18;
            let t356 = t354 * t355;
            let t358 = t251 * t101 * t105;
            let t362 = f64x8::splat(1.0) / t21 / t223;
            let t363 = t90 * t362;
            let t364 = t114 * t18;
            let t365 = t363 * t364;
            let t366 = t97 * t105;
            let t367 = t251 * t366;
            let t371 = t251 * t252 * t97;
            let t374 = t119 * v_sigma;
            let t375 = t374 * t124;
            let t376 = t111 * t375;
            let t379 = t90 * v_sigma;
            let t380 = t119 * t379;
            let t381 = t122 * t81;
            let t382 = f64x8::splat(1.0) / t381;
            let t383 = t382 * t39;
            let t384 = t380 * t383;
            let t385 = t111 * t384;
            let t386 = t127 * t64;
            let t387 = f64x8::splat(1.0) / t386;
            let t388 = t387 * t1;
            let t389 = t388 * t131;
            let t390 = t135 * t97;
            let t392 = t389 * t305 * t390;
            let t395 = t312 * t97;
            let t397 = t389 * t270 * t395;
            let t400 = t356 * t358 / f64x8::splat(96.0) + t365 * t367 / f64x8::splat(384.0) - f64x8::splat(0.0038203125) * t365 * t371 + f64x8::splat(0.0004287401811806974) * t376 * t138 + f64x8::splat(0.00010718504529517435) * t385 * t392 - f64x8::splat(0.00015724046144802075) * t385 * t397;
            let t402 = t400 * t110 * t148;
            let t405 = t72 * t72;
            let t406 = t405 * t211;
            let t407 = t406 * t141;
            let t408 = t277 * t325;
            let t409 = t119 * t400;
            let t410 = t408 * t409;
            let t413 = f64x8::splat(0.6585449182935511) * t80 * t402 - f64x8::splat(0.43368140941025995) * t407 * t410;
            let t414 = t66 * t413;
            let t415 = t414 * t347;
            let tvsigma0 = t353 * t415;
            acc_vsigma = tvsigma0;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        ip += 8;
    }
}

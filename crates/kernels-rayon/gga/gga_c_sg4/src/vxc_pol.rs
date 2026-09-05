//! GGA_C_SG4 vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_sg4.c`
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

/// Accumulate 8 elements with a given stride and offset.
///
/// `+=`, not `=`: the scalar kernel this was translated from writes
/// `out[ip * stride + offset] += v`, and a plain store is not the same
/// operation. It differs on the sign of zero -- `0.0 + -0.0` is `+0.0`
/// while a store of `-0.0` keeps the sign -- which is a bit difference
/// the fingerprint gate sees, and it would silently drop a caller's
/// existing contribution if one were ever there.
///
/// The read is not free on this path: a polarized `kxc`/`lxc` kernel
/// writes many strided outputs per point, and `lda_c_pw_erf kxc pol`
/// measured 84 -> 114 ns/pt (1.36x). It is charged anyway, because the
/// scalar kernel this is compared against does the same read. Gathering
/// into a vector, adding once and scattering back was tried and is no
/// faster (117 ns/pt), so the cost is the load itself, not scheduling.
#[inline(always)]
fn store_strided(s: &mut [f64], ip: usize, m: usize, stride: usize, offset: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let base = ip * stride + offset;
        s[base] += a[0];
        s[base + stride] += a[1];
        s[base + 2 * stride] += a[2];
        s[base + 3 * stride] += a[3];
        s[base + 4 * stride] += a[4];
        s[base + 5 * stride] += a[5];
        s[base + 6 * stride] += a[6];
        s[base + 7 * stride] += a[7];
    } else {
        for k in 0..m {
            s[(ip + k) * stride + offset] += a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_sg4_vxc_pol(
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
        {
            let t1 = f64x8::splat(M_CBRT3);
            let t2 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t3 = (simd::cbrt(t2));
            let t4 = t1 * t3;
            let t5 = f64x8::splat(M_CBRT4);
            let t6 = t5 * t5;
            let t7 = v_rho0 + v_rho1;
            let t8 = (simd::cbrt(t7));
            let t11 = t4 * t6 / t8;
            let t13 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t11;
            let t14 = ((t11).sqrt());
            let t17 = ((t11) * (t11).sqrt());
            let t19 = t1 * t1;
            let t20 = t3 * t3;
            let t21 = t19 * t20;
            let t22 = t8 * t8;
            let t25 = t21 * t5 / t22;
            let t27 = f64x8::splat(3.79785) * t14 + f64x8::splat(0.8969) * t11 + f64x8::splat(0.204775) * t17 + f64x8::splat(0.123235) * t25;
            let t30 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t27;
            let t31 = (simd::ln(t30));
            let t33 = f64x8::splat(0.0621814) * t13 * t31;
            let t34 = v_rho0 - v_rho1;
            let t35 = t34 * t34;
            let t36 = t35 * t35;
            let t37 = t7 * t7;
            let t38 = t37 * t37;
            let t39 = f64x8::splat(1.0) / t38;
            let t40 = t36 * t39;
            let t41 = f64x8::splat(1.0) / t7;
            let t42 = t34 * t41;
            let t43 = f64x8::splat(1.0) + t42;
            let t44 = (t43).simd_le(zeta_threshold);
            let t45 = (simd::cbrt(zeta_threshold));
            let t46 = t45 * zeta_threshold;
            let t47 = (simd::cbrt(t43));
            let t48 = t47 * t43;
            let t49 = ((t44).select(t46, t48));
            let t50 = f64x8::splat(1.0) - t42;
            let t51 = (t50).simd_le(zeta_threshold);
            let t52 = (simd::cbrt(t50));
            let t53 = t52 * t50;
            let t54 = ((t51).select(t46, t53));
            let t55 = t49 + t54 - f64x8::splat(2.0);
            let t56 = f64x8::splat(M_CBRT2);
            let t59 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t56 - f64x8::splat(2.0));
            let t60 = t55 * t59;
            let t62 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t11;
            let t67 = f64x8::splat(7.05945) * t14 + f64x8::splat(1.549425) * t11 + f64x8::splat(0.420775) * t17 + f64x8::splat(0.1562925) * t25;
            let t70 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t67;
            let t71 = (simd::ln(t70));
            let t75 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t11;
            let t80 = f64x8::splat(5.1785) * t14 + f64x8::splat(0.905775) * t11 + f64x8::splat(0.1100325) * t17 + f64x8::splat(0.1241775) * t25;
            let t83 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t80;
            let t84 = (simd::ln(t83));
            let t85 = t75 * t84;
            let t87 = -f64x8::splat(0.0310907) * t62 * t71 + t33 - f64x8::splat(0.0197516734986138) * t85;
            let t88 = t60 * t87;
            let t89 = t40 * t88;
            let t91 = f64x8::splat(0.0197516734986138) * t60 * t85;
            let t92 = t45 * t45;
            let t93 = t47 * t47;
            let t94 = ((t44).select(t92, t93));
            let t95 = t52 * t52;
            let t96 = ((t51).select(t92, t95));
            let t98 = t94 / f64x8::splat(2.0) + t96 / f64x8::splat(2.0);
            let t100 = v_sigma0 + f64x8::splat(2.0) * v_sigma1 + v_sigma2;
            let t101 = ((t100).sqrt());
            let t102 = t101 * t100;
            let t103 = t102 * t39;
            let t104 = t98 * t98;
            let t105 = t104 * t98;
            let t106 = f64x8::splat(1.0) / t105;
            let t108 = f64x8::splat(1.0) / t14 / t11;
            let t109 = t106 * t108;
            let t112 = (simd::pow(t98, f64x8::splat(0.05) * t103 * t109));
            let t113 = (simd::ln(f64x8::splat(2.0)));
            let t114 = f64x8::splat(1.0) - t113;
            let t115 = t112 * t114;
            let t116 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t117 = f64x8::splat(1.0) / t116;
            let t118 = t117 * t105;
            let t120 = f64x8::splat(1.0) / t8 / t7;
            let t121 = t101 * t120;
            let t122 = t56 * t56;
            let t123 = t121 * t122;
            let t124 = f64x8::splat(1.0) / t98;
            let t125 = f64x8::splat(1.0) / t14;
            let t126 = t124 * t125;
            let t128 = (simd::exp(-t25 / f64x8::splat(4.0)));
            let t129 = f64x8::splat(1.0) - t128;
            let t130 = t126 * t129;
            let t133 = f64x8::splat(0.07963845034287749) + f64x8::splat(0.0175) * t123 * t130;
            let t135 = f64x8::splat(1.0) / t8 / t37;
            let t136 = t100 * t135;
            let t138 = f64x8::splat(1.0) / t104;
            let t140 = f64x8::splat(1.0) / t3;
            let t141 = t140 * t5;
            let t142 = t138 * t19 * t141;
            let t145 = f64x8::splat(1.0) / t114;
            let t146 = t133 * t145;
            let t148 = (-t33 + t89 + t91) * t145;
            let t149 = t116 * t106;
            let t151 = (simd::exp(-t148 * t149));
            let t152 = t151 - f64x8::splat(1.0);
            let t153 = f64x8::splat(1.0) / t152;
            let t154 = t116 * t153;
            let t155 = t100 * t100;
            let t156 = t154 * t155;
            let t157 = t146 * t156;
            let t159 = f64x8::splat(1.0) / t22 / t38;
            let t160 = t159 * t122;
            let t161 = t104 * t104;
            let t162 = f64x8::splat(1.0) / t161;
            let t164 = f64x8::splat(1.0) / t20;
            let t165 = t1 * t164;
            let t166 = t165 * t6;
            let t167 = t160 * t162 * t166;
            let t170 = t136 * t56 * t142 / f64x8::splat(96.0) + t157 * t167 / f64x8::splat(3072.0);
            let t171 = t133 * t170;
            let t172 = t145 * t116;
            let t173 = t154 * t170;
            let t175 = t146 * t173 + f64x8::splat(1.0);
            let t176 = f64x8::splat(1.0) / t175;
            let t177 = t172 * t176;
            let t179 = t171 * t177 + f64x8::splat(1.0);
            let t180 = (simd::ln(t179));
            let t181 = t118 * t180;
            let t182 = t115 * t181;
            let tzk0 = -t33 + t89 + t91 + t182;
            acc_zk = tzk0;
            let t183 = t6 * t120;
            let t185 = t4 * t183 * t31;
            let t186 = f64x8::splat(0.0011073470983333333) * t185;
            let t187 = t27 * t27;
            let t188 = f64x8::splat(1.0) / t187;
            let t189 = t13 * t188;
            let t190 = t125 * t1;
            let t191 = t3 * t6;
            let t192 = t191 * t120;
            let t193 = t190 * t192;
            let t195 = t4 * t183;
            let t197 = ((t11).sqrt());
            let t198 = t197 * t1;
            let t199 = t198 * t192;
            let t203 = t5 / t22 / t7;
            let t204 = t21 * t203;
            let t206 = -f64x8::splat(0.632975) * t193 - f64x8::splat(0.29896666666666666) * t195 - f64x8::splat(0.1023875) * t199 - f64x8::splat(0.08215666666666667) * t204;
            let t207 = f64x8::splat(1.0) / t30;
            let t208 = t206 * t207;
            let t209 = t189 * t208;
            let t210 = f64x8::splat(1.0) * t209;
            let t211 = t35 * t34;
            let t212 = t211 * t39;
            let t213 = t212 * t88;
            let t214 = f64x8::splat(4.0) * t213;
            let t215 = t38 * t7;
            let t216 = f64x8::splat(1.0) / t215;
            let t217 = t36 * t216;
            let t218 = t217 * t88;
            let t219 = f64x8::splat(4.0) * t218;
            let t220 = f64x8::splat(1.0) / t37;
            let t221 = t34 * t220;
            let t222 = t41 - t221;
            let t225 = ((t44).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t47 * t222));
            let t226 = -t222;
            let t229 = ((t51).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t52 * t226));
            let t231 = (t225 + t229) * t59;
            let t232 = t231 * t87;
            let t233 = t40 * t232;
            let t237 = t67 * t67;
            let t238 = f64x8::splat(1.0) / t237;
            let t239 = t62 * t238;
            let t244 = -f64x8::splat(1.176575) * t193 - f64x8::splat(0.516475) * t195 - f64x8::splat(0.2103875) * t199 - f64x8::splat(0.104195) * t204;
            let t245 = f64x8::splat(1.0) / t70;
            let t246 = t244 * t245;
            let t252 = t80 * t80;
            let t253 = f64x8::splat(1.0) / t252;
            let t254 = t75 * t253;
            let t259 = -f64x8::splat(0.8630833333333333) * t193 - f64x8::splat(0.301925) * t195 - f64x8::splat(0.05501625) * t199 - f64x8::splat(0.082785) * t204;
            let t260 = f64x8::splat(1.0) / t83;
            let t261 = t259 * t260;
            let t264 = f64x8::splat(0.0005323764196666666) * t4 * t183 * t71 + f64x8::splat(1.0) * t239 * t246 - t186 - t210 + f64x8::splat(0.00018311447306006544) * t4 * t183 * t84 + f64x8::splat(0.5848223622634646) * t254 * t261;
            let t265 = t60 * t264;
            let t266 = t40 * t265;
            let t267 = t231 * t85;
            let t268 = f64x8::splat(0.0197516734986138) * t267;
            let t269 = t60 * t1;
            let t271 = t191 * t120 * t84;
            let t272 = t269 * t271;
            let t273 = f64x8::splat(0.00018311447306006544) * t272;
            let t274 = t60 * t75;
            let t276 = t253 * t259 * t260;
            let t277 = t274 * t276;
            let t278 = f64x8::splat(0.5848223622634646) * t277;
            let t279 = t102 * t216;
            let t281 = f64x8::splat(0.2) * t279 * t109;
            let t282 = t162 * t108;
            let t283 = f64x8::splat(1.0) / t47;
            let t286 = ((t44).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t283 * t222));
            let t287 = f64x8::splat(1.0) / t52;
            let t290 = ((t51).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t287 * t226));
            let t292 = t286 / f64x8::splat(2.0) + t290 / f64x8::splat(2.0);
            let t293 = t282 * t292;
            let t294 = t103 * t293;
            let t297 = f64x8::splat(1.0) / t8 / t215;
            let t298 = t102 * t297;
            let t302 = f64x8::splat(1.0) / t14 / t25 / f64x8::splat(4.0);
            let t303 = t302 * t1;
            let t304 = t303 * t191;
            let t306 = f64x8::splat(0.025) * t298 * t106 * t304;
            let t307 = -t281 - f64x8::splat(0.15) * t294 + t306;
            let t308 = (simd::ln(t98));
            let t311 = t307 * t308 + f64x8::splat(0.05) * t294;
            let t312 = t112 * t311;
            let t313 = t312 * t114;
            let t314 = t313 * t181;
            let t315 = t115 * t117;
            let t316 = t104 * t180;
            let t318 = t315 * t316 * t292;
            let t319 = f64x8::splat(3.0) * t318;
            let t320 = t101 * t135;
            let t321 = t320 * t122;
            let t323 = f64x8::splat(0.023333333333333334) * t321 * t130;
            let t324 = t138 * t125;
            let t325 = t129 * t292;
            let t326 = t324 * t325;
            let t330 = f64x8::splat(1.0) / t22 / t37;
            let t331 = t101 * t330;
            let t332 = t122 * t124;
            let t335 = t4 * t6;
            let t336 = t108 * t129 * t335;
            let t338 = f64x8::splat(0.002916666666666667) * t331 * t332 * t336;
            let t339 = t37 * t7;
            let t340 = f64x8::splat(1.0) / t339;
            let t341 = t101 * t340;
            let t344 = t20 * t5;
            let t345 = t344 * t128;
            let t346 = t125 * t19 * t345;
            let t348 = f64x8::splat(0.002916666666666667) * t341 * t332 * t346;
            let t349 = -t323 - f64x8::splat(0.0175) * t123 * t326 + t338 - t348;
            let t350 = t349 * t170;
            let t353 = f64x8::splat(1.0) / t8 / t339;
            let t354 = t100 * t353;
            let t357 = f64x8::splat(7.0) / f64x8::splat(288.0) * t354 * t56 * t142;
            let t358 = t56 * t106;
            let t359 = t136 * t358;
            let t360 = t19 * t140;
            let t361 = t5 * t292;
            let t362 = t360 * t361;
            let t365 = t349 * t145;
            let t366 = t365 * t156;
            let t369 = t146 * t116;
            let t370 = t152 * t152;
            let t371 = f64x8::splat(1.0) / t370;
            let t372 = t371 * t155;
            let t373 = t372 * t159;
            let t374 = t369 * t373;
            let t375 = t122 * t162;
            let t376 = t375 * t1;
            let t377 = t164 * t6;
            let t379 = (t186 + t210 + t214 - t219 + t233 + t266 + t268 - t273 - t278) * t145;
            let t381 = t116 * t162;
            let t382 = t381 * t292;
            let t385 = f64x8::splat(3.0) * t148 * t382 - t149 * t379;
            let t386 = t385 * t151;
            let t387 = t377 * t386;
            let t388 = t376 * t387;
            let t392 = f64x8::splat(1.0) / t22 / t215;
            let t393 = t392 * t122;
            let t395 = t393 * t162 * t166;
            let t397 = f64x8::splat(7.0) / f64x8::splat(4608.0) * t157 * t395;
            let t398 = t153 * t155;
            let t399 = t398 * t159;
            let t400 = t369 * t399;
            let t402 = f64x8::splat(1.0) / t161 / t98;
            let t403 = t122 * t402;
            let t404 = t403 * t1;
            let t405 = t377 * t292;
            let t406 = t404 * t405;
            let t409 = -t357 - t359 * t362 / f64x8::splat(48.0) + t366 * t167 / f64x8::splat(3072.0) - t374 * t388 / f64x8::splat(3072.0) - t397 - t400 * t406 / f64x8::splat(768.0);
            let t410 = t133 * t409;
            let t412 = t171 * t145;
            let t413 = t175 * t175;
            let t414 = f64x8::splat(1.0) / t413;
            let t415 = t116 * t414;
            let t417 = t371 * t170;
            let t418 = t417 * t386;
            let t420 = t154 * t409;
            let t422 = t146 * t420 + t173 * t365 - t369 * t418;
            let t423 = t415 * t422;
            let t425 = t177 * t350 + t177 * t410 - t412 * t423;
            let t426 = t105 * t425;
            let t427 = f64x8::splat(1.0) / t179;
            let t429 = t315 * t426 * t427;
            let t430 = t186 + t210 + t214 - t219 + t233 + t266 + t268 - t273 - t278 + t314 + t319 + t429;
            let tvrho0 = t430 * t7 + t182 - t33 + t89 + t91;
            acc_vrho_0 = tvrho0;
            let t432 = -t41 - t221;
            let t435 = ((t44).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t47 * t432));
            let t436 = -t432;
            let t439 = ((t51).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t52 * t436));
            let t441 = (t435 + t439) * t59;
            let t442 = t441 * t87;
            let t443 = t40 * t442;
            let t444 = t441 * t85;
            let t445 = f64x8::splat(0.0197516734986138) * t444;
            let t448 = ((t44).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t283 * t432));
            let t451 = ((t51).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t287 * t436));
            let t453 = t448 / f64x8::splat(2.0) + t451 / f64x8::splat(2.0);
            let t454 = t282 * t453;
            let t455 = t103 * t454;
            let t457 = -t281 - f64x8::splat(0.15) * t455 + t306;
            let t460 = t457 * t308 + f64x8::splat(0.05) * t455;
            let t461 = t112 * t460;
            let t462 = t461 * t114;
            let t463 = t462 * t181;
            let t465 = t315 * t316 * t453;
            let t466 = f64x8::splat(3.0) * t465;
            let t467 = t129 * t453;
            let t468 = t324 * t467;
            let t471 = -t323 - f64x8::splat(0.0175) * t123 * t468 + t338 - t348;
            let t472 = t471 * t170;
            let t474 = t5 * t453;
            let t475 = t360 * t474;
            let t478 = t471 * t145;
            let t479 = t478 * t156;
            let t483 = (t186 + t210 - t214 - t219 + t443 + t266 + t445 - t273 - t278) * t145;
            let t485 = t381 * t453;
            let t488 = f64x8::splat(3.0) * t148 * t485 - t149 * t483;
            let t489 = t488 * t151;
            let t490 = t377 * t489;
            let t491 = t376 * t490;
            let t494 = t377 * t453;
            let t495 = t404 * t494;
            let t498 = -t357 - t359 * t475 / f64x8::splat(48.0) + t479 * t167 / f64x8::splat(3072.0) - t374 * t491 / f64x8::splat(3072.0) - t397 - t400 * t495 / f64x8::splat(768.0);
            let t499 = t133 * t498;
            let t502 = t417 * t489;
            let t504 = t154 * t498;
            let t506 = t146 * t504 + t173 * t478 - t369 * t502;
            let t507 = t415 * t506;
            let t509 = t177 * t472 + t177 * t499 - t412 * t507;
            let t510 = t105 * t509;
            let t512 = t315 * t510 * t427;
            let t513 = t186 + t210 - t214 - t219 + t443 + t266 + t445 - t273 - t278 + t463 + t466 + t512;
            let tvrho1 = t513 * t7 + t182 - t33 + t89 + t91;
            acc_vrho_1 = tvrho1;
            let t515 = t112 * t101;
            let t517 = t108 * t308;
            let t518 = t114 * t180;
            let t519 = t517 * t518;
            let t520 = t515 * t39 * t519;
            let t521 = f64x8::splat(0.007599088773175333) * t520;
            let t522 = f64x8::splat(1.0) / t101;
            let t523 = t522 * t120;
            let t524 = t523 * t332;
            let t525 = t125 * t129;
            let t526 = t170 * t145;
            let t527 = t526 * t176;
            let t528 = t525 * t527;
            let t529 = t524 * t528;
            let t531 = t135 * t56;
            let t533 = t360 * t5;
            let t534 = t531 * t138 * t533;
            let t536 = t38 * t37;
            let t537 = f64x8::splat(1.0) / t536;
            let t538 = t102 * t537;
            let t539 = t56 * t402;
            let t540 = t539 * t125;
            let t542 = t129 * t145;
            let t543 = t542 * t153;
            let t544 = t543 * t166;
            let t545 = t538 * t540 * t544;
            let t547 = t154 * t100;
            let t548 = t146 * t547;
            let t549 = t548 * t167;
            let t551 = t534 / f64x8::splat(96.0) + f64x8::splat(5.622333236297649e-05) * t545 + t549 / f64x8::splat(1536.0);
            let t552 = t133 * t551;
            let t554 = t145 * t153;
            let t555 = t554 * t170;
            let t556 = t525 * t555;
            let t557 = t524 * t556;
            let t559 = t154 * t551;
            let t561 = f64x8::splat(0.08635903850953189) * t557 + t146 * t559;
            let t562 = t415 * t561;
            let t564 = f64x8::splat(0.08635903850953189) * t529 + t552 * t177 - t412 * t562;
            let t565 = t105 * t564;
            let t567 = t315 * t565 * t427;
            let tvsigma0 = t7 * (t521 + t567);
            acc_vsigma_0 = tvsigma0;
            let t569 = f64x8::splat(0.015198177546350666) * t520;
            let t574 = t534 / f64x8::splat(48.0) + f64x8::splat(0.00011244666472595298) * t545 + t549 / f64x8::splat(768.0);
            let t575 = t133 * t574;
            let t578 = t154 * t574;
            let t580 = f64x8::splat(0.17271807701906378) * t557 + t146 * t578;
            let t581 = t415 * t580;
            let t583 = f64x8::splat(0.17271807701906378) * t529 + t575 * t177 - t412 * t581;
            let t584 = t105 * t583;
            let t586 = t315 * t584 * t427;
            let tvsigma1 = t7 * (t569 + t586);
            acc_vsigma_1 = tvsigma1;
            let tvsigma2 = tvsigma0;
            acc_vsigma_2 = tvsigma2;
        }
        store_add(zk, ip, m, acc_zk);
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(vsigma, ip, m, 3, 0, acc_vsigma_0);
        store_strided(vsigma, ip, m, 3, 1, acc_vsigma_1);
        store_strided(vsigma, ip, m, 3, 2, acc_vsigma_2);
        ip += 8;
    }
}

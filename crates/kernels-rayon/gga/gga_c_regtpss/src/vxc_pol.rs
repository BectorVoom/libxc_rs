//! GGA_C_REGTPSS vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_regtpss.c`
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
pub fn gga_c_regtpss_vxc_pol(
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
            let t92 = (simd::ln(f64x8::splat(2.0)));
            let t93 = f64x8::splat(1.0) - t92;
            let t94 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t95 = f64x8::splat(1.0) / t94;
            let t96 = t93 * t95;
            let t97 = t45 * t45;
            let t98 = t47 * t47;
            let t99 = ((t44).select(t97, t98));
            let t100 = t52 * t52;
            let t101 = ((t51).select(t97, t100));
            let t103 = t99 / f64x8::splat(2.0) + t101 / f64x8::splat(2.0);
            let t104 = t103 * t103;
            let t105 = t104 * t103;
            let t107 = f64x8::splat(1.0) + f64x8::splat(0.025) * t11;
            let t109 = f64x8::splat(1.0) + f64x8::splat(0.04445) * t11;
            let t110 = f64x8::splat(1.0) / t109;
            let t111 = t107 * t110;
            let t113 = v_sigma0 + f64x8::splat(2.0) * v_sigma1 + v_sigma2;
            let t115 = f64x8::splat(1.0) / t8 / t37;
            let t116 = t113 * t115;
            let t118 = f64x8::splat(1.0) / t104;
            let t120 = f64x8::splat(1.0) / t3;
            let t121 = t120 * t5;
            let t122 = t118 * t19 * t121;
            let t125 = f64x8::splat(1.0) / t93;
            let t127 = (-t33 + t89 + t91) * t125;
            let t128 = f64x8::splat(1.0) / t105;
            let t129 = t94 * t128;
            let t131 = (simd::exp(-t127 * t129));
            let t132 = t131 - f64x8::splat(1.0);
            let t133 = f64x8::splat(1.0) / t132;
            let t134 = t125 * t133;
            let t135 = t113 * t113;
            let t136 = t134 * t135;
            let t137 = t111 * t136;
            let t139 = f64x8::splat(1.0) / t22 / t38;
            let t140 = t56 * t56;
            let t141 = t139 * t140;
            let t142 = t104 * t104;
            let t143 = f64x8::splat(1.0) / t142;
            let t144 = t141 * t143;
            let t145 = f64x8::splat(1.0) / t20;
            let t146 = t1 * t145;
            let t147 = t146 * t6;
            let t148 = t144 * t147;
            let t151 = t116 * t56 * t122 / f64x8::splat(96.0) + f64x8::splat(0.0002143700905903487) * t137 * t148;
            let t152 = t151 * t125;
            let t153 = t134 * t151;
            let t156 = f64x8::splat(1.0) + f64x8::splat(0.6585449182935511) * t111 * t153;
            let t157 = f64x8::splat(1.0) / t156;
            let t158 = t152 * t157;
            let t161 = f64x8::splat(1.0) + f64x8::splat(0.6585449182935511) * t111 * t158;
            let t162 = (simd::ln(t161));
            let t164 = t96 * t105 * t162;
            let tzk0 = -t33 + t89 + t91 + t164;
            acc_zk = tzk0;
            let t166 = f64x8::splat(1.0) / t8 / t7;
            let t167 = t6 * t166;
            let t169 = t4 * t167 * t31;
            let t170 = f64x8::splat(0.0011073470983333333) * t169;
            let t171 = t27 * t27;
            let t172 = f64x8::splat(1.0) / t171;
            let t173 = t13 * t172;
            let t175 = f64x8::splat(1.0) / t14 * t1;
            let t176 = t3 * t6;
            let t177 = t176 * t166;
            let t178 = t175 * t177;
            let t180 = t4 * t167;
            let t182 = ((t11).sqrt());
            let t183 = t182 * t1;
            let t184 = t183 * t177;
            let t189 = t21 * t5 / t22 / t7;
            let t191 = -f64x8::splat(0.632975) * t178 - f64x8::splat(0.29896666666666666) * t180 - f64x8::splat(0.1023875) * t184 - f64x8::splat(0.08215666666666667) * t189;
            let t192 = f64x8::splat(1.0) / t30;
            let t193 = t191 * t192;
            let t194 = t173 * t193;
            let t195 = f64x8::splat(1.0) * t194;
            let t196 = t35 * t34;
            let t197 = t196 * t39;
            let t198 = t197 * t88;
            let t199 = f64x8::splat(4.0) * t198;
            let t200 = t38 * t7;
            let t201 = f64x8::splat(1.0) / t200;
            let t202 = t36 * t201;
            let t203 = t202 * t88;
            let t204 = f64x8::splat(4.0) * t203;
            let t205 = f64x8::splat(1.0) / t37;
            let t206 = t34 * t205;
            let t207 = t41 - t206;
            let t210 = ((t44).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t47 * t207));
            let t211 = -t207;
            let t214 = ((t51).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t52 * t211));
            let t216 = (t210 + t214) * t59;
            let t217 = t216 * t87;
            let t218 = t40 * t217;
            let t222 = t67 * t67;
            let t223 = f64x8::splat(1.0) / t222;
            let t224 = t62 * t223;
            let t229 = -f64x8::splat(1.176575) * t178 - f64x8::splat(0.516475) * t180 - f64x8::splat(0.2103875) * t184 - f64x8::splat(0.104195) * t189;
            let t230 = f64x8::splat(1.0) / t70;
            let t231 = t229 * t230;
            let t237 = t80 * t80;
            let t238 = f64x8::splat(1.0) / t237;
            let t239 = t75 * t238;
            let t244 = -f64x8::splat(0.8630833333333333) * t178 - f64x8::splat(0.301925) * t180 - f64x8::splat(0.05501625) * t184 - f64x8::splat(0.082785) * t189;
            let t245 = f64x8::splat(1.0) / t83;
            let t246 = t244 * t245;
            let t249 = f64x8::splat(0.0005323764196666666) * t4 * t167 * t71 + f64x8::splat(1.0) * t224 * t231 - t170 - t195 + f64x8::splat(0.00018311447306006544) * t4 * t167 * t84 + f64x8::splat(0.5848223622634646) * t239 * t246;
            let t250 = t60 * t249;
            let t251 = t40 * t250;
            let t252 = t216 * t85;
            let t253 = f64x8::splat(0.0197516734986138) * t252;
            let t254 = t60 * t1;
            let t256 = t176 * t166 * t84;
            let t257 = t254 * t256;
            let t258 = f64x8::splat(0.00018311447306006544) * t257;
            let t259 = t60 * t75;
            let t261 = t238 * t244 * t245;
            let t262 = t259 * t261;
            let t263 = f64x8::splat(0.5848223622634646) * t262;
            let t264 = t104 * t162;
            let t265 = f64x8::splat(1.0) / t47;
            let t268 = ((t44).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t265 * t207));
            let t269 = f64x8::splat(1.0) / t52;
            let t272 = ((t51).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t269 * t211));
            let t274 = t268 / f64x8::splat(2.0) + t272 / f64x8::splat(2.0);
            let t276 = t96 * t264 * t274;
            let t277 = f64x8::splat(3.0) * t276;
            let t278 = t110 * t151;
            let t279 = t125 * t157;
            let t280 = t278 * t279;
            let t282 = f64x8::splat(0.005487874319112926) * t180 * t280;
            let t283 = t109 * t109;
            let t284 = f64x8::splat(1.0) / t283;
            let t285 = t107 * t284;
            let t286 = t285 * t152;
            let t287 = t157 * t1;
            let t288 = t287 * t177;
            let t290 = f64x8::splat(0.009757440539382782) * t286 * t288;
            let t291 = t37 * t7;
            let t293 = f64x8::splat(1.0) / t8 / t291;
            let t294 = t113 * t293;
            let t297 = f64x8::splat(7.0) / f64x8::splat(288.0) * t294 * t56 * t122;
            let t298 = t56 * t128;
            let t299 = t116 * t298;
            let t300 = t19 * t120;
            let t301 = t5 * t274;
            let t302 = t300 * t301;
            let t305 = t38 * t37;
            let t306 = f64x8::splat(1.0) / t305;
            let t309 = t300 * t5 * t306 * t110;
            let t310 = t135 * t140;
            let t312 = t134 * t310 * t143;
            let t314 = f64x8::splat(7.145669686344956e-06) * t309 * t312;
            let t315 = t285 * t136;
            let t316 = t306 * t140;
            let t317 = t316 * t143;
            let t318 = t300 * t5;
            let t319 = t317 * t318;
            let t321 = f64x8::splat(1.2705000702321332e-05) * t315 * t319;
            let t322 = t111 * t125;
            let t323 = t132 * t132;
            let t324 = f64x8::splat(1.0) / t323;
            let t325 = t324 * t135;
            let t327 = t322 * t325 * t139;
            let t328 = t140 * t143;
            let t329 = t328 * t1;
            let t330 = t145 * t6;
            let t332 = (t170 + t195 + t199 - t204 + t218 + t251 + t253 - t258 - t263) * t125;
            let t334 = t94 * t143;
            let t335 = t334 * t274;
            let t338 = f64x8::splat(3.0) * t127 * t335 - t332 * t129;
            let t339 = t338 * t131;
            let t340 = t330 * t339;
            let t341 = t329 * t340;
            let t345 = f64x8::splat(1.0) / t22 / t200;
            let t346 = t345 * t140;
            let t347 = t346 * t143;
            let t348 = t347 * t147;
            let t350 = f64x8::splat(0.0010003937560882938) * t137 * t348;
            let t351 = t133 * t135;
            let t353 = t322 * t351 * t139;
            let t355 = f64x8::splat(1.0) / t142 / t103;
            let t356 = t140 * t355;
            let t357 = t356 * t1;
            let t359 = t357 * t330 * t274;
            let t362 = -t297 - t299 * t302 / f64x8::splat(48.0) - t314 + t321 - f64x8::splat(0.0002143700905903487) * t327 * t341 - t350 - f64x8::splat(0.0008574803623613948) * t353 * t359;
            let t363 = t362 * t125;
            let t364 = t363 * t157;
            let t367 = t111 * t151;
            let t368 = t156 * t156;
            let t369 = f64x8::splat(1.0) / t368;
            let t370 = t125 * t369;
            let t371 = t110 * t125;
            let t372 = t133 * t151;
            let t373 = t371 * t372;
            let t375 = f64x8::splat(0.005487874319112926) * t180 * t373;
            let t376 = t285 * t134;
            let t377 = t151 * t1;
            let t380 = f64x8::splat(0.009757440539382782) * t376 * t377 * t177;
            let t381 = t324 * t151;
            let t382 = t381 * t339;
            let t385 = t134 * t362;
            let t388 = -t375 + t380 - f64x8::splat(0.6585449182935511) * t322 * t382 + f64x8::splat(0.6585449182935511) * t111 * t385;
            let t389 = t370 * t388;
            let t392 = -t282 + t290 + f64x8::splat(0.6585449182935511) * t111 * t364 - f64x8::splat(0.6585449182935511) * t367 * t389;
            let t394 = f64x8::splat(1.0) / t161;
            let t396 = t96 * t105 * t392 * t394;
            let t397 = t170 + t195 + t199 - t204 + t218 + t251 + t253 - t258 - t263 + t277 + t396;
            let tvrho0 = t7 * t397 + t164 - t33 + t89 + t91;
            acc_vrho_0 = tvrho0;
            let t399 = -t41 - t206;
            let t402 = ((t44).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t47 * t399));
            let t403 = -t399;
            let t406 = ((t51).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t52 * t403));
            let t408 = (t402 + t406) * t59;
            let t409 = t408 * t87;
            let t410 = t40 * t409;
            let t411 = t408 * t85;
            let t412 = f64x8::splat(0.0197516734986138) * t411;
            let t415 = ((t44).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t265 * t399));
            let t418 = ((t51).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t269 * t403));
            let t420 = t415 / f64x8::splat(2.0) + t418 / f64x8::splat(2.0);
            let t422 = t96 * t264 * t420;
            let t423 = f64x8::splat(3.0) * t422;
            let t424 = t5 * t420;
            let t425 = t300 * t424;
            let t429 = (t170 + t195 - t199 - t204 + t410 + t251 + t412 - t258 - t263) * t125;
            let t431 = t334 * t420;
            let t434 = f64x8::splat(3.0) * t127 * t431 - t429 * t129;
            let t435 = t434 * t131;
            let t436 = t330 * t435;
            let t437 = t329 * t436;
            let t441 = t357 * t330 * t420;
            let t444 = -t297 - t299 * t425 / f64x8::splat(48.0) - t314 + t321 - f64x8::splat(0.0002143700905903487) * t327 * t437 - t350 - f64x8::splat(0.0008574803623613948) * t353 * t441;
            let t445 = t444 * t125;
            let t446 = t445 * t157;
            let t449 = t381 * t435;
            let t452 = t134 * t444;
            let t455 = -t375 + t380 - f64x8::splat(0.6585449182935511) * t322 * t449 + f64x8::splat(0.6585449182935511) * t111 * t452;
            let t456 = t370 * t455;
            let t459 = -t282 + t290 + f64x8::splat(0.6585449182935511) * t111 * t446 - f64x8::splat(0.6585449182935511) * t367 * t456;
            let t462 = t96 * t105 * t459 * t394;
            let t463 = t170 + t195 - t199 - t204 + t410 + t251 + t412 - t258 - t263 + t423 + t462;
            let tvrho1 = t7 * t463 + t164 - t33 + t89 + t91;
            acc_vrho_1 = tvrho1;
            let t465 = t7 * t93;
            let t466 = t465 * t95;
            let t467 = t115 * t56;
            let t469 = t467 * t118 * t318;
            let t471 = t134 * t113;
            let t472 = t111 * t471;
            let t473 = t472 * t148;
            let t475 = t469 / f64x8::splat(96.0) + f64x8::splat(0.0004287401811806974) * t473;
            let t476 = t475 * t125;
            let t477 = t476 * t157;
            let t480 = t107 * t107;
            let t481 = t480 * t284;
            let t482 = t481 * t151;
            let t483 = t93 * t93;
            let t484 = f64x8::splat(1.0) / t483;
            let t485 = t484 * t369;
            let t486 = t133 * t475;
            let t487 = t485 * t486;
            let t490 = f64x8::splat(0.6585449182935511) * t111 * t477 - f64x8::splat(0.43368140941025995) * t482 * t487;
            let t491 = t105 * t490;
            let t492 = t491 * t394;
            let tvsigma0 = t466 * t492;
            acc_vsigma_0 = tvsigma0;
            let t495 = t469 / f64x8::splat(48.0) + f64x8::splat(0.0008574803623613948) * t473;
            let t496 = t495 * t125;
            let t497 = t496 * t157;
            let t500 = t133 * t495;
            let t501 = t485 * t500;
            let t504 = f64x8::splat(0.6585449182935511) * t111 * t497 - f64x8::splat(0.43368140941025995) * t482 * t501;
            let t505 = t105 * t504;
            let t506 = t505 * t394;
            let tvsigma1 = t466 * t506;
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

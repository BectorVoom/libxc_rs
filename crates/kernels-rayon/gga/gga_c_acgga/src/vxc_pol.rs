//! GGA_C_ACGGA vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_acgga.c`
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
pub fn gga_c_acgga_vxc_pol(
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
            let t107 = v_sigma0 + f64x8::splat(2.0) * v_sigma1 + v_sigma2;
            let t109 = f64x8::splat(1.0) / t8 / t37;
            let t110 = t107 * t109;
            let t111 = f64x8::splat(1.0) / t104;
            let t112 = t56 * t111;
            let t113 = t110 * t112;
            let t114 = f64x8::splat(1.0) / t3;
            let t115 = t19 * t114;
            let t116 = ((t107).sqrt());
            let t118 = f64x8::splat(1.0) / t8 / t7;
            let t119 = t116 * t118;
            let t120 = t56 * t56;
            let t121 = f64x8::splat(1.0) / t103;
            let t122 = t120 * t121;
            let t123 = f64x8::splat(1.0) / t14;
            let t124 = t122 * t123;
            let t125 = t119 * t124;
            let t127 = f64x8::splat(4.5) + t125 / f64x8::splat(4.0);
            let t128 = t5 * t127;
            let t130 = f64x8::splat(4.5) + f64x8::splat(0.36675) * t125;
            let t131 = f64x8::splat(1.0) / t130;
            let t132 = t128 * t131;
            let t133 = t115 * t132;
            let t136 = f64x8::splat(1.0) / t93;
            let t138 = (-t33 + t89 + t91) * t136;
            let t139 = f64x8::splat(1.0) / t105;
            let t140 = t94 * t139;
            let t142 = (simd::exp(-t138 * t140));
            let t143 = t142 - f64x8::splat(1.0);
            let t144 = f64x8::splat(1.0) / t143;
            let t145 = t136 * t144;
            let t146 = t107 * t107;
            let t148 = f64x8::splat(1.0) / t22 / t38;
            let t149 = t146 * t148;
            let t151 = t145 * t149 * t120;
            let t152 = t104 * t104;
            let t153 = f64x8::splat(1.0) / t152;
            let t154 = t153 * t1;
            let t155 = f64x8::splat(1.0) / t20;
            let t156 = t154 * t155;
            let t157 = t127 * t127;
            let t158 = t6 * t157;
            let t159 = t130 * t130;
            let t160 = f64x8::splat(1.0) / t159;
            let t161 = t158 * t160;
            let t162 = t156 * t161;
            let t165 = t113 * t133 / f64x8::splat(96.0) + f64x8::splat(0.0002143700905903487) * t151 * t162;
            let t166 = t165 * t136;
            let t169 = f64x8::splat(1.0) + f64x8::splat(0.6585449182935511) * t145 * t165;
            let t170 = f64x8::splat(1.0) / t169;
            let t173 = f64x8::splat(1.0) + f64x8::splat(0.6585449182935511) * t166 * t170;
            let t174 = (simd::ln(t173));
            let t176 = t96 * t105 * t174;
            let tzk0 = -t33 + t89 + t91 + t176;
            acc_zk = tzk0;
            let t177 = t6 * t118;
            let t179 = t4 * t177 * t31;
            let t180 = f64x8::splat(0.0011073470983333333) * t179;
            let t181 = t27 * t27;
            let t182 = f64x8::splat(1.0) / t181;
            let t183 = t13 * t182;
            let t184 = t123 * t1;
            let t185 = t3 * t6;
            let t186 = t185 * t118;
            let t187 = t184 * t186;
            let t189 = t4 * t177;
            let t191 = ((t11).sqrt());
            let t192 = t191 * t1;
            let t193 = t192 * t186;
            let t198 = t21 * t5 / t22 / t7;
            let t200 = -f64x8::splat(0.632975) * t187 - f64x8::splat(0.29896666666666666) * t189 - f64x8::splat(0.1023875) * t193 - f64x8::splat(0.08215666666666667) * t198;
            let t201 = f64x8::splat(1.0) / t30;
            let t202 = t200 * t201;
            let t203 = t183 * t202;
            let t204 = f64x8::splat(1.0) * t203;
            let t205 = t35 * t34;
            let t206 = t205 * t39;
            let t207 = t206 * t88;
            let t208 = f64x8::splat(4.0) * t207;
            let t209 = t38 * t7;
            let t210 = f64x8::splat(1.0) / t209;
            let t211 = t36 * t210;
            let t212 = t211 * t88;
            let t213 = f64x8::splat(4.0) * t212;
            let t214 = f64x8::splat(1.0) / t37;
            let t215 = t34 * t214;
            let t216 = t41 - t215;
            let t219 = ((t44).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t47 * t216));
            let t220 = -t216;
            let t223 = ((t51).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t52 * t220));
            let t225 = (t219 + t223) * t59;
            let t226 = t225 * t87;
            let t227 = t40 * t226;
            let t231 = t67 * t67;
            let t232 = f64x8::splat(1.0) / t231;
            let t233 = t62 * t232;
            let t238 = -f64x8::splat(1.176575) * t187 - f64x8::splat(0.516475) * t189 - f64x8::splat(0.2103875) * t193 - f64x8::splat(0.104195) * t198;
            let t239 = f64x8::splat(1.0) / t70;
            let t240 = t238 * t239;
            let t246 = t80 * t80;
            let t247 = f64x8::splat(1.0) / t246;
            let t248 = t75 * t247;
            let t253 = -f64x8::splat(0.8630833333333333) * t187 - f64x8::splat(0.301925) * t189 - f64x8::splat(0.05501625) * t193 - f64x8::splat(0.082785) * t198;
            let t254 = f64x8::splat(1.0) / t83;
            let t255 = t253 * t254;
            let t258 = f64x8::splat(0.0005323764196666666) * t4 * t177 * t71 + f64x8::splat(1.0) * t233 * t240 - t180 - t204 + f64x8::splat(0.00018311447306006544) * t4 * t177 * t84 + f64x8::splat(0.5848223622634646) * t248 * t255;
            let t259 = t60 * t258;
            let t260 = t40 * t259;
            let t261 = t225 * t85;
            let t262 = f64x8::splat(0.0197516734986138) * t261;
            let t263 = t60 * t1;
            let t265 = t185 * t118 * t84;
            let t266 = t263 * t265;
            let t267 = f64x8::splat(0.00018311447306006544) * t266;
            let t268 = t60 * t75;
            let t270 = t247 * t253 * t254;
            let t271 = t268 * t270;
            let t272 = f64x8::splat(0.5848223622634646) * t271;
            let t273 = t104 * t174;
            let t274 = f64x8::splat(1.0) / t47;
            let t277 = ((t44).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t274 * t216));
            let t278 = f64x8::splat(1.0) / t52;
            let t281 = ((t51).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t278 * t220));
            let t283 = t277 / f64x8::splat(2.0) + t281 / f64x8::splat(2.0);
            let t285 = t96 * t273 * t283;
            let t286 = f64x8::splat(3.0) * t285;
            let t287 = t37 * t7;
            let t289 = f64x8::splat(1.0) / t8 / t287;
            let t290 = t107 * t289;
            let t291 = t290 * t112;
            let t293 = f64x8::splat(7.0) / f64x8::splat(288.0) * t291 * t133;
            let t295 = t56 * t139 * t19;
            let t296 = t110 * t295;
            let t297 = t114 * t5;
            let t298 = t127 * t131;
            let t300 = t297 * t298 * t283;
            let t303 = t116 * t109;
            let t304 = t303 * t124;
            let t305 = t304 / f64x8::splat(3.0);
            let t306 = t119 * t120;
            let t307 = t111 * t123;
            let t308 = t307 * t283;
            let t309 = t306 * t308;
            let t312 = f64x8::splat(1.0) / t22 / t37;
            let t313 = t116 * t312;
            let t316 = f64x8::splat(1.0) / t14 / t11;
            let t318 = t316 * t1 * t185;
            let t319 = t313 * t122 * t318;
            let t320 = t319 / f64x8::splat(24.0);
            let t321 = -t305 - t309 / f64x8::splat(4.0) + t320;
            let t322 = t5 * t321;
            let t324 = t115 * t322 * t131;
            let t327 = t112 * t19;
            let t328 = t110 * t327;
            let t329 = t127 * t160;
            let t330 = f64x8::splat(0.489) * t304;
            let t332 = f64x8::splat(0.061125) * t319;
            let t333 = -t330 - f64x8::splat(0.36675) * t309 + t332;
            let t335 = t297 * t329 * t333;
            let t338 = t143 * t143;
            let t339 = f64x8::splat(1.0) / t338;
            let t340 = t136 * t339;
            let t341 = t340 * t146;
            let t342 = t148 * t120;
            let t343 = t342 * t153;
            let t344 = t341 * t343;
            let t345 = t1 * t155;
            let t346 = t345 * t6;
            let t347 = t157 * t160;
            let t349 = (t180 + t204 + t208 - t213 + t227 + t260 + t262 - t267 - t272) * t136;
            let t351 = t94 * t153;
            let t352 = t351 * t283;
            let t355 = f64x8::splat(3.0) * t138 * t352 - t349 * t140;
            let t356 = t355 * t142;
            let t358 = t346 * t347 * t356;
            let t362 = f64x8::splat(1.0) / t22 / t209;
            let t365 = t145 * t146 * t362 * t120;
            let t367 = f64x8::splat(0.0010003937560882938) * t365 * t162;
            let t368 = t145 * t146;
            let t370 = f64x8::splat(1.0) / t152 / t103;
            let t371 = t342 * t370;
            let t372 = t368 * t371;
            let t374 = t346 * t347 * t283;
            let t377 = t368 * t343;
            let t379 = t346 * t329 * t321;
            let t383 = f64x8::splat(1.0) / t159 / t130;
            let t384 = t157 * t383;
            let t386 = t346 * t384 * t333;
            let t389 = -t293 - t296 * t300 / f64x8::splat(48.0) + t113 * t324 / f64x8::splat(96.0) - t328 * t335 / f64x8::splat(96.0) - f64x8::splat(0.0002143700905903487) * t344 * t358 - t367 - f64x8::splat(0.0008574803623613948) * t372 * t374 + f64x8::splat(0.0004287401811806974) * t377 * t379 - f64x8::splat(0.0004287401811806974) * t377 * t386;
            let t390 = t389 * t136;
            let t393 = t169 * t169;
            let t394 = f64x8::splat(1.0) / t393;
            let t401 = -f64x8::splat(0.6585449182935511) * t340 * t165 * t355 * t142 + f64x8::splat(0.6585449182935511) * t145 * t389;
            let t402 = t394 * t401;
            let t405 = f64x8::splat(0.6585449182935511) * t390 * t170 - f64x8::splat(0.6585449182935511) * t166 * t402;
            let t407 = f64x8::splat(1.0) / t173;
            let t409 = t96 * t105 * t405 * t407;
            let t410 = t180 + t204 + t208 - t213 + t227 + t260 + t262 - t267 - t272 + t286 + t409;
            let tvrho0 = t7 * t410 + t176 - t33 + t89 + t91;
            acc_vrho_0 = tvrho0;
            let t412 = -t41 - t215;
            let t415 = ((t44).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t47 * t412));
            let t416 = -t412;
            let t419 = ((t51).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t52 * t416));
            let t421 = (t415 + t419) * t59;
            let t422 = t421 * t87;
            let t423 = t40 * t422;
            let t424 = t421 * t85;
            let t425 = f64x8::splat(0.0197516734986138) * t424;
            let t428 = ((t44).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t274 * t412));
            let t431 = ((t51).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t278 * t416));
            let t433 = t428 / f64x8::splat(2.0) + t431 / f64x8::splat(2.0);
            let t435 = t96 * t273 * t433;
            let t436 = f64x8::splat(3.0) * t435;
            let t438 = t297 * t298 * t433;
            let t441 = t307 * t433;
            let t442 = t306 * t441;
            let t444 = -t305 - t442 / f64x8::splat(4.0) + t320;
            let t445 = t5 * t444;
            let t447 = t115 * t445 * t131;
            let t451 = -t330 - f64x8::splat(0.36675) * t442 + t332;
            let t453 = t297 * t329 * t451;
            let t457 = (t180 + t204 - t208 - t213 + t423 + t260 + t425 - t267 - t272) * t136;
            let t459 = t351 * t433;
            let t462 = f64x8::splat(3.0) * t138 * t459 - t457 * t140;
            let t463 = t462 * t142;
            let t465 = t346 * t347 * t463;
            let t469 = t346 * t347 * t433;
            let t473 = t346 * t329 * t444;
            let t477 = t346 * t384 * t451;
            let t480 = -t293 - t296 * t438 / f64x8::splat(48.0) + t113 * t447 / f64x8::splat(96.0) - t328 * t453 / f64x8::splat(96.0) - f64x8::splat(0.0002143700905903487) * t344 * t465 - t367 - f64x8::splat(0.0008574803623613948) * t372 * t469 + f64x8::splat(0.0004287401811806974) * t377 * t473 - f64x8::splat(0.0004287401811806974) * t377 * t477;
            let t481 = t480 * t136;
            let t490 = -f64x8::splat(0.6585449182935511) * t340 * t165 * t462 * t142 + f64x8::splat(0.6585449182935511) * t145 * t480;
            let t491 = t394 * t490;
            let t494 = f64x8::splat(0.6585449182935511) * t481 * t170 - f64x8::splat(0.6585449182935511) * t166 * t491;
            let t497 = t96 * t105 * t494 * t407;
            let t498 = t180 + t204 - t208 - t213 + t423 + t260 + t425 - t267 - t272 + t436 + t497;
            let tvrho1 = t7 * t498 + t176 - t33 + t89 + t91;
            acc_vrho_1 = tvrho1;
            let t500 = t7 * t93;
            let t501 = t500 * t95;
            let t502 = t109 * t56;
            let t503 = t111 * t19;
            let t504 = t502 * t503;
            let t505 = t297 * t298;
            let t506 = t504 * t505;
            let t509 = f64x8::splat(1.0) / t22 / t287;
            let t510 = t116 * t509;
            let t511 = t139 * t19;
            let t512 = t510 * t511;
            let t513 = t123 * t131;
            let t514 = t297 * t513;
            let t515 = t512 * t514;
            let t517 = t329 * t123;
            let t518 = t297 * t517;
            let t519 = t512 * t518;
            let t523 = t145 * t107 * t148 * t120;
            let t524 = t523 * t162;
            let t526 = t116 * t107;
            let t527 = t145 * t526;
            let t528 = t38 * t37;
            let t529 = f64x8::splat(1.0) / t528;
            let t530 = t529 * t56;
            let t531 = t530 * t370;
            let t532 = t527 * t531;
            let t533 = t346 * t517;
            let t534 = t532 * t533;
            let t537 = t346 * t384 * t123;
            let t538 = t532 * t537;
            let t540 = t506 / f64x8::splat(96.0) + t515 / f64x8::splat(384.0) - f64x8::splat(0.0038203125) * t519 + f64x8::splat(0.0004287401811806974) * t524 + f64x8::splat(0.00010718504529517435) * t534 - f64x8::splat(0.00015724046144802075) * t538;
            let t541 = t540 * t136;
            let t544 = t93 * t93;
            let t545 = f64x8::splat(1.0) / t544;
            let t546 = t165 * t545;
            let t547 = t394 * t144;
            let t548 = t547 * t540;
            let t551 = f64x8::splat(0.6585449182935511) * t541 * t170 - f64x8::splat(0.43368140941025995) * t546 * t548;
            let t552 = t105 * t551;
            let t553 = t552 * t407;
            let tvsigma0 = t501 * t553;
            acc_vsigma_0 = tvsigma0;
            let t560 = t506 / f64x8::splat(48.0) + t515 / f64x8::splat(192.0) - f64x8::splat(0.007640625) * t519 + f64x8::splat(0.0008574803623613948) * t524 + f64x8::splat(0.0002143700905903487) * t534 - f64x8::splat(0.0003144809228960415) * t538;
            let t561 = t560 * t136;
            let t564 = t547 * t560;
            let t567 = f64x8::splat(0.6585449182935511) * t561 * t170 - f64x8::splat(0.43368140941025995) * t546 * t564;
            let t568 = t105 * t567;
            let t569 = t568 * t407;
            let tvsigma1 = t501 * t569;
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

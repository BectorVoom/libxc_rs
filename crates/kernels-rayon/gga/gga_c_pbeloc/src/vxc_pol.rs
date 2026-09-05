//! GGA_C_PBELOC vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_pbeloc.c`
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
pub fn gga_c_pbeloc_vxc_pol(
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
            let t114 = f64x8::splat(1.0) / t3;
            let t115 = t19 * t114;
            let t117 = (simd::exp(-t25 / f64x8::splat(4.0)));
            let t118 = f64x8::splat(1.0) - t117;
            let t119 = t5 * t118;
            let t120 = t115 * t119;
            let t123 = f64x8::splat(0.0375) + f64x8::splat(0.0008333333333333334) * t110 * t112 * t120;
            let t125 = t111 * t19;
            let t126 = t114 * t5;
            let t127 = t125 * t126;
            let t130 = f64x8::splat(1.0) / t93;
            let t131 = t123 * t130;
            let t133 = (-t33 + t89 + t91) * t130;
            let t134 = f64x8::splat(1.0) / t105;
            let t135 = t94 * t134;
            let t137 = (simd::exp(-t133 * t135));
            let t138 = t137 - f64x8::splat(1.0);
            let t139 = f64x8::splat(1.0) / t138;
            let t140 = t94 * t139;
            let t141 = t107 * t107;
            let t142 = t140 * t141;
            let t143 = t131 * t142;
            let t145 = f64x8::splat(1.0) / t22 / t38;
            let t146 = t56 * t56;
            let t147 = t145 * t146;
            let t148 = t104 * t104;
            let t149 = f64x8::splat(1.0) / t148;
            let t151 = f64x8::splat(1.0) / t20;
            let t152 = t1 * t151;
            let t153 = t152 * t6;
            let t154 = t147 * t149 * t153;
            let t157 = t110 * t56 * t127 / f64x8::splat(96.0) + t143 * t154 / f64x8::splat(3072.0);
            let t158 = t123 * t157;
            let t159 = t130 * t94;
            let t160 = t140 * t157;
            let t162 = t131 * t160 + f64x8::splat(1.0);
            let t163 = f64x8::splat(1.0) / t162;
            let t164 = t159 * t163;
            let t166 = t158 * t164 + f64x8::splat(1.0);
            let t167 = (simd::ln(t166));
            let t169 = t96 * t105 * t167;
            let tzk0 = -t33 + t89 + t91 + t169;
            acc_zk = tzk0;
            let t171 = f64x8::splat(1.0) / t8 / t7;
            let t172 = t6 * t171;
            let t174 = t4 * t172 * t31;
            let t175 = f64x8::splat(0.0011073470983333333) * t174;
            let t176 = t27 * t27;
            let t177 = f64x8::splat(1.0) / t176;
            let t178 = t13 * t177;
            let t180 = f64x8::splat(1.0) / t14 * t1;
            let t181 = t3 * t6;
            let t182 = t181 * t171;
            let t183 = t180 * t182;
            let t185 = t4 * t172;
            let t187 = ((t11).sqrt());
            let t188 = t187 * t1;
            let t189 = t188 * t182;
            let t194 = t21 * t5 / t22 / t7;
            let t196 = -f64x8::splat(0.632975) * t183 - f64x8::splat(0.29896666666666666) * t185 - f64x8::splat(0.1023875) * t189 - f64x8::splat(0.08215666666666667) * t194;
            let t197 = f64x8::splat(1.0) / t30;
            let t198 = t196 * t197;
            let t199 = t178 * t198;
            let t200 = f64x8::splat(1.0) * t199;
            let t201 = t35 * t34;
            let t202 = t201 * t39;
            let t203 = t202 * t88;
            let t204 = f64x8::splat(4.0) * t203;
            let t205 = t38 * t7;
            let t206 = f64x8::splat(1.0) / t205;
            let t207 = t36 * t206;
            let t208 = t207 * t88;
            let t209 = f64x8::splat(4.0) * t208;
            let t210 = f64x8::splat(1.0) / t37;
            let t211 = t34 * t210;
            let t212 = t41 - t211;
            let t215 = ((t44).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t47 * t212));
            let t216 = -t212;
            let t219 = ((t51).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t52 * t216));
            let t221 = (t215 + t219) * t59;
            let t222 = t221 * t87;
            let t223 = t40 * t222;
            let t227 = t67 * t67;
            let t228 = f64x8::splat(1.0) / t227;
            let t229 = t62 * t228;
            let t234 = -f64x8::splat(1.176575) * t183 - f64x8::splat(0.516475) * t185 - f64x8::splat(0.2103875) * t189 - f64x8::splat(0.104195) * t194;
            let t235 = f64x8::splat(1.0) / t70;
            let t236 = t234 * t235;
            let t242 = t80 * t80;
            let t243 = f64x8::splat(1.0) / t242;
            let t244 = t75 * t243;
            let t249 = -f64x8::splat(0.8630833333333333) * t183 - f64x8::splat(0.301925) * t185 - f64x8::splat(0.05501625) * t189 - f64x8::splat(0.082785) * t194;
            let t250 = f64x8::splat(1.0) / t83;
            let t251 = t249 * t250;
            let t254 = f64x8::splat(0.0005323764196666666) * t4 * t172 * t71 + f64x8::splat(1.0) * t229 * t236 - t175 - t200 + f64x8::splat(0.00018311447306006544) * t4 * t172 * t84 + f64x8::splat(0.5848223622634646) * t244 * t251;
            let t255 = t60 * t254;
            let t256 = t40 * t255;
            let t257 = t221 * t85;
            let t258 = f64x8::splat(0.0197516734986138) * t257;
            let t259 = t60 * t1;
            let t261 = t181 * t171 * t84;
            let t262 = t259 * t261;
            let t263 = f64x8::splat(0.00018311447306006544) * t262;
            let t264 = t60 * t75;
            let t266 = t243 * t249 * t250;
            let t267 = t264 * t266;
            let t268 = f64x8::splat(0.5848223622634646) * t267;
            let t269 = t104 * t167;
            let t270 = f64x8::splat(1.0) / t47;
            let t273 = ((t44).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t270 * t212));
            let t274 = f64x8::splat(1.0) / t52;
            let t277 = ((t51).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t274 * t216));
            let t279 = t273 / f64x8::splat(2.0) + t277 / f64x8::splat(2.0);
            let t281 = t96 * t269 * t279;
            let t282 = f64x8::splat(3.0) * t281;
            let t283 = t37 * t7;
            let t285 = f64x8::splat(1.0) / t8 / t283;
            let t286 = t107 * t285;
            let t289 = f64x8::splat(0.0019444444444444444) * t286 * t112 * t120;
            let t290 = t56 * t134;
            let t291 = t110 * t290;
            let t293 = t115 * t119 * t279;
            let t296 = t107 * t39;
            let t298 = t6 * t117;
            let t299 = t4 * t298;
            let t301 = f64x8::splat(0.0004166666666666667) * t296 * t112 * t299;
            let t302 = -t289 - f64x8::splat(0.0016666666666666668) * t291 * t293 - t301;
            let t303 = t302 * t157;
            let t307 = f64x8::splat(7.0) / f64x8::splat(288.0) * t286 * t56 * t127;
            let t308 = t5 * t279;
            let t309 = t115 * t308;
            let t312 = t302 * t130;
            let t313 = t312 * t142;
            let t316 = t131 * t94;
            let t317 = t138 * t138;
            let t318 = f64x8::splat(1.0) / t317;
            let t319 = t318 * t141;
            let t320 = t319 * t145;
            let t321 = t316 * t320;
            let t322 = t146 * t149;
            let t323 = t322 * t1;
            let t324 = t151 * t6;
            let t326 = (t175 + t200 + t204 - t209 + t223 + t256 + t258 - t263 - t268) * t130;
            let t328 = t94 * t149;
            let t329 = t328 * t279;
            let t332 = f64x8::splat(3.0) * t133 * t329 - t326 * t135;
            let t333 = t332 * t137;
            let t335 = t323 * t324 * t333;
            let t339 = f64x8::splat(1.0) / t22 / t205;
            let t340 = t339 * t146;
            let t342 = t340 * t149 * t153;
            let t344 = f64x8::splat(7.0) / f64x8::splat(4608.0) * t143 * t342;
            let t345 = t139 * t141;
            let t346 = t345 * t145;
            let t347 = t316 * t346;
            let t349 = f64x8::splat(1.0) / t148 / t103;
            let t351 = t146 * t349 * t1;
            let t353 = t351 * t324 * t279;
            let t356 = -t307 - t291 * t309 / f64x8::splat(48.0) + t313 * t154 / f64x8::splat(3072.0) - t321 * t335 / f64x8::splat(3072.0) - t344 - t347 * t353 / f64x8::splat(768.0);
            let t357 = t123 * t356;
            let t359 = t158 * t130;
            let t360 = t162 * t162;
            let t361 = f64x8::splat(1.0) / t360;
            let t362 = t94 * t361;
            let t364 = t318 * t157;
            let t365 = t364 * t333;
            let t367 = t140 * t356;
            let t369 = t131 * t367 + t312 * t160 - t316 * t365;
            let t370 = t362 * t369;
            let t372 = t303 * t164 + t357 * t164 - t359 * t370;
            let t374 = f64x8::splat(1.0) / t166;
            let t376 = t96 * t105 * t372 * t374;
            let t377 = t175 + t200 + t204 - t209 + t223 + t256 + t258 - t263 - t268 + t282 + t376;
            let tvrho0 = t7 * t377 + t169 - t33 + t89 + t91;
            acc_vrho_0 = tvrho0;
            let t379 = -t41 - t211;
            let t382 = ((t44).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t47 * t379));
            let t383 = -t379;
            let t386 = ((t51).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t52 * t383));
            let t388 = (t382 + t386) * t59;
            let t389 = t388 * t87;
            let t390 = t40 * t389;
            let t391 = t388 * t85;
            let t392 = f64x8::splat(0.0197516734986138) * t391;
            let t395 = ((t44).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t270 * t379));
            let t398 = ((t51).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t274 * t383));
            let t400 = t395 / f64x8::splat(2.0) + t398 / f64x8::splat(2.0);
            let t402 = t96 * t269 * t400;
            let t403 = f64x8::splat(3.0) * t402;
            let t405 = t115 * t119 * t400;
            let t408 = -t289 - f64x8::splat(0.0016666666666666668) * t291 * t405 - t301;
            let t409 = t408 * t157;
            let t411 = t5 * t400;
            let t412 = t115 * t411;
            let t415 = t408 * t130;
            let t416 = t415 * t142;
            let t420 = (t175 + t200 - t204 - t209 + t390 + t256 + t392 - t263 - t268) * t130;
            let t422 = t328 * t400;
            let t425 = f64x8::splat(3.0) * t133 * t422 - t420 * t135;
            let t426 = t425 * t137;
            let t428 = t323 * t324 * t426;
            let t432 = t351 * t324 * t400;
            let t435 = -t307 - t291 * t412 / f64x8::splat(48.0) + t416 * t154 / f64x8::splat(3072.0) - t321 * t428 / f64x8::splat(3072.0) - t344 - t347 * t432 / f64x8::splat(768.0);
            let t436 = t123 * t435;
            let t439 = t364 * t426;
            let t441 = t140 * t435;
            let t443 = t131 * t441 + t415 * t160 - t316 * t439;
            let t444 = t362 * t443;
            let t446 = t409 * t164 + t436 * t164 - t359 * t444;
            let t449 = t96 * t105 * t446 * t374;
            let t450 = t175 + t200 - t204 - t209 + t390 + t256 + t392 - t263 - t268 + t403 + t449;
            let tvrho1 = t7 * t450 + t169 - t33 + t89 + t91;
            acc_vrho_1 = tvrho1;
            let t452 = t7 * t93;
            let t453 = t452 * t95;
            let t454 = t109 * t56;
            let t455 = t125 * t114;
            let t456 = t454 * t455;
            let t458 = t157 * t130 * t163;
            let t459 = t119 * t458;
            let t460 = t456 * t459;
            let t463 = t115 * t5;
            let t464 = t454 * t111 * t463;
            let t466 = t38 * t283;
            let t467 = f64x8::splat(1.0) / t466;
            let t469 = f64x8::splat(1.0) / t148 / t104;
            let t470 = t467 * t469;
            let t471 = t470 * t118;
            let t472 = t130 * t139;
            let t473 = t472 * t141;
            let t474 = t471 * t473;
            let t476 = t140 * t107;
            let t477 = t131 * t476;
            let t478 = t477 * t154;
            let t480 = t464 / f64x8::splat(96.0) + f64x8::splat(0.00020186378047070194) * t474 + t478 / f64x8::splat(1536.0);
            let t481 = t123 * t480;
            let t483 = t472 * t157;
            let t484 = t119 * t483;
            let t485 = t456 * t484;
            let t487 = t140 * t480;
            let t489 = f64x8::splat(0.008224670334241133) * t485 + t131 * t487;
            let t490 = t362 * t489;
            let t492 = f64x8::splat(0.008224670334241133) * t460 + t481 * t164 - t359 * t490;
            let t493 = t105 * t492;
            let t494 = t493 * t374;
            let tvsigma0 = t453 * t494;
            acc_vsigma_0 = tvsigma0;
            let t499 = t464 / f64x8::splat(48.0) + f64x8::splat(0.0004037275609414039) * t474 + t478 / f64x8::splat(768.0);
            let t500 = t123 * t499;
            let t503 = t140 * t499;
            let t505 = f64x8::splat(0.016449340668482266) * t485 + t131 * t503;
            let t506 = t362 * t505;
            let t508 = f64x8::splat(0.016449340668482266) * t460 + t500 * t164 - t359 * t506;
            let t509 = t105 * t508;
            let t510 = t509 * t374;
            let tvsigma1 = t453 * t510;
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

//! GGA_C_ZVPBEINT vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_zvpbeint.c`
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
pub fn gga_c_zvpbeint_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_alpha: f64,
    param_omega: f64,
    param_beta: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_alpha = f64x8::splat(param_alpha);
    let param_omega = f64x8::splat(param_omega);
    let param_beta = f64x8::splat(param_beta);
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
            let t93 = v_sigma0 + f64x8::splat(2.0) * v_sigma1 + v_sigma2;
            let t94 = ((t93).sqrt());
            let t95 = t94 * t93;
            let t96 = param_alpha * t95;
            let t99 = f64x8::splat(1.0) / t14 / t11;
            let t100 = f64x8::splat(1.0) / t3;
            let t101 = t19 * t100;
            let t103 = t101 * t5 * t8;
            let t104 = ((t103).sqrt());
            let t105 = t99 * t104;
            let t106 = f64x8::splat(1.0) / t37;
            let t107 = t35 * t106;
            let t108 = (f64x8::splat(1e-20)).simd_lt(t107);
            let t109 = ((t108).select(t107, f64x8::splat(1e-20)));
            let t111 = (simd::pow(t109, param_omega / f64x8::splat(2.0)));
            let t112 = t105 * t111;
            let t115 = (simd::exp(-t96 * t39 * t112 / f64x8::splat(16.0)));
            let t116 = (simd::ln(f64x8::splat(2.0)));
            let t117 = f64x8::splat(1.0) - t116;
            let t118 = t115 * t117;
            let t119 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t120 = f64x8::splat(1.0) / t119;
            let t121 = t45 * t45;
            let t122 = t47 * t47;
            let t123 = ((t44).select(t121, t122));
            let t124 = t52 * t52;
            let t125 = ((t51).select(t121, t124));
            let t127 = t123 / f64x8::splat(2.0) + t125 / f64x8::splat(2.0);
            let t128 = t127 * t127;
            let t129 = t128 * t127;
            let t130 = t120 * t129;
            let t132 = f64x8::splat(1.0) / t8 / t37;
            let t133 = t93 * t132;
            let t135 = f64x8::splat(1.0) / t128;
            let t137 = t100 * t5;
            let t138 = t135 * t19 * t137;
            let t141 = f64x8::splat(1.0) / t117;
            let t142 = param_beta * t141;
            let t144 = (-t33 + t89 + t91) * t141;
            let t145 = f64x8::splat(1.0) / t129;
            let t146 = t119 * t145;
            let t148 = (simd::exp(-t144 * t146));
            let t149 = t148 - f64x8::splat(1.0);
            let t150 = f64x8::splat(1.0) / t149;
            let t151 = t119 * t150;
            let t152 = t93 * t93;
            let t154 = t142 * t151 * t152;
            let t156 = f64x8::splat(1.0) / t22 / t38;
            let t157 = t56 * t56;
            let t158 = t156 * t157;
            let t159 = t128 * t128;
            let t160 = f64x8::splat(1.0) / t159;
            let t161 = t158 * t160;
            let t162 = f64x8::splat(1.0) / t20;
            let t163 = t1 * t162;
            let t164 = t163 * t6;
            let t165 = t161 * t164;
            let t168 = t133 * t56 * t138 / f64x8::splat(96.0) + t154 * t165 / f64x8::splat(3072.0);
            let t169 = param_beta * t168;
            let t170 = t141 * t119;
            let t173 = t142 * t151 * t168 + f64x8::splat(1.0);
            let t174 = f64x8::splat(1.0) / t173;
            let t175 = t170 * t174;
            let t177 = t169 * t175 + f64x8::splat(1.0);
            let t178 = (simd::ln(t177));
            let t179 = t130 * t178;
            let t180 = t118 * t179;
            let tzk0 = -t33 + t89 + t91 + t180;
            acc_zk = tzk0;
            let t182 = f64x8::splat(1.0) / t8 / t7;
            let t183 = t6 * t182;
            let t185 = t4 * t183 * t31;
            let t186 = f64x8::splat(0.0011073470983333333) * t185;
            let t187 = t27 * t27;
            let t188 = f64x8::splat(1.0) / t187;
            let t189 = t13 * t188;
            let t191 = f64x8::splat(1.0) / t14 * t1;
            let t192 = t3 * t6;
            let t193 = t192 * t182;
            let t194 = t191 * t193;
            let t196 = t4 * t183;
            let t198 = ((t11).sqrt());
            let t199 = t198 * t1;
            let t200 = t199 * t193;
            let t204 = t5 / t22 / t7;
            let t205 = t21 * t204;
            let t207 = -f64x8::splat(0.632975) * t194 - f64x8::splat(0.29896666666666666) * t196 - f64x8::splat(0.1023875) * t200 - f64x8::splat(0.08215666666666667) * t205;
            let t208 = f64x8::splat(1.0) / t30;
            let t209 = t207 * t208;
            let t210 = t189 * t209;
            let t211 = f64x8::splat(1.0) * t210;
            let t212 = t35 * t34;
            let t213 = t212 * t39;
            let t214 = t213 * t88;
            let t215 = f64x8::splat(4.0) * t214;
            let t216 = t38 * t7;
            let t217 = f64x8::splat(1.0) / t216;
            let t218 = t36 * t217;
            let t219 = t218 * t88;
            let t220 = f64x8::splat(4.0) * t219;
            let t221 = t34 * t106;
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
            let t244 = -f64x8::splat(1.176575) * t194 - f64x8::splat(0.516475) * t196 - f64x8::splat(0.2103875) * t200 - f64x8::splat(0.104195) * t205;
            let t245 = f64x8::splat(1.0) / t70;
            let t246 = t244 * t245;
            let t252 = t80 * t80;
            let t253 = f64x8::splat(1.0) / t252;
            let t254 = t75 * t253;
            let t259 = -f64x8::splat(0.8630833333333333) * t194 - f64x8::splat(0.301925) * t196 - f64x8::splat(0.05501625) * t200 - f64x8::splat(0.082785) * t205;
            let t260 = f64x8::splat(1.0) / t83;
            let t261 = t259 * t260;
            let t264 = f64x8::splat(0.0005323764196666666) * t4 * t183 * t71 + f64x8::splat(1.0) * t239 * t246 - t186 - t211 + f64x8::splat(0.00018311447306006544) * t4 * t183 * t84 + f64x8::splat(0.5848223622634646) * t254 * t261;
            let t265 = t60 * t264;
            let t266 = t40 * t265;
            let t267 = t231 * t85;
            let t268 = f64x8::splat(0.0197516734986138) * t267;
            let t269 = t60 * t1;
            let t271 = t192 * t182 * t84;
            let t272 = t269 * t271;
            let t273 = f64x8::splat(0.00018311447306006544) * t272;
            let t274 = t60 * t75;
            let t276 = t253 * t259 * t260;
            let t277 = t274 * t276;
            let t278 = f64x8::splat(0.5848223622634646) * t277;
            let t281 = t96 * t217 * t112 / f64x8::splat(4.0);
            let t283 = f64x8::splat(1.0) / t8 / t216;
            let t286 = f64x8::splat(1.0) / t14 / t25 / f64x8::splat(4.0);
            let t287 = t283 * t286;
            let t289 = t104 * t111;
            let t290 = t4 * t6;
            let t291 = t289 * t290;
            let t293 = t96 * t287 * t291 / f64x8::splat(32.0);
            let t294 = t156 * t99;
            let t296 = f64x8::splat(1.0) / t104;
            let t297 = t296 * t111;
            let t298 = t101 * t5;
            let t299 = t297 * t298;
            let t301 = t96 * t294 * t299 / f64x8::splat(96.0);
            let t302 = t39 * t99;
            let t303 = t96 * t302;
            let t304 = t37 * t7;
            let t305 = f64x8::splat(1.0) / t304;
            let t306 = t35 * t305;
            let t309 = ((t108).select(f64x8::splat(2.0) * t221 - f64x8::splat(2.0) * t306, f64x8::splat(0.0)));
            let t310 = param_omega * t309;
            let t311 = f64x8::splat(1.0) / t109;
            let t312 = t310 * t311;
            let t313 = t289 * t312;
            let t316 = t281 - t293 - t301 - t303 * t313 / f64x8::splat(32.0);
            let t317 = t316 * t115;
            let t318 = t317 * t117;
            let t319 = t318 * t179;
            let t320 = t118 * t120;
            let t321 = t128 * t178;
            let t322 = f64x8::splat(1.0) / t47;
            let t325 = ((t44).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t322 * t222));
            let t326 = f64x8::splat(1.0) / t52;
            let t329 = ((t51).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t326 * t226));
            let t331 = t325 / f64x8::splat(2.0) + t329 / f64x8::splat(2.0);
            let t332 = t321 * t331;
            let t333 = t320 * t332;
            let t334 = f64x8::splat(3.0) * t333;
            let t336 = f64x8::splat(1.0) / t8 / t304;
            let t337 = t93 * t336;
            let t340 = f64x8::splat(7.0) / f64x8::splat(288.0) * t337 * t56 * t138;
            let t341 = t56 * t145;
            let t342 = t133 * t341;
            let t343 = t5 * t331;
            let t344 = t101 * t343;
            let t347 = t142 * t119;
            let t348 = t149 * t149;
            let t349 = f64x8::splat(1.0) / t348;
            let t350 = t349 * t152;
            let t352 = t347 * t350 * t156;
            let t353 = t157 * t160;
            let t354 = t353 * t1;
            let t355 = t162 * t6;
            let t357 = (t186 + t211 + t215 - t220 + t233 + t266 + t268 - t273 - t278) * t141;
            let t359 = t119 * t160;
            let t360 = t359 * t331;
            let t363 = f64x8::splat(3.0) * t144 * t360 - t357 * t146;
            let t364 = t363 * t148;
            let t365 = t355 * t364;
            let t366 = t354 * t365;
            let t370 = f64x8::splat(1.0) / t22 / t216;
            let t371 = t370 * t157;
            let t372 = t371 * t160;
            let t373 = t372 * t164;
            let t375 = f64x8::splat(7.0) / f64x8::splat(4608.0) * t154 * t373;
            let t376 = t150 * t152;
            let t378 = t347 * t376 * t156;
            let t380 = f64x8::splat(1.0) / t159 / t127;
            let t381 = t157 * t380;
            let t382 = t381 * t1;
            let t384 = t382 * t355 * t331;
            let t387 = -t340 - t342 * t344 / f64x8::splat(48.0) - t352 * t366 / f64x8::splat(3072.0) - t375 - t378 * t384 / f64x8::splat(768.0);
            let t388 = param_beta * t387;
            let t390 = t169 * t141;
            let t391 = t173 * t173;
            let t392 = f64x8::splat(1.0) / t391;
            let t393 = t119 * t392;
            let t394 = t349 * t168;
            let t399 = t142 * t151 * t387 - t347 * t394 * t364;
            let t400 = t393 * t399;
            let t402 = t388 * t175 - t390 * t400;
            let t403 = t129 * t402;
            let t404 = f64x8::splat(1.0) / t177;
            let t405 = t403 * t404;
            let t406 = t320 * t405;
            let t407 = t186 + t211 + t215 - t220 + t233 + t266 + t268 - t273 - t278 + t319 + t334 + t406;
            let tvrho0 = t7 * t407 + t180 - t33 + t89 + t91;
            acc_vrho_0 = tvrho0;
            let t409 = -t41 - t221;
            let t412 = ((t44).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t47 * t409));
            let t413 = -t409;
            let t416 = ((t51).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t52 * t413));
            let t418 = (t412 + t416) * t59;
            let t419 = t418 * t87;
            let t420 = t40 * t419;
            let t421 = t418 * t85;
            let t422 = f64x8::splat(0.0197516734986138) * t421;
            let t425 = ((t108).select(-f64x8::splat(2.0) * t221 - f64x8::splat(2.0) * t306, f64x8::splat(0.0)));
            let t426 = param_omega * t425;
            let t427 = t426 * t311;
            let t428 = t289 * t427;
            let t431 = t281 - t293 - t301 - t303 * t428 / f64x8::splat(32.0);
            let t432 = t431 * t115;
            let t433 = t432 * t117;
            let t434 = t433 * t179;
            let t437 = ((t44).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t322 * t409));
            let t440 = ((t51).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t326 * t413));
            let t442 = t437 / f64x8::splat(2.0) + t440 / f64x8::splat(2.0);
            let t443 = t321 * t442;
            let t444 = t320 * t443;
            let t445 = f64x8::splat(3.0) * t444;
            let t446 = t5 * t442;
            let t447 = t101 * t446;
            let t451 = (t186 + t211 - t215 - t220 + t420 + t266 + t422 - t273 - t278) * t141;
            let t453 = t359 * t442;
            let t456 = f64x8::splat(3.0) * t144 * t453 - t451 * t146;
            let t457 = t456 * t148;
            let t458 = t355 * t457;
            let t459 = t354 * t458;
            let t463 = t382 * t355 * t442;
            let t466 = -t340 - t342 * t447 / f64x8::splat(48.0) - t352 * t459 / f64x8::splat(3072.0) - t375 - t378 * t463 / f64x8::splat(768.0);
            let t467 = param_beta * t466;
            let t473 = t142 * t151 * t466 - t347 * t394 * t457;
            let t474 = t393 * t473;
            let t476 = t467 * t175 - t390 * t474;
            let t477 = t129 * t476;
            let t478 = t477 * t404;
            let t479 = t320 * t478;
            let t480 = t186 + t211 - t215 - t220 + t420 + t266 + t422 - t273 - t278 + t434 + t445 + t479;
            let tvrho1 = t7 * t480 + t180 - t33 + t89 + t91;
            acc_vrho_1 = tvrho1;
            let t482 = param_alpha * t94;
            let t483 = t302 * t104;
            let t484 = t482 * t483;
            let t485 = t111 * t115;
            let t486 = t485 * t117;
            let t487 = t486 * t179;
            let t488 = t484 * t487;
            let t489 = f64x8::splat(3.0) / f64x8::splat(32.0) * t488;
            let t490 = t132 * t56;
            let t492 = t490 * t135 * t298;
            let t495 = t142 * t151 * t93;
            let t496 = t495 * t165;
            let t498 = t492 / f64x8::splat(96.0) + t496 / f64x8::splat(1536.0);
            let t499 = param_beta * t498;
            let t501 = param_beta * param_beta;
            let t502 = t501 * t168;
            let t503 = t117 * t117;
            let t504 = f64x8::splat(1.0) / t503;
            let t505 = t502 * t504;
            let t506 = t119 * t119;
            let t507 = t506 * t392;
            let t509 = t507 * t150 * t498;
            let t511 = t499 * t175 - t505 * t509;
            let t512 = t129 * t511;
            let t513 = t512 * t404;
            let t514 = t320 * t513;
            let tvsigma0 = t7 * (-t489 + t514);
            acc_vsigma_0 = tvsigma0;
            let t516 = f64x8::splat(3.0) / f64x8::splat(16.0) * t488;
            let t519 = t492 / f64x8::splat(48.0) + t496 / f64x8::splat(768.0);
            let t520 = param_beta * t519;
            let t523 = t507 * t150 * t519;
            let t525 = t520 * t175 - t505 * t523;
            let t526 = t129 * t525;
            let t527 = t526 * t404;
            let t528 = t320 * t527;
            let tvsigma1 = t7 * (-t516 + t528);
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

//! MGGA_C_VSXC exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_vsxc.c`
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
pub fn mgga_c_vsxc_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_dss_0: f64,
    param_alpha_ss: f64,
    param_dss_1: f64,
    param_dss_2: f64,
    param_dss_3: f64,
    param_dss_4: f64,
    param_dss_5: f64,
    param_dab_0: f64,
    param_alpha_ab: f64,
    param_dab_1: f64,
    param_dab_2: f64,
    param_dab_3: f64,
    param_dab_4: f64,
    param_dab_5: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_dss_0 = f64x8::splat(param_dss_0);
    let param_alpha_ss = f64x8::splat(param_alpha_ss);
    let param_dss_1 = f64x8::splat(param_dss_1);
    let param_dss_2 = f64x8::splat(param_dss_2);
    let param_dss_3 = f64x8::splat(param_dss_3);
    let param_dss_4 = f64x8::splat(param_dss_4);
    let param_dss_5 = f64x8::splat(param_dss_5);
    let param_dab_0 = f64x8::splat(param_dab_0);
    let param_alpha_ab = f64x8::splat(param_alpha_ab);
    let param_dab_1 = f64x8::splat(param_dab_1);
    let param_dab_2 = f64x8::splat(param_dab_2);
    let param_dab_3 = f64x8::splat(param_dab_3);
    let param_dab_4 = f64x8::splat(param_dab_4);
    let param_dab_5 = f64x8::splat(param_dab_5);
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
        let v_lapl0 = load_strided(lapl, ip, np, 2, 0);
        let v_lapl1 = load_strided(lapl, ip, np, 2, 1);
        let v_tau0 = load_strided(tau, ip, np, 2, 0);
        let v_tau1 = load_strided(tau, ip, np, 2, 1);
        let mut acc_zk = V_ZERO;
        {
            let t3 = v_rho0 - v_rho1;
            let t4 = v_rho0 + v_rho1;
            let t5 = f64x8::splat(1.0) / t4;
            let t6 = t3 * t5;
            let t7 = f64x8::splat(1.0) + t6;
            let t8 = (t7).simd_le(zeta_threshold);
            let t9 = ((v_rho0).simd_le(dens_threshold)) | (t8);
            let t10 = ((t8).select(zeta_threshold, t7));
            let t11 = f64x8::splat(M_CBRT3);
            let t12 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t13 = (simd::cbrt(t12));
            let t14 = t11 * t13;
            let t15 = f64x8::splat(M_CBRT4);
            let t16 = t15 * t15;
            let t17 = t14 * t16;
            let t18 = (simd::cbrt(t4));
            let t19 = f64x8::splat(1.0) / t18;
            let t20 = f64x8::splat(M_CBRT2);
            let t21 = t19 * t20;
            let t22 = (simd::cbrt(zeta_threshold));
            let t23 = f64x8::splat(1.0) / t22;
            let t24 = (simd::cbrt(t7));
            let t26 = ((t8).select(t23, f64x8::splat(1.0) / t24));
            let t28 = t17 * t21 * t26;
            let t30 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t28;
            let t31 = ((t28).sqrt());
            let t34 = ((t28) * (t28).sqrt());
            let t36 = t11 * t11;
            let t37 = t13 * t13;
            let t38 = t36 * t37;
            let t39 = t38 * t15;
            let t40 = t18 * t18;
            let t41 = f64x8::splat(1.0) / t40;
            let t42 = t20 * t20;
            let t43 = t41 * t42;
            let t44 = t26 * t26;
            let t46 = t39 * t43 * t44;
            let t48 = f64x8::splat(3.79785) * t31 + f64x8::splat(0.8969) * t28 + f64x8::splat(0.204775) * t34 + f64x8::splat(0.123235) * t46;
            let t51 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t48;
            let t52 = (simd::ln(t51));
            let t54 = f64x8::splat(0.0621814) * t30 * t52;
            let t56 = t22 * zeta_threshold;
            let t58 = (((f64x8::splat(2.0)).simd_le(zeta_threshold)).select(t56, f64x8::splat(2.0) * t20));
            let t60 = (((f64x8::splat(0.0)).simd_le(zeta_threshold)).select(t56, f64x8::splat(0.0)));
            let t64 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t20 - f64x8::splat(2.0));
            let t65 = (t58 + t60 - f64x8::splat(2.0)) * t64;
            let t67 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t28;
            let t72 = f64x8::splat(7.05945) * t31 + f64x8::splat(1.549425) * t28 + f64x8::splat(0.420775) * t34 + f64x8::splat(0.1562925) * t46;
            let t75 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t72;
            let t76 = (simd::ln(t75));
            let t80 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t28;
            let t85 = f64x8::splat(5.1785) * t31 + f64x8::splat(0.905775) * t28 + f64x8::splat(0.1100325) * t34 + f64x8::splat(0.1241775) * t46;
            let t88 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t85;
            let t89 = (simd::ln(t88));
            let t90 = t80 * t89;
            let t96 = -t54 + t65 * (-f64x8::splat(0.0310907) * t67 * t76 + t54 - f64x8::splat(0.0197516734986138) * t90) + f64x8::splat(0.0197516734986138) * t65 * t90;
            let t99 = ((t9).select(f64x8::splat(0.0), t10 * t96 / f64x8::splat(2.0)));
            let t100 = param_dss_0;
            let t101 = v_rho0 * v_rho0;
            let t102 = (simd::cbrt(v_rho0));
            let t103 = t102 * t102;
            let t105 = f64x8::splat(1.0) / t103 / t101;
            let t106 = v_sigma0 * t105;
            let t108 = f64x8::splat(1.0) / t103 / v_rho0;
            let t110 = f64x8::splat(2.0) * v_tau0 * t108;
            let t111 = f64x8::splat(M_CBRT6);
            let t112 = t111 * t111;
            let t113 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t114 = (simd::cbrt(t113));
            let t115 = t114 * t114;
            let t116 = t112 * t115;
            let t117 = f64x8::splat(3.0) / f64x8::splat(5.0) * t116;
            let t120 = f64x8::splat(1.0) + param_alpha_ss * (t106 + t110 - t117);
            let t123 = param_dss_1;
            let t124 = t123 * v_sigma0;
            let t126 = param_dss_2;
            let t127 = t110 - t117;
            let t129 = t124 * t105 + t126 * t127;
            let t130 = t120 * t120;
            let t131 = f64x8::splat(1.0) / t130;
            let t133 = param_dss_3;
            let t134 = v_sigma0 * v_sigma0;
            let t135 = t133 * t134;
            let t136 = t101 * t101;
            let t137 = t136 * v_rho0;
            let t139 = f64x8::splat(1.0) / t102 / t137;
            let t141 = param_dss_4;
            let t142 = t141 * v_sigma0;
            let t145 = param_dss_5;
            let t146 = t127 * t127;
            let t148 = t142 * t105 * t127 + t135 * t139 + t145 * t146;
            let t149 = t130 * t120;
            let t150 = f64x8::splat(1.0) / t149;
            let t152 = t100 / t120 + t129 * t131 + t148 * t150;
            let t153 = t99 * t152;
            let t154 = f64x8::splat(1.0) / v_rho0;
            let t155 = v_sigma0 * t154;
            let t156 = f64x8::splat(1.0) / v_tau0;
            let t159 = f64x8::splat(1.0) - t155 * t156 / f64x8::splat(8.0);
            let t160 = t153 * t159;
            let t162 = f64x8::splat(1.0) - t6;
            let t163 = (t162).simd_le(zeta_threshold);
            let t164 = ((v_rho1).simd_le(dens_threshold)) | (t163);
            let t165 = ((t163).select(zeta_threshold, t162));
            let t166 = (simd::cbrt(t162));
            let t168 = ((t163).select(t23, f64x8::splat(1.0) / t166));
            let t170 = t17 * t21 * t168;
            let t172 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t170;
            let t173 = ((t170).sqrt());
            let t176 = ((t170) * (t170).sqrt());
            let t178 = t168 * t168;
            let t180 = t39 * t43 * t178;
            let t182 = f64x8::splat(3.79785) * t173 + f64x8::splat(0.8969) * t170 + f64x8::splat(0.204775) * t176 + f64x8::splat(0.123235) * t180;
            let t185 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t182;
            let t186 = (simd::ln(t185));
            let t188 = f64x8::splat(0.0621814) * t172 * t186;
            let t190 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t170;
            let t195 = f64x8::splat(7.05945) * t173 + f64x8::splat(1.549425) * t170 + f64x8::splat(0.420775) * t176 + f64x8::splat(0.1562925) * t180;
            let t198 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t195;
            let t199 = (simd::ln(t198));
            let t203 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t170;
            let t208 = f64x8::splat(5.1785) * t173 + f64x8::splat(0.905775) * t170 + f64x8::splat(0.1100325) * t176 + f64x8::splat(0.1241775) * t180;
            let t211 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t208;
            let t212 = (simd::ln(t211));
            let t213 = t203 * t212;
            let t219 = -t188 + t65 * (-f64x8::splat(0.0310907) * t190 * t199 + t188 - f64x8::splat(0.0197516734986138) * t213) + f64x8::splat(0.0197516734986138) * t65 * t213;
            let t222 = ((t164).select(f64x8::splat(0.0), t165 * t219 / f64x8::splat(2.0)));
            let t223 = v_rho1 * v_rho1;
            let t224 = (simd::cbrt(v_rho1));
            let t225 = t224 * t224;
            let t227 = f64x8::splat(1.0) / t225 / t223;
            let t228 = v_sigma2 * t227;
            let t230 = f64x8::splat(1.0) / t225 / v_rho1;
            let t232 = f64x8::splat(2.0) * v_tau1 * t230;
            let t235 = f64x8::splat(1.0) + param_alpha_ss * (t228 + t232 - t117);
            let t238 = t123 * v_sigma2;
            let t240 = t232 - t117;
            let t242 = t126 * t240 + t238 * t227;
            let t243 = t235 * t235;
            let t244 = f64x8::splat(1.0) / t243;
            let t246 = v_sigma2 * v_sigma2;
            let t247 = t133 * t246;
            let t248 = t223 * t223;
            let t249 = t248 * v_rho1;
            let t251 = f64x8::splat(1.0) / t224 / t249;
            let t253 = t141 * v_sigma2;
            let t256 = t240 * t240;
            let t258 = t253 * t227 * t240 + t145 * t256 + t247 * t251;
            let t259 = t243 * t235;
            let t260 = f64x8::splat(1.0) / t259;
            let t262 = t100 / t235 + t242 * t244 + t258 * t260;
            let t263 = t222 * t262;
            let t264 = f64x8::splat(1.0) / v_rho1;
            let t265 = v_sigma2 * t264;
            let t266 = f64x8::splat(1.0) / v_tau1;
            let t269 = f64x8::splat(1.0) - t265 * t266 / f64x8::splat(8.0);
            let t270 = t263 * t269;
            let t272 = t14 * t16 * t19;
            let t274 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t272;
            let t275 = ((t272).sqrt());
            let t278 = ((t272) * (t272).sqrt());
            let t281 = t38 * t15 * t41;
            let t283 = f64x8::splat(3.79785) * t275 + f64x8::splat(0.8969) * t272 + f64x8::splat(0.204775) * t278 + f64x8::splat(0.123235) * t281;
            let t286 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t283;
            let t287 = (simd::ln(t286));
            let t289 = f64x8::splat(0.0621814) * t274 * t287;
            let t290 = t3 * t3;
            let t291 = t290 * t290;
            let t292 = t4 * t4;
            let t293 = t292 * t292;
            let t294 = f64x8::splat(1.0) / t293;
            let t295 = t291 * t294;
            let t296 = t24 * t7;
            let t297 = ((t8).select(t56, t296));
            let t298 = t166 * t162;
            let t299 = ((t163).select(t56, t298));
            let t300 = t297 + t299 - f64x8::splat(2.0);
            let t301 = t300 * t64;
            let t303 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t272;
            let t308 = f64x8::splat(7.05945) * t275 + f64x8::splat(1.549425) * t272 + f64x8::splat(0.420775) * t278 + f64x8::splat(0.1562925) * t281;
            let t311 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t308;
            let t312 = (simd::ln(t311));
            let t316 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t272;
            let t321 = f64x8::splat(5.1785) * t275 + f64x8::splat(0.905775) * t272 + f64x8::splat(0.1100325) * t278 + f64x8::splat(0.1241775) * t281;
            let t324 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t321;
            let t325 = (simd::ln(t324));
            let t326 = t316 * t325;
            let t328 = -f64x8::splat(0.0310907) * t303 * t312 + t289 - f64x8::splat(0.0197516734986138) * t326;
            let t329 = t301 * t328;
            let t333 = -t289 + t295 * t329 + f64x8::splat(0.0197516734986138) * t301 * t326 - t99 - t222;
            let t334 = param_dab_0;
            let t335 = f64x8::splat(6.0) / f64x8::splat(5.0) * t116;
            let t338 = f64x8::splat(1.0) + param_alpha_ab * (t106 + t228 + t110 + t232 - t335);
            let t341 = param_dab_1;
            let t342 = t106 + t228;
            let t344 = param_dab_2;
            let t345 = t110 + t232 - t335;
            let t347 = t341 * t342 + t344 * t345;
            let t348 = t338 * t338;
            let t349 = f64x8::splat(1.0) / t348;
            let t351 = param_dab_3;
            let t352 = t342 * t342;
            let t354 = param_dab_4;
            let t355 = t354 * t342;
            let t357 = param_dab_5;
            let t358 = t345 * t345;
            let t360 = t355 * t345 + t351 * t352 + t357 * t358;
            let t361 = t348 * t338;
            let t362 = f64x8::splat(1.0) / t361;
            let t364 = t334 / t338 + t347 * t349 + t360 * t362;
            let t365 = t333 * t364;
            let tzk0 = t160 + t270 + t365;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}

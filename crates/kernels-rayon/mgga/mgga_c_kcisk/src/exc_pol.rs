//! MGGA_C_KCISK exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_kcisk.c`
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
pub fn mgga_c_kcisk_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
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
        let v_lapl0 = load_strided(lapl, ip, np, 2, 0);
        let v_lapl1 = load_strided(lapl, ip, np, 2, 1);
        let v_tau0 = load_strided(tau, ip, np, 2, 0);
        let v_tau1 = load_strided(tau, ip, np, 2, 1);
        let mut acc_zk = V_ZERO;
        {
            let t2 = f64x8::splat(M_CBRT3);
            let t3 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t4 = (simd::cbrt(t3));
            let t5 = t2 * t4;
            let t6 = f64x8::splat(M_CBRT4);
            let t7 = t6 * t6;
            let t8 = v_rho0 + v_rho1;
            let t9 = (simd::cbrt(t8));
            let t10 = f64x8::splat(1.0) / t9;
            let t11 = t7 * t10;
            let t12 = t5 * t11;
            let t14 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t12;
            let t15 = ((t12).sqrt());
            let t18 = ((t12) * (t12).sqrt());
            let t20 = t2 * t2;
            let t21 = t4 * t4;
            let t22 = t20 * t21;
            let t23 = t9 * t9;
            let t24 = f64x8::splat(1.0) / t23;
            let t25 = t6 * t24;
            let t26 = t22 * t25;
            let t28 = f64x8::splat(3.79785) * t15 + f64x8::splat(0.8969) * t12 + f64x8::splat(0.204775) * t18 + f64x8::splat(0.123235) * t26;
            let t31 = f64x8::splat(1.0) + f64x8::splat(16.081824322151103) / t28;
            let t32 = (simd::ln(t31));
            let t34 = f64x8::splat(0.062182) * t14 * t32;
            let t36 = (simd::cbrt(zeta_threshold));
            let t37 = t36 * zeta_threshold;
            let t38 = (((f64x8::splat(1.0)).simd_le(zeta_threshold)).select(t37, f64x8::splat(1.0)));
            let t41 = f64x8::splat(M_CBRT2);
            let t44 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t41 - f64x8::splat(2.0));
            let t45 = (f64x8::splat(2.0) * t38 - f64x8::splat(2.0)) * t44;
            let t47 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t12;
            let t52 = f64x8::splat(5.1785) * t15 + f64x8::splat(0.905775) * t12 + f64x8::splat(0.1100325) * t18 + f64x8::splat(0.1241775) * t26;
            let t55 = f64x8::splat(1.0) + f64x8::splat(29.608574643216677) / t52;
            let t56 = (simd::ln(t55));
            let t57 = t47 * t56;
            let t60 = -t34 + f64x8::splat(0.019751789702565206) * t45 * t57;
            let t62 = f64x8::splat(M_CBRT6);
            let t63 = t62 * t62;
            let t64 = t41 * t3 * t63;
            let t65 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t66 = (simd::cbrt(t65));
            let t67 = f64x8::splat(1.0) / t66;
            let t68 = t41 * t2;
            let t69 = t4 * t7;
            let t70 = t69 * t10;
            let t71 = t68 * t70;
            let t73 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t71;
            let t74 = ((t71).sqrt());
            let t77 = ((t71) * (t71).sqrt());
            let t79 = t41 * t41;
            let t80 = t79 * t20;
            let t81 = t21 * t6;
            let t82 = t81 * t24;
            let t83 = t80 * t82;
            let t85 = f64x8::splat(3.79785) * t74 + f64x8::splat(0.8969) * t71 + f64x8::splat(0.204775) * t77 + f64x8::splat(0.123235) * t83;
            let t88 = f64x8::splat(1.0) + f64x8::splat(16.081824322151103) / t85;
            let t89 = (simd::ln(t88));
            let t93 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t71;
            let t98 = f64x8::splat(5.1785) * t74 + f64x8::splat(0.905775) * t71 + f64x8::splat(0.1100325) * t77 + f64x8::splat(0.1241775) * t83;
            let t101 = f64x8::splat(1.0) + f64x8::splat(29.608574643216677) / t98;
            let t102 = (simd::ln(t101));
            let t107 = t67 * (-f64x8::splat(0.062182) * t73 * t89 + f64x8::splat(0.019751789702565206) * t45 * t93 * t102);
            let t110 = f64x8::splat(10.0) / f64x8::splat(9.0) * t64 * t107 * t10;
            let t111 = (t110).simd_lt(-f64x8::splat(0.066725));
            let t113 = ((t111).select(f64x8::splat(0.0), f64x8::splat(0.066725) + t110));
            let t114 = t113 * t41;
            let t116 = v_sigma0 + f64x8::splat(2.0) * v_sigma1 + v_sigma2;
            let t117 = t8 * t8;
            let t119 = f64x8::splat(1.0) / t9 / t117;
            let t120 = t116 * t119;
            let t121 = t114 * t120;
            let t122 = f64x8::splat(1.0) / t4;
            let t123 = t20 * t122;
            let t124 = (f64x8::splat(0.0)).simd_lt(t60);
            let t126 = ((t124).select(t60, -t60));
            let t127 = f64x8::splat(1.0) / t126;
            let t128 = t6 * t127;
            let t129 = t123 * t128;
            let t132 = f64x8::splat(1.0) + f64x8::splat(0.05397236614853195) * t121 * t129;
            let t133 = (simd::ln(t132));
            let t135 = f64x8::splat(1.0) + f64x8::splat(0.193) * t133;
            let t136 = f64x8::splat(1.0) / t135;
            let t138 = f64x8::splat(1.0) / t21;
            let t139 = t2 * t138;
            let t140 = t139 * t7;
            let t141 = t9 * t8;
            let t142 = f64x8::splat(1.0) / t141;
            let t143 = f64x8::splat(1.0) / t8;
            let t146 = f64x8::splat(1.07924) + f64x8::splat(0.03964) * t15 + f64x8::splat(0.0123825) * t12;
            let t149 = f64x8::splat(1.0) + t15 * t146 / f64x8::splat(2.0);
            let t150 = t149 * t149;
            let t151 = f64x8::splat(1.0) / t150;
            let t156 = t2 * t4 * t3;
            let t157 = t7 * t142;
            let t158 = t156 * t157;
            let t161 = t20 * t21 * t3;
            let t163 = f64x8::splat(1.0) / t23 / t8;
            let t164 = t6 * t163;
            let t165 = t161 * t164;
            let t167 = f64x8::splat(1.0) / t117;
            let t171 = t2 * t4 / t65;
            let t172 = t7 * t119;
            let t173 = t171 * t172;
            let t175 = -f64x8::splat(0.005977859662531589) * t143 + f64x8::splat(0.001317375) * t158 - f64x8::splat(0.00023775) * t165 + f64x8::splat(6.474423634745383e-06) * t167 - f64x8::splat(5.40140625e-07) * t173;
            let t177 = f64x8::splat(0.0011713266981940448) * t143 * t151 - t60 * t175;
            let t178 = t142 * t177;
            let t179 = (simd::pow(f64x8::splat(4.0), f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t180 = t139 * t179;
            let t181 = t15 * t12;
            let t182 = t23 * t181;
            let t183 = f64x8::splat(1.0) / t149;
            let t187 = t60 * t60;
            let t189 = f64x8::splat(0.0019711289) * t180 * t182 * t183 - f64x8::splat(2.0) * t187;
            let t190 = f64x8::splat(1.0) / t189;
            let t191 = t190 * t116;
            let t193 = t140 * t178 * t191;
            let t195 = t60 * t136 + f64x8::splat(0.009949166666666667) * t193;
            let t196 = ((f64x8::splat(4.0)).sqrt());
            let t197 = t60 * t196;
            let t198 = t181 * t183;
            let t201 = t7 * t23;
            let t205 = f64x8::splat(0.00619125) * t197 * t198 - f64x8::splat(0.07959333333333334) * t139 * t201 * t175;
            let t206 = t205 * t190;
            let t207 = t116 * t167;
            let t208 = t206 * t207;
            let t210 = t177 * t190;
            let t211 = t116 * t116;
            let t212 = t117 * t117;
            let t213 = f64x8::splat(1.0) / t212;
            let t214 = t211 * t213;
            let t215 = t210 * t214;
            let t217 = f64x8::splat(1.0) + t208 / f64x8::splat(8.0) - t215 / f64x8::splat(64.0);
            let t218 = f64x8::splat(1.0) / t217;
            let t219 = t195 * t218;
            let t220 = v_rho0 - v_rho1;
            let t221 = t220 * t143;
            let t222 = f64x8::splat(1.0) + t221;
            let t223 = (t222).simd_le(zeta_threshold);
            let t224 = (simd::cbrt(t222));
            let t226 = ((t223).select(t37, t224 * t222));
            let t227 = f64x8::splat(1.0) - t221;
            let t228 = (t227).simd_le(zeta_threshold);
            let t229 = (simd::cbrt(t227));
            let t231 = ((t228).select(t37, t229 * t227));
            let t233 = (t226 + t231 - f64x8::splat(2.0)) * t44;
            let t236 = (((f64x8::splat(2.0)).simd_le(zeta_threshold)).select(t37, f64x8::splat(2.0) * t41));
            let t238 = (((f64x8::splat(0.0)).simd_le(zeta_threshold)).select(t37, f64x8::splat(0.0)));
            let t240 = (t236 + t238 - f64x8::splat(2.0)) * t44;
            let t242 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t12;
            let t247 = f64x8::splat(7.05945) * t15 + f64x8::splat(1.549425) * t12 + f64x8::splat(0.420775) * t18 + f64x8::splat(0.1562925) * t26;
            let t250 = f64x8::splat(1.0) + f64x8::splat(32.1646831778707) / t247;
            let t251 = (simd::ln(t250));
            let t259 = -t34 + t240 * (-f64x8::splat(0.03109) * t242 * t251 + t34 - f64x8::splat(0.019751789702565206) * t57) + f64x8::splat(0.019751789702565206) * t240 * t57;
            let t260 = t3 * t63;
            let t261 = t67 * t60;
            let t264 = f64x8::splat(10.0) / f64x8::splat(9.0) * t260 * t261 * t10;
            let t265 = (t264).simd_lt(-f64x8::splat(0.066725));
            let t267 = ((t265).select(f64x8::splat(0.0), f64x8::splat(0.066725) + t264));
            let t268 = t267 * t116;
            let t270 = (f64x8::splat(0.0)).simd_lt(t259);
            let t272 = ((t270).select(t259, -t259));
            let t273 = f64x8::splat(1.0) / t272;
            let t274 = t6 * t273;
            let t275 = t123 * t274;
            let t278 = f64x8::splat(1.0) + f64x8::splat(0.05397236614853195) * t268 * t119 * t275;
            let t279 = (simd::ln(t278));
            let t281 = f64x8::splat(1.0) + f64x8::splat(0.193) * t279;
            let t282 = f64x8::splat(1.0) / t281;
            let t285 = t259 * t282 + f64x8::splat(0.0069644166666666665) * t193;
            let t288 = f64x8::splat(1.0) + f64x8::splat(0.1875) * t208 - f64x8::splat(0.04046875) * t215;
            let t289 = f64x8::splat(1.0) / t288;
            let t291 = t285 * t289 - t219;
            let t292 = t233 * t291;
            let t293 = f64x8::splat(1.0) / v_rho0;
            let t294 = v_sigma0 * t293;
            let t295 = f64x8::splat(1.0) / v_tau0;
            let t296 = ((t223).select(zeta_threshold, t222));
            let t297 = t295 * t296;
            let t298 = t5 * t7;
            let t299 = t10 * t41;
            let t300 = f64x8::splat(1.0) / t222;
            let t301 = (simd::cbrt(t300));
            let t303 = t298 * t299 * t301;
            let t305 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t303;
            let t306 = ((t303).sqrt());
            let t309 = ((t303) * (t303).sqrt());
            let t311 = t22 * t6;
            let t312 = t24 * t79;
            let t313 = t301 * t301;
            let t315 = t311 * t312 * t313;
            let t317 = f64x8::splat(3.79785) * t306 + f64x8::splat(0.8969) * t303 + f64x8::splat(0.204775) * t309 + f64x8::splat(0.123235) * t315;
            let t320 = f64x8::splat(1.0) + f64x8::splat(16.081824322151103) / t317;
            let t321 = (simd::ln(t320));
            let t323 = f64x8::splat(0.062182) * t305 * t321;
            let t325 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t303;
            let t330 = f64x8::splat(5.1785) * t306 + f64x8::splat(0.905775) * t303 + f64x8::splat(0.1100325) * t309 + f64x8::splat(0.1241775) * t315;
            let t333 = f64x8::splat(1.0) + f64x8::splat(29.608574643216677) / t330;
            let t334 = (simd::ln(t333));
            let t335 = t325 * t334;
            let t338 = -t323 + f64x8::splat(0.019751789702565206) * t45 * t335;
            let t339 = t79 * t3;
            let t340 = t339 * t63;
            let t341 = t79 * t2;
            let t342 = t341 * t4;
            let t344 = t342 * t11 * t301;
            let t346 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t344;
            let t347 = ((t344).sqrt());
            let t350 = ((t344) * (t344).sqrt());
            let t352 = t41 * t20;
            let t353 = t352 * t21;
            let t355 = t353 * t25 * t313;
            let t357 = f64x8::splat(3.79785) * t347 + f64x8::splat(0.8969) * t344 + f64x8::splat(0.204775) * t350 + f64x8::splat(0.24647) * t355;
            let t360 = f64x8::splat(1.0) + f64x8::splat(16.081824322151103) / t357;
            let t361 = (simd::ln(t360));
            let t365 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t344;
            let t370 = f64x8::splat(5.1785) * t347 + f64x8::splat(0.905775) * t344 + f64x8::splat(0.1100325) * t350 + f64x8::splat(0.248355) * t355;
            let t373 = f64x8::splat(1.0) + f64x8::splat(29.608574643216677) / t370;
            let t374 = (simd::ln(t373));
            let t378 = -f64x8::splat(0.062182) * t346 * t361 + f64x8::splat(0.019751789702565206) * t45 * t365 * t374;
            let t379 = t67 * t378;
            let t380 = t8 * t222;
            let t381 = (simd::cbrt(t380));
            let t382 = f64x8::splat(1.0) / t381;
            let t385 = f64x8::splat(10.0) / f64x8::splat(9.0) * t340 * t379 * t382;
            let t386 = (t385).simd_lt(-f64x8::splat(0.066725));
            let t388 = ((t386).select(f64x8::splat(0.0), f64x8::splat(0.066725) + t385));
            let t389 = t388 * v_sigma0;
            let t390 = v_rho0 * v_rho0;
            let t391 = (simd::cbrt(v_rho0));
            let t392 = t391 * t391;
            let t394 = f64x8::splat(1.0) / t392 / t390;
            let t395 = t394 * t20;
            let t396 = t389 * t395;
            let t397 = t122 * t6;
            let t398 = f64x8::splat(1.0) / t301;
            let t399 = t9 * t398;
            let t400 = (f64x8::splat(0.0)).simd_lt(t338);
            let t402 = ((t400).select(t338, -t338));
            let t403 = f64x8::splat(1.0) / t402;
            let t405 = t397 * t399 * t403;
            let t408 = f64x8::splat(1.0) + f64x8::splat(0.05397236614853195) * t396 * t405;
            let t409 = (simd::ln(t408));
            let t411 = f64x8::splat(1.0) + f64x8::splat(0.193) * t409;
            let t412 = f64x8::splat(1.0) / t411;
            let t415 = t139 * t201 * t79;
            let t416 = f64x8::splat(1.0) / t313;
            let t417 = t143 * t300;
            let t420 = f64x8::splat(1.07924) + f64x8::splat(0.03964) * t306 + f64x8::splat(0.0123825) * t303;
            let t423 = f64x8::splat(1.0) + t306 * t420 / f64x8::splat(2.0);
            let t424 = t423 * t423;
            let t425 = f64x8::splat(1.0) / t424;
            let t429 = t156 * t7;
            let t430 = t142 * t41;
            let t431 = t301 * t300;
            let t435 = t161 * t6;
            let t436 = t163 * t79;
            let t437 = t313 * t300;
            let t441 = t222 * t222;
            let t442 = f64x8::splat(1.0) / t441;
            let t443 = t167 * t442;
            let t445 = t171 * t7;
            let t446 = t119 * t41;
            let t447 = t301 * t442;
            let t451 = -f64x8::splat(0.011955719325063178) * t417 + f64x8::splat(0.00263475) * t429 * t430 * t431 - f64x8::splat(0.0004755) * t435 * t436 * t437 + f64x8::splat(2.5897694538981533e-05) * t443 - f64x8::splat(2.1605625e-06) * t445 * t446 * t447;
            let t453 = f64x8::splat(0.0023426533963880895) * t417 * t425 - t338 * t451;
            let t454 = t416 * t453;
            let t455 = t179 * t23;
            let t456 = t139 * t455;
            let t457 = t41 * t416;
            let t458 = t306 * t303;
            let t459 = f64x8::splat(1.0) / t423;
            let t460 = t458 * t459;
            let t461 = t457 * t460;
            let t464 = t338 * t338;
            let t466 = f64x8::splat(0.00098556445) * t456 * t461 - f64x8::splat(2.0) * t464;
            let t467 = f64x8::splat(1.0) / t466;
            let t468 = t454 * t467;
            let t469 = v_sigma0 * t394;
            let t470 = t381 * t381;
            let t471 = t469 * t470;
            let t472 = t468 * t471;
            let t473 = t415 * t472;
            let t475 = t338 * t412 + f64x8::splat(0.0024872916666666667) * t473;
            let t476 = t338 * t196;
            let t479 = t23 * t41;
            let t480 = t416 * t451;
            let t484 = f64x8::splat(0.00619125) * t476 * t460 - f64x8::splat(0.03979666666666667) * t140 * t479 * t480;
            let t485 = t484 * t467;
            let t486 = t485 * v_sigma0;
            let t487 = t394 * t41;
            let t488 = t487 * t470;
            let t489 = t486 * t488;
            let t491 = t453 * t467;
            let t492 = v_sigma0 * v_sigma0;
            let t493 = t491 * t492;
            let t494 = t390 * t390;
            let t495 = t494 * v_rho0;
            let t497 = f64x8::splat(1.0) / t391 / t495;
            let t498 = t497 * t79;
            let t499 = t381 * t380;
            let t500 = t498 * t499;
            let t501 = t493 * t500;
            let t503 = f64x8::splat(1.0) + t489 / f64x8::splat(16.0) - t501 / f64x8::splat(256.0);
            let t504 = f64x8::splat(1.0) / t503;
            let t505 = t475 * t504;
            let t507 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t303;
            let t512 = f64x8::splat(7.05945) * t306 + f64x8::splat(1.549425) * t303 + f64x8::splat(0.420775) * t309 + f64x8::splat(0.1562925) * t315;
            let t515 = f64x8::splat(1.0) + f64x8::splat(32.1646831778707) / t512;
            let t516 = (simd::ln(t515));
            let t524 = -t323 + t240 * (-f64x8::splat(0.03109) * t507 * t516 + t323 - f64x8::splat(0.019751789702565206) * t335) + f64x8::splat(0.019751789702565206) * t240 * t335;
            let t525 = t260 * t67;
            let t526 = t338 * t41;
            let t529 = f64x8::splat(10.0) / f64x8::splat(9.0) * t525 * t526 * t382;
            let t530 = (t529).simd_lt(-f64x8::splat(0.066725));
            let t532 = ((t530).select(f64x8::splat(0.0), f64x8::splat(0.066725) + t529));
            let t533 = t79 * t532;
            let t534 = t469 * t20;
            let t535 = t533 * t534;
            let t536 = (f64x8::splat(0.0)).simd_lt(t524);
            let t538 = ((t536).select(t524, -t524));
            let t539 = f64x8::splat(1.0) / t538;
            let t541 = t397 * t399 * t539;
            let t544 = f64x8::splat(1.0) + f64x8::splat(0.026986183074265976) * t535 * t541;
            let t545 = (simd::ln(t544));
            let t547 = f64x8::splat(1.0) + f64x8::splat(0.193) * t545;
            let t548 = f64x8::splat(1.0) / t547;
            let t551 = t524 * t548 + f64x8::splat(0.0017411041666666666) * t473;
            let t554 = f64x8::splat(1.0) + f64x8::splat(0.09375) * t489 - f64x8::splat(0.0101171875) * t501;
            let t555 = f64x8::splat(1.0) / t554;
            let t559 = t505 + t240 * (t551 * t555 - t505);
            let t560 = t297 * t559;
            let t562 = t294 * t560 / f64x8::splat(16.0);
            let t563 = f64x8::splat(1.0) / v_rho1;
            let t564 = v_sigma2 * t563;
            let t565 = f64x8::splat(1.0) / v_tau1;
            let t566 = ((t228).select(zeta_threshold, t227));
            let t567 = t565 * t566;
            let t568 = f64x8::splat(1.0) / t227;
            let t569 = (simd::cbrt(t568));
            let t571 = t298 * t299 * t569;
            let t573 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t571;
            let t574 = ((t571).sqrt());
            let t577 = ((t571) * (t571).sqrt());
            let t579 = t569 * t569;
            let t581 = t311 * t312 * t579;
            let t583 = f64x8::splat(3.79785) * t574 + f64x8::splat(0.8969) * t571 + f64x8::splat(0.204775) * t577 + f64x8::splat(0.123235) * t581;
            let t586 = f64x8::splat(1.0) + f64x8::splat(16.081824322151103) / t583;
            let t587 = (simd::ln(t586));
            let t589 = f64x8::splat(0.062182) * t573 * t587;
            let t591 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t571;
            let t596 = f64x8::splat(5.1785) * t574 + f64x8::splat(0.905775) * t571 + f64x8::splat(0.1100325) * t577 + f64x8::splat(0.1241775) * t581;
            let t599 = f64x8::splat(1.0) + f64x8::splat(29.608574643216677) / t596;
            let t600 = (simd::ln(t599));
            let t601 = t591 * t600;
            let t604 = -t589 + f64x8::splat(0.019751789702565206) * t45 * t601;
            let t606 = t342 * t11 * t569;
            let t608 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t606;
            let t609 = ((t606).sqrt());
            let t612 = ((t606) * (t606).sqrt());
            let t615 = t353 * t25 * t579;
            let t617 = f64x8::splat(3.79785) * t609 + f64x8::splat(0.8969) * t606 + f64x8::splat(0.204775) * t612 + f64x8::splat(0.24647) * t615;
            let t620 = f64x8::splat(1.0) + f64x8::splat(16.081824322151103) / t617;
            let t621 = (simd::ln(t620));
            let t625 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t606;
            let t630 = f64x8::splat(5.1785) * t609 + f64x8::splat(0.905775) * t606 + f64x8::splat(0.1100325) * t612 + f64x8::splat(0.248355) * t615;
            let t633 = f64x8::splat(1.0) + f64x8::splat(29.608574643216677) / t630;
            let t634 = (simd::ln(t633));
            let t638 = -f64x8::splat(0.062182) * t608 * t621 + f64x8::splat(0.019751789702565206) * t45 * t625 * t634;
            let t639 = t67 * t638;
            let t640 = t8 * t227;
            let t641 = (simd::cbrt(t640));
            let t642 = f64x8::splat(1.0) / t641;
            let t645 = f64x8::splat(10.0) / f64x8::splat(9.0) * t340 * t639 * t642;
            let t646 = (t645).simd_lt(-f64x8::splat(0.066725));
            let t648 = ((t646).select(f64x8::splat(0.0), f64x8::splat(0.066725) + t645));
            let t649 = t648 * v_sigma2;
            let t650 = v_rho1 * v_rho1;
            let t651 = (simd::cbrt(v_rho1));
            let t652 = t651 * t651;
            let t654 = f64x8::splat(1.0) / t652 / t650;
            let t655 = t654 * t20;
            let t656 = t649 * t655;
            let t657 = f64x8::splat(1.0) / t569;
            let t658 = t9 * t657;
            let t659 = (f64x8::splat(0.0)).simd_lt(t604);
            let t661 = ((t659).select(t604, -t604));
            let t662 = f64x8::splat(1.0) / t661;
            let t664 = t397 * t658 * t662;
            let t667 = f64x8::splat(1.0) + f64x8::splat(0.05397236614853195) * t656 * t664;
            let t668 = (simd::ln(t667));
            let t670 = f64x8::splat(1.0) + f64x8::splat(0.193) * t668;
            let t671 = f64x8::splat(1.0) / t670;
            let t673 = f64x8::splat(1.0) / t579;
            let t674 = t143 * t568;
            let t677 = f64x8::splat(1.07924) + f64x8::splat(0.03964) * t574 + f64x8::splat(0.0123825) * t571;
            let t680 = f64x8::splat(1.0) + t574 * t677 / f64x8::splat(2.0);
            let t681 = t680 * t680;
            let t682 = f64x8::splat(1.0) / t681;
            let t686 = t569 * t568;
            let t690 = t579 * t568;
            let t694 = t227 * t227;
            let t695 = f64x8::splat(1.0) / t694;
            let t696 = t167 * t695;
            let t698 = t569 * t695;
            let t702 = -f64x8::splat(0.011955719325063178) * t674 + f64x8::splat(0.00263475) * t429 * t430 * t686 - f64x8::splat(0.0004755) * t435 * t436 * t690 + f64x8::splat(2.5897694538981533e-05) * t696 - f64x8::splat(2.1605625e-06) * t445 * t446 * t698;
            let t704 = f64x8::splat(0.0023426533963880895) * t674 * t682 - t604 * t702;
            let t705 = t673 * t704;
            let t706 = t41 * t673;
            let t707 = t574 * t571;
            let t708 = f64x8::splat(1.0) / t680;
            let t709 = t707 * t708;
            let t710 = t706 * t709;
            let t713 = t604 * t604;
            let t715 = f64x8::splat(0.00098556445) * t456 * t710 - f64x8::splat(2.0) * t713;
            let t716 = f64x8::splat(1.0) / t715;
            let t717 = t705 * t716;
            let t718 = v_sigma2 * t654;
            let t719 = t641 * t641;
            let t720 = t718 * t719;
            let t721 = t717 * t720;
            let t722 = t415 * t721;
            let t724 = t604 * t671 + f64x8::splat(0.0024872916666666667) * t722;
            let t725 = t604 * t196;
            let t728 = t673 * t702;
            let t732 = f64x8::splat(0.00619125) * t725 * t709 - f64x8::splat(0.03979666666666667) * t140 * t479 * t728;
            let t733 = t732 * t716;
            let t734 = t733 * v_sigma2;
            let t735 = t654 * t41;
            let t736 = t735 * t719;
            let t737 = t734 * t736;
            let t739 = t704 * t716;
            let t740 = v_sigma2 * v_sigma2;
            let t741 = t739 * t740;
            let t742 = t650 * t650;
            let t743 = t742 * v_rho1;
            let t745 = f64x8::splat(1.0) / t651 / t743;
            let t746 = t745 * t79;
            let t747 = t641 * t640;
            let t748 = t746 * t747;
            let t749 = t741 * t748;
            let t751 = f64x8::splat(1.0) + t737 / f64x8::splat(16.0) - t749 / f64x8::splat(256.0);
            let t752 = f64x8::splat(1.0) / t751;
            let t753 = t724 * t752;
            let t755 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t571;
            let t760 = f64x8::splat(7.05945) * t574 + f64x8::splat(1.549425) * t571 + f64x8::splat(0.420775) * t577 + f64x8::splat(0.1562925) * t581;
            let t763 = f64x8::splat(1.0) + f64x8::splat(32.1646831778707) / t760;
            let t764 = (simd::ln(t763));
            let t772 = -t589 + t240 * (-f64x8::splat(0.03109) * t755 * t764 + t589 - f64x8::splat(0.019751789702565206) * t601) + f64x8::splat(0.019751789702565206) * t240 * t601;
            let t773 = t604 * t41;
            let t776 = f64x8::splat(10.0) / f64x8::splat(9.0) * t525 * t773 * t642;
            let t777 = (t776).simd_lt(-f64x8::splat(0.066725));
            let t779 = ((t777).select(f64x8::splat(0.0), f64x8::splat(0.066725) + t776));
            let t780 = t79 * t779;
            let t781 = t718 * t20;
            let t782 = t780 * t781;
            let t783 = (f64x8::splat(0.0)).simd_lt(t772);
            let t785 = ((t783).select(t772, -t772));
            let t786 = f64x8::splat(1.0) / t785;
            let t788 = t397 * t658 * t786;
            let t791 = f64x8::splat(1.0) + f64x8::splat(0.026986183074265976) * t782 * t788;
            let t792 = (simd::ln(t791));
            let t794 = f64x8::splat(1.0) + f64x8::splat(0.193) * t792;
            let t795 = f64x8::splat(1.0) / t794;
            let t798 = t772 * t795 + f64x8::splat(0.0017411041666666666) * t722;
            let t801 = f64x8::splat(1.0) + f64x8::splat(0.09375) * t737 - f64x8::splat(0.0101171875) * t749;
            let t802 = f64x8::splat(1.0) / t801;
            let t806 = t753 + t240 * (t798 * t802 - t753);
            let t807 = t567 * t806;
            let t809 = t564 * t807 / f64x8::splat(16.0);
            let tzk0 = t219 + t292 - t562 - t809;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}

//! HYB_LDA_XC_BN05 lxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/hyb_lda_xc_bn05.c`
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
pub fn hyb_lda_xc_bn05_lxc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    v3rho3: &mut [f64],
    v4rho4: &mut [f64],
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_hyb_omega_0 = f64x8::splat(param_hyb_omega_0);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_v2rho2 = V_ZERO;
        let mut acc_v3rho3 = V_ZERO;
        let mut acc_v4rho4 = V_ZERO;
        {
            let t1 = f64x8::splat(M_CBRT3);
            let t2 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t3 = (simd::cbrt(t2));
            let t4 = t3 * t1;
            let t5 = f64x8::splat(M_CBRT4);
            let t6 = t5 * t5;
            let t7 = t6 * t4;
            let t8 = f64x8::splat(M_CBRT2);
            let t9 = t8 * t8;
            let t10 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t11 = (simd::cbrt(zeta_threshold));
            let t13 = ((t10).select(t11 * zeta_threshold, f64x8::splat(1.0)));
            let t14 = t13 * t9;
            let t15 = (simd::cbrt(v_rho));
            let t16 = (simd::cbrt(f64x8::splat(9.0)));
            let t17 = t16 * t16;
            let t18 = t3 * t3;
            let t20 = param_hyb_omega_0 * t18 * t17;
            let t21 = f64x8::splat(1.0) / t15;
            let t23 = ((t10).select(t11, f64x8::splat(1.0)));
            let t24 = f64x8::splat(1.0) / t23;
            let t27 = t24 * t21 * t1 * t20 / f64x8::splat(18.0);
            let t28 = (f64x8::splat(1.92)).simd_le(t27);
            let t29 = (f64x8::splat(1.92)).simd_lt(t27);
            let t30 = ((t29).select(t27, f64x8::splat(1.92)));
            let t31 = t30 * t30;
            let t34 = t31 * t31;
            let t35 = f64x8::splat(1.0) / t34;
            let t37 = t34 * t31;
            let t38 = f64x8::splat(1.0) / t37;
            let t40 = t34 * t34;
            let t41 = f64x8::splat(1.0) / t40;
            let t43 = t40 * t31;
            let t44 = f64x8::splat(1.0) / t43;
            let t46 = t40 * t34;
            let t47 = f64x8::splat(1.0) / t46;
            let t49 = t40 * t37;
            let t50 = f64x8::splat(1.0) / t49;
            let t52 = t40 * t40;
            let t53 = f64x8::splat(1.0) / t52;
            let t56 = f64x8::splat(1.0) / t52 / t31;
            let t59 = f64x8::splat(1.0) / t52 / t34;
            let t62 = f64x8::splat(1.0) / t52 / t37;
            let t65 = f64x8::splat(1.0) / t52 / t40;
            let t68 = f64x8::splat(1.0) / t52 / t43;
            let t71 = f64x8::splat(1.0) / t52 / t46;
            let t74 = f64x8::splat(1.0) / t52 / t49;
            let t76 = t52 * t52;
            let t77 = f64x8::splat(1.0) / t76;
            let t80 = f64x8::splat(1.0) / t76 / t31;
            let t83 = f64x8::splat(1.0) / t76 / t34;
            let t85 = f64x8::splat(1.0) / t31 / f64x8::splat(9.0) - t35 / f64x8::splat(30.0) + t38 / f64x8::splat(70.0) - t41 / f64x8::splat(135.0) + t44 / f64x8::splat(231.0) - t47 / f64x8::splat(364.0) + t50 / f64x8::splat(540.0) - t53 / f64x8::splat(765.0) + t56 / f64x8::splat(1045.0) - t59 / f64x8::splat(1386.0) + t62 / f64x8::splat(1794.0) - t65 / f64x8::splat(2275.0) + t68 / f64x8::splat(2835.0) - t71 / f64x8::splat(3480.0) + t74 / f64x8::splat(4216.0) - t77 / f64x8::splat(5049.0) + t80 / f64x8::splat(5985.0) - t83 / f64x8::splat(7030.0);
            let t86 = ((t29).select(f64x8::splat(1.92), t27));
            let t87 = (simd::atan2(f64x8::splat(1.0), t86));
            let t88 = t86 * t86;
            let t89 = t88 + f64x8::splat(3.0);
            let t90 = f64x8::splat(1.0) / t88;
            let t91 = f64x8::splat(1.0) + t90;
            let t92 = (simd::ln(t91));
            let t94 = -t92 * t89 + f64x8::splat(1.0);
            let t97 = t87 + t94 * t86 / f64x8::splat(4.0);
            let t101 = ((t28).select(t85, f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t97 * t86));
            let t105 = f64x8::splat(3.0) / f64x8::splat(16.0) * t101 * t15 * t14 * t7;
            let t107 = t21 * t6 * t4;
            let t109 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t107;
            let t110 = ((t107).sqrt());
            let t113 = ((t107) * (t107).sqrt());
            let t115 = t1 * t1;
            let t116 = t18 * t115;
            let t117 = t15 * t15;
            let t118 = f64x8::splat(1.0) / t117;
            let t120 = t118 * t5 * t116;
            let t122 = f64x8::splat(3.79785) * t110 + f64x8::splat(0.8969) * t107 + f64x8::splat(0.204775) * t113 + f64x8::splat(0.123235) * t120;
            let t125 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t122;
            let t126 = (simd::ln(t125));
            let t134 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t8 - f64x8::splat(2.0)) * (f64x8::splat(2.0) * t13 - f64x8::splat(2.0));
            let t136 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t107;
            let t141 = f64x8::splat(5.1785) * t110 + f64x8::splat(0.905775) * t107 + f64x8::splat(0.1100325) * t113 + f64x8::splat(0.1241775) * t120;
            let t144 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t141;
            let t145 = (simd::ln(t144));
            let t149 = -f64x8::splat(0.0621814) * t126 * t109 + f64x8::splat(0.0197516734986138) * t145 * t136 * t134;
            let t152 = f64x8::splat(3.2) - f64x8::splat(0.225) * t107 + t120 / f64x8::splat(4.0);
            let t153 = f64x8::splat(1.0) / t152;
            let t155 = f64x8::splat(3.4602) * t153 * t149;
            let tzk0 = -t105 + t155;
            acc_zk = tzk0;
            let t158 = t101 * t118 * t14 * t7;
            let t160 = t31 * t30;
            let t161 = f64x8::splat(1.0) / t160;
            let t163 = f64x8::splat(1.0) / t15 / v_rho;
            let t167 = t24 * t163 * t1 * t20 / f64x8::splat(54.0);
            let t168 = ((t29).select(-t167, f64x8::splat(0.0)));
            let t171 = t34 * t30;
            let t172 = f64x8::splat(1.0) / t171;
            let t175 = t34 * t160;
            let t176 = f64x8::splat(1.0) / t175;
            let t179 = t40 * t30;
            let t180 = f64x8::splat(1.0) / t179;
            let t183 = t40 * t160;
            let t184 = f64x8::splat(1.0) / t183;
            let t187 = t40 * t171;
            let t188 = f64x8::splat(1.0) / t187;
            let t191 = t40 * t175;
            let t192 = f64x8::splat(1.0) / t191;
            let t196 = f64x8::splat(1.0) / t52 / t30;
            let t200 = f64x8::splat(1.0) / t52 / t160;
            let t204 = f64x8::splat(1.0) / t52 / t171;
            let t208 = f64x8::splat(1.0) / t52 / t175;
            let t212 = f64x8::splat(1.0) / t52 / t179;
            let t216 = f64x8::splat(1.0) / t52 / t183;
            let t220 = f64x8::splat(1.0) / t52 / t187;
            let t224 = f64x8::splat(1.0) / t52 / t191;
            let t228 = f64x8::splat(1.0) / t76 / t30;
            let t232 = f64x8::splat(1.0) / t76 / t160;
            let t236 = f64x8::splat(1.0) / t76 / t171;
            let t239 = -f64x8::splat(2.0) / f64x8::splat(9.0) * t168 * t161 + f64x8::splat(2.0) / f64x8::splat(15.0) * t168 * t172 - f64x8::splat(3.0) / f64x8::splat(35.0) * t168 * t176 + f64x8::splat(8.0) / f64x8::splat(135.0) * t168 * t180 - f64x8::splat(10.0) / f64x8::splat(231.0) * t168 * t184 + f64x8::splat(3.0) / f64x8::splat(91.0) * t168 * t188 - f64x8::splat(7.0) / f64x8::splat(270.0) * t168 * t192 + f64x8::splat(16.0) / f64x8::splat(765.0) * t168 * t196 - f64x8::splat(18.0) / f64x8::splat(1045.0) * t168 * t200 + f64x8::splat(10.0) / f64x8::splat(693.0) * t168 * t204 - f64x8::splat(11.0) / f64x8::splat(897.0) * t168 * t208 + f64x8::splat(24.0) / f64x8::splat(2275.0) * t168 * t212 - f64x8::splat(26.0) / f64x8::splat(2835.0) * t168 * t216 + f64x8::splat(7.0) / f64x8::splat(870.0) * t168 * t220 - f64x8::splat(15.0) / f64x8::splat(2108.0) * t168 * t224 + f64x8::splat(32.0) / f64x8::splat(5049.0) * t168 * t228 - f64x8::splat(34.0) / f64x8::splat(5985.0) * t168 * t232 + f64x8::splat(18.0) / f64x8::splat(3515.0) * t168 * t236;
            let t240 = ((t29).select(f64x8::splat(0.0), -t167));
            let t243 = f64x8::splat(1.0) / t91;
            let t249 = t88 * t86;
            let t250 = f64x8::splat(1.0) / t249;
            let t251 = t250 * t89;
            let t252 = t243 * t240;
            let t255 = -f64x8::splat(2.0) * t92 * t240 * t86 + f64x8::splat(2.0) * t252 * t251;
            let t258 = -t243 * t90 * t240 + t94 * t240 / f64x8::splat(4.0) + t255 * t86 / f64x8::splat(4.0);
            let t262 = ((t28).select(t239, -f64x8::splat(8.0) / f64x8::splat(3.0) * t97 * t240 - f64x8::splat(8.0) / f64x8::splat(3.0) * t258 * t86));
            let t265 = t262 * t15 * t14 * t7;
            let t267 = t163 * t6;
            let t271 = t122 * t122;
            let t272 = f64x8::splat(1.0) / t271;
            let t273 = t272 * t109;
            let t275 = t1 / t110;
            let t276 = t6 * t3;
            let t277 = t163 * t276;
            let t278 = t277 * t275;
            let t280 = t267 * t4;
            let t282 = ((t107).sqrt());
            let t283 = t1 * t282;
            let t284 = t277 * t283;
            let t287 = f64x8::splat(1.0) / t117 / v_rho;
            let t289 = t287 * t5 * t116;
            let t291 = -f64x8::splat(0.632975) * t278 - f64x8::splat(0.29896666666666666) * t280 - f64x8::splat(0.1023875) * t284 - f64x8::splat(0.08215666666666667) * t289;
            let t292 = f64x8::splat(1.0) / t125;
            let t293 = t292 * t291;
            let t296 = t1 * t134;
            let t301 = t136 * t134;
            let t302 = t141 * t141;
            let t303 = f64x8::splat(1.0) / t302;
            let t308 = -f64x8::splat(0.8630833333333333) * t278 - f64x8::splat(0.301925) * t280 - f64x8::splat(0.05501625) * t284 - f64x8::splat(0.082785) * t289;
            let t310 = f64x8::splat(1.0) / t144;
            let t311 = t310 * t308 * t303;
            let t314 = f64x8::splat(0.0011073470983333333) * t126 * t267 * t4 + f64x8::splat(1.0) * t293 * t273 - f64x8::splat(0.00018311447306006544) * t145 * t163 * t276 * t296 - f64x8::splat(0.5848223622634646) * t311 * t301;
            let t315 = t153 * t314;
            let t317 = t152 * t152;
            let t318 = f64x8::splat(1.0) / t317;
            let t319 = t318 * t149;
            let t322 = f64x8::splat(0.075) * t280 - t289 / f64x8::splat(6.0);
            let t323 = t322 * t319;
            let tvrho0 = -t105 + t155 + (-t158 / f64x8::splat(16.0) - f64x8::splat(3.0) / f64x8::splat(16.0) * t265 + f64x8::splat(3.4602) * t315 - f64x8::splat(3.4602) * t323) * v_rho;
            acc_vrho = tvrho0;
            let t333 = t101 * t287 * t14 * t7;
            let t337 = t262 * t118 * t14 * t7;
            let t339 = v_rho * v_rho;
            let t341 = f64x8::splat(1.0) / t15 / t339;
            let t345 = f64x8::splat(2.0) / f64x8::splat(81.0) * t24 * t341 * t1 * t20;
            let t346 = ((t29).select(t345, f64x8::splat(0.0)));
            let t355 = t346 * t184;
            let t357 = t346 * t188;
            let t359 = t346 * t192;
            let t361 = t346 * t196;
            let t363 = t346 * t200;
            let t365 = t346 * t204;
            let t367 = t346 * t208;
            let t369 = t346 * t212;
            let t371 = t346 * t216;
            let t373 = t346 * t220;
            let t375 = t346 * t224;
            let t377 = t346 * t228;
            let t379 = t346 * t232;
            let t381 = t346 * t236;
            let t383 = -f64x8::splat(2.0) / f64x8::splat(9.0) * t346 * t161 + f64x8::splat(2.0) / f64x8::splat(15.0) * t346 * t172 - f64x8::splat(3.0) / f64x8::splat(35.0) * t346 * t176 + f64x8::splat(8.0) / f64x8::splat(135.0) * t346 * t180 - f64x8::splat(10.0) / f64x8::splat(231.0) * t355 + f64x8::splat(3.0) / f64x8::splat(91.0) * t357 - f64x8::splat(7.0) / f64x8::splat(270.0) * t359 + f64x8::splat(16.0) / f64x8::splat(765.0) * t361 - f64x8::splat(18.0) / f64x8::splat(1045.0) * t363 + f64x8::splat(10.0) / f64x8::splat(693.0) * t365 - f64x8::splat(11.0) / f64x8::splat(897.0) * t367 + f64x8::splat(24.0) / f64x8::splat(2275.0) * t369 - f64x8::splat(26.0) / f64x8::splat(2835.0) * t371 + f64x8::splat(7.0) / f64x8::splat(870.0) * t373 - f64x8::splat(15.0) / f64x8::splat(2108.0) * t375 + f64x8::splat(32.0) / f64x8::splat(5049.0) * t377 - f64x8::splat(34.0) / f64x8::splat(5985.0) * t379 + f64x8::splat(18.0) / f64x8::splat(3515.0) * t381;
            let t384 = t168 * t168;
            let t420 = f64x8::splat(1.0) / t76 / t37;
            let t423 = f64x8::splat(2.0) / f64x8::splat(3.0) * t384 * t35 - f64x8::splat(2.0) / f64x8::splat(3.0) * t384 * t38 + f64x8::splat(3.0) / f64x8::splat(5.0) * t384 * t41 - f64x8::splat(8.0) / f64x8::splat(15.0) * t384 * t44 + f64x8::splat(10.0) / f64x8::splat(21.0) * t384 * t47 - f64x8::splat(3.0) / f64x8::splat(7.0) * t384 * t50 + f64x8::splat(7.0) / f64x8::splat(18.0) * t384 * t53 - f64x8::splat(16.0) / f64x8::splat(45.0) * t384 * t56 + f64x8::splat(18.0) / f64x8::splat(55.0) * t384 * t59 - f64x8::splat(10.0) / f64x8::splat(33.0) * t384 * t62 + f64x8::splat(11.0) / f64x8::splat(39.0) * t384 * t65 - f64x8::splat(24.0) / f64x8::splat(91.0) * t384 * t68 + f64x8::splat(26.0) / f64x8::splat(105.0) * t384 * t71 - f64x8::splat(7.0) / f64x8::splat(30.0) * t384 * t74 + f64x8::splat(15.0) / f64x8::splat(68.0) * t384 * t77 - f64x8::splat(32.0) / f64x8::splat(153.0) * t384 * t80 + f64x8::splat(34.0) / f64x8::splat(171.0) * t384 * t83 - f64x8::splat(18.0) / f64x8::splat(95.0) * t384 * t420;
            let t425 = ((t29).select(f64x8::splat(0.0), t345));
            let t430 = t90 * t425;
            let t432 = t240 * t240;
            let t433 = t250 * t432;
            let t436 = t88 * t88;
            let t438 = f64x8::splat(1.0) / t436 / t86;
            let t440 = t91 * t91;
            let t441 = f64x8::splat(1.0) / t440;
            let t456 = f64x8::splat(1.0) / t436;
            let t457 = t456 * t89;
            let t458 = t243 * t432;
            let t461 = t243 * t425;
            let t465 = f64x8::splat(1.0) / t436 / t88;
            let t466 = t465 * t89;
            let t467 = t441 * t432;
            let t470 = f64x8::splat(8.0) * t243 * t432 * t90 - f64x8::splat(2.0) * t92 * t425 * t86 + f64x8::splat(2.0) * t461 * t251 - f64x8::splat(2.0) * t92 * t432 - f64x8::splat(6.0) * t458 * t457 + f64x8::splat(4.0) * t467 * t466;
            let t473 = -t243 * t430 + f64x8::splat(2.0) * t243 * t433 - f64x8::splat(2.0) * t441 * t438 * t432 + t94 * t425 / f64x8::splat(4.0) + t255 * t240 / f64x8::splat(2.0) + t470 * t86 / f64x8::splat(4.0);
            let t477 = ((t28).select(t383 + t423, -f64x8::splat(8.0) / f64x8::splat(3.0) * t97 * t425 - f64x8::splat(16.0) / f64x8::splat(3.0) * t258 * t240 - f64x8::splat(8.0) / f64x8::splat(3.0) * t473 * t86));
            let t480 = t477 * t15 * t14 * t7;
            let t482 = t341 * t6;
            let t486 = t272 * t163;
            let t490 = t271 * t122;
            let t491 = f64x8::splat(1.0) / t490;
            let t492 = t491 * t109;
            let t493 = t291 * t291;
            let t494 = t292 * t493;
            let t499 = t115 / t110 / t107;
            let t500 = t5 * t18;
            let t502 = f64x8::splat(1.0) / t117 / t339;
            let t503 = t502 * t500;
            let t504 = t503 * t499;
            let t506 = t341 * t276;
            let t507 = t506 * t275;
            let t509 = t482 * t4;
            let t511 = f64x8::splat(1.0)/((t107).sqrt());
            let t512 = t115 * t511;
            let t513 = t503 * t512;
            let t515 = t506 * t283;
            let t518 = t502 * t5 * t116;
            let t520 = -f64x8::splat(0.4219833333333333) * t504 + f64x8::splat(0.8439666666666666) * t507 + f64x8::splat(0.3986222222222222) * t509 + f64x8::splat(0.06825833333333334) * t513 + f64x8::splat(0.13651666666666668) * t515 + f64x8::splat(0.1369277777777778) * t518;
            let t521 = t292 * t520;
            let t524 = t271 * t271;
            let t525 = f64x8::splat(1.0) / t524;
            let t526 = t525 * t109;
            let t527 = t125 * t125;
            let t528 = f64x8::splat(1.0) / t527;
            let t529 = t528 * t493;
            let t536 = t4 * t134;
            let t540 = t302 * t141;
            let t541 = f64x8::splat(1.0) / t540;
            let t542 = t308 * t308;
            let t544 = t310 * t542 * t541;
            let t553 = -f64x8::splat(0.5753888888888888) * t504 + f64x8::splat(1.1507777777777777) * t507 + f64x8::splat(0.4025666666666667) * t509 + f64x8::splat(0.0366775) * t513 + f64x8::splat(0.073355) * t515 + f64x8::splat(0.137975) * t518;
            let t555 = t310 * t553 * t303;
            let t558 = t302 * t302;
            let t559 = f64x8::splat(1.0) / t558;
            let t560 = t542 * t559;
            let t561 = t144 * t144;
            let t562 = f64x8::splat(1.0) / t561;
            let t563 = t562 * t560;
            let t566 = -f64x8::splat(0.0014764627977777779) * t126 * t482 * t4 - f64x8::splat(0.035616666666666665) * t293 * t486 * t7 - f64x8::splat(2.0) * t494 * t492 + f64x8::splat(1.0) * t521 * t273 + f64x8::splat(16.081979498692537) * t529 * t526 + f64x8::splat(0.00024415263074675396) * t145 * t341 * t276 * t296 + f64x8::splat(0.01084358130030174) * t311 * t267 * t536 + f64x8::splat(1.1696447245269292) * t544 * t301 - f64x8::splat(0.5848223622634646) * t555 * t301 - f64x8::splat(17.315859105681465) * t563 * t301;
            let t567 = t153 * t566;
            let t569 = t318 * t314;
            let t570 = t322 * t569;
            let t573 = f64x8::splat(1.0) / t317 / t152;
            let t574 = t573 * t149;
            let t575 = t322 * t322;
            let t576 = t575 * t574;
            let t580 = -f64x8::splat(0.1) * t509 + f64x8::splat(5.0) / f64x8::splat(18.0) * t518;
            let t581 = t580 * t319;
            let tv2rho20 = -t158 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t265 + f64x8::splat(6.9204) * t315 - f64x8::splat(6.9204) * t323 + (t333 / f64x8::splat(24.0) - t337 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(16.0) * t480 + f64x8::splat(3.4602) * t567 - f64x8::splat(6.9204) * t570 + f64x8::splat(6.9204) * t576 - f64x8::splat(3.4602) * t581) * v_rho;
            acc_v2rho2 = tv2rho20;
            let t594 = t101 * t502 * t14 * t7;
            let t598 = t262 * t287 * t14 * t7;
            let t602 = t477 * t118 * t14 * t7;
            let t604 = t339 * v_rho;
            let t606 = f64x8::splat(1.0) / t15 / t604;
            let t610 = f64x8::splat(14.0) / f64x8::splat(243.0) * t24 * t606 * t1 * t20;
            let t611 = ((t29).select(-t610, f64x8::splat(0.0)));
            let t638 = -f64x8::splat(34.0) / f64x8::splat(5985.0) * t611 * t232 + f64x8::splat(18.0) / f64x8::splat(3515.0) * t611 * t236 - f64x8::splat(2.0) / f64x8::splat(9.0) * t611 * t161 + f64x8::splat(2.0) / f64x8::splat(15.0) * t611 * t172 - f64x8::splat(3.0) / f64x8::splat(35.0) * t611 * t176 + f64x8::splat(8.0) / f64x8::splat(135.0) * t611 * t180 - f64x8::splat(10.0) / f64x8::splat(231.0) * t611 * t184 + f64x8::splat(3.0) / f64x8::splat(91.0) * t611 * t188 - f64x8::splat(7.0) / f64x8::splat(270.0) * t611 * t192 + f64x8::splat(16.0) / f64x8::splat(765.0) * t611 * t196 - f64x8::splat(18.0) / f64x8::splat(1045.0) * t611 * t200 + f64x8::splat(10.0) / f64x8::splat(693.0) * t611 * t204 - f64x8::splat(11.0) / f64x8::splat(897.0) * t611 * t208;
            let t649 = t384 * t168;
            let t668 = f64x8::splat(24.0) / f64x8::splat(2275.0) * t611 * t212 - f64x8::splat(26.0) / f64x8::splat(2835.0) * t611 * t216 + f64x8::splat(7.0) / f64x8::splat(870.0) * t611 * t220 - f64x8::splat(15.0) / f64x8::splat(2108.0) * t611 * t224 + f64x8::splat(32.0) / f64x8::splat(5049.0) * t611 * t228 - f64x8::splat(8.0) / f64x8::splat(3.0) * t649 * t172 + f64x8::splat(4.0) * t649 * t176 - f64x8::splat(24.0) / f64x8::splat(5.0) * t649 * t180 + f64x8::splat(16.0) / f64x8::splat(3.0) * t649 * t184 - f64x8::splat(40.0) / f64x8::splat(7.0) * t649 * t188 + f64x8::splat(6.0) * t649 * t192 - f64x8::splat(56.0) / f64x8::splat(9.0) * t649 * t196 + f64x8::splat(32.0) / f64x8::splat(5.0) * t649 * t200 - f64x8::splat(72.0) / f64x8::splat(11.0) * t649 * t204;
            let t687 = f64x8::splat(1.0) / t76 / t175;
            let t702 = f64x8::splat(20.0) / f64x8::splat(3.0) * t649 * t208 - f64x8::splat(88.0) / f64x8::splat(13.0) * t649 * t212 + f64x8::splat(48.0) / f64x8::splat(7.0) * t649 * t216 - f64x8::splat(104.0) / f64x8::splat(15.0) * t649 * t220 + f64x8::splat(7.0) * t649 * t224 - f64x8::splat(120.0) / f64x8::splat(17.0) * t649 * t228 + f64x8::splat(64.0) / f64x8::splat(9.0) * t649 * t232 - f64x8::splat(136.0) / f64x8::splat(19.0) * t649 * t236 + f64x8::splat(36.0) / f64x8::splat(5.0) * t649 * t687 - f64x8::splat(2.0) * t168 * t346 * t38 + f64x8::splat(9.0) / f64x8::splat(5.0) * t168 * t346 * t41 - f64x8::splat(8.0) / f64x8::splat(5.0) * t168 * t346 * t44 + f64x8::splat(10.0) / f64x8::splat(7.0) * t168 * t346 * t47;
            let t745 = -f64x8::splat(9.0) / f64x8::splat(7.0) * t168 * t346 * t50 + f64x8::splat(7.0) / f64x8::splat(6.0) * t168 * t346 * t53 - f64x8::splat(16.0) / f64x8::splat(15.0) * t168 * t346 * t56 + f64x8::splat(54.0) / f64x8::splat(55.0) * t168 * t346 * t59 - f64x8::splat(10.0) / f64x8::splat(11.0) * t168 * t346 * t62 + f64x8::splat(11.0) / f64x8::splat(13.0) * t168 * t346 * t65 - f64x8::splat(72.0) / f64x8::splat(91.0) * t168 * t346 * t68 + f64x8::splat(26.0) / f64x8::splat(35.0) * t168 * t346 * t71 - f64x8::splat(7.0) / f64x8::splat(10.0) * t168 * t346 * t74 + f64x8::splat(45.0) / f64x8::splat(68.0) * t168 * t346 * t77 - f64x8::splat(32.0) / f64x8::splat(51.0) * t168 * t346 * t80 + f64x8::splat(34.0) / f64x8::splat(57.0) * t168 * t346 * t83 - f64x8::splat(54.0) / f64x8::splat(95.0) * t168 * t346 * t420 + f64x8::splat(2.0) * t168 * t346 * t35;
            let t748 = ((t29).select(f64x8::splat(0.0), -t610));
            let t755 = t90 * t748;
            let t760 = t438 * t425;
            let t761 = t240 * t441;
            let t764 = t432 * t240;
            let t771 = t436 * t436;
            let t772 = f64x8::splat(1.0) / t771;
            let t775 = f64x8::splat(1.0) / t440 / t91;
            let t784 = t92 * t240;
            let t798 = t438 * t89;
            let t806 = f64x8::splat(1.0) / t436 / t249;
            let t807 = t806 * t89;
            let t819 = f64x8::splat(1.0) / t771 / t86;
            let t820 = t819 * t89;
            let t824 = f64x8::splat(12.0) * t240 * t441 * t425 * t466 - f64x8::splat(24.0) * t243 * t250 * t764 + f64x8::splat(2.0) * t243 * t748 * t251 + f64x8::splat(24.0) * t243 * t764 * t798 - f64x8::splat(18.0) * t425 * t252 * t457 + f64x8::splat(24.0) * t441 * t764 * t438 - f64x8::splat(36.0) * t441 * t764 * t807 - f64x8::splat(2.0) * t92 * t748 * t86 + f64x8::splat(16.0) * t775 * t764 * t820 + f64x8::splat(24.0) * t252 * t430 - f64x8::splat(6.0) * t425 * t784;
            let t827 = -t243 * t755 + f64x8::splat(6.0) * t252 * t250 * t425 - f64x8::splat(6.0) * t761 * t760 - f64x8::splat(6.0) * t243 * t456 * t764 + f64x8::splat(14.0) * t441 * t465 * t764 - f64x8::splat(8.0) * t775 * t772 * t764 + t94 * t748 / f64x8::splat(4.0) + f64x8::splat(3.0) / f64x8::splat(4.0) * t255 * t425 + f64x8::splat(3.0) / f64x8::splat(4.0) * t470 * t240 + t824 * t86 / f64x8::splat(4.0);
            let t831 = ((t28).select(t638 + t668 + t702 + t745, -f64x8::splat(8.0) / f64x8::splat(3.0) * t97 * t748 - f64x8::splat(8.0) * t258 * t425 - f64x8::splat(8.0) * t473 * t240 - f64x8::splat(8.0) / f64x8::splat(3.0) * t827 * t86));
            let t834 = t831 * t15 * t14 * t7;
            let t837 = f64x8::splat(1.0) / t558 / t302;
            let t838 = t542 * t308;
            let t841 = f64x8::splat(1.0) / t561 / t144;
            let t842 = t841 * t838 * t837;
            let t846 = f64x8::splat(1.0) / t558 / t141;
            let t848 = t562 * t838 * t846;
            let t854 = t2 / t110 / t120 / f64x8::splat(4.0);
            let t855 = t339 * t339;
            let t856 = f64x8::splat(1.0) / t855;
            let t857 = t856 * t854;
            let t860 = f64x8::splat(1.0) / t117 / t604;
            let t861 = t860 * t500;
            let t862 = t861 * t499;
            let t864 = t606 * t276;
            let t865 = t864 * t275;
            let t867 = t606 * t6;
            let t868 = t867 * t4;
            let t870 = f64x8::splat(1.0)/((t107) * (t107).sqrt());
            let t871 = t2 * t870;
            let t872 = t856 * t871;
            let t874 = t861 * t512;
            let t876 = t864 * t283;
            let t879 = t860 * t5 * t116;
            let t881 = -f64x8::splat(3.4523333333333333) * t857 + f64x8::splat(2.3015555555555554) * t862 - f64x8::splat(2.6851481481481483) * t865 - f64x8::splat(0.9393222222222222) * t868 + f64x8::splat(0.073355) * t872 - f64x8::splat(0.14671) * t874 - f64x8::splat(0.17116166666666666) * t876 - f64x8::splat(0.36793333333333333) * t879;
            let t883 = t310 * t881 * t303;
            let t886 = t528 * t520;
            let t890 = t493 * t291;
            let t891 = t292 * t890;
            let t905 = -f64x8::splat(2.5319) * t857 + f64x8::splat(1.6879333333333333) * t862 - f64x8::splat(1.9692555555555555) * t865 - f64x8::splat(0.9301185185185186) * t868 + f64x8::splat(0.13651666666666668) * t872 - f64x8::splat(0.27303333333333335) * t874 - f64x8::splat(0.31853888888888887) * t876 - f64x8::splat(0.36514074074074077) * t879;
            let t906 = t292 * t905;
            let t910 = f64x8::splat(1.0) / t524 / t271;
            let t911 = t910 * t109;
            let t913 = f64x8::splat(1.0) / t527 / t125;
            let t914 = t913 * t890;
            let t918 = f64x8::splat(1.0) / t524 / t122;
            let t919 = t918 * t109;
            let t920 = t528 * t890;
            let t929 = -f64x8::splat(1025.4018858216407) * t842 * t301 + f64x8::splat(103.89515463408878) * t848 * t301 - f64x8::splat(0.5848223622634646) * t883 * t301 + f64x8::splat(48.245938496077606) * t291 * t886 * t526 + f64x8::splat(6.0) * t891 * t526 - f64x8::splat(6.0) * t520 * t293 * t492 + f64x8::splat(1.0) * t906 * t273 + f64x8::splat(517.260129192734) * t914 * t911 - f64x8::splat(96.49187699215521) * t920 * t919 + f64x8::splat(0.01626537195045261) * t555 * t267 * t536 + f64x8::splat(0.4815973313767657) * t563 * t267 * t536;
            let t934 = t308 * t562;
            let t939 = t310 * t838 * t559;
            let t943 = t553 * t310;
            let t950 = t525 * t163;
            let t954 = t272 * t341;
            let t972 = -f64x8::splat(0.02168716260060348) * t311 * t482 * t536 - f64x8::splat(51.94757731704439) * t934 * t553 * t559 * t301 - f64x8::splat(3.5089341735807875) * t939 * t301 + f64x8::splat(3.5089341735807875) * t943 * t308 * t541 * t301 - f64x8::splat(0.053425) * t521 * t486 * t7 - f64x8::splat(0.8591797547176487) * t529 * t950 * t7 + f64x8::splat(0.07123333333333333) * t293 * t954 * t7 + f64x8::splat(0.10685) * t494 * t491 * t163 * t7 - f64x8::splat(0.0005696894717424259) * t145 * t606 * t276 * t296 - f64x8::splat(0.03253074390090522) * t544 * t267 * t536 + f64x8::splat(0.0034450798614814814) * t126 * t867 * t4;
            let t973 = t929 + t972;
            let t974 = t153 * t973;
            let t976 = t318 * t566;
            let t977 = t322 * t976;
            let t979 = t573 * t314;
            let t980 = t575 * t979;
            let t982 = t580 * t569;
            let t984 = t317 * t317;
            let t985 = f64x8::splat(1.0) / t984;
            let t986 = t985 * t149;
            let t987 = t575 * t322;
            let t988 = t987 * t986;
            let t990 = t580 * t322;
            let t991 = t990 * t574;
            let t995 = f64x8::splat(0.23333333333333334) * t868 - f64x8::splat(20.0) / f64x8::splat(27.0) * t879;
            let t996 = t995 * t319;
            let t998 = -f64x8::splat(5.0) / f64x8::splat(72.0) * t594 + t598 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(16.0) * t602 - f64x8::splat(3.0) / f64x8::splat(16.0) * t834 + f64x8::splat(3.4602) * t974 - f64x8::splat(10.3806) * t977 + f64x8::splat(20.7612) * t980 - f64x8::splat(10.3806) * t982 - f64x8::splat(20.7612) * t988 + f64x8::splat(20.7612) * t991 - f64x8::splat(3.4602) * t996;
            let tv3rho30 = t333 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t337 - f64x8::splat(9.0) / f64x8::splat(16.0) * t480 + f64x8::splat(10.3806) * t567 - f64x8::splat(20.7612) * t570 + f64x8::splat(20.7612) * t576 - f64x8::splat(10.3806) * t581 + t998 * v_rho;
            acc_v3rho3 = tv3rho30;
            let t1006 = t855 * v_rho;
            let t1008 = f64x8::splat(1.0) / t15 / t1006;
            let t1010 = t7 * t1008 / t110 * v_rho / f64x8::splat(48.0);
            let t1012 = f64x8::splat(1.0) / t1006;
            let t1013 = t1012 * t854;
            let t1016 = f64x8::splat(1.0) / t117 / t855;
            let t1017 = t1016 * t500;
            let t1018 = t1017 * t499;
            let t1021 = f64x8::splat(1.0) / t15 / t855;
            let t1022 = t1021 * t276;
            let t1023 = t1022 * t275;
            let t1025 = t1021 * t6;
            let t1026 = t1025 * t4;
            let t1028 = (simd::pow(t107, -f64x8::splat(2.5)));
            let t1031 = t7 * t1008 * t2 * t1028;
            let t1033 = t1012 * t871;
            let t1035 = t1017 * t512;
            let t1037 = t1022 * t283;
            let t1040 = t1016 * t5 * t116;
            let t1046 = t493 * t493;
            let t1053 = t524 * t524;
            let t1056 = t527 * t527;
            let t1072 = t520 * t520;
            let t1090 = f64x8::splat(1.0) * t292 * (-f64x8::splat(2.109916666666667) * t1010 + f64x8::splat(20.2552) * t1013 - f64x8::splat(7.501925925925926) * t1018 + f64x8::splat(6.564185185185186) * t1023 + f64x8::splat(3.100395061728395) * t1026 + f64x8::splat(0.06825833333333334) * t1031 - f64x8::splat(1.0921333333333334) * t1033 + f64x8::splat(1.2134814814814814) * t1035 + f64x8::splat(1.0617962962962963) * t1037 + f64x8::splat(1.3388493827160495) * t1040) * t273 + f64x8::splat(578.9512619529313) * t528 * t1046 * t911 - f64x8::splat(24.0) * t292 * t1046 * t919 + f64x8::splat(24955.7003795058) / t1056 * t1046 / t1053 * t109 - f64x8::splat(578.9512619529313) * t493 * t886 * t919 + f64x8::splat(3103.560775156404) * t493 * t913 * t520 * t911 + f64x8::splat(64.32791799477015) * t291 * t528 * t905 * t526 - f64x8::splat(6.0) * t292 * t1072 * t492 + f64x8::splat(48.245938496077606) * t528 * t1072 * t526 - f64x8::splat(6207.121550312808) * t913 * t1046 / t524 / t490 * t109 - f64x8::splat(3.436719018870595) * t291 * t528 * t520 * t525 * t280;
            let t1104 = t542 * t542;
            let t1115 = t553 * t553;
            let t1143 = t558 * t558;
            let t1146 = t561 * t561;
            let t1156 = f64x8::splat(0.4274) * t521 * t291 * t491 * t280 - f64x8::splat(8.0) * t905 * t293 * t492 + f64x8::splat(36.0) * t520 * t494 * t526 - f64x8::splat(0.011483599538271605) * t126 * t1025 * t4 - f64x8::splat(623.3709278045327) * t562 * t1104 * t837 * t301 + f64x8::splat(12304.822629859687) * t841 * t1104 / t558 / t540 * t301 - f64x8::splat(51.94757731704439) * t562 * t1115 * t559 * t301 + f64x8::splat(14.03573669432315) * t310 * t1104 * t846 * t301 + f64x8::splat(3.5089341735807875) * t310 * t1115 * t541 * t301 - f64x8::splat(0.5848223622634646) * t310 * (-f64x8::splat(2.8769444444444443) * t1010 + f64x8::splat(27.618666666666666) * t1013 - f64x8::splat(10.229135802469136) * t1018 + f64x8::splat(8.950493827160495) * t1023 + f64x8::splat(3.131074074074074) * t1026 + f64x8::splat(0.0366775) * t1031 - f64x8::splat(0.58684) * t1033 + f64x8::splat(0.6520444444444444) * t1035 + f64x8::splat(0.5705388888888889) * t1037 + f64x8::splat(1.3490888888888888) * t1040) * t303 * t301 - f64x8::splat(91082.60419215256) / t1146 * t1104 / t1143 * t301 - f64x8::splat(6152.411314929844) * t553 * t841 * t542 * t837 * t301;
            let t1195 = t553 * t562;
            let t1199 = -f64x8::splat(0.22161481481481482) * t293 * t272 * t606 * t7 - f64x8::splat(0.2849333333333333) * t494 * t491 * t341 * t7 + f64x8::splat(0.0018989649058080863) * t145 * t1021 * t276 * t296 + f64x8::splat(0.14246666666666666) * t521 * t954 * t7 + f64x8::splat(2.2911460125803966) * t529 * t525 * t341 * t7 - f64x8::splat(21.053605041484726) * t943 * t560 * t301 - f64x8::splat(0.07123333333333333) * t906 * t486 * t7 - f64x8::splat(36.84616320282908) * t914 * t910 * t163 * t7 + f64x8::splat(6.87343803774119) * t920 * t918 * t163 * t7 - f64x8::splat(0.4274) * t891 * t950 * t7 + f64x8::splat(623.3709278045327) * t1195 * t542 * t846 * t301;
            let t1233 = t7 * t134;
            let t1244 = f64x8::splat(4.678578898107717) * t308 * t310 * t881 * t541 * t301 - f64x8::splat(69.26343642272586) * t934 * t881 * t559 * t301 - f64x8::splat(3.8527786510141255) * t848 * t267 * t536 + f64x8::splat(0.02168716260060348) * t883 * t267 * t536 + f64x8::splat(0.13012297560362088) * t939 * t267 * t536 - f64x8::splat(0.04337432520120696) * t555 * t482 * t536 - f64x8::splat(1.2842595503380418) * t563 * t482 * t536 + f64x8::splat(0.06747117253521083) * t311 * t867 * t536 + f64x8::splat(0.08674865040241392) * t544 * t482 * t536 + f64x8::splat(38.025319932552506) * t842 * t267 * t536 - f64x8::splat(0.13012297560362088) * t308 * t943 * t541 * t163 * t1233 + f64x8::splat(1.9263893255070628) * t308 * t1195 * t559 * t163 * t1233;
            let t1252 = t580 * t580;
            let t1261 = t575 * t575;
            let t1303 = f64x8::splat(140.0) / f64x8::splat(729.0) * t24 * t1021 * t1 * t20;
            let t1304 = ((t29).select(t1303, f64x8::splat(0.0)));
            let t1309 = t384 * t384;
            let t1318 = t346 * t346;
            let t1331 = f64x8::splat(8.0) / f64x8::splat(135.0) * t1304 * t180 - f64x8::splat(10.0) / f64x8::splat(231.0) * t1304 * t184 - f64x8::splat(2240.0) / f64x8::splat(9.0) * t1309 * t83 + f64x8::splat(5032.0) / f64x8::splat(19.0) * t1309 * t420 - f64x8::splat(1404.0) / f64x8::splat(5.0) * t1309 / t76 / t40 + f64x8::splat(2.0) * t1318 * t35 - f64x8::splat(2.0) * t1318 * t38 + f64x8::splat(9.0) / f64x8::splat(5.0) * t1318 * t41 - f64x8::splat(8.0) / f64x8::splat(5.0) * t1318 * t44 + f64x8::splat(10.0) / f64x8::splat(7.0) * t1318 * t47 - f64x8::splat(9.0) / f64x8::splat(7.0) * t1318 * t50;
            let t1354 = f64x8::splat(7.0) / f64x8::splat(6.0) * t1318 * t53 - f64x8::splat(16.0) / f64x8::splat(15.0) * t1318 * t56 + f64x8::splat(54.0) / f64x8::splat(55.0) * t1318 * t59 - f64x8::splat(10.0) / f64x8::splat(11.0) * t1318 * t62 + f64x8::splat(11.0) / f64x8::splat(13.0) * t1318 * t65 - f64x8::splat(72.0) / f64x8::splat(91.0) * t1318 * t68 + f64x8::splat(26.0) / f64x8::splat(35.0) * t1318 * t71 - f64x8::splat(7.0) / f64x8::splat(10.0) * t1318 * t74 + f64x8::splat(45.0) / f64x8::splat(68.0) * t1318 * t77 - f64x8::splat(32.0) / f64x8::splat(51.0) * t1318 * t80 + f64x8::splat(34.0) / f64x8::splat(57.0) * t1318 * t83;
            let t1378 = -f64x8::splat(54.0) / f64x8::splat(95.0) * t1318 * t420 + f64x8::splat(40.0) / f64x8::splat(3.0) * t1309 * t38 - f64x8::splat(28.0) * t1309 * t41 + f64x8::splat(216.0) / f64x8::splat(5.0) * t1309 * t44 - f64x8::splat(176.0) / f64x8::splat(3.0) * t1309 * t47 + f64x8::splat(520.0) / f64x8::splat(7.0) * t1309 * t50 - f64x8::splat(90.0) * t1309 * t53 + f64x8::splat(952.0) / f64x8::splat(9.0) * t1309 * t56 - f64x8::splat(608.0) / f64x8::splat(5.0) * t1309 * t59 + f64x8::splat(1512.0) / f64x8::splat(11.0) * t1309 * t62 - f64x8::splat(460.0) / f64x8::splat(3.0) * t1309 * t65;
            let t1403 = f64x8::splat(2200.0) / f64x8::splat(13.0) * t1309 * t68 - f64x8::splat(1296.0) / f64x8::splat(7.0) * t1309 * t71 + f64x8::splat(3016.0) / f64x8::splat(15.0) * t1309 * t74 - f64x8::splat(217.0) * t1309 * t77 + f64x8::splat(3960.0) / f64x8::splat(17.0) * t1309 * t80 + f64x8::splat(3.0) / f64x8::splat(91.0) * t1304 * t188 - f64x8::splat(7.0) / f64x8::splat(270.0) * t1304 * t192 + f64x8::splat(16.0) / f64x8::splat(765.0) * t1304 * t196 - f64x8::splat(18.0) / f64x8::splat(1045.0) * t1304 * t200 + f64x8::splat(10.0) / f64x8::splat(693.0) * t1304 * t204 - f64x8::splat(11.0) / f64x8::splat(897.0) * t1304 * t208 + f64x8::splat(24.0) / f64x8::splat(2275.0) * t1304 * t212;
            let t1430 = -f64x8::splat(26.0) / f64x8::splat(2835.0) * t1304 * t216 + f64x8::splat(7.0) / f64x8::splat(870.0) * t1304 * t220 - f64x8::splat(15.0) / f64x8::splat(2108.0) * t1304 * t224 + f64x8::splat(32.0) / f64x8::splat(5049.0) * t1304 * t228 - f64x8::splat(34.0) / f64x8::splat(5985.0) * t1304 * t232 + f64x8::splat(18.0) / f64x8::splat(3515.0) * t1304 * t236 - f64x8::splat(2.0) / f64x8::splat(9.0) * t1304 * t161 + f64x8::splat(2.0) / f64x8::splat(15.0) * t1304 * t172 - f64x8::splat(3.0) / f64x8::splat(35.0) * t1304 * t176 - f64x8::splat(16.0) * t346 * t384 * t172 + f64x8::splat(24.0) * t346 * t384 * t176;
            let t1464 = -f64x8::splat(144.0) / f64x8::splat(5.0) * t346 * t384 * t180 + f64x8::splat(136.0) / f64x8::splat(171.0) * t168 * t611 * t83 - f64x8::splat(72.0) / f64x8::splat(95.0) * t168 * t611 * t420 + f64x8::splat(8.0) / f64x8::splat(3.0) * t168 * t611 * t35 - f64x8::splat(8.0) / f64x8::splat(3.0) * t168 * t611 * t38 + f64x8::splat(12.0) / f64x8::splat(5.0) * t168 * t611 * t41 - f64x8::splat(32.0) / f64x8::splat(15.0) * t168 * t611 * t44 + f64x8::splat(40.0) / f64x8::splat(21.0) * t168 * t611 * t47 - f64x8::splat(12.0) / f64x8::splat(7.0) * t168 * t611 * t50 + f64x8::splat(14.0) / f64x8::splat(9.0) * t168 * t611 * t53 - f64x8::splat(64.0) / f64x8::splat(45.0) * t168 * t611 * t56;
            let t1496 = f64x8::splat(72.0) / f64x8::splat(55.0) * t168 * t611 * t59 - f64x8::splat(40.0) / f64x8::splat(33.0) * t168 * t611 * t62 + f64x8::splat(44.0) / f64x8::splat(39.0) * t168 * t611 * t65 - f64x8::splat(96.0) / f64x8::splat(91.0) * t168 * t611 * t68 + f64x8::splat(104.0) / f64x8::splat(105.0) * t168 * t611 * t71 - f64x8::splat(14.0) / f64x8::splat(15.0) * t168 * t611 * t74 + f64x8::splat(15.0) / f64x8::splat(17.0) * t168 * t611 * t77 - f64x8::splat(128.0) / f64x8::splat(153.0) * t168 * t611 * t80 + f64x8::splat(32.0) * t384 * t355 - f64x8::splat(240.0) / f64x8::splat(7.0) * t384 * t357 + f64x8::splat(36.0) * t384 * t359;
            let t1522 = -f64x8::splat(112.0) / f64x8::splat(3.0) * t384 * t361 + f64x8::splat(192.0) / f64x8::splat(5.0) * t384 * t363 - f64x8::splat(432.0) / f64x8::splat(11.0) * t384 * t365 + f64x8::splat(40.0) * t384 * t367 - f64x8::splat(528.0) / f64x8::splat(13.0) * t384 * t369 + f64x8::splat(288.0) / f64x8::splat(7.0) * t384 * t371 - f64x8::splat(208.0) / f64x8::splat(5.0) * t384 * t373 + f64x8::splat(42.0) * t384 * t375 - f64x8::splat(720.0) / f64x8::splat(17.0) * t384 * t377 + f64x8::splat(128.0) / f64x8::splat(3.0) * t384 * t379 - f64x8::splat(816.0) / f64x8::splat(19.0) * t384 * t381 + f64x8::splat(216.0) / f64x8::splat(5.0) * t384 * t346 * t687;
            let t1526 = ((t29).select(f64x8::splat(0.0), t1303));
            let t1536 = t432 * t432;
            let t1543 = t425 * t425;
            let t1567 = t440 * t440;
            let t1568 = f64x8::splat(1.0) / t1567;
            let t1577 = -f64x8::splat(120.0) * t243 * t1536 * t466 + f64x8::splat(144.0) * t425 * t458 * t798 - f64x8::splat(18.0) * t243 * t1543 * t457 - f64x8::splat(24.0) * t748 * t252 * t457 - f64x8::splat(216.0) * t425 * t467 * t807 + f64x8::splat(16.0) * t240 * t441 * t748 * t466 + f64x8::splat(12.0) * t441 * t1543 * t466 + f64x8::splat(96.0) * t432 * t775 * t425 * t820 + f64x8::splat(96.0) * t1568 * t1536 / t771 / t436 * t89 - f64x8::splat(144.0) * t461 * t433 + f64x8::splat(120.0) * t243 * t456 * t1536;
            let t1611 = f64x8::splat(32.0) * t252 * t755 + f64x8::splat(24.0) * t243 * t1543 * t90 + f64x8::splat(144.0) * t467 * t760 + f64x8::splat(300.0) * t441 * t1536 * t772 * t89 - f64x8::splat(288.0) * t775 * t1536 / t771 / t88 * t89 + f64x8::splat(2.0) * t243 * t1526 * t251 - f64x8::splat(6.0) * t92 * t1543 - f64x8::splat(8.0) * t748 * t784 - f64x8::splat(240.0) * t441 * t465 * t1536 - f64x8::splat(2.0) * t92 * t1526 * t86 + f64x8::splat(128.0) * t775 * t1536 * t772;
            let t1658 = t824 * t240 + (t1577 + t1611) * t86 / f64x8::splat(4.0) + f64x8::splat(8.0) * t252 * t250 * t748 - f64x8::splat(8.0) * t761 * t438 * t748 + f64x8::splat(6.0) * t243 * t250 * t1543 + f64x8::splat(84.0) * t467 * t465 * t425 - f64x8::splat(6.0) * t441 * t438 * t1543 - f64x8::splat(48.0) * t432 * t775 * t772 * t425 + f64x8::splat(24.0) * t243 * t438 * t1536 - t243 * t90 * t1526 - f64x8::splat(96.0) * t441 * t806 * t1536 + f64x8::splat(120.0) * t775 * t819 * t1536 - f64x8::splat(48.0) * t1568 / t771 / t249 * t1536 + t94 * t1526 / f64x8::splat(4.0) + t255 * t748 + f64x8::splat(3.0) / f64x8::splat(2.0) * t470 * t425 - f64x8::splat(36.0) * t458 * t456 * t425;
            let t1662 = ((t28).select(t1331 + t1354 + t1378 + t1403 + t1430 + t1464 + t1496 + t1522, -f64x8::splat(8.0) / f64x8::splat(3.0) * t97 * t1526 - f64x8::splat(32.0) / f64x8::splat(3.0) * t258 * t748 - f64x8::splat(16.0) * t473 * t425 - f64x8::splat(32.0) / f64x8::splat(3.0) * t827 * t240 - f64x8::splat(8.0) / f64x8::splat(3.0) * t1658 * t86));
            let t1667 = f64x8::splat(3.4602) * t153 * (t1090 + t1156 + t1199 + t1244) - f64x8::splat(124.5672) * t580 * t575 * t986 + f64x8::splat(20.7612) * t1252 * t574 + f64x8::splat(27.6816) * t995 * t322 * t574 + f64x8::splat(83.0448) * t1261 / t984 / t152 * t149 - f64x8::splat(83.0448) * t987 * t985 * t314 + f64x8::splat(83.0448) * t990 * t979 + f64x8::splat(41.5224) * t575 * t573 * t566 - f64x8::splat(3.4602) * (-f64x8::splat(0.7777777777777778) * t1026 + f64x8::splat(220.0) / f64x8::splat(81.0) * t1040) * t319 - f64x8::splat(13.8408) * t995 * t569 - f64x8::splat(20.7612) * t580 * t976 - f64x8::splat(13.8408) * t322 * t318 * t973 + f64x8::splat(5.0) / f64x8::splat(27.0) * t101 * t860 * t14 * t7 - f64x8::splat(5.0) / f64x8::splat(18.0) * t262 * t502 * t14 * t7 + t477 * t287 * t14 * t7 / f64x8::splat(4.0) - t831 * t118 * t14 * t7 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(16.0) * t1662 * t15 * t14 * t7;
            let tv4rho40 = t1667 * v_rho + f64x8::splat(13.8408) * t974 - f64x8::splat(41.5224) * t982 - f64x8::splat(13.8408) * t996 - f64x8::splat(41.5224) * t977 - f64x8::splat(83.0448) * t988 + f64x8::splat(83.0448) * t991 + f64x8::splat(83.0448) * t980 + t598 / f64x8::splat(2.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t602 - f64x8::splat(3.0) / f64x8::splat(4.0) * t834 - f64x8::splat(5.0) / f64x8::splat(18.0) * t594;
            acc_v4rho4 = tv4rho40;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(v2rho2, ip, m, acc_v2rho2);
        store_add(v3rho3, ip, m, acc_v3rho3);
        store_add(v4rho4, ip, m, acc_v4rho4);
        ip += 8;
    }
}

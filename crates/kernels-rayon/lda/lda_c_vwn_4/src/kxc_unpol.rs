//! LDA_C_VWN_4 kxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_vwn_4.c`
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
pub fn lda_c_vwn_4_kxc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    v3rho3: &mut [f64],
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
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_v2rho2 = V_ZERO;
        let mut acc_v3rho3 = V_ZERO;
        {
            let t1 = f64x8::splat(M_CBRT3);
            let t2 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t3 = (simd::cbrt(t2));
            let t4 = t1 * t3;
            let t5 = f64x8::splat(M_CBRT4);
            let t6 = t5 * t5;
            let t7 = (simd::cbrt(v_rho));
            let t8 = f64x8::splat(1.0) / t7;
            let t9 = t6 * t8;
            let t10 = t4 * t9;
            let t11 = t10 / f64x8::splat(4.0);
            let t12 = ((t10).sqrt());
            let t14 = t11 + f64x8::splat(1.86372) * t12 + f64x8::splat(12.9352);
            let t15 = f64x8::splat(1.0) / t14;
            let t19 = (simd::ln(t4 * t9 * t15 / f64x8::splat(4.0)));
            let t20 = f64x8::splat(0.0310907) * t19;
            let t21 = t12 + f64x8::splat(3.72744);
            let t24 = (simd::atan(f64x8::splat(6.15199081975908) / t21));
            let t25 = f64x8::splat(0.038783294878113016) * t24;
            let t26 = t12 / f64x8::splat(2.0);
            let t27 = t26 + f64x8::splat(0.10498);
            let t28 = t27 * t27;
            let t30 = (simd::ln(t28 * t15));
            let t31 = f64x8::splat(0.0009690227711544374) * t30;
            let t32 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t33 = f64x8::splat(1.0) / t32;
            let t35 = t11 + f64x8::splat(0.534175) * t12 + f64x8::splat(11.4813);
            let t36 = f64x8::splat(1.0) / t35;
            let t40 = (simd::ln(t4 * t9 * t36 / f64x8::splat(4.0)));
            let t41 = t12 + f64x8::splat(1.06835);
            let t44 = (simd::atan(f64x8::splat(6.692072046645942) / t41));
            let t46 = t26 + f64x8::splat(0.228344);
            let t47 = t46 * t46;
            let t49 = (simd::ln(t47 * t36));
            let t54 = (simd::cbrt(zeta_threshold));
            let t56 = (((f64x8::splat(1.0)).simd_le(zeta_threshold)).select(t54 * zeta_threshold, f64x8::splat(1.0)));
            let t59 = f64x8::splat(M_CBRT2);
            let t60 = t59 - f64x8::splat(1.0);
            let t65 = f64x8::splat(9.0) * t56 - f64x8::splat(9.0);
            let t67 = t33 * (t40 + f64x8::splat(0.32323836906055065) * t44 + f64x8::splat(0.021608710360898266) * t49) * t65 / f64x8::splat(24.0);
            let tzk0 = t20 + t25 + t31 - t67;
            acc_zk = tzk0;
            let t69 = f64x8::splat(1.0) / t7 / v_rho;
            let t70 = t6 * t69;
            let t74 = t4 * t6;
            let t75 = t14 * t14;
            let t76 = f64x8::splat(1.0) / t75;
            let t77 = t8 * t76;
            let t78 = t4 * t70;
            let t79 = t78 / f64x8::splat(12.0);
            let t80 = f64x8::splat(1.0) / t12;
            let t81 = t80 * t1;
            let t82 = t3 * t6;
            let t84 = t81 * t82 * t69;
            let t86 = -t79 - f64x8::splat(0.31062) * t84;
            let t91 = t1 * t1;
            let t93 = f64x8::splat(1.0) / t3;
            let t94 = (-t4 * t70 * t15 / f64x8::splat(12.0) - t74 * t77 * t86 / f64x8::splat(4.0)) * t91 * t93;
            let t95 = t5 * t7;
            let t96 = t95 * t14;
            let t97 = t94 * t96;
            let t99 = t21 * t21;
            let t100 = f64x8::splat(1.0) / t99;
            let t102 = t100 * t80 * t1;
            let t104 = f64x8::splat(37.8469910464) * t100 + f64x8::splat(1.0);
            let t105 = f64x8::splat(1.0) / t104;
            let t108 = t102 * t82 * t69 * t105;
            let t110 = t27 * t15;
            let t111 = t110 * t80;
            let t114 = t28 * t76;
            let t116 = -t111 * t78 / f64x8::splat(6.0) - t114 * t86;
            let t117 = f64x8::splat(1.0) / t28;
            let t118 = t116 * t117;
            let t119 = t118 * t14;
            let t124 = t35 * t35;
            let t125 = f64x8::splat(1.0) / t124;
            let t126 = t8 * t125;
            let t128 = -t79 - f64x8::splat(0.08902916666666667) * t84;
            let t134 = (-t4 * t70 * t36 / f64x8::splat(12.0) - t74 * t126 * t128 / f64x8::splat(4.0)) * t91 * t93;
            let t135 = t95 * t35;
            let t138 = t41 * t41;
            let t139 = f64x8::splat(1.0) / t138;
            let t141 = t139 * t80 * t1;
            let t143 = f64x8::splat(44.7838282775) * t139 + f64x8::splat(1.0);
            let t144 = f64x8::splat(1.0) / t143;
            let t149 = t46 * t36;
            let t150 = t149 * t80;
            let t153 = t47 * t125;
            let t155 = -t150 * t78 / f64x8::splat(6.0) - t153 * t128;
            let t156 = f64x8::splat(1.0) / t47;
            let t157 = t155 * t156;
            let t162 = t33 * (t134 * t135 / f64x8::splat(3.0) + f64x8::splat(0.36052240899892257) * t141 * t82 * t69 * t144 + f64x8::splat(0.021608710360898266) * t157 * t35) * t65;
            let tvrho0 = t20 + t25 + t31 - t67 + v_rho * (f64x8::splat(0.010363566666666667) * t97 + f64x8::splat(0.03976574567502677) * t108 + f64x8::splat(0.0009690227711544374) * t119 - t162 / f64x8::splat(24.0));
            acc_vrho = tvrho0;
            let t170 = v_rho * v_rho;
            let t172 = f64x8::splat(1.0) / t7 / t170;
            let t173 = t6 * t172;
            let t175 = t4 * t173 * t15;
            let t177 = t69 * t76;
            let t182 = f64x8::splat(1.0) / t75 / t14;
            let t183 = t8 * t182;
            let t184 = t86 * t86;
            let t188 = t4 * t173;
            let t189 = t188 / f64x8::splat(9.0);
            let t191 = f64x8::splat(1.0) / t12 / t10;
            let t192 = t191 * t91;
            let t193 = t3 * t3;
            let t194 = t193 * t5;
            let t195 = t7 * t7;
            let t197 = f64x8::splat(1.0) / t195 / t170;
            let t199 = t192 * t194 * t197;
            let t202 = t81 * t82 * t172;
            let t204 = t189 - f64x8::splat(0.20708) * t199 + f64x8::splat(0.41416) * t202;
            let t210 = (t175 / f64x8::splat(9.0) + t74 * t177 * t86 / f64x8::splat(6.0) + t74 * t183 * t184 / f64x8::splat(2.0) - t74 * t77 * t204 / f64x8::splat(4.0)) * t91 * t93;
            let t211 = t210 * t96;
            let t214 = t5 / t195;
            let t215 = t214 * t14;
            let t216 = t94 * t215;
            let t218 = t95 * t86;
            let t219 = t94 * t218;
            let t221 = t99 * t21;
            let t222 = f64x8::splat(1.0) / t221;
            let t223 = t222 * t1;
            let t224 = t223 * t3;
            let t226 = t224 * t173 * t105;
            let t229 = t100 * t191 * t91;
            let t232 = t229 * t194 * t197 * t105;
            let t236 = t102 * t82 * t172 * t105;
            let t238 = t99 * t99;
            let t240 = f64x8::splat(1.0) / t238 / t21;
            let t241 = t240 * t1;
            let t242 = t241 * t3;
            let t243 = t104 * t104;
            let t244 = f64x8::splat(1.0) / t243;
            let t246 = t242 * t173 * t244;
            let t249 = t27 * t76;
            let t250 = t249 * t81;
            let t251 = t69 * t86;
            let t255 = t110 * t191;
            let t256 = t91 * t193;
            let t257 = t5 * t197;
            let t258 = t256 * t257;
            let t263 = t28 * t182;
            let t267 = t175 / f64x8::splat(72.0) + t250 * t82 * t251 / f64x8::splat(3.0) - t255 * t258 / f64x8::splat(9.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t111 * t188 + f64x8::splat(2.0) * t263 * t184 - t114 * t204;
            let t268 = t267 * t117;
            let t269 = t268 * t14;
            let t272 = f64x8::splat(1.0) / t28 / t27;
            let t273 = t116 * t272;
            let t274 = t14 * t80;
            let t275 = t273 * t274;
            let t276 = t275 * t78;
            let t278 = t118 * t86;
            let t281 = t4 * t173 * t36;
            let t283 = t69 * t125;
            let t288 = f64x8::splat(1.0) / t124 / t35;
            let t289 = t8 * t288;
            let t290 = t128 * t128;
            let t296 = t189 - f64x8::splat(0.059352777777777775) * t199 + f64x8::splat(0.11870555555555555) * t202;
            let t302 = (t281 / f64x8::splat(9.0) + t74 * t283 * t128 / f64x8::splat(6.0) + t74 * t289 * t290 / f64x8::splat(2.0) - t74 * t126 * t296 / f64x8::splat(4.0)) * t91 * t93;
            let t305 = t214 * t35;
            let t308 = t95 * t128;
            let t311 = t138 * t41;
            let t312 = f64x8::splat(1.0) / t311;
            let t313 = t312 * t1;
            let t314 = t313 * t3;
            let t319 = t139 * t191 * t91;
            let t328 = t138 * t138;
            let t330 = f64x8::splat(1.0) / t328 / t41;
            let t331 = t330 * t1;
            let t332 = t331 * t3;
            let t333 = t143 * t143;
            let t334 = f64x8::splat(1.0) / t333;
            let t339 = t46 * t125;
            let t340 = t339 * t81;
            let t341 = t69 * t128;
            let t345 = t149 * t191;
            let t350 = t47 * t288;
            let t354 = t281 / f64x8::splat(72.0) + t340 * t82 * t341 / f64x8::splat(3.0) - t345 * t258 / f64x8::splat(9.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t150 * t188 + f64x8::splat(2.0) * t350 * t290 - t153 * t296;
            let t355 = t354 * t156;
            let t359 = f64x8::splat(1.0) / t47 / t46;
            let t360 = t155 * t359;
            let t361 = t35 * t80;
            let t362 = t360 * t361;
            let t369 = t33 * (t302 * t135 / f64x8::splat(3.0) + t134 * t305 / f64x8::splat(9.0) + t134 * t308 / f64x8::splat(3.0) + f64x8::splat(0.12017413633297419) * t314 * t173 * t144 + f64x8::splat(0.24034827266594838) * t319 * t194 * t197 * t144 - f64x8::splat(0.48069654533189676) * t141 * t82 * t172 * t144 - f64x8::splat(5.38185788493279) * t332 * t173 * t334 + f64x8::splat(0.021608710360898266) * t355 * t35 + f64x8::splat(0.003601451726816378) * t362 * t78 + f64x8::splat(0.021608710360898266) * t157 * t128) * t65;
            let t371 = f64x8::splat(0.010363566666666667) * t211 + f64x8::splat(0.003454522222222222) * t216 + f64x8::splat(0.010363566666666667) * t219 + f64x8::splat(0.013255248558342257) * t226 + f64x8::splat(0.026510497116684514) * t232 - f64x8::splat(0.05302099423336903) * t236 - f64x8::splat(0.5016712735053859) * t246 + f64x8::splat(0.0009690227711544374) * t269 + f64x8::splat(0.00016150379519240624) * t276 + f64x8::splat(0.0009690227711544374) * t278 - t369 / f64x8::splat(24.0);
            let tv2rho20 = f64x8::splat(0.020727133333333335) * t97 + f64x8::splat(0.07953149135005354) * t108 + f64x8::splat(0.001938045542308875) * t119 - t162 / f64x8::splat(12.0) + v_rho * t371;
            acc_v2rho2 = tv2rho20;
            let t384 = t267 * t272;
            let t385 = t384 * t274;
            let t386 = t385 * t78;
            let t388 = t86 * t80;
            let t389 = t273 * t388;
            let t390 = t389 * t78;
            let t392 = t14 * t191;
            let t393 = t273 * t392;
            let t394 = t393 * t258;
            let t396 = f64x8::splat(1.0) / t193;
            let t397 = t331 * t396;
            let t398 = t170 * v_rho;
            let t400 = f64x8::splat(1.0) / t7 / t398;
            let t401 = t6 * t400;
            let t402 = t401 * t334;
            let t407 = t214 * t128;
            let t410 = t95 * t296;
            let t416 = t4 * t401 * t36;
            let t418 = t172 * t125;
            let t420 = t74 * t418 * t128;
            let t422 = t69 * t288;
            let t429 = t124 * t124;
            let t430 = f64x8::splat(1.0) / t429;
            let t431 = t8 * t430;
            let t432 = t290 * t128;
            let t436 = t128 * t296;
            let t440 = t4 * t401;
            let t441 = f64x8::splat(7.0) / f64x8::splat(27.0) * t440;
            let t445 = f64x8::splat(1.0) / t12 / t256 / t214 / f64x8::splat(4.0);
            let t446 = t445 * t2;
            let t447 = t170 * t170;
            let t448 = f64x8::splat(1.0) / t447;
            let t449 = t446 * t448;
            let t452 = f64x8::splat(1.0) / t195 / t398;
            let t454 = t192 * t194 * t452;
            let t457 = t81 * t82 * t400;
            let t459 = -t441 - f64x8::splat(0.3561166666666667) * t449 + f64x8::splat(0.2374111111111111) * t454 - f64x8::splat(0.27697962962962963) * t457;
            let t465 = (-f64x8::splat(7.0) / f64x8::splat(27.0) * t416 - t420 / f64x8::splat(3.0) - t74 * t422 * t290 / f64x8::splat(2.0) + t74 * t283 * t296 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(2.0) * t74 * t431 * t432 + f64x8::splat(3.0) / f64x8::splat(2.0) * t74 * t289 * t436 - t74 * t126 * t459 / f64x8::splat(4.0)) * t91 * t93;
            let t468 = t139 * t445;
            let t469 = t2 * t448;
            let t477 = t5 / t195 / v_rho;
            let t478 = t477 * t35;
            let t484 = f64x8::splat(1.0) / t328;
            let t485 = t484 * t91;
            let t486 = t485 * t193;
            let t487 = t5 * t452;
            let t488 = t144 * t80;
            let t493 = f64x8::splat(1.0) / t328 / t138;
            let t494 = t493 * t91;
            let t495 = t494 * t193;
            let t496 = t334 * t80;
            let t500 = -f64x8::splat(0.2855164284683821) * t397 * t402 + f64x8::splat(19.733478911420228) * t332 * t402 + f64x8::splat(2.0) / f64x8::splat(9.0) * t134 * t407 + t134 * t410 / f64x8::splat(3.0) + f64x8::splat(2.0) / f64x8::splat(3.0) * t302 * t308 + t465 * t135 / f64x8::splat(3.0) + f64x8::splat(1.4420896359956903) * t468 * t469 * t144 + f64x8::splat(2.0) / f64x8::splat(9.0) * t302 * t305 - f64x8::splat(2.0) / f64x8::splat(27.0) * t134 * t478 - f64x8::splat(0.44063849988757203) * t314 * t401 * t144 + f64x8::splat(0.24034827266594838) * t486 * t487 * t488 - f64x8::splat(25.11533679635302) * t495 * t487 * t496;
            let t501 = t313 * t396;
            let t502 = t2 * t144;
            let t506 = t328 * t328;
            let t507 = f64x8::splat(1.0) / t506;
            let t508 = t507 * t91;
            let t509 = t508 * t193;
            let t511 = f64x8::splat(1.0) / t333 / t143;
            let t512 = t511 * t80;
            let t516 = t47 * t47;
            let t517 = f64x8::splat(1.0) / t516;
            let t518 = t155 * t517;
            let t519 = t518 * t35;
            let t524 = t46 * t288;
            let t525 = t524 * t81;
            let t529 = t339 * t192;
            let t543 = t1 * t396 * t6;
            let t544 = t400 * t2;
            let t550 = t256 * t487;
            let t555 = t47 * t430;
            let t561 = -f64x8::splat(11.0) / f64x8::splat(216.0) * t416 - t420 / f64x8::splat(24.0) - t525 * t82 * t69 * t290 + t529 * t194 * t197 * t128 / f64x8::splat(3.0) - f64x8::splat(2.0) / f64x8::splat(3.0) * t340 * t82 * t172 * t128 + t340 * t82 * t69 * t296 / f64x8::splat(2.0) + t543 * t544 * t36 / f64x8::splat(432.0) - f64x8::splat(2.0) / f64x8::splat(3.0) * t149 * t449 + f64x8::splat(4.0) / f64x8::splat(9.0) * t345 * t550 - f64x8::splat(14.0) / f64x8::splat(27.0) * t150 * t440 - f64x8::splat(6.0) * t555 * t432 + f64x8::splat(6.0) * t350 * t436 - t153 * t459;
            let t562 = t561 * t156;
            let t579 = t354 * t359;
            let t580 = t579 * t361;
            let t583 = t128 * t80;
            let t584 = t360 * t583;
            let t587 = t35 * t191;
            let t588 = t360 * t587;
            let t591 = f64x8::splat(0.020029022722162365) * t501 * t401 * t502 + f64x8::splat(642.7205315539718) * t509 * t487 * t512 + f64x8::splat(0.0009003629317040944) * t519 * t188 + f64x8::splat(0.021608710360898266) * t562 * t35 + f64x8::splat(0.04321742072179653) * t355 * t128 + f64x8::splat(0.021608710360898266) * t157 * t296 - f64x8::splat(0.00480193563575517) * t362 * t188 - f64x8::splat(0.9613930906637935) * t319 * t194 * t452 * t144 + f64x8::splat(1.1216252724410924) * t141 * t82 * t400 * t144 + f64x8::splat(0.007202903453632756) * t580 * t78 + f64x8::splat(0.007202903453632756) * t584 * t78 + f64x8::splat(0.002400967817877585) * t588 * t258;
            let t594 = t33 * (t500 + t591) * t65;
            let t596 = t100 * t445;
            let t598 = t596 * t469 * t105;
            let t600 = t118 * t204;
            let t603 = t4 * t401 * t15;
            let t605 = t172 * t76;
            let t607 = t74 * t605 * t86;
            let t609 = t27 * t182;
            let t610 = t609 * t81;
            let t614 = t249 * t192;
            let t636 = t75 * t75;
            let t637 = f64x8::splat(1.0) / t636;
            let t638 = t28 * t637;
            let t639 = t184 * t86;
            let t642 = t86 * t204;
            let t648 = -t441 - f64x8::splat(1.24248) * t449 + f64x8::splat(0.82832) * t454 - f64x8::splat(0.9663733333333333) * t457;
            let t650 = -f64x8::splat(11.0) / f64x8::splat(216.0) * t603 - t607 / f64x8::splat(24.0) - t610 * t82 * t69 * t184 + t614 * t194 * t197 * t86 / f64x8::splat(3.0) - f64x8::splat(2.0) / f64x8::splat(3.0) * t250 * t82 * t172 * t86 + t250 * t82 * t69 * t204 / f64x8::splat(2.0) + t543 * t544 * t15 / f64x8::splat(432.0) - f64x8::splat(2.0) / f64x8::splat(3.0) * t110 * t449 + f64x8::splat(4.0) / f64x8::splat(9.0) * t255 * t550 - f64x8::splat(14.0) / f64x8::splat(27.0) * t111 * t440 - f64x8::splat(6.0) * t638 * t639 + f64x8::splat(6.0) * t263 * t642 - t114 * t648;
            let t651 = t650 * t117;
            let t652 = t651 * t14;
            let t654 = t268 * t86;
            let t658 = t69 * t182;
            let t665 = t8 * t637;
            let t677 = (-f64x8::splat(7.0) / f64x8::splat(27.0) * t603 - t607 / f64x8::splat(3.0) - t74 * t658 * t184 / f64x8::splat(2.0) + t74 * t177 * t204 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(2.0) * t74 * t665 * t639 + f64x8::splat(3.0) / f64x8::splat(2.0) * t74 * t183 * t642 - t74 * t77 * t648 / f64x8::splat(4.0)) * t91 * t93;
            let t678 = t677 * t96;
            let t680 = t210 * t215;
            let t682 = t477 * t14;
            let t683 = t94 * t682;
            let t686 = t224 * t401 * t105;
            let t688 = f64x8::splat(0.0003230075903848125) * t386 + f64x8::splat(0.0003230075903848125) * t390 + f64x8::splat(0.00010766919679493748) * t394 - t594 / f64x8::splat(24.0) + f64x8::splat(0.15906298270010708) * t598 + f64x8::splat(0.0009690227711544374) * t600 + f64x8::splat(0.0009690227711544374) * t652 + f64x8::splat(0.001938045542308875) * t654 + f64x8::splat(0.010363566666666667) * t678 + f64x8::splat(0.006909044444444444) * t680 - f64x8::splat(0.002303014814814815) * t683 - f64x8::splat(0.04860257804725494) * t686;
            let t689 = t210 * t218;
            let t691 = t214 * t86;
            let t692 = t94 * t691;
            let t694 = t95 * t204;
            let t695 = t94 * t694;
            let t697 = t241 * t396;
            let t698 = t401 * t244;
            let t699 = t697 * t698;
            let t701 = t242 * t698;
            let t703 = t275 * t188;
            let t705 = f64x8::splat(1.0) / t238;
            let t706 = t705 * t91;
            let t707 = t706 * t193;
            let t708 = t105 * t80;
            let t710 = t707 * t487 * t708;
            let t713 = f64x8::splat(1.0) / t238 / t99;
            let t714 = t713 * t91;
            let t715 = t714 * t193;
            let t716 = t244 * t80;
            let t718 = t715 * t487 * t716;
            let t720 = t223 * t396;
            let t721 = t2 * t105;
            let t723 = t720 * t401 * t721;
            let t725 = t238 * t238;
            let t726 = f64x8::splat(1.0) / t725;
            let t727 = t726 * t91;
            let t728 = t727 * t193;
            let t730 = f64x8::splat(1.0) / t243 / t104;
            let t731 = t730 * t80;
            let t733 = t728 * t487 * t731;
            let t735 = t28 * t28;
            let t736 = f64x8::splat(1.0) / t735;
            let t737 = t116 * t736;
            let t738 = t737 * t14;
            let t739 = t738 * t188;
            let t743 = t229 * t194 * t452 * t105;
            let t747 = t102 * t82 * t400 * t105;
            let t749 = f64x8::splat(0.020727133333333335) * t689 + f64x8::splat(0.006909044444444444) * t692 + f64x8::splat(0.010363566666666667) * t695 - f64x8::splat(0.026614487661862786) * t699 + f64x8::splat(1.839461336186415) * t701 - f64x8::splat(0.00021533839358987497) * t703 + f64x8::splat(0.026510497116684514) * t710 - f64x8::splat(2.341132609691801) * t718 + f64x8::splat(0.002209208093057043) * t723 + f64x8::splat(50.6313285242518) * t733 + f64x8::splat(4.037594879810156e-05) * t739 - f64x8::splat(0.10604198846673805) * t743 + f64x8::splat(0.12371565321119439) * t747;
            let tv3rho30 = f64x8::splat(0.0310907) * t211 + f64x8::splat(0.010363566666666667) * t216 + f64x8::splat(0.0310907) * t219 + f64x8::splat(0.03976574567502677) * t226 + f64x8::splat(0.07953149135005354) * t232 - f64x8::splat(0.15906298270010708) * t236 - f64x8::splat(1.5050138205161576) * t246 + f64x8::splat(0.0029070683134633122) * t269 + f64x8::splat(0.0004845113855772187) * t276 + f64x8::splat(0.0029070683134633122) * t278 - t369 / f64x8::splat(8.0) + v_rho * (t688 + t749);
            acc_v3rho3 = tv3rho30;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(v2rho2, ip, m, acc_v2rho2);
        store_add(v3rho3, ip, m, acc_v3rho3);
        ip += 8;
    }
}

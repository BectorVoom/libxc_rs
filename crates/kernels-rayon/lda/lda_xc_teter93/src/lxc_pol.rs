//! LDA_XC_TETER93 lxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_xc_teter93.c`
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

/// Store 8 elements with a given stride and offset.
#[inline(always)]
fn store_strided(s: &mut [f64], ip: usize, m: usize, stride: usize, offset: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let base = ip * stride + offset;
        s[base] = a[0];
        s[base + stride] = a[1];
        s[base + 2 * stride] = a[2];
        s[base + 3 * stride] = a[3];
        s[base + 4 * stride] = a[4];
        s[base + 5 * stride] = a[5];
        s[base + 6 * stride] = a[6];
        s[base + 7 * stride] = a[7];
    } else {
        for k in 0..m {
            s[(ip + k) * stride + offset] = a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn lda_xc_teter93_lxc_pol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    v3rho3: &mut [f64],
    v4rho4: &mut [f64],
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
        let mut acc_zk = V_ZERO;
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        let mut acc_v2rho2_0 = V_ZERO;
        let mut acc_v2rho2_1 = V_ZERO;
        let mut acc_v2rho2_2 = V_ZERO;
        let mut acc_v3rho3_0 = V_ZERO;
        let mut acc_v3rho3_1 = V_ZERO;
        let mut acc_v3rho3_2 = V_ZERO;
        let mut acc_v3rho3_3 = V_ZERO;
        let mut acc_v4rho4_0 = V_ZERO;
        let mut acc_v4rho4_1 = V_ZERO;
        let mut acc_v4rho4_2 = V_ZERO;
        let mut acc_v4rho4_3 = V_ZERO;
        let mut acc_v4rho4_4 = V_ZERO;
        {
            let t1 = v_rho0 - v_rho1;
            let t2 = v_rho0 + v_rho1;
            let t3 = f64x8::splat(1.0) / t2;
            let t4 = t1 * t3;
            let t5 = f64x8::splat(1.0) + t4;
            let t6 = (t5).simd_le(zeta_threshold);
            let t7 = (simd::cbrt(zeta_threshold));
            let t8 = t7 * zeta_threshold;
            let t9 = (simd::cbrt(t5));
            let t11 = ((t6).select(t8, t9 * t5));
            let t12 = f64x8::splat(1.0) - t4;
            let t13 = (t12).simd_le(zeta_threshold);
            let t14 = (simd::cbrt(t12));
            let t16 = ((t13).select(t8, t14 * t12));
            let t18 = f64x8::splat(M_CBRT2);
            let t21 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t18 - f64x8::splat(2.0));
            let t22 = (t11 + t16 - f64x8::splat(2.0)) * t21;
            let t26 = f64x8::splat(M_CBRT3);
            let t27 = (f64x8::splat(2.217058676663745) + f64x8::splat(0.6157402568883344) * t22) * t26;
            let t28 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t29 = (simd::cbrt(t28));
            let t30 = f64x8::splat(M_CBRT4);
            let t31 = t30 * t30;
            let t32 = t29 * t31;
            let t33 = (simd::cbrt(t2));
            let t34 = f64x8::splat(1.0) / t33;
            let t35 = t32 * t34;
            let t40 = t26 * t26;
            let t41 = (f64x8::splat(0.7405551735357053) + f64x8::splat(0.1574201515892867) * t22) * t40;
            let t42 = t29 * t29;
            let t43 = t42 * t30;
            let t44 = t33 * t33;
            let t46 = t43 / t44;
            let t51 = (f64x8::splat(0.01968227878617998) + f64x8::splat(0.003532336663397157) * t22) * t28;
            let t54 = f64x8::splat(0.4581652932831429) + f64x8::splat(0.119086804055547) * t22 + t27 * t35 / f64x8::splat(4.0) + t41 * t46 / f64x8::splat(4.0) + f64x8::splat(3.0) / f64x8::splat(4.0) * t51 * t3;
            let t55 = t26 * t29;
            let t61 = (f64x8::splat(4.504130959426697) + f64x8::splat(0.2673612973836267) * t22) * t40;
            let t66 = (f64x8::splat(1.110667363742916) + f64x8::splat(0.2052004607777787) * t22) * t28;
            let t71 = (f64x8::splat(0.02359291751427506) + f64x8::splat(0.004200005045691381) * t22) * t26;
            let t73 = t29 * t28 * t31;
            let t75 = f64x8::splat(1.0) / t33 / t2;
            let t76 = t73 * t75;
            let t79 = f64x8::splat(0.25) * t55 * t31 * t34 + t61 * t46 / f64x8::splat(4.0) + f64x8::splat(3.0) / f64x8::splat(4.0) * t66 * t3 + f64x8::splat(3.0) / f64x8::splat(16.0) * t71 * t76;
            let t80 = f64x8::splat(1.0) / t79;
            let tzk0 = -t54 * t80;
            acc_zk = tzk0;
            let t82 = t2 * t2;
            let t83 = f64x8::splat(1.0) / t82;
            let t84 = t1 * t83;
            let t85 = t3 - t84;
            let t88 = ((t6).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t9 * t85));
            let t89 = -t85;
            let t92 = ((t13).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t14 * t89));
            let t94 = (t88 + t92) * t21;
            let t96 = t94 * t26;
            let t99 = t32 * t75;
            let t101 = t27 * t99 / f64x8::splat(12.0);
            let t102 = t94 * t40;
            let t103 = t102 * t46;
            let t107 = t43 / t44 / t2;
            let t109 = t41 * t107 / f64x8::splat(6.0);
            let t110 = t94 * t3;
            let t113 = f64x8::splat(3.0) / f64x8::splat(4.0) * t51 * t83;
            let t114 = f64x8::splat(0.119086804055547) * t94 + f64x8::splat(0.1539350642220836) * t96 * t35 - t101 + f64x8::splat(0.03935503789732168) * t103 - t109 + f64x8::splat(0.0008432832609665849) * t110 - t113;
            let t115 = t2 * t114;
            let t117 = t2 * t54;
            let t118 = t79 * t79;
            let t119 = f64x8::splat(1.0) / t118;
            let t122 = f64x8::splat(0.08333333333333333) * t55 * t31 * t75;
            let t125 = t61 * t107 / f64x8::splat(6.0);
            let t128 = f64x8::splat(3.0) / f64x8::splat(4.0) * t66 * t83;
            let t132 = f64x8::splat(1.0) / t33 / t82;
            let t133 = t73 * t132;
            let t135 = t71 * t133 / f64x8::splat(4.0);
            let t136 = -t122 + f64x8::splat(0.06684032434590667) * t103 - t125 + f64x8::splat(0.048988001486277105) * t110 - t128 + f64x8::splat(0.0007875009460671339) * t96 * t76 - t135;
            let t137 = t119 * t136;
            let tvrho0 = -t115 * t80 + t117 * t137 + tzk0;
            acc_vrho_0 = tvrho0;
            let t139 = -t3 - t84;
            let t142 = ((t6).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t9 * t139));
            let t143 = -t139;
            let t146 = ((t13).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t14 * t143));
            let t148 = (t142 + t146) * t21;
            let t150 = t148 * t26;
            let t153 = t148 * t40;
            let t154 = t153 * t46;
            let t156 = t148 * t3;
            let t158 = f64x8::splat(0.119086804055547) * t148 + f64x8::splat(0.1539350642220836) * t150 * t35 - t101 + f64x8::splat(0.03935503789732168) * t154 - t109 + f64x8::splat(0.0008432832609665849) * t156 - t113;
            let t159 = t2 * t158;
            let t165 = -t122 + f64x8::splat(0.06684032434590667) * t154 - t125 + f64x8::splat(0.048988001486277105) * t156 - t128 + f64x8::splat(0.0007875009460671339) * t150 * t76 - t135;
            let t166 = t119 * t165;
            let tvrho1 = t117 * t166 - t159 * t80 + tzk0;
            acc_vrho_1 = tvrho1;
            let t168 = t114 * t80;
            let t170 = t54 * t119;
            let t171 = t170 * t136;
            let t173 = t9 * t9;
            let t174 = f64x8::splat(1.0) / t173;
            let t175 = t85 * t85;
            let t178 = t82 * t2;
            let t179 = f64x8::splat(1.0) / t178;
            let t180 = t1 * t179;
            let t182 = -f64x8::splat(2.0) * t83 + f64x8::splat(2.0) * t180;
            let t186 = ((t6).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t174 * t175 + f64x8::splat(4.0) / f64x8::splat(3.0) * t9 * t182));
            let t187 = t14 * t14;
            let t188 = f64x8::splat(1.0) / t187;
            let t189 = t89 * t89;
            let t192 = -t182;
            let t196 = ((t13).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t188 * t189 + f64x8::splat(4.0) / f64x8::splat(3.0) * t14 * t192));
            let t198 = (t186 + t196) * t21;
            let t200 = t198 * t26;
            let t203 = t96 * t99;
            let t205 = t32 * t132;
            let t207 = t27 * t205 / f64x8::splat(9.0);
            let t208 = t198 * t40;
            let t209 = t208 * t46;
            let t211 = t102 * t107;
            let t215 = t43 / t44 / t82;
            let t217 = f64x8::splat(5.0) / f64x8::splat(18.0) * t41 * t215;
            let t218 = t198 * t3;
            let t220 = t94 * t83;
            let t223 = f64x8::splat(3.0) / f64x8::splat(2.0) * t51 * t179;
            let t224 = f64x8::splat(0.119086804055547) * t198 + f64x8::splat(0.1539350642220836) * t200 * t35 - f64x8::splat(0.10262337614805575) * t203 + t207 + f64x8::splat(0.03935503789732168) * t209 - f64x8::splat(0.052473383863095566) * t211 + t217 + f64x8::splat(0.0008432832609665849) * t218 - f64x8::splat(0.0016865665219331699) * t220 + t223;
            let t225 = t2 * t224;
            let t230 = f64x8::splat(1.0) / t118 / t79;
            let t231 = t136 * t136;
            let t232 = t230 * t231;
            let t237 = f64x8::splat(0.1111111111111111) * t55 * t31 * t132;
            let t241 = f64x8::splat(5.0) / f64x8::splat(18.0) * t61 * t215;
            let t245 = f64x8::splat(3.0) / f64x8::splat(2.0) * t66 * t179;
            let t248 = t96 * t133;
            let t251 = f64x8::splat(1.0) / t33 / t178;
            let t252 = t73 * t251;
            let t254 = f64x8::splat(7.0) / f64x8::splat(12.0) * t71 * t252;
            let t255 = t237 + f64x8::splat(0.06684032434590667) * t209 - f64x8::splat(0.0891204324612089) * t211 + t241 + f64x8::splat(0.048988001486277105) * t218 - f64x8::splat(0.09797600297255421) * t220 + t245 + f64x8::splat(0.0007875009460671339) * t200 * t76 - f64x8::splat(0.0021000025228456905) * t248 + t254;
            let t256 = t119 * t255;
            let tv2rho20 = f64x8::splat(2.0) * t115 * t137 - f64x8::splat(2.0) * t117 * t232 + t117 * t256 - t225 * t80 - f64x8::splat(2.0) * t168 + f64x8::splat(2.0) * t171;
            acc_v2rho2_0 = tv2rho20;
            let t258 = t158 * t80;
            let t259 = t174 * t139;
            let t262 = t9 * t1;
            let t266 = ((t6).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t259 * t85 + f64x8::splat(8.0) / f64x8::splat(3.0) * t262 * t179));
            let t267 = t188 * t143;
            let t270 = t14 * t1;
            let t274 = ((t13).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t267 * t89 - f64x8::splat(8.0) / f64x8::splat(3.0) * t270 * t179));
            let t276 = (t266 + t274) * t21;
            let t278 = t276 * t26;
            let t281 = t150 * t99;
            let t284 = t276 * t40;
            let t285 = t284 * t46;
            let t287 = t153 * t107;
            let t290 = t276 * t3;
            let t292 = t148 * t83;
            let t295 = f64x8::splat(0.119086804055547) * t276 + f64x8::splat(0.1539350642220836) * t278 * t35 - f64x8::splat(0.051311688074027875) * t281 - f64x8::splat(0.051311688074027875) * t203 + t207 + f64x8::splat(0.03935503789732168) * t285 - f64x8::splat(0.026236691931547783) * t287 - f64x8::splat(0.026236691931547783) * t211 + t217 + f64x8::splat(0.0008432832609665849) * t290 - f64x8::splat(0.0008432832609665849) * t292 - f64x8::splat(0.0008432832609665849) * t220 + t223;
            let t296 = t2 * t295;
            let t299 = t170 * t165;
            let t301 = t230 * t165;
            let t302 = t301 * t136;
            let t313 = t150 * t133;
            let t316 = t237 + f64x8::splat(0.06684032434590667) * t285 - f64x8::splat(0.04456021623060445) * t287 - f64x8::splat(0.04456021623060445) * t211 + t241 + f64x8::splat(0.048988001486277105) * t290 - f64x8::splat(0.048988001486277105) * t292 - f64x8::splat(0.048988001486277105) * t220 + t245 + f64x8::splat(0.0007875009460671339) * t278 * t76 - f64x8::splat(0.0010500012614228452) * t313 - f64x8::splat(0.0010500012614228452) * t248 + t254;
            let t317 = t119 * t316;
            let tv2rho21 = t115 * t166 - f64x8::splat(2.0) * t117 * t302 + t117 * t317 + t159 * t137 - t296 * t80 - t168 + t171 - t258 + t299;
            acc_v2rho2_1 = tv2rho21;
            let t321 = t139 * t139;
            let t325 = f64x8::splat(2.0) * t83 + f64x8::splat(2.0) * t180;
            let t329 = ((t6).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t174 * t321 + f64x8::splat(4.0) / f64x8::splat(3.0) * t9 * t325));
            let t330 = t143 * t143;
            let t333 = -t325;
            let t337 = ((t13).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t188 * t330 + f64x8::splat(4.0) / f64x8::splat(3.0) * t14 * t333));
            let t339 = (t329 + t337) * t21;
            let t341 = t339 * t26;
            let t345 = t339 * t40;
            let t346 = t345 * t46;
            let t349 = t339 * t3;
            let t352 = f64x8::splat(0.119086804055547) * t339 + f64x8::splat(0.1539350642220836) * t341 * t35 - f64x8::splat(0.10262337614805575) * t281 + t207 + f64x8::splat(0.03935503789732168) * t346 - f64x8::splat(0.052473383863095566) * t287 + t217 + f64x8::splat(0.0008432832609665849) * t349 - f64x8::splat(0.0016865665219331699) * t292 + t223;
            let t353 = t2 * t352;
            let t357 = t165 * t165;
            let t358 = t230 * t357;
            let t368 = t237 + f64x8::splat(0.06684032434590667) * t346 - f64x8::splat(0.0891204324612089) * t287 + t241 + f64x8::splat(0.048988001486277105) * t349 - f64x8::splat(0.09797600297255421) * t292 + t245 + f64x8::splat(0.0007875009460671339) * t341 * t76 - f64x8::splat(0.0021000025228456905) * t313 + t254;
            let t369 = t119 * t368;
            let tv2rho22 = -f64x8::splat(2.0) * t117 * t358 + t117 * t369 + f64x8::splat(2.0) * t159 * t166 - t353 * t80 - f64x8::splat(2.0) * t258 + f64x8::splat(2.0) * t299;
            acc_v2rho2_2 = tv2rho22;
            let t371 = t224 * t80;
            let t373 = t114 * t119;
            let t374 = t373 * t136;
            let t376 = t54 * t230;
            let t377 = t376 * t231;
            let t379 = t170 * t255;
            let t382 = f64x8::splat(1.0) / t173 / t5;
            let t383 = t175 * t85;
            let t386 = t174 * t85;
            let t389 = t82 * t82;
            let t390 = f64x8::splat(1.0) / t389;
            let t391 = t1 * t390;
            let t393 = f64x8::splat(6.0) * t179 - f64x8::splat(6.0) * t391;
            let t397 = ((t6).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t382 * t383 + f64x8::splat(4.0) / f64x8::splat(3.0) * t386 * t182 + f64x8::splat(4.0) / f64x8::splat(3.0) * t9 * t393));
            let t399 = f64x8::splat(1.0) / t187 / t12;
            let t400 = t189 * t89;
            let t403 = t188 * t89;
            let t406 = -t393;
            let t410 = ((t13).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t399 * t400 + f64x8::splat(4.0) / f64x8::splat(3.0) * t403 * t192 + f64x8::splat(4.0) / f64x8::splat(3.0) * t14 * t406));
            let t412 = (t397 + t410) * t21;
            let t414 = t412 * t26;
            let t417 = t200 * t99;
            let t419 = t96 * t205;
            let t421 = t32 * t251;
            let t423 = f64x8::splat(7.0) / f64x8::splat(27.0) * t27 * t421;
            let t424 = t412 * t40;
            let t425 = t424 * t46;
            let t427 = t208 * t107;
            let t429 = t102 * t215;
            let t433 = t43 / t44 / t178;
            let t435 = f64x8::splat(20.0) / f64x8::splat(27.0) * t41 * t433;
            let t436 = t412 * t3;
            let t438 = t198 * t83;
            let t440 = t94 * t179;
            let t443 = f64x8::splat(9.0) / f64x8::splat(2.0) * t51 * t390;
            let t444 = f64x8::splat(0.119086804055547) * t412 + f64x8::splat(0.1539350642220836) * t414 * t35 - f64x8::splat(0.1539350642220836) * t417 + f64x8::splat(0.2052467522961115) * t419 - t423 + f64x8::splat(0.03935503789732168) * t425 - f64x8::splat(0.07871007579464336) * t427 + f64x8::splat(0.13118345965773892) * t429 - t435 + f64x8::splat(0.0008432832609665849) * t436 - f64x8::splat(0.002529849782899755) * t438 + f64x8::splat(0.00505969956579951) * t440 - t443;
            let t445 = t2 * t444;
            let t453 = t118 * t118;
            let t454 = f64x8::splat(1.0) / t453;
            let t455 = t231 * t136;
            let t456 = t454 * t455;
            let t459 = t230 * t136;
            let t460 = t459 * t255;
            let t465 = f64x8::splat(0.25925925925925924) * t55 * t31 * t251;
            let t470 = f64x8::splat(20.0) / f64x8::splat(27.0) * t61 * t433;
            let t475 = f64x8::splat(9.0) / f64x8::splat(2.0) * t66 * t390;
            let t478 = t200 * t133;
            let t480 = t96 * t252;
            let t483 = f64x8::splat(1.0) / t33 / t389;
            let t484 = t73 * t483;
            let t486 = f64x8::splat(35.0) / f64x8::splat(18.0) * t71 * t484;
            let t487 = -t465 + f64x8::splat(0.06684032434590667) * t425 - f64x8::splat(0.13368064869181334) * t427 + f64x8::splat(0.22280108115302225) * t429 - t470 + f64x8::splat(0.048988001486277105) * t436 - f64x8::splat(0.14696400445883132) * t438 + f64x8::splat(0.29392800891766263) * t440 - t475 + f64x8::splat(0.0007875009460671339) * t414 * t76 - f64x8::splat(0.0031500037842685357) * t478 + f64x8::splat(0.007350008829959917) * t480 - t486;
            let t488 = t119 * t487;
            let tv3rho30 = -f64x8::splat(6.0) * t115 * t232 + f64x8::splat(3.0) * t115 * t256 + f64x8::splat(6.0) * t117 * t456 - f64x8::splat(6.0) * t117 * t460 + t117 * t488 + f64x8::splat(3.0) * t225 * t137 - t445 * t80 - f64x8::splat(3.0) * t371 + f64x8::splat(6.0) * t374 - f64x8::splat(6.0) * t377 + f64x8::splat(3.0) * t379;
            acc_v3rho3_0 = tv3rho30;
            let t494 = t165 * t136;
            let t496 = f64x8::splat(4.0) * t376 * t494;
            let t502 = t276 * t83;
            let t503 = f64x8::splat(0.09797600297255421) * t502;
            let t504 = t148 * t179;
            let t510 = t382 * t139;
            let t513 = t174 * t1;
            let t524 = ((t6).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t510 * t175 + f64x8::splat(16.0) / f64x8::splat(9.0) * t513 * t179 * t85 + f64x8::splat(4.0) / f64x8::splat(9.0) * t259 * t182 + f64x8::splat(8.0) / f64x8::splat(3.0) * t9 * t179 - f64x8::splat(8.0) * t262 * t390));
            let t525 = t399 * t143;
            let t528 = t188 * t1;
            let t539 = ((t13).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t525 * t189 - f64x8::splat(16.0) / f64x8::splat(9.0) * t528 * t179 * t89 + f64x8::splat(4.0) / f64x8::splat(9.0) * t267 * t192 - f64x8::splat(8.0) / f64x8::splat(3.0) * t14 * t179 + f64x8::splat(8.0) * t270 * t390));
            let t541 = (t524 + t539) * t21;
            let t542 = t541 * t40;
            let t543 = t542 * t46;
            let t545 = t284 * t107;
            let t546 = f64x8::splat(0.0891204324612089) * t545;
            let t547 = t153 * t215;
            let t550 = f64x8::splat(0.0021000025228456905) * t278 * t133;
            let t551 = t150 * t252;
            let t553 = t541 * t3;
            let t555 = t541 * t26;
            let t558 = -f64x8::splat(0.048988001486277105) * t438 + f64x8::splat(0.19595200594510842) * t440 - t475 - t503 + f64x8::splat(0.09797600297255421) * t504 - f64x8::splat(0.04456021623060445) * t427 + f64x8::splat(0.14853405410201484) * t429 - f64x8::splat(0.0010500012614228452) * t478 + f64x8::splat(0.0049000058866399444) * t480 - t465 - t470 - t486 + f64x8::splat(0.06684032434590667) * t543 - t546 + f64x8::splat(0.07426702705100742) * t547 - t550 + f64x8::splat(0.0024500029433199722) * t551 + f64x8::splat(0.048988001486277105) * t553 + f64x8::splat(0.0007875009460671339) * t555 * t76;
            let t559 = t119 * t558;
            let t566 = t230 * t316;
            let t567 = t566 * t136;
            let t570 = t301 * t255;
            let t574 = t454 * t165 * t231;
            let t578 = f64x8::splat(2.0) * t295 * t80;
            let t579 = t158 * t119;
            let t581 = f64x8::splat(2.0) * t579 * t136;
            let t584 = f64x8::splat(0.0016865665219331699) * t502;
            let t594 = f64x8::splat(0.10262337614805575) * t278 * t99;
            let t595 = t150 * t205;
            let t597 = f64x8::splat(0.052473383863095566) * t545;
            let t601 = -f64x8::splat(0.0008432832609665849) * t438 + f64x8::splat(0.0033731330438663398) * t440 - t443 - t584 + f64x8::splat(0.0016865665219331699) * t504 - f64x8::splat(0.051311688074027875) * t417 + f64x8::splat(0.13683116819740768) * t419 - f64x8::splat(0.026236691931547783) * t427 + f64x8::splat(0.08745563977182594) * t429 - t423 - t435 + f64x8::splat(0.1539350642220836) * t555 * t35 + f64x8::splat(0.03935503789732168) * t543 - t594 + f64x8::splat(0.06841558409870384) * t595 - t597 + f64x8::splat(0.04372781988591297) * t547 + f64x8::splat(0.119086804055547) * t541 + f64x8::splat(0.0008432832609665849) * t553;
            let t602 = t2 * t601;
            let t605 = f64x8::splat(2.0) * t373 * t165;
            let t607 = f64x8::splat(2.0) * t170 * t316;
            let tv3rho31 = -f64x8::splat(4.0) * t115 * t302 + f64x8::splat(2.0) * t115 * t317 + t117 * t559 - f64x8::splat(4.0) * t117 * t567 - f64x8::splat(2.0) * t117 * t570 + f64x8::splat(6.0) * t117 * t574 + f64x8::splat(2.0) * t296 * t137 - f64x8::splat(2.0) * t159 * t232 + t159 * t256 + t225 * t166 - t602 * t80 - t371 + f64x8::splat(2.0) * t374 - f64x8::splat(2.0) * t377 + t379 - t496 - t578 + t581 + t605 + t607;
            acc_v3rho3_1 = tv3rho31;
            let t608 = t352 * t80;
            let t609 = t382 * t321;
            let t614 = t174 * t325;
            let t619 = -f64x8::splat(2.0) * t179 - f64x8::splat(6.0) * t391;
            let t623 = ((t6).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t609 * t85 + f64x8::splat(16.0) / f64x8::splat(9.0) * t259 * t180 + f64x8::splat(4.0) / f64x8::splat(9.0) * t614 * t85 + f64x8::splat(4.0) / f64x8::splat(3.0) * t9 * t619));
            let t624 = t399 * t330;
            let t629 = t188 * t333;
            let t632 = -t619;
            let t636 = ((t13).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t624 * t89 - f64x8::splat(16.0) / f64x8::splat(9.0) * t267 * t180 + f64x8::splat(4.0) / f64x8::splat(9.0) * t629 * t89 + f64x8::splat(4.0) / f64x8::splat(3.0) * t14 * t632));
            let t638 = (t623 + t636) * t21;
            let t640 = t638 * t26;
            let t643 = t341 * t99;
            let t647 = t638 * t40;
            let t648 = t647 * t46;
            let t650 = t345 * t107;
            let t654 = t638 * t3;
            let t656 = t339 * t83;
            let t660 = f64x8::splat(0.119086804055547) * t638 + f64x8::splat(0.1539350642220836) * t640 * t35 - f64x8::splat(0.051311688074027875) * t643 - t594 + f64x8::splat(0.13683116819740768) * t595 + f64x8::splat(0.06841558409870384) * t419 - t423 + f64x8::splat(0.03935503789732168) * t648 - f64x8::splat(0.026236691931547783) * t650 - t597 + f64x8::splat(0.08745563977182594) * t547 + f64x8::splat(0.04372781988591297) * t429 - t435 + f64x8::splat(0.0008432832609665849) * t654 - f64x8::splat(0.0008432832609665849) * t656 - t584 + f64x8::splat(0.0033731330438663398) * t504 + f64x8::splat(0.0016865665219331699) * t440 - t443;
            let t661 = t2 * t660;
            let t664 = t579 * t165;
            let t672 = t376 * t357;
            let t676 = t454 * t357;
            let t677 = t676 * t136;
            let t680 = t301 * t316;
            let t683 = t170 * t368;
            let t685 = t230 * t368;
            let t686 = t685 * t136;
            let t699 = t341 * t133;
            let t703 = -t465 + f64x8::splat(0.06684032434590667) * t648 - f64x8::splat(0.04456021623060445) * t650 - t546 + f64x8::splat(0.14853405410201484) * t547 + f64x8::splat(0.07426702705100742) * t429 - t470 + f64x8::splat(0.048988001486277105) * t654 - f64x8::splat(0.048988001486277105) * t656 - t503 + f64x8::splat(0.19595200594510842) * t504 + f64x8::splat(0.09797600297255421) * t440 - t475 + f64x8::splat(0.0007875009460671339) * t640 * t76 - f64x8::splat(0.0010500012614228452) * t699 - t550 + f64x8::splat(0.0049000058866399444) * t551 + f64x8::splat(0.0024500029433199722) * t480 - t486;
            let t704 = t119 * t703;
            let tv3rho32 = -f64x8::splat(2.0) * t115 * t358 + t115 * t369 + f64x8::splat(6.0) * t117 * t677 - f64x8::splat(4.0) * t117 * t680 - f64x8::splat(2.0) * t117 * t686 + t117 * t704 + t353 * t137 - f64x8::splat(4.0) * t159 * t302 + f64x8::splat(2.0) * t159 * t317 + f64x8::splat(2.0) * t296 * t166 - t661 * t80 - t496 - t578 + t581 + t605 + t607 - t608 + f64x8::splat(2.0) * t664 - f64x8::splat(2.0) * t672 + t683;
            acc_v3rho3_2 = tv3rho32;
            let t710 = t321 * t139;
            let t716 = -f64x8::splat(6.0) * t179 - f64x8::splat(6.0) * t391;
            let t720 = ((t6).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t382 * t710 + f64x8::splat(4.0) / f64x8::splat(3.0) * t259 * t325 + f64x8::splat(4.0) / f64x8::splat(3.0) * t9 * t716));
            let t721 = t330 * t143;
            let t726 = -t716;
            let t730 = ((t13).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t399 * t721 + f64x8::splat(4.0) / f64x8::splat(3.0) * t267 * t333 + f64x8::splat(4.0) / f64x8::splat(3.0) * t14 * t726));
            let t732 = (t720 + t730) * t21;
            let t734 = t732 * t26;
            let t739 = t732 * t40;
            let t740 = t739 * t46;
            let t744 = t732 * t3;
            let t748 = f64x8::splat(0.119086804055547) * t732 + f64x8::splat(0.1539350642220836) * t734 * t35 - f64x8::splat(0.1539350642220836) * t643 + f64x8::splat(0.2052467522961115) * t595 - t423 + f64x8::splat(0.03935503789732168) * t740 - f64x8::splat(0.07871007579464336) * t650 + f64x8::splat(0.13118345965773892) * t547 - t435 + f64x8::splat(0.0008432832609665849) * t744 - f64x8::splat(0.002529849782899755) * t656 + f64x8::splat(0.00505969956579951) * t504 - t443;
            let t749 = t2 * t748;
            let t757 = t357 * t165;
            let t758 = t454 * t757;
            let t761 = t301 * t368;
            let t774 = -t465 + f64x8::splat(0.06684032434590667) * t740 - f64x8::splat(0.13368064869181334) * t650 + f64x8::splat(0.22280108115302225) * t547 - t470 + f64x8::splat(0.048988001486277105) * t744 - f64x8::splat(0.14696400445883132) * t656 + f64x8::splat(0.29392800891766263) * t504 - t475 + f64x8::splat(0.0007875009460671339) * t734 * t76 - f64x8::splat(0.0031500037842685357) * t699 + f64x8::splat(0.007350008829959917) * t551 - t486;
            let t775 = t119 * t774;
            let tv3rho33 = f64x8::splat(6.0) * t117 * t758 - f64x8::splat(6.0) * t117 * t761 + t117 * t775 - f64x8::splat(6.0) * t159 * t358 + f64x8::splat(3.0) * t159 * t369 + f64x8::splat(3.0) * t353 * t166 - t749 * t80 - f64x8::splat(3.0) * t608 + f64x8::splat(6.0) * t664 - f64x8::splat(6.0) * t672 + f64x8::splat(3.0) * t683;
            acc_v3rho3_3 = tv3rho33;
            let t777 = t114 * t230;
            let t778 = t777 * t231;
            let t780 = t54 * t454;
            let t781 = t780 * t455;
            let t784 = t376 * t136 * t255;
            let t792 = t412 * t83;
            let t794 = t198 * t179;
            let t796 = t94 * t390;
            let t798 = t389 * t2;
            let t799 = f64x8::splat(1.0) / t798;
            let t801 = f64x8::splat(18.0) * t66 * t799;
            let t804 = f64x8::splat(0.8641975308641975) * t55 * t31 * t483;
            let t805 = t5 * t5;
            let t807 = f64x8::splat(1.0) / t173 / t805;
            let t808 = t175 * t175;
            let t814 = t182 * t182;
            let t819 = t1 * t799;
            let t821 = -f64x8::splat(24.0) * t390 + f64x8::splat(24.0) * t819;
            let t825 = ((t6).select(f64x8::splat(0.0), f64x8::splat(40.0) / f64x8::splat(81.0) * t807 * t808 - f64x8::splat(16.0) / f64x8::splat(9.0) * t382 * t175 * t182 + f64x8::splat(4.0) / f64x8::splat(3.0) * t174 * t814 + f64x8::splat(16.0) / f64x8::splat(9.0) * t386 * t393 + f64x8::splat(4.0) / f64x8::splat(3.0) * t9 * t821));
            let t826 = t12 * t12;
            let t828 = f64x8::splat(1.0) / t187 / t826;
            let t829 = t189 * t189;
            let t835 = t192 * t192;
            let t844 = ((t13).select(f64x8::splat(0.0), f64x8::splat(40.0) / f64x8::splat(81.0) * t828 * t829 - f64x8::splat(16.0) / f64x8::splat(9.0) * t399 * t189 * t192 + f64x8::splat(4.0) / f64x8::splat(3.0) * t188 * t835 + f64x8::splat(16.0) / f64x8::splat(9.0) * t403 * t406 - f64x8::splat(4.0) / f64x8::splat(3.0) * t14 * t821));
            let t846 = (t825 + t844) * t21;
            let t847 = t846 * t3;
            let t849 = t424 * t107;
            let t851 = t208 * t215;
            let t853 = t102 * t433;
            let t856 = t846 * t40 * t46;
            let t860 = t43 / t44 / t389;
            let t862 = f64x8::splat(220.0) / f64x8::splat(81.0) * t61 * t860;
            let t863 = t846 * t26;
            let t870 = f64x8::splat(455.0) / f64x8::splat(54.0) * t71 * t73 / t33 / t798;
            let t871 = t414 * t133;
            let t873 = t200 * t252;
            let t875 = t96 * t484;
            let t877 = -f64x8::splat(0.19595200594510842) * t792 + f64x8::splat(0.5878560178353253) * t794 - f64x8::splat(1.1757120356706505) * t796 + t801 + t804 + f64x8::splat(0.048988001486277105) * t847 - f64x8::splat(0.1782408649224178) * t849 + f64x8::splat(0.4456021623060445) * t851 - f64x8::splat(0.7921816218774125) * t853 + f64x8::splat(0.06684032434590667) * t856 + t862 + f64x8::splat(0.0007875009460671339) * t863 * t76 + t870 - f64x8::splat(0.004200005045691381) * t871 + f64x8::splat(0.014700017659919833) * t873 - f64x8::splat(0.032666705910932965) * t875;
            let t887 = f64x8::splat(1.0) / t453 / t79;
            let t888 = t231 * t231;
            let t896 = t255 * t255;
            let t903 = t224 * t119;
            let t904 = t903 * t136;
            let t906 = t373 * t255;
            let t908 = t170 * t487;
            let t914 = f64x8::splat(18.0) * t51 * t799;
            let t916 = t414 * t99;
            let t918 = t200 * t205;
            let t920 = t96 * t421;
            let t930 = f64x8::splat(70.0) / f64x8::splat(81.0) * t27 * t32 * t483;
            let t933 = f64x8::splat(220.0) / f64x8::splat(81.0) * t41 * t860;
            let t934 = -f64x8::splat(0.0033731330438663398) * t792 + f64x8::splat(0.01011939913159902) * t794 - f64x8::splat(0.02023879826319804) * t796 + t914 + f64x8::splat(0.0008432832609665849) * t847 - f64x8::splat(0.2052467522961115) * t916 + f64x8::splat(0.410493504592223) * t918 - f64x8::splat(0.6385454515879024) * t920 - f64x8::splat(0.10494676772619113) * t849 + f64x8::splat(0.26236691931547784) * t851 - f64x8::splat(0.4664300787830717) * t853 + f64x8::splat(0.119086804055547) * t846 + f64x8::splat(0.1539350642220836) * t863 * t35 + t930 + f64x8::splat(0.03935503789732168) * t856 + t933;
            let t937 = t444 * t80;
            let tv4rho40 = -f64x8::splat(24.0) * t778 + f64x8::splat(24.0) * t781 - f64x8::splat(24.0) * t784 + f64x8::splat(4.0) * t445 * t137 + f64x8::splat(6.0) * t225 * t256 + f64x8::splat(4.0) * t115 * t488 + t117 * t119 * t877 - f64x8::splat(12.0) * t225 * t232 + f64x8::splat(24.0) * t115 * t456 - f64x8::splat(24.0) * t115 * t460 - f64x8::splat(24.0) * t117 * t887 * t888 + f64x8::splat(36.0) * t117 * t454 * t231 * t255 - f64x8::splat(6.0) * t117 * t230 * t896 - f64x8::splat(8.0) * t117 * t459 * t487 + f64x8::splat(12.0) * t904 + f64x8::splat(12.0) * t906 + f64x8::splat(4.0) * t908 - t2 * t934 * t80 - f64x8::splat(4.0) * t937;
            acc_v4rho4_0 = tv4rho40;
            let t979 = f64x8::splat(32.0) * t262 * t799;
            let t981 = ((t6).select(f64x8::splat(0.0), f64x8::splat(40.0) / f64x8::splat(81.0) * t807 * t139 * t383 - f64x8::splat(16.0) / f64x8::splat(9.0) * t382 * t1 * t179 * t175 - f64x8::splat(8.0) / f64x8::splat(9.0) * t510 * t85 * t182 + f64x8::splat(8.0) / f64x8::splat(3.0) * t174 * t179 * t85 - f64x8::splat(8.0) * t513 * t390 * t85 + f64x8::splat(8.0) / f64x8::splat(3.0) * t513 * t179 * t182 + f64x8::splat(4.0) / f64x8::splat(9.0) * t259 * t393 - f64x8::splat(16.0) * t9 * t390 + t979));
            let t1006 = f64x8::splat(32.0) * t270 * t799;
            let t1008 = ((t13).select(f64x8::splat(0.0), f64x8::splat(40.0) / f64x8::splat(81.0) * t828 * t143 * t400 + f64x8::splat(16.0) / f64x8::splat(9.0) * t399 * t1 * t179 * t189 - f64x8::splat(8.0) / f64x8::splat(9.0) * t525 * t89 * t192 - f64x8::splat(8.0) / f64x8::splat(3.0) * t188 * t179 * t89 + f64x8::splat(8.0) * t528 * t390 * t89 - f64x8::splat(8.0) / f64x8::splat(3.0) * t528 * t179 * t192 + f64x8::splat(4.0) / f64x8::splat(9.0) * t267 * t406 + f64x8::splat(16.0) * t14 * t390 - t1006));
            let t1010 = (t981 + t1008) * t21;
            let t1011 = t1010 * t3;
            let t1013 = t276 * t179;
            let t1014 = f64x8::splat(0.29392800891766263) * t1013;
            let t1015 = t148 * t390;
            let t1017 = t541 * t83;
            let t1023 = t1010 * t40 * t46;
            let t1025 = t542 * t107;
            let t1027 = t284 * t215;
            let t1028 = f64x8::splat(0.22280108115302225) * t1027;
            let t1029 = t153 * t433;
            let t1031 = f64x8::splat(0.048988001486277105) * t1011 + t1014 - f64x8::splat(0.29392800891766263) * t1015 - f64x8::splat(0.14696400445883132) * t1017 - f64x8::splat(0.048988001486277105) * t792 + f64x8::splat(0.29392800891766263) * t794 - f64x8::splat(0.8817840267529878) * t796 + t801 + f64x8::splat(0.06684032434590667) * t1023 - f64x8::splat(0.13368064869181334) * t1025 + t1028 - f64x8::splat(0.19804540546935312) * t1029;
            let t1038 = t555 * t133;
            let t1040 = t1010 * t26;
            let t1043 = t278 * t252;
            let t1044 = f64x8::splat(0.007350008829959917) * t1043;
            let t1045 = t150 * t484;
            let t1047 = -f64x8::splat(0.04456021623060445) * t849 + f64x8::splat(0.22280108115302225) * t851 - f64x8::splat(0.5941362164080594) * t853 - f64x8::splat(0.0010500012614228452) * t871 + f64x8::splat(0.007350008829959917) * t873 - f64x8::splat(0.024500029433199722) * t875 + t862 + t870 + t804 - f64x8::splat(0.0031500037842685357) * t1038 + f64x8::splat(0.0007875009460671339) * t1040 * t76 + t1044 - f64x8::splat(0.008166676477733241) * t1045;
            let t1057 = t777 * t494;
            let t1060 = t376 * t316 * t136;
            let t1062 = t165 * t255;
            let t1063 = t376 * t1062;
            let t1074 = t780 * t165 * t231;
            let t1076 = -f64x8::splat(24.0) * t117 * t887 * t165 * t455 - f64x8::splat(6.0) * t117 * t566 * t255 - f64x8::splat(2.0) * t117 * t301 * t487 + f64x8::splat(18.0) * t115 * t574 + f64x8::splat(18.0) * t117 * t454 * t316 * t231 + t117 * t119 * (t1031 + t1047) + t445 * t166 + f64x8::splat(3.0) * t225 * t317 + f64x8::splat(3.0) * t115 * t559 + t159 * t488 - f64x8::splat(12.0) * t1057 - f64x8::splat(12.0) * t1060 - f64x8::splat(6.0) * t1063 + f64x8::splat(3.0) * t602 * t137 + f64x8::splat(3.0) * t296 * t256 + f64x8::splat(6.0) * t159 * t456 - f64x8::splat(6.0) * t296 * t232 + f64x8::splat(18.0) * t1074 - t937;
            let t1077 = t579 * t255;
            let t1079 = t295 * t119;
            let t1080 = t1079 * t136;
            let t1084 = f64x8::splat(0.00505969956579951) * t1013;
            let t1090 = t555 * t99;
            let t1092 = t278 * t205;
            let t1093 = f64x8::splat(0.2052467522961115) * t1092;
            let t1094 = t150 * t421;
            let t1096 = f64x8::splat(0.119086804055547) * t1010 + f64x8::splat(0.0008432832609665849) * t1011 + t1084 - f64x8::splat(0.00505969956579951) * t1015 - f64x8::splat(0.002529849782899755) * t1017 + t914 - f64x8::splat(0.0008432832609665849) * t792 + f64x8::splat(0.00505969956579951) * t794 - f64x8::splat(0.01517909869739853) * t796 - f64x8::splat(0.1539350642220836) * t1090 + t1093 - f64x8::splat(0.1596363628969756) * t1094;
            let t1099 = f64x8::splat(0.13118345965773892) * t1027;
            let t1109 = f64x8::splat(0.03935503789732168) * t1023 - f64x8::splat(0.07871007579464336) * t1025 + t1099 - f64x8::splat(0.11660751969576792) * t1029 + f64x8::splat(0.1539350642220836) * t1040 * t35 - f64x8::splat(0.051311688074027875) * t916 + f64x8::splat(0.2052467522961115) * t918 - f64x8::splat(0.47890908869092685) * t920 - f64x8::splat(0.026236691931547783) * t849 + f64x8::splat(0.13118345965773892) * t851 - f64x8::splat(0.34982255908730375) * t853 + t930 + t933;
            let t1113 = t158 * t230;
            let t1114 = t1113 * t231;
            let t1118 = t117 * t454;
            let t1137 = t601 * t80;
            let t1139 = t903 * t165;
            let t1141 = t373 * t316;
            let t1143 = t170 * t558;
            let t1145 = f64x8::splat(3.0) * t1077 + f64x8::splat(6.0) * t1080 - t2 * (t1096 + t1109) * t80 - f64x8::splat(6.0) * t1114 + f64x8::splat(3.0) * t904 + f64x8::splat(3.0) * t906 + t908 + f64x8::splat(18.0) * t1118 * t1062 * t136 - f64x8::splat(6.0) * t778 + f64x8::splat(6.0) * t781 - f64x8::splat(12.0) * t115 * t567 - f64x8::splat(6.0) * t117 * t230 * t558 * t136 - f64x8::splat(6.0) * t115 * t570 - f64x8::splat(6.0) * t225 * t302 - f64x8::splat(6.0) * t159 * t460 - f64x8::splat(6.0) * t784 - f64x8::splat(3.0) * t1137 + f64x8::splat(3.0) * t1139 + f64x8::splat(6.0) * t1141 + f64x8::splat(3.0) * t1143;
            let tv4rho41 = t1076 + t1145;
            acc_v4rho4_1 = tv4rho41;
            let t1177 = -f64x8::splat(4.0) * t117 * t230 * t703 * t136 + f64x8::splat(6.0) * t117 * t454 * t368 * t231 + f64x8::splat(6.0) * t117 * t676 * t255 - f64x8::splat(2.0) * t117 * t685 * t255 - f64x8::splat(4.0) * t117 * t301 * t558 + f64x8::splat(12.0) * t115 * t677 - f64x8::splat(8.0) * t115 * t680 - f64x8::splat(4.0) * t115 * t686 - f64x8::splat(8.0) * t159 * t567 - f64x8::splat(4.0) * t159 * t570 + f64x8::splat(12.0) * t159 * t574 - f64x8::splat(8.0) * t296 * t302;
            let t1183 = t376 * t165 * t316;
            let t1188 = t376 * t368 * t136;
            let t1196 = t780 * t357 * t136;
            let t1203 = t1113 * t494;
            let t1205 = -f64x8::splat(24.0) * t117 * t887 * t357 * t231 + f64x8::splat(2.0) * t115 * t704 + f64x8::splat(2.0) * t661 * t137 + f64x8::splat(2.0) * t159 * t559 - f64x8::splat(2.0) * t225 * t358 + t225 * t369 + t353 * t256 + f64x8::splat(4.0) * t296 * t317 - f64x8::splat(8.0) * t1183 - f64x8::splat(4.0) * t1188 + f64x8::splat(12.0) * t1196 - f64x8::splat(8.0) * t1203;
            let t1211 = t316 * t316;
            let t1235 = f64x8::splat(0.0067462660877326795) * t1013 - f64x8::splat(0.01011939913159902) * t1015 - f64x8::splat(0.0016865665219331699) * t1017 + t914 + f64x8::splat(0.0016865665219331699) * t794 - f64x8::splat(0.01011939913159902) * t796 - f64x8::splat(0.10262337614805575) * t1090 + f64x8::splat(0.27366233639481535) * t1092 - f64x8::splat(0.3192727257939512) * t1094 - f64x8::splat(0.052473383863095566) * t1025 + f64x8::splat(0.17491127954365188) * t1027 - f64x8::splat(0.23321503939153584) * t1029 + f64x8::splat(0.06841558409870384) * t918 - f64x8::splat(0.3192727257939512) * t920;
            let t1238 = t638 * t83;
            let t1240 = t339 * t179;
            let t1251 = t1 * t1;
            let t1254 = f64x8::splat(1.0) / t389 / t82;
            let t1270 = ((t6).select(f64x8::splat(0.0), f64x8::splat(40.0) / f64x8::splat(81.0) * t807 * t321 * t175 - f64x8::splat(64.0) / f64x8::splat(27.0) * t510 * t85 * t1 * t179 - f64x8::splat(8.0) / f64x8::splat(27.0) * t609 * t182 + f64x8::splat(32.0) / f64x8::splat(9.0) * t174 * t1251 * t1254 + f64x8::splat(16.0) / f64x8::splat(9.0) * t259 * t179 - f64x8::splat(16.0) / f64x8::splat(3.0) * t259 * t391 - f64x8::splat(8.0) / f64x8::splat(27.0) * t382 * t325 * t175 + f64x8::splat(8.0) / f64x8::splat(9.0) * t174 * t619 * t85 + f64x8::splat(4.0) / f64x8::splat(9.0) * t614 * t182 + t979));
            let t1296 = ((t13).select(f64x8::splat(0.0), f64x8::splat(40.0) / f64x8::splat(81.0) * t828 * t330 * t189 + f64x8::splat(64.0) / f64x8::splat(27.0) * t525 * t89 * t1 * t179 - f64x8::splat(8.0) / f64x8::splat(27.0) * t624 * t192 + f64x8::splat(32.0) / f64x8::splat(9.0) * t188 * t1251 * t1254 - f64x8::splat(16.0) / f64x8::splat(9.0) * t267 * t179 + f64x8::splat(16.0) / f64x8::splat(3.0) * t267 * t391 - f64x8::splat(8.0) / f64x8::splat(27.0) * t399 * t333 * t189 + f64x8::splat(8.0) / f64x8::splat(9.0) * t188 * t632 * t89 + f64x8::splat(4.0) / f64x8::splat(9.0) * t629 * t192 - t1006));
            let t1298 = (t1270 + t1296) * t21;
            let t1299 = t1298 * t3;
            let t1301 = t1298 * t26;
            let t1305 = t1298 * t40 * t46;
            let t1307 = t647 * t107;
            let t1309 = t345 * t215;
            let t1311 = t640 * t99;
            let t1313 = t341 * t205;
            let t1316 = f64x8::splat(0.04372781988591297) * t851 - f64x8::splat(0.23321503939153584) * t853 + t930 + t933 - f64x8::splat(0.0016865665219331699) * t1238 + f64x8::splat(0.0016865665219331699) * t1240 + f64x8::splat(0.0008432832609665849) * t1299 + f64x8::splat(0.1539350642220836) * t1301 * t35 + f64x8::splat(0.03935503789732168) * t1305 - f64x8::splat(0.052473383863095566) * t1307 + f64x8::splat(0.04372781988591297) * t1309 - f64x8::splat(0.10262337614805575) * t1311 + f64x8::splat(0.06841558409870384) * t1313 + f64x8::splat(0.119086804055547) * t1298;
            let t1320 = t1079 * t165;
            let t1322 = f64x8::splat(2.0) * t602 * t166 - f64x8::splat(2.0) * t353 * t232 - f64x8::splat(4.0) * t117 * t230 * t1211 - f64x8::splat(8.0) * t1057 - f64x8::splat(8.0) * t1060 - f64x8::splat(4.0) * t1063 + f64x8::splat(12.0) * t1074 + f64x8::splat(2.0) * t1077 + f64x8::splat(4.0) * t1080 - f64x8::splat(4.0) * t1114 - t2 * (t1235 + t1316) * t80 + f64x8::splat(4.0) * t1320;
            let t1323 = t579 * t316;
            let t1325 = t777 * t357;
            let t1327 = t660 * t80;
            let t1341 = f64x8::splat(0.39190401189021684) * t1013 - f64x8::splat(0.5878560178353253) * t1015 - f64x8::splat(0.09797600297255421) * t1017 + f64x8::splat(0.09797600297255421) * t794 - f64x8::splat(0.5878560178353253) * t796 + t801 - f64x8::splat(0.0891204324612089) * t1025 + f64x8::splat(0.2970681082040297) * t1027 - f64x8::splat(0.39609081093870624) * t1029 + f64x8::splat(0.07426702705100742) * t851 - f64x8::splat(0.39609081093870624) * t853 + f64x8::splat(0.0024500029433199722) * t873 - f64x8::splat(0.016333352955466483) * t875 + t862;
            let t1351 = t640 * t133;
            let t1353 = t341 * t252;
            let t1357 = t870 + t804 - f64x8::splat(0.09797600297255421) * t1238 + f64x8::splat(0.09797600297255421) * t1240 + f64x8::splat(0.048988001486277105) * t1299 - f64x8::splat(0.0021000025228456905) * t1038 + f64x8::splat(0.009800011773279889) * t1043 - f64x8::splat(0.016333352955466483) * t1045 + f64x8::splat(0.06684032434590667) * t1305 - f64x8::splat(0.0891204324612089) * t1307 + f64x8::splat(0.07426702705100742) * t1309 - f64x8::splat(0.0021000025228456905) * t1351 + f64x8::splat(0.0024500029433199722) * t1353 + f64x8::splat(0.0007875009460671339) * t1301 * t76;
            let t1361 = t373 * t368;
            let t1363 = t170 * t703;
            let t1372 = t352 * t119;
            let t1373 = t1372 * t136;
            let t1375 = f64x8::splat(4.0) * t1323 - f64x8::splat(4.0) * t1325 - f64x8::splat(2.0) * t1327 + t117 * t119 * (t1341 + t1357) + f64x8::splat(2.0) * t1361 + f64x8::splat(2.0) * t1363 + f64x8::splat(24.0) * t1118 * t494 * t316 - f64x8::splat(2.0) * t1137 + f64x8::splat(2.0) * t1139 + f64x8::splat(4.0) * t1141 + f64x8::splat(2.0) * t1143 + f64x8::splat(2.0) * t1373;
            let tv4rho42 = t1177 + t1205 + t1322 + t1375;
            acc_v4rho4_2 = tv4rho42;
            let t1381 = t748 * t80;
            let t1382 = t165 * t368;
            let t1405 = t734 * t133;
            let t1407 = t1014 - f64x8::splat(0.8817840267529878) * t1015 - f64x8::splat(0.29392800891766263) * t796 + t801 + t1028 - f64x8::splat(0.5941362164080594) * t1029 - f64x8::splat(0.19804540546935312) * t853 - f64x8::splat(0.008166676477733241) * t875 + t862 + t870 + t804 - f64x8::splat(0.0010500012614228452) * t1405;
            let t1426 = f64x8::splat(12.0) * t390 + f64x8::splat(24.0) * t819;
            let t1430 = ((t6).select(f64x8::splat(0.0), f64x8::splat(40.0) / f64x8::splat(81.0) * t807 * t710 * t85 - f64x8::splat(16.0) / f64x8::splat(9.0) * t609 * t180 - f64x8::splat(8.0) / f64x8::splat(9.0) * t510 * t325 * t85 + f64x8::splat(8.0) / f64x8::splat(3.0) * t513 * t179 * t325 + f64x8::splat(4.0) / f64x8::splat(3.0) * t259 * t619 + f64x8::splat(4.0) / f64x8::splat(9.0) * t174 * t716 * t85 + f64x8::splat(4.0) / f64x8::splat(3.0) * t9 * t1426));
            let t1451 = ((t13).select(f64x8::splat(0.0), f64x8::splat(40.0) / f64x8::splat(81.0) * t828 * t721 * t89 + f64x8::splat(16.0) / f64x8::splat(9.0) * t624 * t180 - f64x8::splat(8.0) / f64x8::splat(9.0) * t525 * t333 * t89 - f64x8::splat(8.0) / f64x8::splat(3.0) * t528 * t179 * t333 + f64x8::splat(4.0) / f64x8::splat(3.0) * t267 * t632 + f64x8::splat(4.0) / f64x8::splat(9.0) * t188 * t726 * t89 - f64x8::splat(4.0) / f64x8::splat(3.0) * t14 * t1426));
            let t1453 = (t1430 + t1451) * t21;
            let t1454 = t1453 * t26;
            let t1458 = t1453 * t40 * t46;
            let t1460 = t739 * t107;
            let t1464 = t732 * t83;
            let t1466 = t1453 * t3;
            let t1473 = f64x8::splat(0.0007875009460671339) * t1454 * t76 + f64x8::splat(0.06684032434590667) * t1458 - f64x8::splat(0.04456021623060445) * t1460 - f64x8::splat(0.14696400445883132) * t1238 + f64x8::splat(0.29392800891766263) * t1240 - f64x8::splat(0.048988001486277105) * t1464 + f64x8::splat(0.048988001486277105) * t1466 + t1044 - f64x8::splat(0.024500029433199722) * t1045 - f64x8::splat(0.13368064869181334) * t1307 + f64x8::splat(0.22280108115302225) * t1309 - f64x8::splat(0.0031500037842685357) * t1351 + f64x8::splat(0.007350008829959917) * t1353;
            let t1485 = -f64x8::splat(12.0) * t1183 - f64x8::splat(6.0) * t1188 + f64x8::splat(18.0) * t1196 - f64x8::splat(12.0) * t1203 - t1381 + f64x8::splat(18.0) * t1118 * t1382 * t136 + f64x8::splat(6.0) * t1320 + f64x8::splat(6.0) * t1323 - f64x8::splat(6.0) * t1325 - f64x8::splat(3.0) * t1327 - f64x8::splat(6.0) * t117 * t301 * t703 - f64x8::splat(2.0) * t117 * t230 * t774 * t136 + f64x8::splat(6.0) * t115 * t758 + t115 * t775 + t117 * t119 * (t1407 + t1473) + f64x8::splat(3.0) * t353 * t317 - f64x8::splat(6.0) * t296 * t358 + f64x8::splat(3.0) * t296 * t369 + f64x8::splat(3.0) * t159 * t704;
            let t1489 = t376 * t1382;
            let t1493 = t780 * t757;
            let t1495 = t170 * t774;
            let t1496 = t1113 * t357;
            let t1498 = t579 * t368;
            let t1500 = t1372 * t165;
            let t1508 = t1084 - f64x8::splat(0.01517909869739853) * t1015 + t914 - f64x8::splat(0.00505969956579951) * t796 + t1093 - f64x8::splat(0.47890908869092685) * t1094 + t1099 - f64x8::splat(0.34982255908730375) * t1029 - f64x8::splat(0.1596363628969756) * t920 - f64x8::splat(0.11660751969576792) * t853 + t930 + t933;
            let t1512 = t734 * t99;
            let t1524 = f64x8::splat(0.119086804055547) * t1453 + f64x8::splat(0.1539350642220836) * t1454 * t35 - f64x8::splat(0.051311688074027875) * t1512 + f64x8::splat(0.03935503789732168) * t1458 - f64x8::splat(0.026236691931547783) * t1460 - f64x8::splat(0.002529849782899755) * t1238 + f64x8::splat(0.00505969956579951) * t1240 - f64x8::splat(0.0008432832609665849) * t1464 + f64x8::splat(0.0008432832609665849) * t1466 - f64x8::splat(0.07871007579464336) * t1307 + f64x8::splat(0.13118345965773892) * t1309 - f64x8::splat(0.1539350642220836) * t1311 + f64x8::splat(0.2052467522961115) * t1313;
            let t1549 = t749 * t137 + f64x8::splat(3.0) * t661 * t166 - f64x8::splat(6.0) * t1489 + f64x8::splat(3.0) * t1361 + f64x8::splat(3.0) * t1363 + f64x8::splat(6.0) * t1493 + t1495 - f64x8::splat(6.0) * t1496 + f64x8::splat(3.0) * t1498 + f64x8::splat(3.0) * t1500 - t2 * (t1508 + t1524) * t80 - f64x8::splat(24.0) * t117 * t887 * t757 * t136 + f64x8::splat(18.0) * t117 * t676 * t316 - f64x8::splat(6.0) * t115 * t761 - f64x8::splat(6.0) * t117 * t566 * t368 - f64x8::splat(12.0) * t159 * t680 - f64x8::splat(6.0) * t159 * t686 - f64x8::splat(6.0) * t353 * t302 + f64x8::splat(18.0) * t159 * t677 + f64x8::splat(3.0) * t1373;
            let tv4rho43 = t1485 + t1549;
            acc_v4rho4_3 = tv4rho43;
            let t1550 = t368 * t368;
            let t1557 = t321 * t321;
            let t1562 = t325 * t325;
            let t1568 = f64x8::splat(24.0) * t390 + f64x8::splat(24.0) * t819;
            let t1572 = ((t6).select(f64x8::splat(0.0), f64x8::splat(40.0) / f64x8::splat(81.0) * t807 * t1557 - f64x8::splat(16.0) / f64x8::splat(9.0) * t609 * t325 + f64x8::splat(4.0) / f64x8::splat(3.0) * t174 * t1562 + f64x8::splat(16.0) / f64x8::splat(9.0) * t259 * t716 + f64x8::splat(4.0) / f64x8::splat(3.0) * t9 * t1568));
            let t1573 = t330 * t330;
            let t1578 = t333 * t333;
            let t1587 = ((t13).select(f64x8::splat(0.0), f64x8::splat(40.0) / f64x8::splat(81.0) * t828 * t1573 - f64x8::splat(16.0) / f64x8::splat(9.0) * t624 * t333 + f64x8::splat(4.0) / f64x8::splat(3.0) * t188 * t1578 + f64x8::splat(16.0) / f64x8::splat(9.0) * t267 * t726 - f64x8::splat(4.0) / f64x8::splat(3.0) * t14 * t1568));
            let t1589 = (t1572 + t1587) * t21;
            let t1590 = t1589 * t3;
            let t1593 = t1589 * t26;
            let t1597 = t1589 * t40 * t46;
            let t1608 = f64x8::splat(0.0008432832609665849) * t1590 + f64x8::splat(0.119086804055547) * t1589 + f64x8::splat(0.1539350642220836) * t1593 * t35 + f64x8::splat(0.03935503789732168) * t1597 - f64x8::splat(0.0033731330438663398) * t1464 - f64x8::splat(0.2052467522961115) * t1512 - f64x8::splat(0.10494676772619113) * t1460 + f64x8::splat(0.26236691931547784) * t1309 + f64x8::splat(0.410493504592223) * t1313 - f64x8::splat(0.6385454515879024) * t1094 + f64x8::splat(0.01011939913159902) * t1240 - f64x8::splat(0.4664300787830717) * t1029 + t914 - f64x8::splat(0.02023879826319804) * t1015 + t930 + t933;
            let t1624 = f64x8::splat(0.0007875009460671339) * t1593 * t76 + f64x8::splat(0.048988001486277105) * t1590 + f64x8::splat(0.06684032434590667) * t1597 - f64x8::splat(0.004200005045691381) * t1405 - f64x8::splat(0.19595200594510842) * t1464 - f64x8::splat(0.1782408649224178) * t1460 + f64x8::splat(0.014700017659919833) * t1353 + f64x8::splat(0.4456021623060445) * t1309 + f64x8::splat(0.5878560178353253) * t1240 - f64x8::splat(0.7921816218774125) * t1029 - f64x8::splat(0.032666705910932965) * t1045 + t801 - f64x8::splat(1.1757120356706505) * t1015 + t804 + t862 + t870;
            let t1638 = t357 * t357;
            let tv4rho44 = t117 * t119 * t1624 - f64x8::splat(6.0) * t117 * t230 * t1550 - f64x8::splat(24.0) * t117 * t887 * t1638 - f64x8::splat(8.0) * t117 * t301 * t774 + f64x8::splat(36.0) * t117 * t676 * t368 - t2 * t1608 * t80 + f64x8::splat(24.0) * t159 * t758 - f64x8::splat(24.0) * t159 * t761 + f64x8::splat(4.0) * t159 * t775 + f64x8::splat(4.0) * t749 * t166 - f64x8::splat(12.0) * t353 * t358 + f64x8::splat(6.0) * t353 * t369 - f64x8::splat(4.0) * t1381 - f64x8::splat(24.0) * t1489 + f64x8::splat(24.0) * t1493 + f64x8::splat(4.0) * t1495 - f64x8::splat(24.0) * t1496 + f64x8::splat(12.0) * t1498 + f64x8::splat(12.0) * t1500;
            acc_v4rho4_4 = tv4rho44;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(v2rho2, ip, m, 3, 0, acc_v2rho2_0);
        store_strided(v2rho2, ip, m, 3, 1, acc_v2rho2_1);
        store_strided(v2rho2, ip, m, 3, 2, acc_v2rho2_2);
        store_strided(v3rho3, ip, m, 4, 0, acc_v3rho3_0);
        store_strided(v3rho3, ip, m, 4, 1, acc_v3rho3_1);
        store_strided(v3rho3, ip, m, 4, 2, acc_v3rho3_2);
        store_strided(v3rho3, ip, m, 4, 3, acc_v3rho3_3);
        store_strided(v4rho4, ip, m, 5, 0, acc_v4rho4_0);
        store_strided(v4rho4, ip, m, 5, 1, acc_v4rho4_1);
        store_strided(v4rho4, ip, m, 5, 2, acc_v4rho4_2);
        store_strided(v4rho4, ip, m, 5, 3, acc_v4rho4_3);
        store_strided(v4rho4, ip, m, 5, 4, acc_v4rho4_4);
        ip += 8;
    }
}

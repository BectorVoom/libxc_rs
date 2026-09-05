//! LDA_C_HL kxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_hl.c`
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
pub fn lda_c_hl_kxc_pol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    v3rho3: &mut [f64],
    param_hl_c_0: f64,
    param_hl_r_0: f64,
    param_hl_c_1: f64,
    param_hl_r_1: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_hl_c_0 = f64x8::splat(param_hl_c_0);
    let param_hl_r_0 = f64x8::splat(param_hl_r_0);
    let param_hl_c_1 = f64x8::splat(param_hl_c_1);
    let param_hl_r_1 = f64x8::splat(param_hl_r_1);
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
        {
            let t1 = param_hl_c_0;
            let t2 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t3 = v_rho0 + v_rho1;
            let t4 = f64x8::splat(1.0) / t3;
            let t5 = t2 * t4;
            let t6 = param_hl_r_0;
            let t7 = t6 * t6;
            let t8 = t7 * t6;
            let t9 = f64x8::splat(1.0) / t8;
            let t12 = f64x8::splat(1.0) + f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t9;
            let t13 = f64x8::splat(M_CBRT3);
            let t14 = t13 * t13;
            let t15 = (simd::cbrt(t2));
            let t16 = f64x8::splat(1.0) / t15;
            let t17 = t14 * t16;
            let t18 = f64x8::splat(M_CBRT4);
            let t19 = (simd::cbrt(t3));
            let t20 = t18 * t19;
            let t24 = f64x8::splat(1.0) + t17 * t20 * t6 / f64x8::splat(3.0);
            let t25 = (simd::ln(t24));
            let t27 = t15 * t15;
            let t28 = t14 * t27;
            let t29 = t19 * t19;
            let t31 = t18 / t29;
            let t32 = f64x8::splat(1.0) / t7;
            let t36 = t13 * t15;
            let t37 = t18 * t18;
            let t39 = t37 / t19;
            let t40 = f64x8::splat(1.0) / t6;
            let t45 = t1 * (t12 * t25 - t28 * t31 * t32 / f64x8::splat(4.0) + t36 * t39 * t40 / f64x8::splat(8.0) - f64x8::splat(1.0) / f64x8::splat(3.0));
            let t46 = v_rho0 - v_rho1;
            let t47 = t46 * t4;
            let t48 = f64x8::splat(1.0) + t47;
            let t49 = (t48).simd_le(zeta_threshold);
            let t50 = (simd::cbrt(zeta_threshold));
            let t51 = t50 * zeta_threshold;
            let t52 = (simd::cbrt(t48));
            let t54 = ((t49).select(t51, t52 * t48));
            let t55 = f64x8::splat(1.0) - t47;
            let t56 = (t55).simd_le(zeta_threshold);
            let t57 = (simd::cbrt(t55));
            let t59 = ((t56).select(t51, t57 * t55));
            let t61 = f64x8::splat(M_CBRT2);
            let t64 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t61 - f64x8::splat(2.0));
            let t65 = (t54 + t59 - f64x8::splat(2.0)) * t64;
            let t66 = param_hl_c_1;
            let t67 = param_hl_r_1;
            let t68 = t67 * t67;
            let t69 = t68 * t67;
            let t70 = f64x8::splat(1.0) / t69;
            let t73 = f64x8::splat(1.0) + f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t70;
            let t77 = f64x8::splat(1.0) + t17 * t20 * t67 / f64x8::splat(3.0);
            let t78 = (simd::ln(t77));
            let t80 = f64x8::splat(1.0) / t68;
            let t84 = f64x8::splat(1.0) / t67;
            let t90 = -t66 * (t73 * t78 - t28 * t31 * t80 / f64x8::splat(4.0) + t36 * t39 * t84 / f64x8::splat(8.0) - f64x8::splat(1.0) / f64x8::splat(3.0)) + t45;
            let t91 = t65 * t90;
            let tzk0 = -t45 + t91;
            acc_zk = tzk0;
            let t92 = t3 * t3;
            let t93 = f64x8::splat(1.0) / t92;
            let t94 = t2 * t93;
            let t95 = t9 * t25;
            let t99 = t12 * t14 * t16;
            let t100 = f64x8::splat(1.0) / t24;
            let t101 = t6 * t100;
            let t107 = t18 / t29 / t3;
            let t113 = t37 / t19 / t3;
            let t118 = t1 * (-f64x8::splat(3.0) / f64x8::splat(4.0) * t94 * t95 + t99 * t31 * t101 / f64x8::splat(9.0) + t28 * t107 * t32 / f64x8::splat(6.0) - t36 * t113 * t40 / f64x8::splat(24.0));
            let t119 = t46 * t93;
            let t120 = t4 - t119;
            let t123 = ((t49).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t52 * t120));
            let t124 = -t120;
            let t127 = ((t56).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t57 * t124));
            let t129 = (t123 + t127) * t64;
            let t130 = t129 * t90;
            let t131 = t70 * t78;
            let t135 = t73 * t14 * t16;
            let t136 = f64x8::splat(1.0) / t77;
            let t137 = t67 * t136;
            let t149 = -t66 * (-f64x8::splat(3.0) / f64x8::splat(4.0) * t94 * t131 + t135 * t31 * t137 / f64x8::splat(9.0) + t28 * t107 * t80 / f64x8::splat(6.0) - t36 * t113 * t84 / f64x8::splat(24.0)) + t118;
            let t150 = t65 * t149;
            let tvrho0 = -t45 + t91 + t3 * (-t118 + t130 + t150);
            acc_vrho_0 = tvrho0;
            let t153 = -t4 - t119;
            let t156 = ((t49).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t52 * t153));
            let t157 = -t153;
            let t160 = ((t56).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t57 * t157));
            let t162 = (t156 + t160) * t64;
            let t163 = t162 * t90;
            let tvrho1 = -t45 + t91 + t3 * (-t118 + t163 + t150);
            acc_vrho_1 = tvrho1;
            let t166 = f64x8::splat(2.0) * t118;
            let t168 = f64x8::splat(2.0) * t150;
            let t169 = t92 * t3;
            let t170 = f64x8::splat(1.0) / t169;
            let t171 = t2 * t170;
            let t175 = f64x8::splat(1.0) / t29 / t92;
            let t176 = t2 * t175;
            let t179 = t17 * t18 * t100;
            let t186 = f64x8::splat(1.0) / t27;
            let t187 = t12 * t13 * t186;
            let t188 = t24 * t24;
            let t189 = f64x8::splat(1.0) / t188;
            let t190 = t7 * t189;
            let t194 = t18 * t175;
            let t200 = t37 / t19 / t92;
            let t205 = t1 * (f64x8::splat(3.0) / f64x8::splat(2.0) * t171 * t95 - t176 * t32 * t179 / f64x8::splat(6.0) - f64x8::splat(2.0) / f64x8::splat(27.0) * t99 * t107 * t101 - t187 * t113 * t190 / f64x8::splat(27.0) - f64x8::splat(5.0) / f64x8::splat(18.0) * t28 * t194 * t32 + t36 * t200 * t40 / f64x8::splat(18.0));
            let t206 = t52 * t52;
            let t207 = f64x8::splat(1.0) / t206;
            let t208 = t120 * t120;
            let t211 = t46 * t170;
            let t213 = -f64x8::splat(2.0) * t93 + f64x8::splat(2.0) * t211;
            let t217 = ((t49).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t207 * t208 + f64x8::splat(4.0) / f64x8::splat(3.0) * t52 * t213));
            let t218 = t57 * t57;
            let t219 = f64x8::splat(1.0) / t218;
            let t220 = t124 * t124;
            let t223 = -t213;
            let t227 = ((t56).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t219 * t220 + f64x8::splat(4.0) / f64x8::splat(3.0) * t57 * t223));
            let t229 = (t217 + t227) * t64;
            let t230 = t229 * t90;
            let t231 = t129 * t149;
            let t232 = f64x8::splat(2.0) * t231;
            let t237 = t17 * t18 * t136;
            let t244 = t73 * t13 * t186;
            let t245 = t77 * t77;
            let t246 = f64x8::splat(1.0) / t245;
            let t247 = t68 * t246;
            let t259 = -t66 * (f64x8::splat(3.0) / f64x8::splat(2.0) * t171 * t131 - t176 * t80 * t237 / f64x8::splat(6.0) - f64x8::splat(2.0) / f64x8::splat(27.0) * t135 * t107 * t137 - t244 * t113 * t247 / f64x8::splat(27.0) - f64x8::splat(5.0) / f64x8::splat(18.0) * t28 * t194 * t80 + t36 * t200 * t84 / f64x8::splat(18.0)) + t205;
            let t260 = t65 * t259;
            let tv2rho20 = -t166 + f64x8::splat(2.0) * t130 + t168 + t3 * (-t205 + t230 + t232 + t260);
            acc_v2rho2_0 = tv2rho20;
            let t263 = t207 * t153;
            let t266 = t52 * t46;
            let t270 = ((t49).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t263 * t120 + f64x8::splat(8.0) / f64x8::splat(3.0) * t266 * t170));
            let t271 = t219 * t157;
            let t274 = t57 * t46;
            let t278 = ((t56).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t271 * t124 - f64x8::splat(8.0) / f64x8::splat(3.0) * t274 * t170));
            let t280 = (t270 + t278) * t64;
            let t281 = t280 * t90;
            let t282 = t162 * t149;
            let tv2rho21 = -t166 + t130 + t168 + t163 + t3 * (-t205 + t281 + t282 + t231 + t260);
            acc_v2rho2_1 = tv2rho21;
            let t286 = t153 * t153;
            let t290 = f64x8::splat(2.0) * t93 + f64x8::splat(2.0) * t211;
            let t294 = ((t49).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t207 * t286 + f64x8::splat(4.0) / f64x8::splat(3.0) * t52 * t290));
            let t295 = t157 * t157;
            let t298 = -t290;
            let t302 = ((t56).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t219 * t295 + f64x8::splat(4.0) / f64x8::splat(3.0) * t57 * t298));
            let t304 = (t294 + t302) * t64;
            let t305 = t304 * t90;
            let t306 = f64x8::splat(2.0) * t282;
            let tv2rho22 = -t166 + f64x8::splat(2.0) * t163 + t168 + t3 * (-t205 + t305 + t306 + t260);
            acc_v2rho2_2 = tv2rho22;
            let t309 = f64x8::splat(3.0) * t205;
            let t312 = f64x8::splat(3.0) * t260;
            let t313 = t92 * t92;
            let t314 = f64x8::splat(1.0) / t313;
            let t315 = t2 * t314;
            let t319 = f64x8::splat(1.0) / t29 / t169;
            let t320 = t2 * t319;
            let t325 = f64x8::splat(1.0) / t19 / t169;
            let t326 = t2 * t325;
            let t328 = t13 * t186;
            let t330 = t328 * t37 * t189;
            let t339 = t12 * f64x8::splat(M_PI);
            let t342 = f64x8::splat(1.0) / t188 / t24;
            let t346 = t18 * t319;
            let t350 = t37 * t325;
            let t355 = t1 * (-f64x8::splat(9.0) / f64x8::splat(2.0) * t315 * t95 + f64x8::splat(2.0) / f64x8::splat(3.0) * t320 * t32 * t179 + t326 * t40 * t330 / f64x8::splat(12.0) + f64x8::splat(10.0) / f64x8::splat(81.0) * t99 * t194 * t101 + f64x8::splat(2.0) / f64x8::splat(27.0) * t187 * t200 * t190 + f64x8::splat(8.0) / f64x8::splat(81.0) * t339 * t93 * t8 * t342 + f64x8::splat(20.0) / f64x8::splat(27.0) * t28 * t346 * t32 - f64x8::splat(7.0) / f64x8::splat(54.0) * t36 * t350 * t40);
            let t357 = f64x8::splat(1.0) / t206 / t48;
            let t358 = t208 * t120;
            let t361 = t207 * t120;
            let t364 = t46 * t314;
            let t366 = f64x8::splat(6.0) * t170 - f64x8::splat(6.0) * t364;
            let t370 = ((t49).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t357 * t358 + f64x8::splat(4.0) / f64x8::splat(3.0) * t361 * t213 + f64x8::splat(4.0) / f64x8::splat(3.0) * t52 * t366));
            let t372 = f64x8::splat(1.0) / t218 / t55;
            let t373 = t220 * t124;
            let t376 = t219 * t124;
            let t379 = -t366;
            let t383 = ((t56).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t372 * t373 + f64x8::splat(4.0) / f64x8::splat(3.0) * t376 * t223 + f64x8::splat(4.0) / f64x8::splat(3.0) * t57 * t379));
            let t385 = (t370 + t383) * t64;
            let t386 = t385 * t90;
            let t387 = t229 * t149;
            let t389 = t129 * t259;
            let t390 = f64x8::splat(3.0) * t389;
            let t398 = t328 * t37 * t246;
            let t407 = t73 * f64x8::splat(M_PI);
            let t410 = f64x8::splat(1.0) / t245 / t77;
            let t422 = -t66 * (-f64x8::splat(9.0) / f64x8::splat(2.0) * t315 * t131 + f64x8::splat(2.0) / f64x8::splat(3.0) * t320 * t80 * t237 + t326 * t84 * t398 / f64x8::splat(12.0) + f64x8::splat(10.0) / f64x8::splat(81.0) * t135 * t194 * t137 + f64x8::splat(2.0) / f64x8::splat(27.0) * t244 * t200 * t247 + f64x8::splat(8.0) / f64x8::splat(81.0) * t407 * t93 * t69 * t410 + f64x8::splat(20.0) / f64x8::splat(27.0) * t28 * t346 * t80 - f64x8::splat(7.0) / f64x8::splat(54.0) * t36 * t350 * t84) + t355;
            let t423 = t65 * t422;
            let tv3rho30 = -t309 + f64x8::splat(3.0) * t230 + f64x8::splat(6.0) * t231 + t312 + t3 * (-t355 + t386 + f64x8::splat(3.0) * t387 + t390 + t423);
            acc_v3rho3_0 = tv3rho30;
            let t427 = f64x8::splat(2.0) * t281;
            let t428 = t357 * t153;
            let t431 = t207 * t46;
            let t442 = ((t49).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t428 * t208 + f64x8::splat(16.0) / f64x8::splat(9.0) * t431 * t170 * t120 + f64x8::splat(4.0) / f64x8::splat(9.0) * t263 * t213 + f64x8::splat(8.0) / f64x8::splat(3.0) * t52 * t170 - f64x8::splat(8.0) * t266 * t314));
            let t443 = t372 * t157;
            let t446 = t219 * t46;
            let t457 = ((t56).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t443 * t220 - f64x8::splat(16.0) / f64x8::splat(9.0) * t446 * t170 * t124 + f64x8::splat(4.0) / f64x8::splat(9.0) * t271 * t223 - f64x8::splat(8.0) / f64x8::splat(3.0) * t57 * t170 + f64x8::splat(8.0) * t274 * t314));
            let t459 = (t442 + t457) * t64;
            let t460 = t459 * t90;
            let t461 = t280 * t149;
            let t462 = f64x8::splat(2.0) * t461;
            let t463 = t162 * t259;
            let tv3rho31 = -t309 + t230 + f64x8::splat(4.0) * t231 + t312 + t427 + t306 + t3 * (-t355 + t460 + t462 + t463 + t387 + f64x8::splat(2.0) * t389 + t423);
            acc_v3rho3_1 = tv3rho31;
            let t468 = t357 * t286;
            let t473 = t207 * t290;
            let t478 = -f64x8::splat(2.0) * t170 - f64x8::splat(6.0) * t364;
            let t482 = ((t49).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t468 * t120 + f64x8::splat(16.0) / f64x8::splat(9.0) * t263 * t211 + f64x8::splat(4.0) / f64x8::splat(9.0) * t473 * t120 + f64x8::splat(4.0) / f64x8::splat(3.0) * t52 * t478));
            let t483 = t372 * t295;
            let t488 = t219 * t298;
            let t491 = -t478;
            let t495 = ((t56).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t483 * t124 - f64x8::splat(16.0) / f64x8::splat(9.0) * t271 * t211 + f64x8::splat(4.0) / f64x8::splat(9.0) * t488 * t124 + f64x8::splat(4.0) / f64x8::splat(3.0) * t57 * t491));
            let t497 = (t482 + t495) * t64;
            let t498 = t497 * t90;
            let t499 = t304 * t149;
            let tv3rho32 = -t309 + t427 + f64x8::splat(4.0) * t282 + t232 + t312 + t305 + t3 * (-t355 + t498 + t499 + t462 + f64x8::splat(2.0) * t463 + t389 + t423);
            acc_v3rho3_2 = tv3rho32;
            let t505 = t286 * t153;
            let t511 = -f64x8::splat(6.0) * t170 - f64x8::splat(6.0) * t364;
            let t515 = ((t49).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t357 * t505 + f64x8::splat(4.0) / f64x8::splat(3.0) * t263 * t290 + f64x8::splat(4.0) / f64x8::splat(3.0) * t52 * t511));
            let t516 = t295 * t157;
            let t521 = -t511;
            let t525 = ((t56).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t372 * t516 + f64x8::splat(4.0) / f64x8::splat(3.0) * t271 * t298 + f64x8::splat(4.0) / f64x8::splat(3.0) * t57 * t521));
            let t527 = (t515 + t525) * t64;
            let t528 = t527 * t90;
            let t530 = f64x8::splat(3.0) * t463;
            let tv3rho33 = -t309 + f64x8::splat(3.0) * t305 + f64x8::splat(6.0) * t282 + t312 + t3 * (-t355 + t528 + f64x8::splat(3.0) * t499 + t530 + t423);
            acc_v3rho3_3 = tv3rho33;
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
        ip += 8;
    }
}

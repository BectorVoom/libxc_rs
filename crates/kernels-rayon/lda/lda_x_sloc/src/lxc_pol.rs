//! LDA_X_SLOC lxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_x_sloc.c`
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
pub fn lda_x_sloc_lxc_pol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    v3rho3: &mut [f64],
    v4rho4: &mut [f64],
    param_a: f64,
    param_b: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_a = f64x8::splat(param_a);
    let param_b = f64x8::splat(param_b);
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
            let t1 = param_b + f64x8::splat(1.0);
            let t3 = f64x8::splat(1.0) / t1 / f64x8::splat(2.0);
            let t4 = param_a * t3;
            let t5 = v_rho0 + v_rho1;
            let t6 = (simd::pow(t5, param_b));
            let t7 = v_rho0 - v_rho1;
            let t8 = f64x8::splat(1.0) / t5;
            let t9 = t7 * t8;
            let t10 = f64x8::splat(1.0) + t9;
            let t11 = (t10).simd_le(zeta_threshold);
            let t12 = (simd::pow(zeta_threshold, t1));
            let t13 = (simd::pow(t10, t1));
            let t14 = ((t11).select(t12, t13));
            let t15 = f64x8::splat(1.0) - t9;
            let t16 = (t15).simd_le(zeta_threshold);
            let t17 = (simd::pow(t15, t1));
            let t18 = ((t16).select(t12, t17));
            let t19 = t14 + t18;
            let tzk0 = -t4 * t6 * t19;
            acc_zk = tzk0;
            let t22 = t6 * param_b;
            let t24 = t4 * t22 * t19;
            let t25 = t5 * param_a;
            let t26 = t3 * t6;
            let t27 = t13 * t1;
            let t28 = t5 * t5;
            let t29 = f64x8::splat(1.0) / t28;
            let t30 = t7 * t29;
            let t31 = t8 - t30;
            let t32 = f64x8::splat(1.0) / t10;
            let t35 = ((t11).select(f64x8::splat(0.0), t27 * t31 * t32));
            let t36 = t17 * t1;
            let t37 = -t31;
            let t38 = f64x8::splat(1.0) / t15;
            let t41 = ((t16).select(f64x8::splat(0.0), t36 * t37 * t38));
            let t42 = t35 + t41;
            let tvrho0 = -t25 * t26 * t42 - t24 + tzk0;
            acc_vrho_0 = tvrho0;
            let t45 = -t8 - t30;
            let t48 = ((t11).select(f64x8::splat(0.0), t27 * t45 * t32));
            let t49 = -t45;
            let t52 = ((t16).select(f64x8::splat(0.0), t36 * t49 * t38));
            let t53 = t48 + t52;
            let tvrho1 = -t25 * t26 * t53 - t24 + tzk0;
            acc_vrho_1 = tvrho1;
            let t56 = t4 * t6;
            let t57 = param_b * t8;
            let t59 = t56 * t57 * t19;
            let t61 = t4 * t6 * t42;
            let t63 = param_b * param_b;
            let t64 = t63 * t8;
            let t66 = t56 * t64 * t19;
            let t68 = t4 * t22 * t42;
            let t70 = t1 * t1;
            let t71 = t13 * t70;
            let t72 = t31 * t31;
            let t73 = t10 * t10;
            let t74 = f64x8::splat(1.0) / t73;
            let t75 = t72 * t74;
            let t78 = f64x8::splat(1.0) / t28 / t5;
            let t79 = t7 * t78;
            let t81 = -f64x8::splat(2.0) * t29 + f64x8::splat(2.0) * t79;
            let t86 = ((t11).select(f64x8::splat(0.0), t27 * t81 * t32 - t27 * t75 + t71 * t75));
            let t87 = t17 * t70;
            let t88 = t37 * t37;
            let t89 = t15 * t15;
            let t90 = f64x8::splat(1.0) / t89;
            let t91 = t88 * t90;
            let t93 = -t81;
            let t98 = ((t16).select(f64x8::splat(0.0), t36 * t93 * t38 - t36 * t91 + t87 * t91));
            let t99 = t86 + t98;
            let tv2rho20 = -t25 * t26 * t99 - t59 - f64x8::splat(2.0) * t61 - t66 - f64x8::splat(2.0) * t68;
            acc_v2rho2_0 = tv2rho20;
            let t103 = t4 * t6 * t53;
            let t105 = t4 * t22 * t53;
            let t106 = t31 * t74;
            let t107 = t106 * t45;
            let t114 = ((t11).select(f64x8::splat(0.0), f64x8::splat(2.0) * t27 * t79 * t32 - t27 * t107 + t71 * t107));
            let t115 = t37 * t90;
            let t116 = t115 * t49;
            let t123 = ((t16).select(f64x8::splat(0.0), -f64x8::splat(2.0) * t36 * t79 * t38 - t36 * t116 + t87 * t116));
            let t124 = t114 + t123;
            let tv2rho21 = -t25 * t26 * t124 - t103 - t105 - t59 - t61 - t66 - t68;
            acc_v2rho2_1 = tv2rho21;
            let t129 = t45 * t45;
            let t130 = t129 * t74;
            let t133 = f64x8::splat(2.0) * t29 + f64x8::splat(2.0) * t79;
            let t138 = ((t11).select(f64x8::splat(0.0), t27 * t133 * t32 - t27 * t130 + t71 * t130));
            let t139 = t49 * t49;
            let t140 = t139 * t90;
            let t142 = -t133;
            let t147 = ((t16).select(f64x8::splat(0.0), t36 * t142 * t38 - t36 * t140 + t87 * t140));
            let t148 = t138 + t147;
            let tv2rho22 = -t25 * t26 * t148 - f64x8::splat(2.0) * t103 - f64x8::splat(2.0) * t105 - t59 - t66;
            acc_v2rho2_2 = tv2rho22;
            let t151 = param_b * t29;
            let t153 = t56 * t151 * t19;
            let t155 = t56 * t57 * t42;
            let t158 = t4 * t6 * t99;
            let t160 = t63 * param_b;
            let t161 = t160 * t29;
            let t163 = t56 * t161 * t19;
            let t165 = t56 * t64 * t42;
            let t168 = t4 * t22 * t99;
            let t170 = t70 * t1;
            let t171 = t13 * t170;
            let t172 = t72 * t31;
            let t174 = f64x8::splat(1.0) / t73 / t10;
            let t175 = t172 * t174;
            let t177 = t106 * t81;
            let t182 = t28 * t28;
            let t183 = f64x8::splat(1.0) / t182;
            let t184 = t7 * t183;
            let t186 = f64x8::splat(6.0) * t78 - f64x8::splat(6.0) * t184;
            let t194 = ((t11).select(f64x8::splat(0.0), t27 * t186 * t32 + t171 * t175 + f64x8::splat(2.0) * t27 * t175 - f64x8::splat(3.0) * t71 * t175 - f64x8::splat(3.0) * t27 * t177 + f64x8::splat(3.0) * t71 * t177));
            let t195 = t17 * t170;
            let t196 = t88 * t37;
            let t198 = f64x8::splat(1.0) / t89 / t15;
            let t199 = t196 * t198;
            let t201 = t115 * t93;
            let t206 = -t186;
            let t214 = ((t16).select(f64x8::splat(0.0), t36 * t206 * t38 + t195 * t199 + f64x8::splat(2.0) * t36 * t199 - f64x8::splat(3.0) * t87 * t199 - f64x8::splat(3.0) * t36 * t201 + f64x8::splat(3.0) * t87 * t201));
            let t215 = t194 + t214;
            let tv3rho30 = -t25 * t26 * t215 + t153 - f64x8::splat(3.0) * t155 - f64x8::splat(3.0) * t158 - t163 - f64x8::splat(3.0) * t165 - f64x8::splat(3.0) * t168;
            acc_v3rho3_0 = tv3rho30;
            let t221 = t56 * t57 * t53;
            let t224 = f64x8::splat(2.0) * t4 * t6 * t124;
            let t226 = t56 * t64 * t53;
            let t229 = f64x8::splat(2.0) * t4 * t22 * t124;
            let t230 = t72 * t174;
            let t231 = t230 * t45;
            let t233 = t81 * t74;
            let t234 = t233 * t45;
            let t238 = t71 * t31;
            let t239 = t74 * t7;
            let t240 = t239 * t78;
            let t249 = t27 * t7;
            let t250 = t78 * t74;
            let t251 = t250 * t31;
            let t258 = ((t11).select(f64x8::splat(0.0), -f64x8::splat(6.0) * t27 * t184 * t32 + f64x8::splat(2.0) * t27 * t78 * t32 + t171 * t231 + f64x8::splat(2.0) * t27 * t231 - f64x8::splat(3.0) * t71 * t231 - t27 * t234 + t71 * t234 + f64x8::splat(4.0) * t238 * t240 - f64x8::splat(4.0) * t249 * t251));
            let t259 = t88 * t198;
            let t260 = t259 * t49;
            let t262 = t93 * t90;
            let t263 = t262 * t49;
            let t267 = t87 * t37;
            let t268 = t90 * t7;
            let t269 = t268 * t78;
            let t278 = t36 * t7;
            let t279 = t78 * t90;
            let t280 = t279 * t37;
            let t287 = ((t16).select(f64x8::splat(0.0), f64x8::splat(6.0) * t36 * t184 * t38 - f64x8::splat(2.0) * t36 * t78 * t38 + t195 * t260 + f64x8::splat(2.0) * t36 * t260 - f64x8::splat(3.0) * t87 * t260 - t36 * t263 + t87 * t263 - f64x8::splat(4.0) * t267 * t269 + f64x8::splat(4.0) * t278 * t280));
            let t288 = t258 + t287;
            let tv3rho31 = -t25 * t26 * t288 + t153 - f64x8::splat(2.0) * t155 - t158 - t163 - f64x8::splat(2.0) * t165 - t168 - t221 - t224 - t226 - t229;
            acc_v3rho3_1 = tv3rho31;
            let t294 = t4 * t6 * t148;
            let t296 = t4 * t22 * t148;
            let t297 = t31 * t174;
            let t298 = t297 * t129;
            let t300 = t71 * t45;
            let t305 = t106 * t133;
            let t309 = -f64x8::splat(2.0) * t78 - f64x8::splat(6.0) * t184;
            let t313 = t27 * t45;
            let t319 = ((t11).select(f64x8::splat(0.0), t27 * t309 * t32 + t171 * t298 + f64x8::splat(4.0) * t300 * t240 - f64x8::splat(4.0) * t313 * t240 + f64x8::splat(2.0) * t27 * t298 - t27 * t305 - f64x8::splat(3.0) * t71 * t298 + t71 * t305));
            let t320 = t37 * t198;
            let t321 = t320 * t139;
            let t323 = t87 * t49;
            let t328 = t115 * t142;
            let t330 = -t309;
            let t334 = t36 * t49;
            let t340 = ((t16).select(f64x8::splat(0.0), t36 * t330 * t38 + t195 * t321 - f64x8::splat(4.0) * t323 * t269 + f64x8::splat(4.0) * t334 * t269 + f64x8::splat(2.0) * t36 * t321 - f64x8::splat(3.0) * t87 * t321 - t36 * t328 + t87 * t328));
            let t341 = t319 + t340;
            let tv3rho32 = -t25 * t26 * t341 + t153 - t155 - t163 - t165 - f64x8::splat(2.0) * t221 - t224 - f64x8::splat(2.0) * t226 - t229 - t294 - t296;
            acc_v3rho3_2 = tv3rho32;
            let t348 = t129 * t45;
            let t349 = t348 * t174;
            let t351 = t45 * t74;
            let t352 = t351 * t133;
            let t358 = -f64x8::splat(6.0) * t78 - f64x8::splat(6.0) * t184;
            let t366 = ((t11).select(f64x8::splat(0.0), t27 * t358 * t32 + t171 * t349 + f64x8::splat(2.0) * t27 * t349 - f64x8::splat(3.0) * t27 * t352 - f64x8::splat(3.0) * t71 * t349 + f64x8::splat(3.0) * t71 * t352));
            let t367 = t139 * t49;
            let t368 = t367 * t198;
            let t370 = t49 * t90;
            let t371 = t370 * t142;
            let t376 = -t358;
            let t384 = ((t16).select(f64x8::splat(0.0), t36 * t376 * t38 + t195 * t368 + f64x8::splat(2.0) * t36 * t368 - f64x8::splat(3.0) * t36 * t371 - f64x8::splat(3.0) * t87 * t368 + f64x8::splat(3.0) * t87 * t371));
            let t385 = t366 + t384;
            let tv3rho33 = -t25 * t26 * t385 + t153 - t163 - f64x8::splat(3.0) * t221 - f64x8::splat(3.0) * t226 - f64x8::splat(3.0) * t294 - f64x8::splat(3.0) * t296;
            acc_v3rho3_3 = tv3rho33;
            let t390 = t56 * t63 * t78 * t19;
            let t394 = f64x8::splat(2.0) * t56 * param_b * t78 * t19;
            let t396 = t56 * t151 * t42;
            let t399 = t56 * t57 * t99;
            let t402 = t4 * t6 * t215;
            let t404 = t63 * t63;
            let t407 = t56 * t404 * t78 * t19;
            let t411 = f64x8::splat(2.0) * t56 * t160 * t78 * t19;
            let t413 = t56 * t161 * t42;
            let t416 = t56 * t64 * t99;
            let t419 = t4 * t22 * t215;
            let t421 = t70 * t70;
            let t422 = t13 * t421;
            let t423 = t72 * t72;
            let t424 = t73 * t73;
            let t425 = f64x8::splat(1.0) / t424;
            let t426 = t423 * t425;
            let t428 = t230 * t81;
            let t433 = t81 * t81;
            let t434 = t433 * t74;
            let t439 = t106 * t186;
            let t446 = t7 / t182 / t5;
            let t448 = -f64x8::splat(24.0) * t183 + f64x8::splat(24.0) * t446;
            let t459 = t27 * t448 * t32 - f64x8::splat(6.0) * t171 * t426 + f64x8::splat(6.0) * t171 * t428 - f64x8::splat(6.0) * t27 * t426 + f64x8::splat(12.0) * t27 * t428 - f64x8::splat(3.0) * t27 * t434 - f64x8::splat(4.0) * t27 * t439 + t422 * t426 + f64x8::splat(11.0) * t71 * t426 - f64x8::splat(18.0) * t71 * t428 + f64x8::splat(3.0) * t71 * t434 + f64x8::splat(4.0) * t71 * t439;
            let t460 = ((t11).select(f64x8::splat(0.0), t459));
            let t461 = t17 * t421;
            let t462 = t88 * t88;
            let t463 = t89 * t89;
            let t464 = f64x8::splat(1.0) / t463;
            let t465 = t462 * t464;
            let t467 = t259 * t93;
            let t472 = t93 * t93;
            let t473 = t472 * t90;
            let t478 = t115 * t206;
            let t494 = -t36 * t448 * t38 - f64x8::splat(6.0) * t195 * t465 + f64x8::splat(6.0) * t195 * t467 - f64x8::splat(6.0) * t36 * t465 + f64x8::splat(12.0) * t36 * t467 - f64x8::splat(3.0) * t36 * t473 - f64x8::splat(4.0) * t36 * t478 + t461 * t465 + f64x8::splat(11.0) * t87 * t465 - f64x8::splat(18.0) * t87 * t467 + f64x8::splat(3.0) * t87 * t473 + f64x8::splat(4.0) * t87 * t478;
            let t495 = ((t16).select(f64x8::splat(0.0), t494));
            let tv4rho40 = t390 - t394 + f64x8::splat(4.0) * t396 - f64x8::splat(6.0) * t399 - f64x8::splat(4.0) * t402 - t407 + t411 - f64x8::splat(4.0) * t413 - f64x8::splat(6.0) * t416 - f64x8::splat(4.0) * t419 - t25 * t26 * (t460 + t495);
            acc_v4rho4_0 = tv4rho40;
            let t500 = t4 * t22 * t288;
            let t503 = t186 * t74 * t45;
            let t512 = t172 * t425 * t45;
            let t514 = t171 * t31;
            let t515 = t174 * t45;
            let t522 = t174 * t7;
            let t523 = t522 * t78;
            let t526 = t71 * t81;
            let t527 = t515 * t31;
            let t541 = f64x8::splat(24.0) * t27 * t446 * t32;
            let t554 = t239 * t183;
            let t561 = f64x8::splat(12.0) * t249 * t78 * t174 * t72 + f64x8::splat(18.0) * t249 * t183 * t74 * t31 + f64x8::splat(6.0) * t171 * t72 * t523 - f64x8::splat(12.0) * t27 * t183 * t32 - f64x8::splat(6.0) * t249 * t250 * t81 + f64x8::splat(6.0) * t313 * t297 * t81 + f64x8::splat(3.0) * t514 * t515 * t81 - f64x8::splat(18.0) * t71 * t72 * t523 - f64x8::splat(6.0) * t171 * t512 - f64x8::splat(18.0) * t238 * t554 + f64x8::splat(6.0) * t526 * t240 - f64x8::splat(6.0) * t27 * t251 + f64x8::splat(6.0) * t71 * t251 - t27 * t503 - f64x8::splat(6.0) * t27 * t512 + t422 * t512 + t71 * t503 + f64x8::splat(11.0) * t71 * t512 - f64x8::splat(9.0) * t526 * t527 + t541;
            let t562 = ((t11).select(f64x8::splat(0.0), t561));
            let t564 = t206 * t90 * t49;
            let t573 = t196 * t464 * t49;
            let t575 = t195 * t37;
            let t576 = t198 * t49;
            let t583 = t198 * t7;
            let t584 = t583 * t78;
            let t587 = t87 * t93;
            let t588 = t576 * t37;
            let t602 = f64x8::splat(24.0) * t36 * t446 * t38;
            let t615 = t268 * t183;
            let t622 = -f64x8::splat(18.0) * t278 * t183 * t90 * t37 - f64x8::splat(12.0) * t278 * t78 * t198 * t88 + f64x8::splat(12.0) * t36 * t183 * t38 - f64x8::splat(6.0) * t195 * t88 * t584 + f64x8::splat(6.0) * t278 * t279 * t93 + f64x8::splat(6.0) * t334 * t320 * t93 + f64x8::splat(3.0) * t575 * t576 * t93 + f64x8::splat(18.0) * t87 * t88 * t584 - f64x8::splat(6.0) * t195 * t573 + f64x8::splat(18.0) * t267 * t615 - f64x8::splat(6.0) * t587 * t269 + f64x8::splat(6.0) * t36 * t280 - f64x8::splat(6.0) * t87 * t280 - t36 * t564 - f64x8::splat(6.0) * t36 * t573 + t461 * t573 + t87 * t564 + f64x8::splat(11.0) * t87 * t573 - f64x8::splat(9.0) * t587 * t588 - t602;
            let t623 = ((t16).select(f64x8::splat(0.0), t622));
            let t628 = t4 * t6 * t288;
            let t631 = t56 * t161 * t53;
            let t637 = t56 * t57 * t124;
            let t638 = f64x8::splat(3.0) * t637;
            let t640 = t56 * t64 * t124;
            let t641 = f64x8::splat(3.0) * t640;
            let t643 = t56 * t151 * t53;
            let tv4rho41 = -t419 - f64x8::splat(3.0) * t500 - t25 * t26 * (t562 + t623) - t402 - f64x8::splat(3.0) * t628 + t390 - t407 - t631 + f64x8::splat(3.0) * t396 - f64x8::splat(3.0) * t399 - f64x8::splat(3.0) * t413 - f64x8::splat(3.0) * t416 - t638 - t641 - t394 + t411 + t643;
            acc_v4rho4_1 = tv4rho41;
            let t646 = t4 * t22 * t341;
            let t649 = t81 * t174 * t129;
            let t651 = t7 * t7;
            let t654 = t651 / t182 / t28;
            let t655 = t654 * t74;
            let t660 = t233 * t133;
            let t662 = t106 * t309;
            let t673 = t72 * t425 * t129;
            let t675 = t230 * t133;
            let t677 = t171 * t649 + t171 * t675 + f64x8::splat(2.0) * t27 * t649 - f64x8::splat(8.0) * t27 * t655 - t27 * t660 - f64x8::splat(2.0) * t27 * t662 + t422 * t673 - f64x8::splat(3.0) * t71 * t649 + f64x8::splat(8.0) * t71 * t655 + t71 * t660 + f64x8::splat(2.0) * t71 * t662 + t541;
            let t680 = t351 * t78;
            let t697 = t522 * t78 * t31;
            let t706 = f64x8::splat(8.0) * t514 * t515 * t79 - f64x8::splat(6.0) * t171 * t673 - f64x8::splat(6.0) * t27 * t673 + f64x8::splat(2.0) * t27 * t675 - f64x8::splat(4.0) * t27 * t680 - f64x8::splat(12.0) * t300 * t554 - f64x8::splat(24.0) * t300 * t697 + f64x8::splat(12.0) * t313 * t554 + f64x8::splat(16.0) * t313 * t697 + f64x8::splat(11.0) * t71 * t673 - f64x8::splat(3.0) * t71 * t675 + f64x8::splat(4.0) * t71 * t680;
            let t708 = ((t11).select(f64x8::splat(0.0), t677 + t706));
            let t710 = t93 * t198 * t139;
            let t712 = t654 * t90;
            let t717 = t262 * t142;
            let t719 = t115 * t330;
            let t730 = t88 * t464 * t139;
            let t732 = t259 * t142;
            let t734 = t195 * t710 + t195 * t732 + f64x8::splat(2.0) * t36 * t710 - f64x8::splat(8.0) * t36 * t712 - t36 * t717 - f64x8::splat(2.0) * t36 * t719 + t461 * t730 - f64x8::splat(3.0) * t87 * t710 + f64x8::splat(8.0) * t87 * t712 + t87 * t717 + f64x8::splat(2.0) * t87 * t719 - t602;
            let t737 = t370 * t78;
            let t754 = t583 * t78 * t37;
            let t763 = -f64x8::splat(8.0) * t575 * t576 * t79 - f64x8::splat(6.0) * t195 * t730 + f64x8::splat(12.0) * t323 * t615 + f64x8::splat(24.0) * t323 * t754 - f64x8::splat(12.0) * t334 * t615 - f64x8::splat(16.0) * t334 * t754 - f64x8::splat(6.0) * t36 * t730 + f64x8::splat(2.0) * t36 * t732 + f64x8::splat(4.0) * t36 * t737 + f64x8::splat(11.0) * t87 * t730 - f64x8::splat(3.0) * t87 * t732 - f64x8::splat(4.0) * t87 * t737;
            let t765 = ((t16).select(f64x8::splat(0.0), t734 + t763));
            let t771 = t4 * t6 * t341;
            let t779 = t56 * t57 * t148;
            let t782 = t56 * t64 * t148;
            let tv4rho42 = -f64x8::splat(2.0) * t500 - f64x8::splat(2.0) * t646 - t25 * t26 * (t708 + t765) - f64x8::splat(2.0) * t628 - f64x8::splat(2.0) * t771 + t390 - t407 - f64x8::splat(2.0) * t631 + f64x8::splat(2.0) * t396 - t399 - f64x8::splat(2.0) * t413 - t416 - f64x8::splat(4.0) * t637 - f64x8::splat(4.0) * t640 - t779 - t394 + t411 + f64x8::splat(2.0) * t643 - t782;
            acc_v4rho4_2 = tv4rho42;
            let t790 = t4 * t6 * t385;
            let t792 = t4 * t22 * t385;
            let t794 = t31 * t425 * t348;
            let t812 = t351 * t309;
            let t820 = t106 * t358;
            let t824 = f64x8::splat(12.0) * t183 + f64x8::splat(24.0) * t446;
            let t830 = t27 * t133;
            let t840 = -f64x8::splat(9.0) * t300 * t174 * t133 * t31 + f64x8::splat(6.0) * t71 * t7 * t250 * t133 + f64x8::splat(6.0) * t171 * t129 * t523 + f64x8::splat(12.0) * t27 * t129 * t523 - f64x8::splat(18.0) * t71 * t129 * t523 + f64x8::splat(3.0) * t514 * t515 * t133 + t27 * t824 * t32 - f64x8::splat(6.0) * t171 * t794 - f64x8::splat(6.0) * t830 * t240 - f64x8::splat(6.0) * t27 * t794 - f64x8::splat(3.0) * t27 * t812 - t27 * t820 + t422 * t794 + f64x8::splat(6.0) * t830 * t527 + f64x8::splat(11.0) * t71 * t794 + f64x8::splat(3.0) * t71 * t812 + t71 * t820;
            let t841 = ((t11).select(f64x8::splat(0.0), t840));
            let t843 = t37 * t464 * t367;
            let t861 = t370 * t330;
            let t869 = t115 * t376;
            let t877 = t36 * t142;
            let t887 = -f64x8::splat(9.0) * t323 * t198 * t142 * t37 - f64x8::splat(6.0) * t87 * t7 * t279 * t142 - f64x8::splat(6.0) * t195 * t139 * t584 - f64x8::splat(12.0) * t36 * t139 * t584 + f64x8::splat(18.0) * t87 * t139 * t584 + f64x8::splat(3.0) * t575 * t576 * t142 - t36 * t824 * t38 - f64x8::splat(6.0) * t195 * t843 + f64x8::splat(6.0) * t877 * t269 - f64x8::splat(6.0) * t36 * t843 - f64x8::splat(3.0) * t36 * t861 - t36 * t869 + t461 * t843 + f64x8::splat(6.0) * t877 * t588 + f64x8::splat(11.0) * t87 * t843 + f64x8::splat(3.0) * t87 * t861 + t87 * t869;
            let t888 = ((t16).select(f64x8::splat(0.0), t887));
            let tv4rho43 = t390 - t394 + t396 + f64x8::splat(3.0) * t643 - t638 - f64x8::splat(3.0) * t779 - f64x8::splat(3.0) * t771 - t407 + t411 - t413 - f64x8::splat(3.0) * t631 - t641 - f64x8::splat(3.0) * t782 - f64x8::splat(3.0) * t646 - t790 - t792 - t25 * t26 * (t841 + t888);
            acc_v4rho4_3 = tv4rho43;
            let t898 = t129 * t129;
            let t899 = t898 * t425;
            let t902 = t129 * t174 * t133;
            let t907 = t133 * t133;
            let t908 = t907 * t74;
            let t913 = t351 * t358;
            let t919 = f64x8::splat(24.0) * t183 + f64x8::splat(24.0) * t446;
            let t930 = t27 * t919 * t32 - f64x8::splat(6.0) * t171 * t899 + f64x8::splat(6.0) * t171 * t902 - f64x8::splat(6.0) * t27 * t899 + f64x8::splat(12.0) * t27 * t902 - f64x8::splat(3.0) * t27 * t908 - f64x8::splat(4.0) * t27 * t913 + t422 * t899 + f64x8::splat(11.0) * t71 * t899 - f64x8::splat(18.0) * t71 * t902 + f64x8::splat(3.0) * t71 * t908 + f64x8::splat(4.0) * t71 * t913;
            let t931 = ((t11).select(f64x8::splat(0.0), t930));
            let t932 = t139 * t139;
            let t933 = t932 * t464;
            let t936 = t139 * t198 * t142;
            let t941 = t142 * t142;
            let t942 = t941 * t90;
            let t947 = t370 * t376;
            let t963 = -t36 * t919 * t38 - f64x8::splat(6.0) * t195 * t933 + f64x8::splat(6.0) * t195 * t936 - f64x8::splat(6.0) * t36 * t933 + f64x8::splat(12.0) * t36 * t936 - f64x8::splat(3.0) * t36 * t942 - f64x8::splat(4.0) * t36 * t947 + t461 * t933 + f64x8::splat(11.0) * t87 * t933 - f64x8::splat(18.0) * t87 * t936 + f64x8::splat(3.0) * t87 * t942 + f64x8::splat(4.0) * t87 * t947;
            let t964 = ((t16).select(f64x8::splat(0.0), t963));
            let tv4rho44 = t390 - t394 + f64x8::splat(4.0) * t643 - f64x8::splat(6.0) * t779 - f64x8::splat(4.0) * t790 - t407 + t411 - f64x8::splat(4.0) * t631 - f64x8::splat(6.0) * t782 - f64x8::splat(4.0) * t792 - t25 * t26 * (t931 + t964);
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

//! LDA_X_ERF vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_x_erf.c`
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
pub fn lda_x_erf_vxc_pol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
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
        let v_rho0 = load_strided(rho, ip, np, 2, 0);
        let v_rho1 = load_strided(rho, ip, np, 2, 1);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        {
            let t1 = f64x8::splat(M_CBRT3);
            let t3 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t5 = f64x8::splat(M_CBRT4);
            let t6 = t5 * t5;
            let t7 = t1 * t3 * t6;
            let t8 = f64x8::splat(M_CBRT2);
            let t9 = t8 * t8;
            let t10 = v_rho0 - v_rho1;
            let t11 = v_rho0 + v_rho1;
            let t12 = f64x8::splat(1.0) / t11;
            let t13 = t10 * t12;
            let t14 = f64x8::splat(1.0) + t13;
            let t15 = (t14).simd_le(zeta_threshold);
            let t16 = (simd::cbrt(zeta_threshold));
            let t17 = t16 * zeta_threshold;
            let t18 = (simd::cbrt(t14));
            let t20 = ((t15).select(t17, t18 * t14));
            let t21 = t9 * t20;
            let t22 = (simd::cbrt(t11));
            let t23 = (simd::cbrt(f64x8::splat(9.0)));
            let t24 = t23 * t23;
            let t25 = t3 * t3;
            let t26 = t24 * t25;
            let t27 = t26 * param_hyb_omega_0;
            let t28 = f64x8::splat(1.0) / t22;
            let t29 = t1 * t28;
            let t30 = ((t15).select(t16, t18));
            let t31 = f64x8::splat(1.0) / t30;
            let t34 = t27 * t29 * t31 / f64x8::splat(18.0);
            let t35 = (f64x8::splat(1.35)).simd_le(t34);
            let t36 = (f64x8::splat(1.35)).simd_lt(t34);
            let t37 = ((t36).select(t34, f64x8::splat(1.35)));
            let t38 = t37 * t37;
            let t41 = t38 * t38;
            let t42 = f64x8::splat(1.0) / t41;
            let t44 = t41 * t38;
            let t45 = f64x8::splat(1.0) / t44;
            let t47 = t41 * t41;
            let t48 = f64x8::splat(1.0) / t47;
            let t51 = f64x8::splat(1.0) / t47 / t38;
            let t54 = f64x8::splat(1.0) / t47 / t41;
            let t57 = f64x8::splat(1.0) / t47 / t44;
            let t59 = t47 * t47;
            let t60 = f64x8::splat(1.0) / t59;
            let t63 = ((t36).select(f64x8::splat(1.35), t34));
            let t64 = ((f64x8::splat(M_PI)).sqrt());
            let t65 = f64x8::splat(1.0) / t63;
            let t67 = (simd::erf(t65 / f64x8::splat(2.0)));
            let t69 = t63 * t63;
            let t70 = f64x8::splat(1.0) / t69;
            let t72 = (simd::exp(-t70 / f64x8::splat(4.0)));
            let t73 = t72 - f64x8::splat(1.0);
            let t76 = t72 - f64x8::splat(3.0) / f64x8::splat(2.0) - f64x8::splat(2.0) * t69 * t73;
            let t79 = f64x8::splat(2.0) * t63 * t76 + t64 * t67;
            let t83 = ((t35).select(f64x8::splat(1.0) / t38 / f64x8::splat(36.0) - t42 / f64x8::splat(960.0) + t45 / f64x8::splat(26880.0) - t48 / f64x8::splat(829440.0) + t51 / f64x8::splat(28385280.0) - t54 / f64x8::splat(1073479680.0) + t57 / f64x8::splat(44590694400.0) - t60 / f64x8::splat(2021444812800.0), f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t63 * t79));
            let t84 = t22 * t83;
            let t86 = t7 * t21 * t84;
            let t87 = f64x8::splat(1.0) - t13;
            let t88 = (t87).simd_le(zeta_threshold);
            let t89 = (simd::cbrt(t87));
            let t91 = ((t88).select(t17, t89 * t87));
            let t92 = t9 * t91;
            let t93 = ((t88).select(t16, t89));
            let t94 = f64x8::splat(1.0) / t93;
            let t97 = t27 * t29 * t94 / f64x8::splat(18.0);
            let t98 = (f64x8::splat(1.35)).simd_le(t97);
            let t99 = (f64x8::splat(1.35)).simd_lt(t97);
            let t100 = ((t99).select(t97, f64x8::splat(1.35)));
            let t101 = t100 * t100;
            let t104 = t101 * t101;
            let t105 = f64x8::splat(1.0) / t104;
            let t107 = t104 * t101;
            let t108 = f64x8::splat(1.0) / t107;
            let t110 = t104 * t104;
            let t111 = f64x8::splat(1.0) / t110;
            let t114 = f64x8::splat(1.0) / t110 / t101;
            let t117 = f64x8::splat(1.0) / t110 / t104;
            let t120 = f64x8::splat(1.0) / t110 / t107;
            let t122 = t110 * t110;
            let t123 = f64x8::splat(1.0) / t122;
            let t126 = ((t99).select(f64x8::splat(1.35), t97));
            let t127 = f64x8::splat(1.0) / t126;
            let t129 = (simd::erf(t127 / f64x8::splat(2.0)));
            let t131 = t126 * t126;
            let t132 = f64x8::splat(1.0) / t131;
            let t134 = (simd::exp(-t132 / f64x8::splat(4.0)));
            let t135 = t134 - f64x8::splat(1.0);
            let t138 = t134 - f64x8::splat(3.0) / f64x8::splat(2.0) - f64x8::splat(2.0) * t131 * t135;
            let t141 = f64x8::splat(2.0) * t126 * t138 + t64 * t129;
            let t145 = ((t98).select(f64x8::splat(1.0) / t101 / f64x8::splat(36.0) - t105 / f64x8::splat(960.0) + t108 / f64x8::splat(26880.0) - t111 / f64x8::splat(829440.0) + t114 / f64x8::splat(28385280.0) - t117 / f64x8::splat(1073479680.0) + t120 / f64x8::splat(44590694400.0) - t123 / f64x8::splat(2021444812800.0), f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t126 * t141));
            let t146 = t22 * t145;
            let t148 = t7 * t92 * t146;
            let tzk0 = -f64x8::splat(3.0) / f64x8::splat(32.0) * t86 - f64x8::splat(3.0) / f64x8::splat(32.0) * t148;
            acc_zk = tzk0;
            let t150 = f64x8::splat(3.0) / f64x8::splat(32.0) * t86;
            let t151 = f64x8::splat(3.0) / f64x8::splat(32.0) * t148;
            let t152 = t11 * t11;
            let t153 = f64x8::splat(1.0) / t152;
            let t154 = t10 * t153;
            let t155 = t12 - t154;
            let t158 = ((t15).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t18 * t155));
            let t159 = t9 * t158;
            let t161 = t7 * t159 * t84;
            let t162 = f64x8::splat(3.0) / f64x8::splat(32.0) * t161;
            let t163 = t22 * t22;
            let t164 = f64x8::splat(1.0) / t163;
            let t165 = t164 * t83;
            let t167 = t7 * t21 * t165;
            let t168 = t167 / f64x8::splat(32.0);
            let t169 = t38 * t37;
            let t170 = f64x8::splat(1.0) / t169;
            let t172 = f64x8::splat(1.0) / t22 / t11;
            let t173 = t1 * t172;
            let t176 = t27 * t173 * t31 / f64x8::splat(54.0);
            let t177 = t30 * t30;
            let t178 = f64x8::splat(1.0) / t177;
            let t179 = t18 * t18;
            let t180 = f64x8::splat(1.0) / t179;
            let t181 = t180 * t155;
            let t183 = ((t15).select(f64x8::splat(0.0), t181 / f64x8::splat(3.0)));
            let t184 = t178 * t183;
            let t188 = -t176 - t27 * t29 * t184 / f64x8::splat(18.0);
            let t189 = ((t36).select(t188, f64x8::splat(0.0)));
            let t192 = t41 * t37;
            let t193 = f64x8::splat(1.0) / t192;
            let t196 = t41 * t169;
            let t197 = f64x8::splat(1.0) / t196;
            let t201 = f64x8::splat(1.0) / t47 / t37;
            let t205 = f64x8::splat(1.0) / t47 / t169;
            let t209 = f64x8::splat(1.0) / t47 / t192;
            let t213 = f64x8::splat(1.0) / t47 / t196;
            let t217 = f64x8::splat(1.0) / t59 / t37;
            let t221 = ((t36).select(f64x8::splat(0.0), t188));
            let t223 = t72 * t70;
            let t227 = t69 * t63;
            let t228 = f64x8::splat(1.0) / t227;
            let t232 = t63 * t73;
            let t237 = t228 * t221 * t72 / f64x8::splat(2.0) - f64x8::splat(4.0) * t232 * t221 - t65 * t221 * t72;
            let t240 = -t223 * t221 + f64x8::splat(2.0) * t221 * t76 + f64x8::splat(2.0) * t63 * t237;
            let t244 = ((t35).select(-t170 * t189 / f64x8::splat(18.0) + t193 * t189 / f64x8::splat(240.0) - t197 * t189 / f64x8::splat(4480.0) + t201 * t189 / f64x8::splat(103680.0) - t205 * t189 / f64x8::splat(2838528.0) + t209 * t189 / f64x8::splat(89456640.0) - t213 * t189 / f64x8::splat(3185049600.0) + t217 * t189 / f64x8::splat(126340300800.0), -f64x8::splat(8.0) / f64x8::splat(3.0) * t221 * t79 - f64x8::splat(8.0) / f64x8::splat(3.0) * t63 * t240));
            let t245 = t22 * t244;
            let t247 = t7 * t21 * t245;
            let t248 = f64x8::splat(3.0) / f64x8::splat(32.0) * t247;
            let t249 = -t155;
            let t252 = ((t88).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t89 * t249));
            let t253 = t9 * t252;
            let t255 = t7 * t253 * t146;
            let t256 = f64x8::splat(3.0) / f64x8::splat(32.0) * t255;
            let t257 = t164 * t145;
            let t259 = t7 * t92 * t257;
            let t260 = t259 / f64x8::splat(32.0);
            let t261 = t101 * t100;
            let t262 = f64x8::splat(1.0) / t261;
            let t265 = t27 * t173 * t94 / f64x8::splat(54.0);
            let t266 = t93 * t93;
            let t267 = f64x8::splat(1.0) / t266;
            let t268 = t89 * t89;
            let t269 = f64x8::splat(1.0) / t268;
            let t270 = t269 * t249;
            let t272 = ((t88).select(f64x8::splat(0.0), t270 / f64x8::splat(3.0)));
            let t273 = t267 * t272;
            let t277 = -t265 - t27 * t29 * t273 / f64x8::splat(18.0);
            let t278 = ((t99).select(t277, f64x8::splat(0.0)));
            let t281 = t104 * t100;
            let t282 = f64x8::splat(1.0) / t281;
            let t285 = t104 * t261;
            let t286 = f64x8::splat(1.0) / t285;
            let t290 = f64x8::splat(1.0) / t110 / t100;
            let t294 = f64x8::splat(1.0) / t110 / t261;
            let t298 = f64x8::splat(1.0) / t110 / t281;
            let t302 = f64x8::splat(1.0) / t110 / t285;
            let t306 = f64x8::splat(1.0) / t122 / t100;
            let t310 = ((t99).select(f64x8::splat(0.0), t277));
            let t312 = t134 * t132;
            let t316 = t131 * t126;
            let t317 = f64x8::splat(1.0) / t316;
            let t321 = t126 * t135;
            let t326 = t317 * t310 * t134 / f64x8::splat(2.0) - f64x8::splat(4.0) * t321 * t310 - t127 * t310 * t134;
            let t329 = f64x8::splat(2.0) * t126 * t326 + f64x8::splat(2.0) * t310 * t138 - t312 * t310;
            let t333 = ((t98).select(-t262 * t278 / f64x8::splat(18.0) + t282 * t278 / f64x8::splat(240.0) - t286 * t278 / f64x8::splat(4480.0) + t290 * t278 / f64x8::splat(103680.0) - t294 * t278 / f64x8::splat(2838528.0) + t298 * t278 / f64x8::splat(89456640.0) - t302 * t278 / f64x8::splat(3185049600.0) + t306 * t278 / f64x8::splat(126340300800.0), -f64x8::splat(8.0) / f64x8::splat(3.0) * t126 * t329 - f64x8::splat(8.0) / f64x8::splat(3.0) * t310 * t141));
            let t334 = t22 * t333;
            let t336 = t7 * t92 * t334;
            let t337 = f64x8::splat(3.0) / f64x8::splat(32.0) * t336;
            let tvrho0 = -t150 - t151 + t11 * (-t162 - t168 - t248 - t256 - t260 - t337);
            acc_vrho_0 = tvrho0;
            let t340 = -t12 - t154;
            let t343 = ((t15).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t18 * t340));
            let t344 = t9 * t343;
            let t346 = t7 * t344 * t84;
            let t347 = f64x8::splat(3.0) / f64x8::splat(32.0) * t346;
            let t348 = t180 * t340;
            let t350 = ((t15).select(f64x8::splat(0.0), t348 / f64x8::splat(3.0)));
            let t351 = t178 * t350;
            let t355 = -t176 - t27 * t29 * t351 / f64x8::splat(18.0);
            let t356 = ((t36).select(t355, f64x8::splat(0.0)));
            let t359 = t193 * t356;
            let t361 = t197 * t356;
            let t363 = t201 * t356;
            let t365 = t205 * t356;
            let t367 = t209 * t356;
            let t369 = t213 * t356;
            let t371 = t217 * t356;
            let t374 = ((t36).select(f64x8::splat(0.0), t355));
            let t386 = t228 * t374 * t72 / f64x8::splat(2.0) - f64x8::splat(4.0) * t232 * t374 - t65 * t374 * t72;
            let t389 = -t223 * t374 + f64x8::splat(2.0) * t374 * t76 + f64x8::splat(2.0) * t63 * t386;
            let t393 = ((t35).select(-t170 * t356 / f64x8::splat(18.0) + t359 / f64x8::splat(240.0) - t361 / f64x8::splat(4480.0) + t363 / f64x8::splat(103680.0) - t365 / f64x8::splat(2838528.0) + t367 / f64x8::splat(89456640.0) - t369 / f64x8::splat(3185049600.0) + t371 / f64x8::splat(126340300800.0), -f64x8::splat(8.0) / f64x8::splat(3.0) * t374 * t79 - f64x8::splat(8.0) / f64x8::splat(3.0) * t63 * t389));
            let t394 = t22 * t393;
            let t396 = t7 * t21 * t394;
            let t397 = f64x8::splat(3.0) / f64x8::splat(32.0) * t396;
            let t398 = -t340;
            let t401 = ((t88).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t89 * t398));
            let t402 = t9 * t401;
            let t404 = t7 * t402 * t146;
            let t405 = f64x8::splat(3.0) / f64x8::splat(32.0) * t404;
            let t406 = t269 * t398;
            let t408 = ((t88).select(f64x8::splat(0.0), t406 / f64x8::splat(3.0)));
            let t409 = t267 * t408;
            let t413 = -t265 - t27 * t29 * t409 / f64x8::splat(18.0);
            let t414 = ((t99).select(t413, f64x8::splat(0.0)));
            let t417 = t282 * t414;
            let t419 = t286 * t414;
            let t421 = t290 * t414;
            let t423 = t294 * t414;
            let t425 = t298 * t414;
            let t427 = t302 * t414;
            let t429 = t306 * t414;
            let t432 = ((t99).select(f64x8::splat(0.0), t413));
            let t444 = t317 * t432 * t134 / f64x8::splat(2.0) - f64x8::splat(4.0) * t321 * t432 - t127 * t432 * t134;
            let t447 = f64x8::splat(2.0) * t126 * t444 + f64x8::splat(2.0) * t432 * t138 - t312 * t432;
            let t451 = ((t98).select(-t262 * t414 / f64x8::splat(18.0) + t417 / f64x8::splat(240.0) - t419 / f64x8::splat(4480.0) + t421 / f64x8::splat(103680.0) - t423 / f64x8::splat(2838528.0) + t425 / f64x8::splat(89456640.0) - t427 / f64x8::splat(3185049600.0) + t429 / f64x8::splat(126340300800.0), -f64x8::splat(8.0) / f64x8::splat(3.0) * t126 * t447 - f64x8::splat(8.0) / f64x8::splat(3.0) * t432 * t141));
            let t452 = t22 * t451;
            let t454 = t7 * t92 * t452;
            let t455 = f64x8::splat(3.0) / f64x8::splat(32.0) * t454;
            let tvrho1 = -t150 - t151 + t11 * (-t347 - t168 - t397 - t405 - t260 - t455);
            acc_vrho_1 = tvrho1;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        ip += 8;
    }
}

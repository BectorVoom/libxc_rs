//! LDA_C_RC04 kxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_rc04.c`
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
pub fn lda_c_rc04_kxc_pol(
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
            let t1 = v_rho0 - v_rho1;
            let t2 = v_rho0 + v_rho1;
            let t3 = f64x8::splat(1.0) / t2;
            let t4 = t1 * t3;
            let t5 = f64x8::splat(1.0) + t4;
            let t6 = (t5).simd_le(zeta_threshold);
            let t7 = (simd::cbrt(zeta_threshold));
            let t8 = t7 * t7;
            let t9 = (simd::cbrt(t5));
            let t10 = t9 * t9;
            let t11 = ((t6).select(t8, t10));
            let t12 = f64x8::splat(1.0) - t4;
            let t13 = (t12).simd_le(zeta_threshold);
            let t14 = (simd::cbrt(t12));
            let t15 = t14 * t14;
            let t16 = ((t13).select(t8, t15));
            let t18 = t11 / f64x8::splat(2.0) + t16 / f64x8::splat(2.0);
            let t19 = t18 * t18;
            let t20 = t19 * t18;
            let t21 = f64x8::splat(M_CBRT3);
            let t23 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t24 = t21 * t23;
            let t25 = f64x8::splat(M_CBRT4);
            let t26 = t25 * t25;
            let t27 = (simd::cbrt(t2));
            let t32 = f64x8::splat(4.88827) + f64x8::splat(0.79425925) * t24 * t26 / t27;
            let t33 = (simd::atan(t32));
            let t35 = -f64x8::splat(0.655868) * t33 + f64x8::splat(0.897889);
            let t37 = t21 * t21;
            let t38 = t20 * t35 * t37;
            let t39 = f64x8::splat(1.0) / t23;
            let t40 = t39 * t25;
            let t41 = t40 * t27;
            let t42 = t38 * t41;
            let tzk0 = t42 / f64x8::splat(3.0);
            acc_zk = tzk0;
            let t43 = f64x8::splat(4.0) / f64x8::splat(9.0) * t42;
            let t44 = t27 * t2;
            let t46 = t44 * t19 * t35;
            let t47 = t37 * t39;
            let t48 = f64x8::splat(1.0) / t9;
            let t49 = t2 * t2;
            let t50 = f64x8::splat(1.0) / t49;
            let t51 = t1 * t50;
            let t52 = t3 - t51;
            let t55 = ((t6).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t48 * t52));
            let t56 = f64x8::splat(1.0) / t14;
            let t57 = -t52;
            let t60 = ((t13).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t56 * t57));
            let t62 = t55 / f64x8::splat(2.0) + t60 / f64x8::splat(2.0);
            let t66 = t32 * t32;
            let t67 = t66 + f64x8::splat(1.0);
            let t68 = f64x8::splat(1.0) / t67;
            let t70 = f64x8::splat(0.6945723010386666) * t20 * t68;
            let tvrho0 = t46 * t47 * t25 * t62 + t43 + t70;
            acc_vrho_0 = tvrho0;
            let t71 = -t3 - t51;
            let t74 = ((t6).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t48 * t71));
            let t75 = -t71;
            let t78 = ((t13).select(f64x8::splat(0.0), f64x8::splat(2.0) / f64x8::splat(3.0) * t56 * t75));
            let t80 = t74 / f64x8::splat(2.0) + t78 / f64x8::splat(2.0);
            let t82 = t47 * t25 * t80;
            let tvrho1 = t46 * t82 + t43 + t70;
            acc_vrho_1 = tvrho1;
            let t85 = t19 * t35 * t37;
            let t86 = t27 * t62;
            let t88 = t85 * t40 * t86;
            let t92 = f64x8::splat(0.9260964013848889) * t20 * t3 * t68;
            let t93 = t27 * t27;
            let t94 = f64x8::splat(1.0) / t93;
            let t97 = f64x8::splat(4.0) / f64x8::splat(27.0) * t38 * t40 * t94;
            let t98 = t44 * t18;
            let t99 = t98 * t35;
            let t100 = t62 * t62;
            let t105 = t19 * t68;
            let t106 = t105 * t62;
            let t109 = f64x8::splat(1.0) / t9 / t5;
            let t110 = t52 * t52;
            let t113 = t49 * t2;
            let t114 = f64x8::splat(1.0) / t113;
            let t115 = t1 * t114;
            let t117 = -f64x8::splat(2.0) * t50 + f64x8::splat(2.0) * t115;
            let t121 = ((t6).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(9.0) * t109 * t110 + f64x8::splat(2.0) / f64x8::splat(3.0) * t48 * t117));
            let t123 = f64x8::splat(1.0) / t14 / t12;
            let t124 = t57 * t57;
            let t127 = -t117;
            let t131 = ((t13).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(9.0) * t123 * t124 + f64x8::splat(2.0) / f64x8::splat(3.0) * t56 * t127));
            let t133 = t121 / f64x8::splat(2.0) + t131 / f64x8::splat(2.0);
            let t135 = t47 * t25 * t133;
            let t137 = t67 * t67;
            let t138 = f64x8::splat(1.0) / t137;
            let t139 = t20 * t138;
            let t141 = f64x8::splat(1.0) / t44;
            let t143 = t24 * t26 * t141;
            let t145 = f64x8::splat(0.3677803165958304) * t139 * t32 * t143;
            let tv2rho20 = f64x8::splat(8.0) / f64x8::splat(3.0) * t88 + t92 + t97 + f64x8::splat(2.0) * t99 * t47 * t25 * t100 + f64x8::splat(4.167433806232) * t106 + t46 * t135 + t145;
            acc_v2rho2_0 = tv2rho20;
            let t148 = t27 * t19 * t35;
            let t149 = t148 * t82;
            let t151 = t35 * t37;
            let t152 = t98 * t151;
            let t153 = t80 * t62;
            let t154 = t40 * t153;
            let t157 = t105 * t80;
            let t159 = t109 * t71;
            let t162 = t48 * t1;
            let t166 = ((t6).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(9.0) * t159 * t52 + f64x8::splat(4.0) / f64x8::splat(3.0) * t162 * t114));
            let t167 = t123 * t75;
            let t170 = t56 * t1;
            let t174 = ((t13).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(9.0) * t167 * t57 - f64x8::splat(4.0) / f64x8::splat(3.0) * t170 * t114));
            let t176 = t166 / f64x8::splat(2.0) + t174 / f64x8::splat(2.0);
            let t178 = t47 * t25 * t176;
            let tv2rho21 = f64x8::splat(4.0) / f64x8::splat(3.0) * t88 + t92 + t97 + f64x8::splat(4.0) / f64x8::splat(3.0) * t149 + f64x8::splat(2.0) * t152 * t154 + f64x8::splat(2.083716903116) * t157 + t46 * t178 + f64x8::splat(2.083716903116) * t106 + t145;
            acc_v2rho2_1 = tv2rho21;
            let t182 = t80 * t80;
            let t184 = t47 * t25 * t182;
            let t188 = t71 * t71;
            let t192 = f64x8::splat(2.0) * t50 + f64x8::splat(2.0) * t115;
            let t196 = ((t6).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(9.0) * t109 * t188 + f64x8::splat(2.0) / f64x8::splat(3.0) * t48 * t192));
            let t197 = t75 * t75;
            let t200 = -t192;
            let t204 = ((t13).select(f64x8::splat(0.0), -f64x8::splat(2.0) / f64x8::splat(9.0) * t123 * t197 + f64x8::splat(2.0) / f64x8::splat(3.0) * t56 * t200));
            let t206 = t196 / f64x8::splat(2.0) + t204 / f64x8::splat(2.0);
            let t208 = t47 * t25 * t206;
            let tv2rho22 = f64x8::splat(8.0) / f64x8::splat(3.0) * t149 + t92 + t97 + f64x8::splat(2.0) * t99 * t184 + f64x8::splat(4.167433806232) * t157 + t46 * t208 + t145;
            acc_v2rho2_2 = tv2rho22;
            let t210 = t19 * t3;
            let t211 = t68 * t62;
            let t212 = t210 * t211;
            let t216 = f64x8::splat(0.6173976009232592) * t20 * t50 * t68;
            let t217 = t18 * t35;
            let t218 = t217 * t37;
            let t219 = t27 * t100;
            let t221 = t218 * t40 * t219;
            let t225 = t85 * t40 * t94 * t62;
            let t227 = t62 * t133;
            let t231 = t19 * t138;
            let t234 = t231 * t62 * t32 * t143;
            let t236 = t18 * t68;
            let t237 = t236 * t100;
            let t239 = t105 * t133;
            let t241 = t139 * t37;
            let t242 = t23 * t23;
            let t243 = t242 * t25;
            let t245 = f64x8::splat(1.0) / t93 / t49;
            let t248 = f64x8::splat(0.38948389123222243) * t241 * t243 * t245;
            let t251 = t85 * t40 * t27 * t133;
            let t254 = f64x8::splat(1.0) / t27 / t49;
            let t257 = t32 * t21;
            let t258 = t23 * t26;
            let t259 = t257 * t258;
            let t261 = f64x8::splat(1e-20) * t20 * t254 * t138 * t259;
            let t263 = f64x8::splat(1.0) / t93 / t2;
            let t266 = f64x8::splat(8.0) / f64x8::splat(81.0) * t38 * t40 * t263;
            let t267 = t100 * t62;
            let t270 = t47 * t25;
            let t273 = t5 * t5;
            let t275 = f64x8::splat(1.0) / t9 / t273;
            let t276 = t110 * t52;
            let t279 = t109 * t52;
            let t282 = t49 * t49;
            let t283 = f64x8::splat(1.0) / t282;
            let t284 = t1 * t283;
            let t286 = f64x8::splat(6.0) * t114 - f64x8::splat(6.0) * t284;
            let t290 = ((t6).select(f64x8::splat(0.0), f64x8::splat(8.0) / f64x8::splat(27.0) * t275 * t276 - f64x8::splat(2.0) / f64x8::splat(3.0) * t279 * t117 + f64x8::splat(2.0) / f64x8::splat(3.0) * t48 * t286));
            let t291 = t12 * t12;
            let t293 = f64x8::splat(1.0) / t14 / t291;
            let t294 = t124 * t57;
            let t297 = t123 * t57;
            let t300 = -t286;
            let t304 = ((t13).select(f64x8::splat(0.0), f64x8::splat(8.0) / f64x8::splat(27.0) * t293 * t294 - f64x8::splat(2.0) / f64x8::splat(3.0) * t297 * t127 + f64x8::splat(2.0) / f64x8::splat(3.0) * t56 * t300));
            let t306 = t290 / f64x8::splat(2.0) + t304 / f64x8::splat(2.0);
            let t311 = f64x8::splat(1.0) / t137 / t67;
            let t312 = t20 * t311;
            let t316 = t37 * t242 * t25 * t245;
            let t318 = f64x8::splat(1.5579355649288897) * t312 * t66 * t316;
            let tv3rho30 = f64x8::splat(8.334867612464) * t212 - t216 + f64x8::splat(8.0) * t221 + f64x8::splat(4.0) / f64x8::splat(3.0) * t225 + f64x8::splat(6.0) * t152 * t40 * t227 + f64x8::splat(3.3100228493624737) * t234 + f64x8::splat(12.502301418696) * t237 + f64x8::splat(6.251150709348) * t239 - t248 + f64x8::splat(4.0) * t251 - t261 - t266 + f64x8::splat(2.0) * t44 * t267 * t35 * t270 + t46 * t47 * t25 * t306 + t318;
            acc_v3rho3_0 = tv3rho30;
            let t320 = t68 * t80;
            let t321 = t210 * t320;
            let t324 = f64x8::splat(8.334867612464) * t236 * t153;
            let t329 = t94 * t19 * t35;
            let t330 = t329 * t82;
            let t332 = t27 * t18;
            let t333 = t332 * t151;
            let t335 = f64x8::splat(16.0) / f64x8::splat(3.0) * t333 * t154;
            let t337 = t44 * t100 * t35;
            let t340 = t176 * t62;
            let t341 = t40 * t340;
            let t344 = f64x8::splat(5.556578408309333) * t212 - t216 + f64x8::splat(2.7782892041546665) * t321 + t324 + f64x8::splat(8.0) / f64x8::splat(3.0) * t221 + f64x8::splat(8.0) / f64x8::splat(9.0) * t225 + f64x8::splat(2.2066818995749826) * t234 + f64x8::splat(4.0) / f64x8::splat(9.0) * t330 + t335 + f64x8::splat(2.0) * t337 * t82 + f64x8::splat(4.0) * t152 * t341;
            let t345 = t80 * t133;
            let t346 = t40 * t345;
            let t351 = t231 * t80 * t32 * t143;
            let t356 = f64x8::splat(4.167433806232) * t105 * t176;
            let t359 = f64x8::splat(8.0) / f64x8::splat(3.0) * t148 * t178;
            let t360 = t275 * t71;
            let t363 = t109 * t1;
            let t374 = ((t6).select(f64x8::splat(0.0), f64x8::splat(8.0) / f64x8::splat(27.0) * t360 * t110 - f64x8::splat(8.0) / f64x8::splat(9.0) * t363 * t114 * t52 - f64x8::splat(2.0) / f64x8::splat(9.0) * t159 * t117 + f64x8::splat(4.0) / f64x8::splat(3.0) * t48 * t114 - f64x8::splat(4.0) * t162 * t283));
            let t375 = t293 * t75;
            let t378 = t123 * t1;
            let t389 = ((t13).select(f64x8::splat(0.0), f64x8::splat(8.0) / f64x8::splat(27.0) * t375 * t124 + f64x8::splat(8.0) / f64x8::splat(9.0) * t378 * t114 * t57 - f64x8::splat(2.0) / f64x8::splat(9.0) * t167 * t127 - f64x8::splat(4.0) / f64x8::splat(3.0) * t56 * t114 + f64x8::splat(4.0) * t170 * t283));
            let t391 = t374 / f64x8::splat(2.0) + t389 / f64x8::splat(2.0);
            let t393 = t47 * t25 * t391;
            let t395 = f64x8::splat(2.0) * t152 * t346 + f64x8::splat(1.1033409497874913) * t351 + f64x8::splat(4.167433806232) * t237 + f64x8::splat(2.083716903116) * t239 + t356 - t248 + f64x8::splat(4.0) / f64x8::splat(3.0) * t251 - t261 - t266 + t318 + t359 + t46 * t393;
            let tv3rho31 = t344 + t395;
            acc_v3rho3_1 = tv3rho31;
            let t403 = t332 * t35 * t184;
            let t405 = t80 * t176;
            let t406 = t40 * t405;
            let t409 = f64x8::splat(2.7782892041546665) * t212 - t216 + f64x8::splat(5.556578408309333) * t321 + t324 + f64x8::splat(4.0) / f64x8::splat(9.0) * t225 + f64x8::splat(1.1033409497874913) * t234 + f64x8::splat(8.0) / f64x8::splat(9.0) * t330 + t335 + f64x8::splat(2.2066818995749826) * t351 + f64x8::splat(8.0) / f64x8::splat(3.0) * t403 + f64x8::splat(4.0) * t152 * t406;
            let t410 = t148 * t208;
            let t412 = t206 * t62;
            let t413 = t40 * t412;
            let t416 = t236 * t182;
            let t418 = t105 * t206;
            let t420 = t44 * t62;
            let t424 = t275 * t188;
            let t429 = t109 * t192;
            let t434 = -f64x8::splat(2.0) * t114 - f64x8::splat(6.0) * t284;
            let t438 = ((t6).select(f64x8::splat(0.0), f64x8::splat(8.0) / f64x8::splat(27.0) * t424 * t52 - f64x8::splat(8.0) / f64x8::splat(9.0) * t159 * t115 - f64x8::splat(2.0) / f64x8::splat(9.0) * t429 * t52 + f64x8::splat(2.0) / f64x8::splat(3.0) * t48 * t434));
            let t439 = t293 * t197;
            let t444 = t123 * t200;
            let t447 = -t434;
            let t451 = ((t13).select(f64x8::splat(0.0), f64x8::splat(8.0) / f64x8::splat(27.0) * t439 * t57 + f64x8::splat(8.0) / f64x8::splat(9.0) * t167 * t115 - f64x8::splat(2.0) / f64x8::splat(9.0) * t444 * t57 + f64x8::splat(2.0) / f64x8::splat(3.0) * t56 * t447));
            let t453 = t438 / f64x8::splat(2.0) + t451 / f64x8::splat(2.0);
            let t455 = t47 * t25 * t453;
            let t457 = f64x8::splat(4.0) / f64x8::splat(3.0) * t410 + f64x8::splat(2.0) * t152 * t413 + t356 + f64x8::splat(4.167433806232) * t416 + f64x8::splat(2.083716903116) * t418 - t248 - t261 - t266 + t318 + t359 + f64x8::splat(2.0) * t420 * t35 * t184 + t46 * t455;
            let tv3rho32 = t409 + t457;
            acc_v3rho3_2 = tv3rho32;
            let t463 = t80 * t206;
            let t464 = t40 * t463;
            let t469 = t182 * t80;
            let t474 = t188 * t71;
            let t480 = -f64x8::splat(6.0) * t114 - f64x8::splat(6.0) * t284;
            let t484 = ((t6).select(f64x8::splat(0.0), f64x8::splat(8.0) / f64x8::splat(27.0) * t275 * t474 - f64x8::splat(2.0) / f64x8::splat(3.0) * t159 * t192 + f64x8::splat(2.0) / f64x8::splat(3.0) * t48 * t480));
            let t485 = t197 * t75;
            let t490 = -t480;
            let t494 = ((t13).select(f64x8::splat(0.0), f64x8::splat(8.0) / f64x8::splat(27.0) * t293 * t485 - f64x8::splat(2.0) / f64x8::splat(3.0) * t167 * t200 + f64x8::splat(2.0) / f64x8::splat(3.0) * t56 * t490));
            let t496 = t484 / f64x8::splat(2.0) + t494 / f64x8::splat(2.0);
            let t498 = t47 * t25 * t496;
            let tv3rho33 = -t216 + f64x8::splat(8.334867612464) * t321 + f64x8::splat(4.0) / f64x8::splat(3.0) * t330 + f64x8::splat(3.3100228493624737) * t351 + f64x8::splat(8.0) * t403 + f64x8::splat(4.0) * t410 + f64x8::splat(6.0) * t152 * t464 + f64x8::splat(12.502301418696) * t416 + f64x8::splat(6.251150709348) * t418 - t248 - t261 - t266 + t318 + f64x8::splat(2.0) * t44 * t469 * t35 * t270 + t46 * t498;
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

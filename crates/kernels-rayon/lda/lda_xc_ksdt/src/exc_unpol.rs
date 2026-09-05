//! LDA_XC_KSDT exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_xc_ksdt.c`
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
pub fn lda_xc_ksdt_exc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    param_T: f64,
    param_b_0_1: f64,
    param_b_0_2: f64,
    param_b_0_0: f64,
    param_b_0_3: f64,
    param_b_0_4: f64,
    param_c_0_1: f64,
    param_c_0_2: f64,
    param_c_0_0: f64,
    param_e_0_1: f64,
    param_e_0_2: f64,
    param_e_0_0: f64,
    param_e_0_3: f64,
    param_e_0_4: f64,
    param_d_0_1: f64,
    param_d_0_2: f64,
    param_d_0_0: f64,
    param_d_0_3: f64,
    param_d_0_4: f64,
    param_b_1_1: f64,
    param_b_1_2: f64,
    param_b_1_0: f64,
    param_b_1_3: f64,
    param_b_1_4: f64,
    param_c_1_1: f64,
    param_c_1_2: f64,
    param_c_1_0: f64,
    param_e_1_1: f64,
    param_e_1_2: f64,
    param_e_1_0: f64,
    param_e_1_3: f64,
    param_e_1_4: f64,
    param_d_1_1: f64,
    param_d_1_2: f64,
    param_d_1_0: f64,
    param_d_1_3: f64,
    param_d_1_4: f64,
    param_thetaParam: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_T = f64x8::splat(param_T);
    let param_b_0_1 = f64x8::splat(param_b_0_1);
    let param_b_0_2 = f64x8::splat(param_b_0_2);
    let param_b_0_0 = f64x8::splat(param_b_0_0);
    let param_b_0_3 = f64x8::splat(param_b_0_3);
    let param_b_0_4 = f64x8::splat(param_b_0_4);
    let param_c_0_1 = f64x8::splat(param_c_0_1);
    let param_c_0_2 = f64x8::splat(param_c_0_2);
    let param_c_0_0 = f64x8::splat(param_c_0_0);
    let param_e_0_1 = f64x8::splat(param_e_0_1);
    let param_e_0_2 = f64x8::splat(param_e_0_2);
    let param_e_0_0 = f64x8::splat(param_e_0_0);
    let param_e_0_3 = f64x8::splat(param_e_0_3);
    let param_e_0_4 = f64x8::splat(param_e_0_4);
    let param_d_0_1 = f64x8::splat(param_d_0_1);
    let param_d_0_2 = f64x8::splat(param_d_0_2);
    let param_d_0_0 = f64x8::splat(param_d_0_0);
    let param_d_0_3 = f64x8::splat(param_d_0_3);
    let param_d_0_4 = f64x8::splat(param_d_0_4);
    let param_b_1_1 = f64x8::splat(param_b_1_1);
    let param_b_1_2 = f64x8::splat(param_b_1_2);
    let param_b_1_0 = f64x8::splat(param_b_1_0);
    let param_b_1_3 = f64x8::splat(param_b_1_3);
    let param_b_1_4 = f64x8::splat(param_b_1_4);
    let param_c_1_1 = f64x8::splat(param_c_1_1);
    let param_c_1_2 = f64x8::splat(param_c_1_2);
    let param_c_1_0 = f64x8::splat(param_c_1_0);
    let param_e_1_1 = f64x8::splat(param_e_1_1);
    let param_e_1_2 = f64x8::splat(param_e_1_2);
    let param_e_1_0 = f64x8::splat(param_e_1_0);
    let param_e_1_3 = f64x8::splat(param_e_1_3);
    let param_e_1_4 = f64x8::splat(param_e_1_4);
    let param_d_1_1 = f64x8::splat(param_d_1_1);
    let param_d_1_2 = f64x8::splat(param_d_1_2);
    let param_d_1_0 = f64x8::splat(param_d_1_0);
    let param_d_1_3 = f64x8::splat(param_d_1_3);
    let param_d_1_4 = f64x8::splat(param_d_1_4);
    let param_thetaParam = f64x8::splat(param_thetaParam);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let mut acc_zk = V_ZERO;
        {
            let t1 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t2 = f64x8::splat(M_CBRT4);
            let t3 = t2 * t2;
            let t4 = t1 * t3;
            let t5 = (simd::cbrt(f64x8::splat(9.0)));
            let t6 = t4 * t5;
            let t7 = (simd::cbrt(t1));
            let t8 = f64x8::splat(1.0) / t7;
            let t9 = t5 * t5;
            let t10 = t7 * t1;
            let t11 = f64x8::splat(1.0) / t10;
            let t12 = t9 * t11;
            let t13 = f64x8::splat(1.0) / param_T;
            let t14 = f64x8::splat(M_CBRT3);
            let t15 = t13 * t14;
            let t16 = (simd::cbrt(v_rho));
            let t17 = t16 * t16;
            let t18 = t15 * t17;
            let t21 = (simd::tanh(t12 * t18 / f64x8::splat(6.0)));
            let t22 = t8 * t21;
            let t23 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t24 = f64x8::splat(1.0) / t23;
            let t25 = t7 * t7;
            let t26 = t25 * t24;
            let t27 = t9 * t26;
            let t28 = param_T * param_T;
            let t29 = t28 * t14;
            let t31 = f64x8::splat(1.0) / t16 / v_rho;
            let t32 = t29 * t31;
            let t33 = t27 * t32;
            let t35 = t28 * param_T;
            let t36 = v_rho * v_rho;
            let t37 = f64x8::splat(1.0) / t36;
            let t38 = t35 * t37;
            let t40 = t23 * t23;
            let t41 = t40 * f64x8::splat(M_PI);
            let t43 = t7 / t41;
            let t44 = t5 * t43;
            let t45 = t28 * t28;
            let t46 = t14 * t14;
            let t47 = t45 * t46;
            let t49 = f64x8::splat(1.0) / t17 / t36;
            let t50 = t47 * t49;
            let t51 = t44 * t50;
            let t53 = f64x8::splat(0.75) + f64x8::splat(0.45090814814814817) * t33 - f64x8::splat(0.0008419930512353099) * t38 + f64x8::splat(0.3364938271604938) * t51;
            let t56 = f64x8::splat(1.0) + f64x8::splat(1.2311866666666667) * t33 + f64x8::splat(1.0094814814814814) * t51;
            let t57 = f64x8::splat(1.0) / t56;
            let t58 = t53 * t57;
            let t62 = f64x8::splat(M_SQRT2);
            let t63 = t5 * t10;
            let t64 = param_T * t46;
            let t65 = f64x8::splat(1.0) / t17;
            let t67 = t63 * t64 * t65;
            let t68 = ((t67).sqrt());
            let t72 = (simd::tanh(f64x8::splat(3.0) / f64x8::splat(2.0) * t62 / t68));
            let t76 = param_b_0_1 * t9 * t26;
            let t81 = param_b_0_2 * t5 * t43;
            let t84 = param_b_0_0 + f64x8::splat(4.0) / f64x8::splat(27.0) * t76 * t32 + f64x8::splat(16.0) / f64x8::splat(81.0) * t81 * t50;
            let t85 = t72 * t84;
            let t88 = param_b_0_3 * t9 * t26;
            let t93 = param_b_0_4 * t5 * t43;
            let t96 = f64x8::splat(1.0) + f64x8::splat(4.0) / f64x8::splat(27.0) * t88 * t32 + f64x8::splat(16.0) / f64x8::splat(81.0) * t93 * t50;
            let t97 = f64x8::splat(1.0) / t96;
            let t98 = t14 * t7;
            let t99 = f64x8::splat(1.0) / t16;
            let t100 = t3 * t99;
            let t101 = t98 * t100;
            let t102 = ((t101).sqrt());
            let t103 = t97 * t102;
            let t107 = param_c_0_1;
            let t108 = param_c_0_2;
            let t113 = (simd::exp(-t108 * t9 * t11 * t18 / f64x8::splat(6.0)));
            let t115 = t107 * t113 + param_c_0_0;
            let t116 = t115 * t21;
            let t120 = param_e_0_1 * t9 * t26;
            let t125 = param_e_0_2 * t5 * t43;
            let t128 = param_e_0_0 + f64x8::splat(4.0) / f64x8::splat(27.0) * t120 * t32 + f64x8::splat(16.0) / f64x8::splat(81.0) * t125 * t50;
            let t131 = param_e_0_3 * t9 * t26;
            let t136 = param_e_0_4 * t5 * t43;
            let t139 = f64x8::splat(1.0) + f64x8::splat(4.0) / f64x8::splat(27.0) * t131 * t32 + f64x8::splat(16.0) / f64x8::splat(81.0) * t136 * t50;
            let t140 = f64x8::splat(1.0) / t139;
            let t141 = t128 * t140;
            let t142 = t116 * t141;
            let t146 = (t6 * t22 * t58 / f64x8::splat(4.0) + t85 * t103 / f64x8::splat(2.0) + t142 * t101 / f64x8::splat(4.0)) * t46;
            let t147 = t146 * t8;
            let t148 = t2 * t16;
            let t152 = param_d_0_1 * t9 * t26;
            let t157 = param_d_0_2 * t5 * t43;
            let t160 = param_d_0_0 + f64x8::splat(4.0) / f64x8::splat(27.0) * t152 * t32 + f64x8::splat(16.0) / f64x8::splat(81.0) * t157 * t50;
            let t161 = t72 * t160;
            let t164 = param_d_0_3 * t9 * t26;
            let t169 = param_d_0_4 * t5 * t43;
            let t172 = f64x8::splat(1.0) + f64x8::splat(4.0) / f64x8::splat(27.0) * t164 * t32 + f64x8::splat(16.0) / f64x8::splat(81.0) * t169 * t50;
            let t173 = f64x8::splat(1.0) / t172;
            let t174 = t173 * t102;
            let t177 = t21 * t128;
            let t178 = t177 * t140;
            let t181 = f64x8::splat(1.0) + t161 * t174 / f64x8::splat(2.0) + t178 * t101 / f64x8::splat(4.0);
            let t182 = f64x8::splat(1.0) / t181;
            let t183 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t185 = f64x8::splat(2.0) / f64x8::splat(3.0) - f64x8::splat(0.003481525) * t101;
            let t187 = f64x8::splat(1.0) + f64x8::splat(0.045802) * t101;
            let t188 = f64x8::splat(1.0) / t187;
            let t189 = t185 * t188;
            let t190 = t63 * param_T;
            let t191 = t46 * t65;
            let t195 = f64x8::splat(1.064009) + f64x8::splat(0.06361833333333333) * t190 * t191 * t102;
            let t199 = (simd::exp(-f64x8::splat(2.0) / f64x8::splat(9.0) * t190 * t191 * t195));
            let t201 = -t189 * t199 + f64x8::splat(2.0);
            let t202 = (simd::pow(zeta_threshold, t201));
            let t203 = ((t183).select(t202, f64x8::splat(1.0)));
            let t205 = f64x8::splat(2.0) * t203 - f64x8::splat(2.0);
            let t206 = (simd::pow(f64x8::splat(2.0), t201));
            let t207 = t206 - f64x8::splat(2.0);
            let t208 = f64x8::splat(1.0) / t207;
            let t209 = t205 * t208;
            let t210 = f64x8::splat(1.0) - t209;
            let t211 = t182 * t210;
            let t212 = t148 * t211;
            let t213 = t147 * t212;
            let t214 = f64x8::splat(M_CBRT2);
            let t215 = t214 * t1;
            let t216 = t3 * t5;
            let t217 = t215 * t216;
            let t220 = t214 * t214;
            let t224 = (simd::tanh(t12 * t13 * t14 * t17 * t220 / f64x8::splat(6.0)));
            let t225 = t8 * t224;
            let t226 = t27 * t28;
            let t227 = t14 * t31;
            let t229 = t226 * t227 * t220;
            let t232 = t44 * t45;
            let t233 = t46 * t49;
            let t235 = t232 * t233 * t214;
            let t237 = f64x8::splat(0.75) + f64x8::splat(0.11272703703703704) * t229 - f64x8::splat(0.00021049826280882748) * t38 + f64x8::splat(0.042061728395061726) * t235;
            let t240 = f64x8::splat(1.0) + f64x8::splat(0.30779666666666666) * t229 + f64x8::splat(0.12618518518518518) * t235;
            let t241 = f64x8::splat(1.0) / t240;
            let t247 = t190 * t191 * t214;
            let t248 = ((t247).sqrt());
            let t251 = (simd::tanh(f64x8::splat(3.0) / t248));
            let t255 = param_b_1_1 * t9 * t26;
            let t257 = t29 * t31 * t220;
            let t262 = param_b_1_2 * t5 * t43;
            let t263 = t49 * t214;
            let t264 = t47 * t263;
            let t267 = param_b_1_0 + t255 * t257 / f64x8::splat(27.0) + f64x8::splat(2.0) / f64x8::splat(81.0) * t262 * t264;
            let t268 = t251 * t267;
            let t271 = param_b_1_3 * t9 * t26;
            let t276 = param_b_1_4 * t5 * t43;
            let t279 = f64x8::splat(1.0) + t271 * t257 / f64x8::splat(27.0) + f64x8::splat(2.0) / f64x8::splat(81.0) * t276 * t264;
            let t280 = f64x8::splat(1.0) / t279;
            let t281 = t280 * t102;
            let t285 = param_c_1_1;
            let t286 = param_c_1_2;
            let t293 = (simd::exp(-t286 * t9 * t11 * t15 * t17 * t220 / f64x8::splat(6.0)));
            let t295 = t285 * t293 + param_c_1_0;
            let t296 = t295 * t224;
            let t300 = param_e_1_1 * t9 * t26;
            let t305 = param_e_1_2 * t5 * t43;
            let t308 = param_e_1_0 + t300 * t257 / f64x8::splat(27.0) + f64x8::splat(2.0) / f64x8::splat(81.0) * t305 * t264;
            let t311 = param_e_1_3 * t9 * t26;
            let t316 = param_e_1_4 * t5 * t43;
            let t319 = f64x8::splat(1.0) + t311 * t257 / f64x8::splat(27.0) + f64x8::splat(2.0) / f64x8::splat(81.0) * t316 * t264;
            let t320 = f64x8::splat(1.0) / t319;
            let t321 = t308 * t320;
            let t322 = t296 * t321;
            let t326 = (t217 * t225 * t237 * t241 / f64x8::splat(4.0) + t268 * t281 / f64x8::splat(2.0) + t322 * t101 / f64x8::splat(4.0)) * t46;
            let t327 = t8 * t2;
            let t328 = t326 * t327;
            let t332 = param_d_1_1 * t9 * t26;
            let t337 = param_d_1_2 * t5 * t43;
            let t340 = param_d_1_0 + t332 * t257 / f64x8::splat(27.0) + f64x8::splat(2.0) / f64x8::splat(81.0) * t337 * t264;
            let t341 = t251 * t340;
            let t344 = param_d_1_3 * t9 * t26;
            let t349 = param_d_1_4 * t5 * t43;
            let t352 = f64x8::splat(1.0) + t344 * t257 / f64x8::splat(27.0) + f64x8::splat(2.0) / f64x8::splat(81.0) * t349 * t264;
            let t353 = f64x8::splat(1.0) / t352;
            let t354 = t353 * t102;
            let t357 = t224 * t308;
            let t358 = t357 * t320;
            let t361 = f64x8::splat(1.0) + t341 * t354 / f64x8::splat(2.0) + t358 * t101 / f64x8::splat(4.0);
            let t362 = f64x8::splat(1.0) / t361;
            let t363 = t16 * t362;
            let t364 = t363 * t209;
            let t365 = t328 * t364;
            let tzk0 = -t213 / f64x8::splat(3.0) - t365 / f64x8::splat(3.0);
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}

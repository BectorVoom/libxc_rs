//! LDA_XC_KSDT exc pol kernel — explicit SIMD (bit-exact).
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
pub fn lda_xc_ksdt_exc_pol(
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
        let v_rho0 = load_strided(rho, ip, np, 2, 0);
        let v_rho1 = load_strided(rho, ip, np, 2, 1);
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
            let t14 = t12 * t13;
            let t15 = f64x8::splat(M_CBRT3);
            let t16 = v_rho0 + v_rho1;
            let t17 = (simd::cbrt(t16));
            let t18 = t17 * t17;
            let t19 = t15 * t18;
            let t20 = v_rho0 - v_rho1;
            let t21 = param_thetaParam * t20;
            let t22 = f64x8::splat(1.0) / t16;
            let t24 = t21 * t22 + f64x8::splat(1.0);
            let t25 = (simd::cbrt(t24));
            let t26 = t25 * t25;
            let t27 = f64x8::splat(1.0) / t26;
            let t31 = (simd::tanh(t14 * t19 * t27 / f64x8::splat(6.0)));
            let t32 = t8 * t31;
            let t33 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t34 = f64x8::splat(1.0) / t33;
            let t35 = t7 * t7;
            let t36 = t35 * t34;
            let t37 = t9 * t36;
            let t38 = param_T * param_T;
            let t39 = t37 * t38;
            let t40 = t17 * t16;
            let t41 = f64x8::splat(1.0) / t40;
            let t42 = t15 * t41;
            let t43 = t25 * t24;
            let t45 = t39 * t42 * t43;
            let t47 = t38 * param_T;
            let t48 = t16 * t16;
            let t49 = f64x8::splat(1.0) / t48;
            let t50 = t47 * t49;
            let t51 = t24 * t24;
            let t52 = t50 * t51;
            let t54 = t33 * t33;
            let t55 = t54 * f64x8::splat(M_PI);
            let t57 = t7 / t55;
            let t58 = t5 * t57;
            let t59 = t38 * t38;
            let t60 = t58 * t59;
            let t61 = t15 * t15;
            let t62 = t18 * t48;
            let t63 = f64x8::splat(1.0) / t62;
            let t64 = t61 * t63;
            let t65 = t26 * t51;
            let t67 = t60 * t64 * t65;
            let t69 = f64x8::splat(0.75) + f64x8::splat(0.45090814814814817) * t45 - f64x8::splat(0.0008419930512353099) * t52 + f64x8::splat(0.3364938271604938) * t67;
            let t72 = f64x8::splat(1.0) + f64x8::splat(1.2311866666666667) * t45 + f64x8::splat(1.0094814814814814) * t67;
            let t73 = f64x8::splat(1.0) / t72;
            let t74 = t69 * t73;
            let t78 = f64x8::splat(M_SQRT2);
            let t79 = t5 * t10;
            let t80 = t79 * param_T;
            let t81 = f64x8::splat(1.0) / t18;
            let t82 = t61 * t81;
            let t84 = t80 * t82 * t26;
            let t85 = ((t84).sqrt());
            let t89 = (simd::tanh(f64x8::splat(3.0) / f64x8::splat(2.0) * t78 / t85));
            let t92 = param_b_0_1 * t9;
            let t93 = t92 * t36;
            let t94 = t38 * t15;
            let t96 = t94 * t41 * t43;
            let t100 = param_b_0_2 * t5;
            let t101 = t100 * t57;
            let t102 = t59 * t61;
            let t104 = t102 * t63 * t65;
            let t107 = param_b_0_0 + f64x8::splat(4.0) / f64x8::splat(27.0) * t93 * t96 + f64x8::splat(16.0) / f64x8::splat(81.0) * t101 * t104;
            let t108 = t89 * t107;
            let t110 = param_b_0_3 * t9;
            let t111 = t110 * t36;
            let t115 = param_b_0_4 * t5;
            let t116 = t115 * t57;
            let t119 = f64x8::splat(1.0) + f64x8::splat(4.0) / f64x8::splat(27.0) * t111 * t96 + f64x8::splat(16.0) / f64x8::splat(81.0) * t116 * t104;
            let t120 = f64x8::splat(1.0) / t119;
            let t121 = t15 * t7;
            let t122 = f64x8::splat(1.0) / t17;
            let t123 = t3 * t122;
            let t124 = t121 * t123;
            let t125 = ((t124).sqrt());
            let t126 = t120 * t125;
            let t130 = param_c_0_1;
            let t132 = param_c_0_2 * t9;
            let t133 = t132 * t11;
            let t134 = t13 * t15;
            let t139 = (simd::exp(-t133 * t134 * t18 * t27 / f64x8::splat(6.0)));
            let t141 = t130 * t139 + param_c_0_0;
            let t142 = t141 * t31;
            let t145 = param_e_0_1 * t9;
            let t146 = t145 * t36;
            let t150 = param_e_0_2 * t5;
            let t151 = t150 * t57;
            let t154 = param_e_0_0 + f64x8::splat(4.0) / f64x8::splat(27.0) * t146 * t96 + f64x8::splat(16.0) / f64x8::splat(81.0) * t151 * t104;
            let t156 = param_e_0_3 * t9;
            let t157 = t156 * t36;
            let t161 = param_e_0_4 * t5;
            let t162 = t161 * t57;
            let t165 = f64x8::splat(1.0) + f64x8::splat(4.0) / f64x8::splat(27.0) * t157 * t96 + f64x8::splat(16.0) / f64x8::splat(81.0) * t162 * t104;
            let t166 = f64x8::splat(1.0) / t165;
            let t167 = t154 * t166;
            let t168 = t142 * t167;
            let t172 = (t6 * t32 * t74 / f64x8::splat(4.0) + t108 * t126 / f64x8::splat(2.0) + t168 * t124 / f64x8::splat(4.0)) * t61;
            let t173 = t172 * t8;
            let t174 = t2 * t17;
            let t177 = param_d_0_1 * t9;
            let t178 = t177 * t36;
            let t182 = param_d_0_2 * t5;
            let t183 = t182 * t57;
            let t186 = param_d_0_0 + f64x8::splat(4.0) / f64x8::splat(27.0) * t178 * t96 + f64x8::splat(16.0) / f64x8::splat(81.0) * t183 * t104;
            let t187 = t89 * t186;
            let t189 = param_d_0_3 * t9;
            let t190 = t189 * t36;
            let t194 = param_d_0_4 * t5;
            let t195 = t194 * t57;
            let t198 = f64x8::splat(1.0) + f64x8::splat(4.0) / f64x8::splat(27.0) * t190 * t96 + f64x8::splat(16.0) / f64x8::splat(81.0) * t195 * t104;
            let t199 = f64x8::splat(1.0) / t198;
            let t200 = t199 * t125;
            let t203 = t31 * t154;
            let t204 = t203 * t166;
            let t207 = f64x8::splat(1.0) + t187 * t200 / f64x8::splat(2.0) + t204 * t124 / f64x8::splat(4.0);
            let t208 = f64x8::splat(1.0) / t207;
            let t209 = t20 * t22;
            let t210 = f64x8::splat(1.0) + t209;
            let t211 = (t210).simd_le(zeta_threshold);
            let t213 = f64x8::splat(2.0) / f64x8::splat(3.0) - f64x8::splat(0.003481525) * t124;
            let t215 = f64x8::splat(1.0) + f64x8::splat(0.045802) * t124;
            let t216 = f64x8::splat(1.0) / t215;
            let t217 = t213 * t216;
            let t218 = t26 * t125;
            let t222 = f64x8::splat(1.064009) + f64x8::splat(0.06361833333333333) * t80 * t82 * t218;
            let t223 = t26 * t222;
            let t227 = (simd::exp(-f64x8::splat(2.0) / f64x8::splat(9.0) * t80 * t82 * t223));
            let t229 = -t217 * t227 + f64x8::splat(2.0);
            let t230 = (simd::pow(zeta_threshold, t229));
            let t231 = (simd::pow(t210, t229));
            let t232 = ((t211).select(t230, t231));
            let t233 = f64x8::splat(1.0) - t209;
            let t234 = (t233).simd_le(zeta_threshold);
            let t235 = (simd::pow(t233, t229));
            let t236 = ((t234).select(t230, t235));
            let t237 = t232 + t236 - f64x8::splat(2.0);
            let t238 = (simd::pow(f64x8::splat(2.0), t229));
            let t239 = t238 - f64x8::splat(2.0);
            let t240 = f64x8::splat(1.0) / t239;
            let t241 = t237 * t240;
            let t242 = f64x8::splat(1.0) - t241;
            let t243 = t208 * t242;
            let t244 = t174 * t243;
            let t245 = t173 * t244;
            let t246 = f64x8::splat(M_CBRT2);
            let t247 = t246 * t1;
            let t248 = t3 * t5;
            let t249 = t247 * t248;
            let t250 = t246 * t246;
            let t251 = t27 * t250;
            let t252 = t19 * t251;
            let t255 = (simd::tanh(t14 * t252 / f64x8::splat(6.0)));
            let t256 = t8 * t255;
            let t257 = t43 * t250;
            let t258 = t42 * t257;
            let t259 = t39 * t258;
            let t262 = t65 * t246;
            let t263 = t64 * t262;
            let t264 = t60 * t263;
            let t266 = f64x8::splat(0.75) + f64x8::splat(0.11272703703703704) * t259 - f64x8::splat(0.00021049826280882748) * t52 + f64x8::splat(0.042061728395061726) * t264;
            let t269 = f64x8::splat(1.0) + f64x8::splat(0.30779666666666666) * t259 + f64x8::splat(0.12618518518518518) * t264;
            let t270 = f64x8::splat(1.0) / t269;
            let t271 = t266 * t270;
            let t275 = t26 * t246;
            let t277 = t80 * t82 * t275;
            let t278 = ((t277).sqrt());
            let t281 = (simd::tanh(f64x8::splat(3.0) / t278));
            let t284 = param_b_1_1 * t9;
            let t285 = t36 * t38;
            let t286 = t284 * t285;
            let t290 = param_b_1_2 * t5;
            let t291 = t57 * t59;
            let t292 = t290 * t291;
            let t295 = param_b_1_0 + t286 * t258 / f64x8::splat(27.0) + f64x8::splat(2.0) / f64x8::splat(81.0) * t292 * t263;
            let t296 = t281 * t295;
            let t298 = param_b_1_3 * t9;
            let t299 = t298 * t285;
            let t303 = param_b_1_4 * t5;
            let t304 = t303 * t291;
            let t307 = f64x8::splat(1.0) + t299 * t258 / f64x8::splat(27.0) + f64x8::splat(2.0) / f64x8::splat(81.0) * t304 * t263;
            let t308 = f64x8::splat(1.0) / t307;
            let t309 = t308 * t125;
            let t313 = param_c_1_1;
            let t315 = param_c_1_2 * t9;
            let t316 = t11 * t13;
            let t317 = t315 * t316;
            let t320 = (simd::exp(-t317 * t252 / f64x8::splat(6.0)));
            let t322 = t313 * t320 + param_c_1_0;
            let t323 = t322 * t255;
            let t326 = param_e_1_1 * t9;
            let t327 = t326 * t285;
            let t331 = param_e_1_2 * t5;
            let t332 = t331 * t291;
            let t335 = param_e_1_0 + t327 * t258 / f64x8::splat(27.0) + f64x8::splat(2.0) / f64x8::splat(81.0) * t332 * t263;
            let t337 = param_e_1_3 * t9;
            let t338 = t337 * t285;
            let t342 = param_e_1_4 * t5;
            let t343 = t342 * t291;
            let t346 = f64x8::splat(1.0) + t338 * t258 / f64x8::splat(27.0) + f64x8::splat(2.0) / f64x8::splat(81.0) * t343 * t263;
            let t347 = f64x8::splat(1.0) / t346;
            let t348 = t335 * t347;
            let t349 = t323 * t348;
            let t353 = (t249 * t256 * t271 / f64x8::splat(4.0) + t296 * t309 / f64x8::splat(2.0) + t349 * t124 / f64x8::splat(4.0)) * t61;
            let t354 = t8 * t2;
            let t355 = t353 * t354;
            let t358 = param_d_1_1 * t9;
            let t359 = t358 * t285;
            let t363 = param_d_1_2 * t5;
            let t364 = t363 * t291;
            let t367 = param_d_1_0 + t359 * t258 / f64x8::splat(27.0) + f64x8::splat(2.0) / f64x8::splat(81.0) * t364 * t263;
            let t368 = t281 * t367;
            let t370 = param_d_1_3 * t9;
            let t371 = t370 * t285;
            let t375 = param_d_1_4 * t5;
            let t376 = t375 * t291;
            let t379 = f64x8::splat(1.0) + t371 * t258 / f64x8::splat(27.0) + f64x8::splat(2.0) / f64x8::splat(81.0) * t376 * t263;
            let t380 = f64x8::splat(1.0) / t379;
            let t381 = t380 * t125;
            let t384 = t255 * t335;
            let t385 = t384 * t347;
            let t388 = f64x8::splat(1.0) + t368 * t381 / f64x8::splat(2.0) + t385 * t124 / f64x8::splat(4.0);
            let t389 = f64x8::splat(1.0) / t388;
            let t390 = t17 * t389;
            let t391 = t390 * t241;
            let t392 = t355 * t391;
            let tzk0 = -t245 / f64x8::splat(3.0) - t392 / f64x8::splat(3.0);
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}

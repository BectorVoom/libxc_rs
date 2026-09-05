//! LDA_XC_KSDT vxc pol kernel — explicit SIMD (bit-exact).
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
pub fn lda_xc_ksdt_vxc_pol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
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
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
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
            let t394 = t245 / f64x8::splat(3.0);
            let t395 = t392 / f64x8::splat(3.0);
            let t396 = t5 * t8;
            let t397 = t4 * t396;
            let t398 = t15 * t122;
            let t400 = t14 * t398 * t27;
            let t401 = t26 * t24;
            let t402 = f64x8::splat(1.0) / t401;
            let t403 = param_thetaParam * t22;
            let t404 = t21 * t49;
            let t405 = t403 - t404;
            let t406 = t402 * t405;
            let t407 = t19 * t406;
            let t410 = -t14 * t407 / f64x8::splat(9.0) + t400 / f64x8::splat(9.0);
            let t411 = t31 * t31;
            let t412 = f64x8::splat(1.0) - t411;
            let t413 = t410 * t412;
            let t418 = f64x8::splat(1.0) / t17 / t48;
            let t419 = t15 * t418;
            let t421 = t39 * t419 * t43;
            let t422 = f64x8::splat(0.6012108641975309) * t421;
            let t423 = t25 * t405;
            let t424 = t42 * t423;
            let t425 = t39 * t424;
            let t427 = t48 * t16;
            let t428 = f64x8::splat(1.0) / t427;
            let t429 = t47 * t428;
            let t430 = t429 * t51;
            let t431 = f64x8::splat(0.0016839861024706198) * t430;
            let t432 = t24 * t405;
            let t433 = t50 * t432;
            let t436 = f64x8::splat(1.0) / t18 / t427;
            let t437 = t61 * t436;
            let t439 = t60 * t437 * t65;
            let t440 = f64x8::splat(0.8973168724279835) * t439;
            let t441 = t401 * t405;
            let t442 = t64 * t441;
            let t443 = t60 * t442;
            let t445 = -t422 + f64x8::splat(0.6012108641975309) * t425 + t431 - f64x8::splat(0.0016839861024706198) * t433 - t440 + f64x8::splat(0.8973168724279835) * t443;
            let t446 = t445 * t73;
            let t450 = t31 * t69;
            let t451 = t72 * t72;
            let t452 = f64x8::splat(1.0) / t451;
            let t453 = f64x8::splat(1.6415822222222223) * t421;
            let t455 = f64x8::splat(2.6919506172839505) * t439;
            let t457 = -t453 + f64x8::splat(1.6415822222222223) * t425 - t455 + f64x8::splat(2.6919506172839505) * t443;
            let t458 = t452 * t457;
            let t463 = f64x8::splat(1.0) / t85 / t84;
            let t464 = t78 * t463;
            let t465 = t18 * t16;
            let t466 = f64x8::splat(1.0) / t465;
            let t467 = t61 * t466;
            let t469 = t80 * t467 * t26;
            let t470 = f64x8::splat(1.0) / t25;
            let t471 = t470 * t405;
            let t475 = f64x8::splat(2.0) / f64x8::splat(3.0) * t80 * t82 * t471 - f64x8::splat(2.0) / f64x8::splat(3.0) * t469;
            let t476 = t464 * t475;
            let t477 = t89 * t89;
            let t478 = f64x8::splat(1.0) - t477;
            let t479 = t478 * t107;
            let t480 = t479 * t126;
            let t481 = t476 * t480;
            let t484 = t94 * t418 * t43;
            let t486 = f64x8::splat(16.0) / f64x8::splat(81.0) * t93 * t484;
            let t487 = t92 * t285;
            let t491 = t102 * t436 * t65;
            let t493 = f64x8::splat(128.0) / f64x8::splat(243.0) * t101 * t491;
            let t494 = t100 * t291;
            let t497 = -t486 + f64x8::splat(16.0) / f64x8::splat(81.0) * t487 * t424 - t493 + f64x8::splat(128.0) / f64x8::splat(243.0) * t494 * t442;
            let t498 = t89 * t497;
            let t501 = t119 * t119;
            let t502 = f64x8::splat(1.0) / t501;
            let t503 = t502 * t125;
            let t505 = f64x8::splat(16.0) / f64x8::splat(81.0) * t111 * t484;
            let t506 = t110 * t285;
            let t510 = f64x8::splat(128.0) / f64x8::splat(243.0) * t116 * t491;
            let t511 = t115 * t291;
            let t514 = -t505 + f64x8::splat(16.0) / f64x8::splat(81.0) * t506 * t424 - t510 + f64x8::splat(128.0) / f64x8::splat(243.0) * t511 * t442;
            let t515 = t503 * t514;
            let t518 = f64x8::splat(1.0) / t125;
            let t519 = t120 * t518;
            let t520 = t108 * t519;
            let t521 = t3 * t41;
            let t522 = t121 * t521;
            let t524 = t520 * t522 / f64x8::splat(12.0);
            let t527 = t133 * t134 * t122 * t27;
            let t528 = t132 * t316;
            let t531 = t528 * t407 / f64x8::splat(9.0) - t527 / f64x8::splat(9.0);
            let t532 = t130 * t531;
            let t533 = t139 * t31;
            let t534 = t533 * t154;
            let t535 = t532 * t534;
            let t536 = t166 * t15;
            let t537 = t7 * t3;
            let t538 = t537 * t122;
            let t539 = t536 * t538;
            let t542 = t141 * t410;
            let t543 = t412 * t154;
            let t544 = t542 * t543;
            let t548 = f64x8::splat(16.0) / f64x8::splat(81.0) * t146 * t484;
            let t549 = t145 * t285;
            let t553 = f64x8::splat(128.0) / f64x8::splat(243.0) * t151 * t491;
            let t554 = t150 * t291;
            let t557 = -t548 + f64x8::splat(16.0) / f64x8::splat(81.0) * t549 * t424 - t553 + f64x8::splat(128.0) / f64x8::splat(243.0) * t554 * t442;
            let t558 = t557 * t166;
            let t559 = t142 * t558;
            let t562 = t165 * t165;
            let t563 = f64x8::splat(1.0) / t562;
            let t564 = t154 * t563;
            let t565 = t142 * t564;
            let t567 = f64x8::splat(16.0) / f64x8::splat(81.0) * t157 * t484;
            let t568 = t156 * t285;
            let t572 = f64x8::splat(128.0) / f64x8::splat(243.0) * t162 * t491;
            let t573 = t161 * t291;
            let t576 = -t567 + f64x8::splat(16.0) / f64x8::splat(81.0) * t568 * t424 - t572 + f64x8::splat(128.0) / f64x8::splat(243.0) * t573 * t442;
            let t577 = t123 * t576;
            let t578 = t121 * t577;
            let t582 = t168 * t522 / f64x8::splat(12.0);
            let t583 = t397 * t413 * t74 / f64x8::splat(4.0) + t6 * t32 * t446 / f64x8::splat(4.0) - t397 * t450 * t458 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t481 + t498 * t126 / f64x8::splat(2.0) - t108 * t515 / f64x8::splat(2.0) - t524 + t535 * t539 / f64x8::splat(4.0) + t544 * t539 / f64x8::splat(4.0) + t559 * t124 / f64x8::splat(4.0) - t565 * t578 / f64x8::splat(4.0) - t582;
            let t584 = t583 * t61;
            let t585 = t584 * t8;
            let t586 = t585 * t244;
            let t587 = t586 / f64x8::splat(3.0);
            let t588 = t2 * t81;
            let t589 = t588 * t243;
            let t590 = t173 * t589;
            let t591 = t590 / f64x8::splat(9.0);
            let t592 = t172 * t354;
            let t593 = t207 * t207;
            let t594 = f64x8::splat(1.0) / t593;
            let t595 = t17 * t594;
            let t596 = t478 * t186;
            let t597 = t596 * t200;
            let t598 = t476 * t597;
            let t601 = f64x8::splat(16.0) / f64x8::splat(81.0) * t178 * t484;
            let t602 = t177 * t285;
            let t606 = f64x8::splat(128.0) / f64x8::splat(243.0) * t183 * t491;
            let t607 = t182 * t291;
            let t610 = -t601 + f64x8::splat(16.0) / f64x8::splat(81.0) * t602 * t424 - t606 + f64x8::splat(128.0) / f64x8::splat(243.0) * t607 * t442;
            let t611 = t89 * t610;
            let t614 = t198 * t198;
            let t615 = f64x8::splat(1.0) / t614;
            let t616 = t615 * t125;
            let t618 = f64x8::splat(16.0) / f64x8::splat(81.0) * t190 * t484;
            let t619 = t189 * t285;
            let t623 = f64x8::splat(128.0) / f64x8::splat(243.0) * t195 * t491;
            let t624 = t194 * t291;
            let t627 = -t618 + f64x8::splat(16.0) / f64x8::splat(81.0) * t619 * t424 - t623 + f64x8::splat(128.0) / f64x8::splat(243.0) * t624 * t442;
            let t628 = t616 * t627;
            let t631 = t199 * t518;
            let t632 = t187 * t631;
            let t634 = t632 * t522 / f64x8::splat(12.0);
            let t635 = t413 * t167;
            let t638 = t31 * t557;
            let t639 = t638 * t166;
            let t642 = t563 * t15;
            let t643 = t203 * t642;
            let t644 = t122 * t576;
            let t645 = t537 * t644;
            let t649 = t204 * t522 / f64x8::splat(12.0);
            let t650 = -f64x8::splat(3.0) / f64x8::splat(8.0) * t598 + t611 * t200 / f64x8::splat(2.0) - t187 * t628 / f64x8::splat(2.0) - t634 + t635 * t124 / f64x8::splat(4.0) + t639 * t124 / f64x8::splat(4.0) - t643 * t645 / f64x8::splat(4.0) - t649;
            let t651 = t242 * t650;
            let t652 = t595 * t651;
            let t653 = t592 * t652;
            let t654 = t653 / f64x8::splat(3.0);
            let t655 = t121 * t3;
            let t656 = t41 * t216;
            let t659 = f64x8::splat(0.0011605083333333334) * t655 * t656 * t227;
            let t660 = t215 * t215;
            let t661 = f64x8::splat(1.0) / t660;
            let t662 = t213 * t661;
            let t663 = t662 * t227;
            let t665 = f64x8::splat(0.015267333333333334) * t663 * t522;
            let t668 = f64x8::splat(4.0) / f64x8::splat(27.0) * t80 * t467 * t223;
            let t669 = param_T * t61;
            let t670 = t79 * t669;
            let t671 = t81 * t470;
            let t672 = t222 * t405;
            let t678 = f64x8::splat(0.04241222222222222) * t80 * t467 * t218;
            let t679 = t125 * t405;
            let t684 = t5 * t35 * t1;
            let t685 = t684 * param_T;
            let t687 = t518 * t3;
            let t690 = f64x8::splat(0.031809166666666666) * t685 * t49 * t26 * t687;
            let t691 = -t678 + f64x8::splat(0.04241222222222222) * t670 * t671 * t679 - t690;
            let t692 = t26 * t691;
            let t696 = t668 - f64x8::splat(4.0) / f64x8::splat(27.0) * t670 * t671 * t672 - f64x8::splat(2.0) / f64x8::splat(9.0) * t80 * t82 * t692;
            let t697 = t696 * t227;
            let t699 = -t217 * t697 - t659 - t665;
            let t700 = t230 * t699;
            let t701 = (simd::ln(zeta_threshold));
            let t702 = t700 * t701;
            let t703 = (simd::ln(t210));
            let t705 = t20 * t49;
            let t706 = t22 - t705;
            let t708 = f64x8::splat(1.0) / t210;
            let t710 = t229 * t706 * t708 + t699 * t703;
            let t711 = t231 * t710;
            let t712 = ((t211).select(t702, t711));
            let t713 = (simd::ln(t233));
            let t715 = -t706;
            let t717 = f64x8::splat(1.0) / t233;
            let t719 = t229 * t715 * t717 + t699 * t713;
            let t720 = t235 * t719;
            let t721 = ((t234).select(t702, t720));
            let t722 = t712 + t721;
            let t723 = t722 * t240;
            let t724 = t239 * t239;
            let t725 = f64x8::splat(1.0) / t724;
            let t726 = t237 * t725;
            let t727 = t238 * t699;
            let t728 = (simd::ln(f64x8::splat(2.0)));
            let t729 = t727 * t728;
            let t731 = t726 * t729 - t723;
            let t732 = t208 * t731;
            let t733 = t174 * t732;
            let t734 = t173 * t733;
            let t735 = t734 / f64x8::splat(3.0);
            let t736 = t398 * t251;
            let t737 = t14 * t736;
            let t738 = t12 * t134;
            let t739 = t18 * t402;
            let t740 = t250 * t405;
            let t744 = -t738 * t739 * t740 / f64x8::splat(9.0) + t737 / f64x8::splat(9.0);
            let t745 = t8 * t744;
            let t746 = t255 * t255;
            let t747 = f64x8::splat(1.0) - t746;
            let t749 = t747 * t266 * t270;
            let t753 = t419 * t257;
            let t754 = t39 * t753;
            let t755 = f64x8::splat(0.15030271604938272) * t754;
            let t756 = t37 * t94;
            let t757 = t41 * t25;
            let t759 = t756 * t757 * t740;
            let t761 = f64x8::splat(0.00042099652561765496) * t430;
            let t763 = t437 * t262;
            let t764 = t60 * t763;
            let t765 = f64x8::splat(0.11216460905349794) * t764;
            let t766 = t58 * t102;
            let t767 = t63 * t401;
            let t768 = t246 * t405;
            let t770 = t766 * t767 * t768;
            let t772 = -t755 + f64x8::splat(0.15030271604938272) * t759 + t761 - f64x8::splat(0.00042099652561765496) * t433 - t765 + f64x8::splat(0.11216460905349794) * t770;
            let t773 = t772 * t270;
            let t777 = t269 * t269;
            let t778 = f64x8::splat(1.0) / t777;
            let t779 = t266 * t778;
            let t780 = f64x8::splat(0.41039555555555557) * t754;
            let t782 = f64x8::splat(0.3364938271604938) * t764;
            let t784 = -t780 + f64x8::splat(0.41039555555555557) * t759 - t782 + f64x8::splat(0.3364938271604938) * t770;
            let t785 = t779 * t784;
            let t790 = f64x8::splat(1.0) / t278 / t277;
            let t792 = t80 * t467 * t275;
            let t796 = f64x8::splat(2.0) / f64x8::splat(3.0) * t670 * t671 * t768 - f64x8::splat(2.0) / f64x8::splat(3.0) * t792;
            let t797 = t790 * t796;
            let t798 = t281 * t281;
            let t799 = f64x8::splat(1.0) - t798;
            let t800 = t797 * t799;
            let t801 = t295 * t308;
            let t802 = t801 * t125;
            let t803 = t800 * t802;
            let t806 = f64x8::splat(4.0) / f64x8::splat(81.0) * t286 * t753;
            let t807 = t25 * t250;
            let t808 = t807 * t405;
            let t809 = t42 * t808;
            let t813 = f64x8::splat(16.0) / f64x8::splat(243.0) * t292 * t763;
            let t814 = t401 * t246;
            let t815 = t814 * t405;
            let t816 = t64 * t815;
            let t819 = -t806 + f64x8::splat(4.0) / f64x8::splat(81.0) * t286 * t809 - t813 + f64x8::splat(16.0) / f64x8::splat(243.0) * t292 * t816;
            let t820 = t281 * t819;
            let t823 = t307 * t307;
            let t824 = f64x8::splat(1.0) / t823;
            let t825 = t824 * t125;
            let t827 = f64x8::splat(4.0) / f64x8::splat(81.0) * t299 * t753;
            let t831 = f64x8::splat(16.0) / f64x8::splat(243.0) * t304 * t763;
            let t834 = -t827 + f64x8::splat(4.0) / f64x8::splat(81.0) * t299 * t809 - t831 + f64x8::splat(16.0) / f64x8::splat(243.0) * t304 * t816;
            let t835 = t825 * t834;
            let t838 = t308 * t518;
            let t839 = t296 * t838;
            let t841 = t839 * t522 / f64x8::splat(12.0);
            let t842 = t317 * t736;
            let t843 = t402 * t250;
            let t844 = t843 * t405;
            let t848 = t317 * t19 * t844 / f64x8::splat(9.0) - t842 / f64x8::splat(9.0);
            let t849 = t313 * t848;
            let t850 = t320 * t255;
            let t851 = t850 * t335;
            let t852 = t849 * t851;
            let t853 = t347 * t15;
            let t854 = t853 * t538;
            let t857 = t322 * t744;
            let t858 = t747 * t335;
            let t859 = t857 * t858;
            let t863 = f64x8::splat(4.0) / f64x8::splat(81.0) * t327 * t753;
            let t867 = f64x8::splat(16.0) / f64x8::splat(243.0) * t332 * t763;
            let t870 = -t863 + f64x8::splat(4.0) / f64x8::splat(81.0) * t327 * t809 - t867 + f64x8::splat(16.0) / f64x8::splat(243.0) * t332 * t816;
            let t871 = t870 * t347;
            let t872 = t323 * t871;
            let t875 = t346 * t346;
            let t876 = f64x8::splat(1.0) / t875;
            let t877 = t335 * t876;
            let t878 = t323 * t877;
            let t880 = f64x8::splat(4.0) / f64x8::splat(81.0) * t338 * t753;
            let t884 = f64x8::splat(16.0) / f64x8::splat(243.0) * t343 * t763;
            let t887 = -t880 + f64x8::splat(4.0) / f64x8::splat(81.0) * t338 * t809 - t884 + f64x8::splat(16.0) / f64x8::splat(243.0) * t343 * t816;
            let t888 = t123 * t887;
            let t889 = t121 * t888;
            let t893 = t349 * t522 / f64x8::splat(12.0);
            let t894 = t249 * t745 * t749 / f64x8::splat(4.0) + t249 * t256 * t773 / f64x8::splat(4.0) - t249 * t256 * t785 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t803 + t820 * t309 / f64x8::splat(2.0) - t296 * t835 / f64x8::splat(2.0) - t841 + t852 * t854 / f64x8::splat(4.0) + t859 * t854 / f64x8::splat(4.0) + t872 * t124 / f64x8::splat(4.0) - t878 * t889 / f64x8::splat(4.0) - t893;
            let t895 = t894 * t61;
            let t896 = t895 * t354;
            let t897 = t896 * t391;
            let t898 = t897 / f64x8::splat(3.0);
            let t899 = t81 * t389;
            let t900 = t899 * t241;
            let t901 = t355 * t900;
            let t902 = t901 / f64x8::splat(9.0);
            let t903 = t388 * t388;
            let t904 = f64x8::splat(1.0) / t903;
            let t905 = t17 * t904;
            let t906 = t367 * t380;
            let t907 = t906 * t125;
            let t908 = t800 * t907;
            let t911 = f64x8::splat(4.0) / f64x8::splat(81.0) * t359 * t753;
            let t915 = f64x8::splat(16.0) / f64x8::splat(243.0) * t364 * t763;
            let t918 = -t911 + f64x8::splat(4.0) / f64x8::splat(81.0) * t359 * t809 - t915 + f64x8::splat(16.0) / f64x8::splat(243.0) * t364 * t816;
            let t919 = t281 * t918;
            let t922 = t379 * t379;
            let t923 = f64x8::splat(1.0) / t922;
            let t924 = t923 * t125;
            let t926 = f64x8::splat(4.0) / f64x8::splat(81.0) * t371 * t753;
            let t930 = f64x8::splat(16.0) / f64x8::splat(243.0) * t376 * t763;
            let t933 = -t926 + f64x8::splat(4.0) / f64x8::splat(81.0) * t371 * t809 - t930 + f64x8::splat(16.0) / f64x8::splat(243.0) * t376 * t816;
            let t934 = t924 * t933;
            let t937 = t380 * t518;
            let t938 = t368 * t937;
            let t940 = t938 * t522 / f64x8::splat(12.0);
            let t941 = t744 * t747;
            let t942 = t941 * t348;
            let t945 = t255 * t870;
            let t946 = t945 * t347;
            let t949 = t876 * t15;
            let t950 = t384 * t949;
            let t951 = t122 * t887;
            let t952 = t537 * t951;
            let t956 = t385 * t522 / f64x8::splat(12.0);
            let t957 = -f64x8::splat(3.0) / f64x8::splat(4.0) * t908 + t919 * t381 / f64x8::splat(2.0) - t368 * t934 / f64x8::splat(2.0) - t940 + t942 * t124 / f64x8::splat(4.0) + t946 * t124 / f64x8::splat(4.0) - t950 * t952 / f64x8::splat(4.0) - t956;
            let t958 = t241 * t957;
            let t959 = t905 * t958;
            let t960 = t355 * t959;
            let t961 = t960 / f64x8::splat(3.0);
            let t962 = t390 * t723;
            let t963 = t355 * t962;
            let t964 = t963 / f64x8::splat(3.0);
            let t965 = t354 * t17;
            let t966 = t353 * t965;
            let t967 = t389 * t237;
            let t968 = t967 * t725;
            let t969 = t968 * t729;
            let t970 = t966 * t969;
            let t971 = t970 / f64x8::splat(3.0);
            let tvrho0 = -t394 - t395 + t16 * (-t587 - t591 + t654 - t735 - t898 - t902 + t961 - t964 + t971);
            acc_vrho_0 = tvrho0;
            let t974 = -t403 - t404;
            let t975 = t402 * t974;
            let t976 = t19 * t975;
            let t979 = -t14 * t976 / f64x8::splat(9.0) + t400 / f64x8::splat(9.0);
            let t980 = t979 * t412;
            let t984 = t25 * t974;
            let t985 = t42 * t984;
            let t986 = t39 * t985;
            let t988 = t24 * t974;
            let t989 = t50 * t988;
            let t991 = t401 * t974;
            let t992 = t64 * t991;
            let t993 = t60 * t992;
            let t995 = -t422 + f64x8::splat(0.6012108641975309) * t986 + t431 - f64x8::splat(0.0016839861024706198) * t989 - t440 + f64x8::splat(0.8973168724279835) * t993;
            let t996 = t995 * t73;
            let t1002 = -t453 + f64x8::splat(1.6415822222222223) * t986 - t455 + f64x8::splat(2.6919506172839505) * t993;
            let t1003 = t452 * t1002;
            let t1007 = t470 * t974;
            let t1011 = f64x8::splat(2.0) / f64x8::splat(3.0) * t80 * t82 * t1007 - f64x8::splat(2.0) / f64x8::splat(3.0) * t469;
            let t1012 = t464 * t1011;
            let t1019 = -t486 + f64x8::splat(16.0) / f64x8::splat(81.0) * t487 * t985 - t493 + f64x8::splat(128.0) / f64x8::splat(243.0) * t494 * t992;
            let t1020 = t89 * t1019;
            let t1027 = -t505 + f64x8::splat(16.0) / f64x8::splat(81.0) * t506 * t985 - t510 + f64x8::splat(128.0) / f64x8::splat(243.0) * t511 * t992;
            let t1028 = t503 * t1027;
            let t1033 = t528 * t976 / f64x8::splat(9.0) - t527 / f64x8::splat(9.0);
            let t1034 = t130 * t1033;
            let t1035 = t1034 * t534;
            let t1038 = t141 * t979;
            let t1039 = t1038 * t543;
            let t1046 = -t548 + f64x8::splat(16.0) / f64x8::splat(81.0) * t549 * t985 - t553 + f64x8::splat(128.0) / f64x8::splat(243.0) * t554 * t992;
            let t1047 = t1046 * t166;
            let t1048 = t142 * t1047;
            let t1055 = -t567 + f64x8::splat(16.0) / f64x8::splat(81.0) * t568 * t985 - t572 + f64x8::splat(128.0) / f64x8::splat(243.0) * t573 * t992;
            let t1056 = t123 * t1055;
            let t1057 = t121 * t1056;
            let t1060 = t397 * t980 * t74 / f64x8::splat(4.0) + t6 * t32 * t996 / f64x8::splat(4.0) - t397 * t450 * t1003 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t1012 * t480 + t1020 * t126 / f64x8::splat(2.0) - t108 * t1028 / f64x8::splat(2.0) - t524 + t1035 * t539 / f64x8::splat(4.0) + t1039 * t539 / f64x8::splat(4.0) + t1048 * t124 / f64x8::splat(4.0) - t565 * t1057 / f64x8::splat(4.0) - t582;
            let t1061 = t1060 * t61;
            let t1062 = t1061 * t8;
            let t1063 = t1062 * t244;
            let t1064 = t1063 / f64x8::splat(3.0);
            let t1071 = -t601 + f64x8::splat(16.0) / f64x8::splat(81.0) * t602 * t985 - t606 + f64x8::splat(128.0) / f64x8::splat(243.0) * t607 * t992;
            let t1072 = t89 * t1071;
            let t1079 = -t618 + f64x8::splat(16.0) / f64x8::splat(81.0) * t619 * t985 - t623 + f64x8::splat(128.0) / f64x8::splat(243.0) * t624 * t992;
            let t1080 = t616 * t1079;
            let t1083 = t980 * t167;
            let t1086 = t31 * t1046;
            let t1087 = t1086 * t166;
            let t1090 = t122 * t1055;
            let t1091 = t537 * t1090;
            let t1094 = -f64x8::splat(3.0) / f64x8::splat(8.0) * t1012 * t597 + t1072 * t200 / f64x8::splat(2.0) - t187 * t1080 / f64x8::splat(2.0) - t634 + t1083 * t124 / f64x8::splat(4.0) + t1087 * t124 / f64x8::splat(4.0) - t643 * t1091 / f64x8::splat(4.0) - t649;
            let t1095 = t242 * t1094;
            let t1096 = t595 * t1095;
            let t1097 = t592 * t1096;
            let t1098 = t1097 / f64x8::splat(3.0);
            let t1099 = t222 * t974;
            let t1103 = t125 * t974;
            let t1107 = -t678 + f64x8::splat(0.04241222222222222) * t670 * t671 * t1103 - t690;
            let t1108 = t26 * t1107;
            let t1112 = t668 - f64x8::splat(4.0) / f64x8::splat(27.0) * t670 * t671 * t1099 - f64x8::splat(2.0) / f64x8::splat(9.0) * t80 * t82 * t1108;
            let t1113 = t1112 * t227;
            let t1115 = -t1113 * t217 - t659 - t665;
            let t1116 = t230 * t1115;
            let t1117 = t1116 * t701;
            let t1119 = -t22 - t705;
            let t1120 = t229 * t1119;
            let t1122 = t1115 * t703 + t1120 * t708;
            let t1123 = t231 * t1122;
            let t1124 = ((t211).select(t1117, t1123));
            let t1126 = -t1119;
            let t1127 = t229 * t1126;
            let t1129 = t1115 * t713 + t1127 * t717;
            let t1130 = t235 * t1129;
            let t1131 = ((t234).select(t1117, t1130));
            let t1132 = t1124 + t1131;
            let t1133 = t1132 * t240;
            let t1134 = t238 * t1115;
            let t1135 = t1134 * t728;
            let t1137 = t1135 * t726 - t1133;
            let t1138 = t208 * t1137;
            let t1139 = t174 * t1138;
            let t1140 = t173 * t1139;
            let t1141 = t1140 / f64x8::splat(3.0);
            let t1142 = t250 * t974;
            let t1146 = -t738 * t739 * t1142 / f64x8::splat(9.0) + t737 / f64x8::splat(9.0);
            let t1147 = t8 * t1146;
            let t1152 = t756 * t757 * t1142;
            let t1155 = t246 * t974;
            let t1157 = t766 * t767 * t1155;
            let t1159 = -t755 + f64x8::splat(0.15030271604938272) * t1152 + t761 - f64x8::splat(0.00042099652561765496) * t989 - t765 + f64x8::splat(0.11216460905349794) * t1157;
            let t1160 = t1159 * t270;
            let t1166 = -t780 + f64x8::splat(0.41039555555555557) * t1152 - t782 + f64x8::splat(0.3364938271604938) * t1157;
            let t1167 = t779 * t1166;
            let t1174 = f64x8::splat(2.0) / f64x8::splat(3.0) * t670 * t671 * t1155 - f64x8::splat(2.0) / f64x8::splat(3.0) * t792;
            let t1175 = t790 * t1174;
            let t1176 = t1175 * t799;
            let t1179 = t807 * t974;
            let t1180 = t42 * t1179;
            let t1183 = t814 * t974;
            let t1184 = t64 * t1183;
            let t1187 = -t806 + f64x8::splat(4.0) / f64x8::splat(81.0) * t286 * t1180 - t813 + f64x8::splat(16.0) / f64x8::splat(243.0) * t292 * t1184;
            let t1188 = t281 * t1187;
            let t1195 = -t827 + f64x8::splat(4.0) / f64x8::splat(81.0) * t299 * t1180 - t831 + f64x8::splat(16.0) / f64x8::splat(243.0) * t304 * t1184;
            let t1196 = t825 * t1195;
            let t1199 = t843 * t974;
            let t1203 = t317 * t19 * t1199 / f64x8::splat(9.0) - t842 / f64x8::splat(9.0);
            let t1204 = t313 * t1203;
            let t1205 = t1204 * t851;
            let t1208 = t322 * t1146;
            let t1209 = t1208 * t858;
            let t1216 = -t863 + f64x8::splat(4.0) / f64x8::splat(81.0) * t327 * t1180 - t867 + f64x8::splat(16.0) / f64x8::splat(243.0) * t332 * t1184;
            let t1217 = t1216 * t347;
            let t1218 = t323 * t1217;
            let t1225 = -t880 + f64x8::splat(4.0) / f64x8::splat(81.0) * t338 * t1180 - t884 + f64x8::splat(16.0) / f64x8::splat(243.0) * t343 * t1184;
            let t1226 = t123 * t1225;
            let t1227 = t121 * t1226;
            let t1230 = t249 * t1147 * t749 / f64x8::splat(4.0) + t249 * t256 * t1160 / f64x8::splat(4.0) - t249 * t256 * t1167 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t1176 * t802 + t1188 * t309 / f64x8::splat(2.0) - t296 * t1196 / f64x8::splat(2.0) - t841 + t1205 * t854 / f64x8::splat(4.0) + t1209 * t854 / f64x8::splat(4.0) + t1218 * t124 / f64x8::splat(4.0) - t878 * t1227 / f64x8::splat(4.0) - t893;
            let t1231 = t1230 * t61;
            let t1232 = t1231 * t354;
            let t1233 = t1232 * t391;
            let t1234 = t1233 / f64x8::splat(3.0);
            let t1241 = -t911 + f64x8::splat(4.0) / f64x8::splat(81.0) * t359 * t1180 - t915 + f64x8::splat(16.0) / f64x8::splat(243.0) * t364 * t1184;
            let t1242 = t281 * t1241;
            let t1249 = -t926 + f64x8::splat(4.0) / f64x8::splat(81.0) * t371 * t1180 - t930 + f64x8::splat(16.0) / f64x8::splat(243.0) * t376 * t1184;
            let t1250 = t924 * t1249;
            let t1253 = t1146 * t747;
            let t1254 = t1253 * t348;
            let t1257 = t255 * t1216;
            let t1258 = t1257 * t347;
            let t1261 = t122 * t1225;
            let t1262 = t537 * t1261;
            let t1265 = -f64x8::splat(3.0) / f64x8::splat(4.0) * t1176 * t907 + t1242 * t381 / f64x8::splat(2.0) - t368 * t1250 / f64x8::splat(2.0) - t940 + t1254 * t124 / f64x8::splat(4.0) + t1258 * t124 / f64x8::splat(4.0) - t950 * t1262 / f64x8::splat(4.0) - t956;
            let t1266 = t241 * t1265;
            let t1267 = t905 * t1266;
            let t1268 = t355 * t1267;
            let t1269 = t1268 / f64x8::splat(3.0);
            let t1270 = t390 * t1133;
            let t1271 = t355 * t1270;
            let t1272 = t1271 / f64x8::splat(3.0);
            let t1273 = t968 * t1135;
            let t1274 = t966 * t1273;
            let t1275 = t1274 / f64x8::splat(3.0);
            let tvrho1 = -t394 - t395 + t16 * (-t1064 - t591 + t1098 - t1141 - t1234 - t902 + t1269 - t1272 + t1275);
            acc_vrho_1 = tvrho1;
        }
        store_add(zk, ip, m, acc_zk);
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        ip += 8;
    }
}

//! GGA_X_N12 exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_n12.c`
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
pub fn gga_x_n12_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_CC_0_1: f64,
    param_CC_0_2: f64,
    param_CC_0_3: f64,
    param_CC_1_1: f64,
    param_CC_1_2: f64,
    param_CC_1_3: f64,
    param_CC_1_0: f64,
    param_CC_2_1: f64,
    param_CC_2_2: f64,
    param_CC_2_3: f64,
    param_CC_2_0: f64,
    param_CC_3_1: f64,
    param_CC_3_2: f64,
    param_CC_3_3: f64,
    param_CC_3_0: f64,
    param_CC_0_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_CC_0_1 = f64x8::splat(param_CC_0_1);
    let param_CC_0_2 = f64x8::splat(param_CC_0_2);
    let param_CC_0_3 = f64x8::splat(param_CC_0_3);
    let param_CC_1_1 = f64x8::splat(param_CC_1_1);
    let param_CC_1_2 = f64x8::splat(param_CC_1_2);
    let param_CC_1_3 = f64x8::splat(param_CC_1_3);
    let param_CC_1_0 = f64x8::splat(param_CC_1_0);
    let param_CC_2_1 = f64x8::splat(param_CC_2_1);
    let param_CC_2_2 = f64x8::splat(param_CC_2_2);
    let param_CC_2_3 = f64x8::splat(param_CC_2_3);
    let param_CC_2_0 = f64x8::splat(param_CC_2_0);
    let param_CC_3_1 = f64x8::splat(param_CC_3_1);
    let param_CC_3_2 = f64x8::splat(param_CC_3_2);
    let param_CC_3_3 = f64x8::splat(param_CC_3_3);
    let param_CC_3_0 = f64x8::splat(param_CC_3_0);
    let param_CC_0_0 = f64x8::splat(param_CC_0_0);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho0 = load_strided(rho, ip, np, 2, 0);
        let v_rho1 = load_strided(rho, ip, np, 2, 1);
        let v_sigma0 = load_strided(sigma, ip, np, 3, 0);
        let v_sigma1 = load_strided(sigma, ip, np, 3, 1);
        let v_sigma2 = load_strided(sigma, ip, np, 3, 2);
        let mut acc_zk = V_ZERO;
        {
            let t1 = (v_rho0).simd_le(dens_threshold);
            let t2 = f64x8::splat(M_CBRT3);
            let t3 = f64x8::splat(M_CBRTPI);
            let t5 = t2 / t3;
            let t6 = v_rho0 + v_rho1;
            let t7 = f64x8::splat(1.0) / t6;
            let t10 = (f64x8::splat(2.0) * v_rho0 * t7).simd_le(zeta_threshold);
            let t11 = zeta_threshold - f64x8::splat(1.0);
            let t14 = (f64x8::splat(2.0) * v_rho1 * t7).simd_le(zeta_threshold);
            let t15 = -t11;
            let t16 = v_rho0 - v_rho1;
            let t17 = t16 * t7;
            let t18 = ((t10).select(t11, (t14).select(t15, t17)));
            let t19 = f64x8::splat(1.0) + t18;
            let t20 = (t19).simd_le(zeta_threshold);
            let t21 = (simd::cbrt(zeta_threshold));
            let t22 = t21 * zeta_threshold;
            let t23 = (simd::cbrt(t19));
            let t25 = ((t20).select(t22, t23 * t19));
            let t26 = (simd::cbrt(t6));
            let t27 = t25 * t26;
            let t28 = param_CC_0_0;
            let t29 = param_CC_0_1;
            let t30 = t29 * v_sigma0;
            let t31 = v_rho0 * v_rho0;
            let t32 = (simd::cbrt(v_rho0));
            let t33 = t32 * t32;
            let t35 = f64x8::splat(1.0) / t33 / t31;
            let t38 = f64x8::splat(1.0) + f64x8::splat(0.004) * v_sigma0 * t35;
            let t39 = f64x8::splat(1.0) / t38;
            let t40 = t35 * t39;
            let t43 = param_CC_0_2;
            let t44 = v_sigma0 * v_sigma0;
            let t45 = t43 * t44;
            let t46 = t31 * t31;
            let t47 = t46 * v_rho0;
            let t49 = f64x8::splat(1.0) / t32 / t47;
            let t50 = t38 * t38;
            let t51 = f64x8::splat(1.0) / t50;
            let t52 = t49 * t51;
            let t55 = param_CC_0_3;
            let t56 = t44 * v_sigma0;
            let t57 = t55 * t56;
            let t58 = t46 * t46;
            let t59 = f64x8::splat(1.0) / t58;
            let t60 = t50 * t38;
            let t61 = f64x8::splat(1.0) / t60;
            let t62 = t59 * t61;
            let t65 = param_CC_1_0;
            let t66 = param_CC_1_1;
            let t67 = t66 * v_sigma0;
            let t70 = param_CC_1_2;
            let t71 = t70 * t44;
            let t74 = param_CC_1_3;
            let t75 = t74 * t56;
            let t78 = t65 + f64x8::splat(0.004) * t67 * t40 + f64x8::splat(1.6e-05) * t71 * t52 + f64x8::splat(6.4e-08) * t75 * t62;
            let t80 = f64x8::splat(M_CBRT2);
            let t81 = f64x8::splat(1.0) / t26 * t80;
            let t83 = (f64x8::splat(1.0) + t17).simd_le(zeta_threshold);
            let t85 = (f64x8::splat(1.0) - t17).simd_le(zeta_threshold);
            let t86 = ((t83).select(t11, (t85).select(t15, t17)));
            let t87 = f64x8::splat(1.0) + t86;
            let t88 = (t87).simd_le(zeta_threshold);
            let t89 = f64x8::splat(1.0) / t21;
            let t90 = (simd::cbrt(t87));
            let t92 = ((t88).select(t89, f64x8::splat(1.0) / t90));
            let t95 = f64x8::splat(1.0) + f64x8::splat(0.4) * t81 * t92;
            let t96 = f64x8::splat(1.0) / t95;
            let t98 = param_CC_2_0;
            let t99 = param_CC_2_1;
            let t100 = t99 * v_sigma0;
            let t103 = param_CC_2_2;
            let t104 = t103 * t44;
            let t107 = param_CC_2_3;
            let t108 = t107 * t56;
            let t111 = t98 + f64x8::splat(0.004) * t100 * t40 + f64x8::splat(1.6e-05) * t104 * t52 + f64x8::splat(6.4e-08) * t108 * t62;
            let t112 = t95 * t95;
            let t113 = f64x8::splat(1.0) / t112;
            let t115 = param_CC_3_0;
            let t116 = param_CC_3_1;
            let t117 = t116 * v_sigma0;
            let t120 = param_CC_3_2;
            let t121 = t120 * t44;
            let t124 = param_CC_3_3;
            let t125 = t124 * t56;
            let t128 = t115 + f64x8::splat(0.004) * t117 * t40 + f64x8::splat(1.6e-05) * t121 * t52 + f64x8::splat(6.4e-08) * t125 * t62;
            let t129 = t112 * t95;
            let t130 = f64x8::splat(1.0) / t129;
            let t132 = t28 + f64x8::splat(0.004) * t30 * t40 + f64x8::splat(1.6e-05) * t45 * t52 + f64x8::splat(6.4e-08) * t57 * t62 + t78 * t96 + t111 * t113 + t128 * t130;
            let t136 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t132));
            let t137 = (v_rho1).simd_le(dens_threshold);
            let t138 = -t16;
            let t140 = ((t14).select(t11, (t10).select(t15, t138 * t7)));
            let t141 = f64x8::splat(1.0) + t140;
            let t142 = (t141).simd_le(zeta_threshold);
            let t143 = (simd::cbrt(t141));
            let t145 = ((t142).select(t22, t143 * t141));
            let t146 = t145 * t26;
            let t147 = t29 * v_sigma2;
            let t148 = v_rho1 * v_rho1;
            let t149 = (simd::cbrt(v_rho1));
            let t150 = t149 * t149;
            let t152 = f64x8::splat(1.0) / t150 / t148;
            let t155 = f64x8::splat(1.0) + f64x8::splat(0.004) * v_sigma2 * t152;
            let t156 = f64x8::splat(1.0) / t155;
            let t157 = t152 * t156;
            let t160 = v_sigma2 * v_sigma2;
            let t161 = t43 * t160;
            let t162 = t148 * t148;
            let t163 = t162 * v_rho1;
            let t165 = f64x8::splat(1.0) / t149 / t163;
            let t166 = t155 * t155;
            let t167 = f64x8::splat(1.0) / t166;
            let t168 = t165 * t167;
            let t171 = t160 * v_sigma2;
            let t172 = t55 * t171;
            let t173 = t162 * t162;
            let t174 = f64x8::splat(1.0) / t173;
            let t175 = t166 * t155;
            let t176 = f64x8::splat(1.0) / t175;
            let t177 = t174 * t176;
            let t180 = t66 * v_sigma2;
            let t183 = t70 * t160;
            let t186 = t74 * t171;
            let t189 = t65 + f64x8::splat(0.004) * t180 * t157 + f64x8::splat(1.6e-05) * t183 * t168 + f64x8::splat(6.4e-08) * t186 * t177;
            let t190 = ((t85).select(t11, (t83).select(t15, -t17)));
            let t191 = f64x8::splat(1.0) + t190;
            let t192 = (t191).simd_le(zeta_threshold);
            let t193 = (simd::cbrt(t191));
            let t195 = ((t192).select(t89, f64x8::splat(1.0) / t193));
            let t198 = f64x8::splat(1.0) + f64x8::splat(0.4) * t81 * t195;
            let t199 = f64x8::splat(1.0) / t198;
            let t201 = t99 * v_sigma2;
            let t204 = t103 * t160;
            let t207 = t107 * t171;
            let t210 = t98 + f64x8::splat(0.004) * t201 * t157 + f64x8::splat(1.6e-05) * t204 * t168 + f64x8::splat(6.4e-08) * t207 * t177;
            let t211 = t198 * t198;
            let t212 = f64x8::splat(1.0) / t211;
            let t214 = t116 * v_sigma2;
            let t217 = t120 * t160;
            let t220 = t124 * t171;
            let t223 = t115 + f64x8::splat(0.004) * t214 * t157 + f64x8::splat(1.6e-05) * t217 * t168 + f64x8::splat(6.4e-08) * t220 * t177;
            let t224 = t211 * t198;
            let t225 = f64x8::splat(1.0) / t224;
            let t227 = t28 + f64x8::splat(0.004) * t147 * t157 + f64x8::splat(1.6e-05) * t161 * t168 + f64x8::splat(6.4e-08) * t172 * t177 + t189 * t199 + t210 * t212 + t223 * t225;
            let t231 = ((t137).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t146 * t227));
            let tzk0 = t136 + t231;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}

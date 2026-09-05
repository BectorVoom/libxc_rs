//! GGA_K_THAKKAR vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_thakkar.c`
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
pub fn gga_k_thakkar_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
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
        let v_sigma0 = load_strided(sigma, ip, np, 3, 0);
        let v_sigma1 = load_strided(sigma, ip, np, 3, 1);
        let v_sigma2 = load_strided(sigma, ip, np, 3, 2);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        let mut acc_vsigma_0 = V_ZERO;
        let mut acc_vsigma_1 = V_ZERO;
        let mut acc_vsigma_2 = V_ZERO;
        {
            let t1 = (v_rho0).simd_le(dens_threshold);
            let t2 = f64x8::splat(M_CBRT3);
            let t3 = t2 * t2;
            let t4 = f64x8::splat(M_CBRTPI);
            let t6 = t3 * t4 * f64x8::splat(M_PI);
            let t7 = v_rho0 + v_rho1;
            let t8 = f64x8::splat(1.0) / t7;
            let t11 = (f64x8::splat(2.0) * v_rho0 * t8).simd_le(zeta_threshold);
            let t12 = zeta_threshold - f64x8::splat(1.0);
            let t15 = (f64x8::splat(2.0) * v_rho1 * t8).simd_le(zeta_threshold);
            let t16 = -t12;
            let t17 = v_rho0 - v_rho1;
            let t19 = ((t11).select(t12, (t15).select(t16, t17 * t8)));
            let t20 = f64x8::splat(1.0) + t19;
            let t21 = (t20).simd_le(zeta_threshold);
            let t22 = (simd::cbrt(zeta_threshold));
            let t23 = t22 * t22;
            let t24 = t23 * zeta_threshold;
            let t25 = (simd::cbrt(t20));
            let t26 = t25 * t25;
            let t28 = ((t21).select(t24, t26 * t20));
            let t29 = (simd::cbrt(t7));
            let t30 = t29 * t29;
            let t31 = t28 * t30;
            let t32 = v_rho0 * v_rho0;
            let t33 = (simd::cbrt(v_rho0));
            let t34 = t33 * t33;
            let t36 = f64x8::splat(1.0) / t34 / t32;
            let t37 = v_sigma0 * t36;
            let t38 = ((v_sigma0).sqrt());
            let t40 = f64x8::splat(1.0) / t33 / v_rho0;
            let t41 = t38 * t40;
            let t42 = (simd::ln(t41 + ((t41 * t41 + f64x8::splat(1.0)).sqrt())));
            let t45 = f64x8::splat(1.0) + f64x8::splat(0.0253) * t41 * t42;
            let t46 = f64x8::splat(1.0) / t45;
            let t49 = f64x8::splat(M_CBRT4);
            let t50 = t49 * t38;
            let t53 = f64x8::splat(2.0) * t50 * t40 + f64x8::splat(1.0);
            let t54 = f64x8::splat(1.0) / t53;
            let t57 = f64x8::splat(1.0) + f64x8::splat(0.0055) * t37 * t46 - f64x8::splat(0.072) * t41 * t54;
            let t61 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t31 * t57));
            let t62 = (v_rho1).simd_le(dens_threshold);
            let t63 = -t17;
            let t65 = ((t15).select(t12, (t11).select(t16, t63 * t8)));
            let t66 = f64x8::splat(1.0) + t65;
            let t67 = (t66).simd_le(zeta_threshold);
            let t68 = (simd::cbrt(t66));
            let t69 = t68 * t68;
            let t71 = ((t67).select(t24, t69 * t66));
            let t72 = t71 * t30;
            let t73 = v_rho1 * v_rho1;
            let t74 = (simd::cbrt(v_rho1));
            let t75 = t74 * t74;
            let t77 = f64x8::splat(1.0) / t75 / t73;
            let t78 = v_sigma2 * t77;
            let t79 = ((v_sigma2).sqrt());
            let t81 = f64x8::splat(1.0) / t74 / v_rho1;
            let t82 = t79 * t81;
            let t83 = (simd::ln(t82 + ((t82 * t82 + f64x8::splat(1.0)).sqrt())));
            let t86 = f64x8::splat(1.0) + f64x8::splat(0.0253) * t82 * t83;
            let t87 = f64x8::splat(1.0) / t86;
            let t90 = t49 * t79;
            let t93 = f64x8::splat(2.0) * t90 * t81 + f64x8::splat(1.0);
            let t94 = f64x8::splat(1.0) / t93;
            let t97 = f64x8::splat(1.0) + f64x8::splat(0.0055) * t78 * t87 - f64x8::splat(0.072) * t82 * t94;
            let t101 = ((t62).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t72 * t97));
            let tzk0 = t61 + t101;
            acc_zk = tzk0;
            let t102 = t7 * t7;
            let t103 = f64x8::splat(1.0) / t102;
            let t104 = t17 * t103;
            let t106 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), t8 - t104)));
            let t109 = ((t21).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t106));
            let t110 = t109 * t30;
            let t114 = f64x8::splat(1.0) / t29;
            let t115 = t28 * t114;
            let t118 = t6 * t115 * t57 / f64x8::splat(10.0);
            let t119 = t32 * v_rho0;
            let t121 = f64x8::splat(1.0) / t34 / t119;
            let t122 = v_sigma0 * t121;
            let t125 = t45 * t45;
            let t126 = f64x8::splat(1.0) / t125;
            let t128 = f64x8::splat(1.0) / t33 / t32;
            let t129 = t38 * t128;
            let t132 = t37 + f64x8::splat(1.0);
            let t133 = ((t132).sqrt());
            let t134 = f64x8::splat(1.0) / t133;
            let t137 = -f64x8::splat(0.03373333333333333) * t129 * t42 - f64x8::splat(0.03373333333333333) * t122 * t134;
            let t138 = t126 * t137;
            let t143 = t53 * t53;
            let t144 = f64x8::splat(1.0) / t143;
            let t145 = t144 * t49;
            let t148 = -f64x8::splat(0.014666666666666666) * t122 * t46 - f64x8::splat(0.0055) * t37 * t138 + f64x8::splat(0.096) * t129 * t54 - f64x8::splat(0.192) * t122 * t145;
            let t153 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t110 * t57 + t118 + f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t31 * t148));
            let t154 = t63 * t103;
            let t156 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), -t8 - t154)));
            let t159 = ((t67).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t69 * t156));
            let t160 = t159 * t30;
            let t164 = t71 * t114;
            let t167 = t6 * t164 * t97 / f64x8::splat(10.0);
            let t169 = ((t62).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t160 * t97 + t167));
            let tvrho0 = t61 + t101 + t7 * (t153 + t169);
            acc_vrho_0 = tvrho0;
            let t173 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), -t8 - t104)));
            let t176 = ((t21).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t173));
            let t177 = t176 * t30;
            let t182 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t177 * t57 + t118));
            let t184 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), t8 - t154)));
            let t187 = ((t67).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t69 * t184));
            let t188 = t187 * t30;
            let t192 = t73 * v_rho1;
            let t194 = f64x8::splat(1.0) / t75 / t192;
            let t195 = v_sigma2 * t194;
            let t198 = t86 * t86;
            let t199 = f64x8::splat(1.0) / t198;
            let t201 = f64x8::splat(1.0) / t74 / t73;
            let t202 = t79 * t201;
            let t205 = t78 + f64x8::splat(1.0);
            let t206 = ((t205).sqrt());
            let t207 = f64x8::splat(1.0) / t206;
            let t210 = -f64x8::splat(0.03373333333333333) * t202 * t83 - f64x8::splat(0.03373333333333333) * t195 * t207;
            let t211 = t199 * t210;
            let t216 = t93 * t93;
            let t217 = f64x8::splat(1.0) / t216;
            let t218 = t217 * t49;
            let t221 = -f64x8::splat(0.014666666666666666) * t195 * t87 - f64x8::splat(0.0055) * t78 * t211 + f64x8::splat(0.096) * t202 * t94 - f64x8::splat(0.192) * t195 * t218;
            let t226 = ((t62).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t188 * t97 + t167 + f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t72 * t221));
            let tvrho1 = t61 + t101 + t7 * (t182 + t226);
            acc_vrho_1 = tvrho1;
            let t231 = f64x8::splat(1.0) / t38;
            let t232 = t231 * t40;
            let t237 = f64x8::splat(0.01265) * t232 * t42 + f64x8::splat(0.01265) * t36 * t134;
            let t238 = t126 * t237;
            let t246 = f64x8::splat(0.0055) * t36 * t46 - f64x8::splat(0.0055) * t37 * t238 - f64x8::splat(0.036) * t232 * t54 + f64x8::splat(0.072) * t36 * t144 * t49;
            let t250 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t31 * t246));
            let tvsigma0 = t7 * t250;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t253 = f64x8::splat(1.0) / t79;
            let t254 = t253 * t81;
            let t259 = f64x8::splat(0.01265) * t254 * t83 + f64x8::splat(0.01265) * t77 * t207;
            let t260 = t199 * t259;
            let t268 = f64x8::splat(0.0055) * t77 * t87 - f64x8::splat(0.0055) * t78 * t260 - f64x8::splat(0.036) * t254 * t94 + f64x8::splat(0.072) * t77 * t217 * t49;
            let t272 = ((t62).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t72 * t268));
            let tvsigma2 = t7 * t272;
            acc_vsigma_2 = tvsigma2;
        }
        store_add(zk, ip, m, acc_zk);
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(vsigma, ip, m, 3, 0, acc_vsigma_0);
        store_strided(vsigma, ip, m, 3, 1, acc_vsigma_1);
        store_strided(vsigma, ip, m, 3, 2, acc_vsigma_2);
        ip += 8;
    }
}

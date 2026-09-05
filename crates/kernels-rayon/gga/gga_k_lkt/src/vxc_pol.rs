//! GGA_K_LKT vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_lkt.c`
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
pub fn gga_k_lkt_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_a: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_a = f64x8::splat(param_a);
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
            let t32 = f64x8::splat(M_CBRT6);
            let t33 = t32 * t32;
            let t34 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t35 = (simd::cbrt(t34));
            let t37 = t33 / t35;
            let t38 = ((v_sigma0).sqrt());
            let t39 = (simd::cbrt(v_rho0));
            let t41 = f64x8::splat(1.0) / t39 / v_rho0;
            let t44 = t37 * t38 * t41 / f64x8::splat(12.0);
            let t45 = (t44).simd_lt(f64x8::splat(200.0));
            let t46 = ((t45).select(t44, f64x8::splat(200.0)));
            let t47 = param_a * t46;
            let t48 = (simd::cosh(t47));
            let t49 = f64x8::splat(1.0) / t48;
            let t50 = t35 * t35;
            let t52 = t32 / t50;
            let t53 = v_rho0 * v_rho0;
            let t54 = t39 * t39;
            let t56 = f64x8::splat(1.0) / t54 / t53;
            let t60 = t49 + f64x8::splat(5.0) / f64x8::splat(72.0) * t52 * v_sigma0 * t56;
            let t64 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t31 * t60));
            let t65 = (v_rho1).simd_le(dens_threshold);
            let t66 = -t17;
            let t68 = ((t15).select(t12, (t11).select(t16, t66 * t8)));
            let t69 = f64x8::splat(1.0) + t68;
            let t70 = (t69).simd_le(zeta_threshold);
            let t71 = (simd::cbrt(t69));
            let t72 = t71 * t71;
            let t74 = ((t70).select(t24, t72 * t69));
            let t75 = t74 * t30;
            let t76 = ((v_sigma2).sqrt());
            let t77 = (simd::cbrt(v_rho1));
            let t79 = f64x8::splat(1.0) / t77 / v_rho1;
            let t82 = t37 * t76 * t79 / f64x8::splat(12.0);
            let t83 = (t82).simd_lt(f64x8::splat(200.0));
            let t84 = ((t83).select(t82, f64x8::splat(200.0)));
            let t85 = param_a * t84;
            let t86 = (simd::cosh(t85));
            let t87 = f64x8::splat(1.0) / t86;
            let t88 = v_rho1 * v_rho1;
            let t89 = t77 * t77;
            let t91 = f64x8::splat(1.0) / t89 / t88;
            let t95 = t87 + f64x8::splat(5.0) / f64x8::splat(72.0) * t52 * v_sigma2 * t91;
            let t99 = ((t65).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t75 * t95));
            let tzk0 = t64 + t99;
            acc_zk = tzk0;
            let t100 = t7 * t7;
            let t101 = f64x8::splat(1.0) / t100;
            let t102 = t17 * t101;
            let t104 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), t8 - t102)));
            let t107 = ((t21).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t104));
            let t108 = t107 * t30;
            let t112 = f64x8::splat(1.0) / t29;
            let t113 = t28 * t112;
            let t116 = t6 * t113 * t60 / f64x8::splat(10.0);
            let t117 = t48 * t48;
            let t118 = f64x8::splat(1.0) / t117;
            let t119 = t118 * param_a;
            let t121 = f64x8::splat(1.0) / t39 / t53;
            let t125 = ((t45).select(-t37 * t38 * t121 / f64x8::splat(9.0), f64x8::splat(0.0)));
            let t126 = (simd::sinh(t47));
            let t127 = t125 * t126;
            let t129 = t53 * v_rho0;
            let t131 = f64x8::splat(1.0) / t54 / t129;
            let t135 = -t119 * t127 - f64x8::splat(5.0) / f64x8::splat(27.0) * t52 * v_sigma0 * t131;
            let t140 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t108 * t60 + t116 + f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t31 * t135));
            let t141 = t66 * t101;
            let t143 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), -t8 - t141)));
            let t146 = ((t70).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t72 * t143));
            let t147 = t146 * t30;
            let t151 = t74 * t112;
            let t154 = t6 * t151 * t95 / f64x8::splat(10.0);
            let t156 = ((t65).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t147 * t95 + t154));
            let tvrho0 = t64 + t99 + t7 * (t140 + t156);
            acc_vrho_0 = tvrho0;
            let t160 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), -t8 - t102)));
            let t163 = ((t21).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t160));
            let t164 = t163 * t30;
            let t169 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t164 * t60 + t116));
            let t171 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), t8 - t141)));
            let t174 = ((t70).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t72 * t171));
            let t175 = t174 * t30;
            let t179 = t86 * t86;
            let t180 = f64x8::splat(1.0) / t179;
            let t181 = t180 * param_a;
            let t183 = f64x8::splat(1.0) / t77 / t88;
            let t187 = ((t83).select(-t37 * t76 * t183 / f64x8::splat(9.0), f64x8::splat(0.0)));
            let t188 = (simd::sinh(t85));
            let t189 = t187 * t188;
            let t191 = t88 * v_rho1;
            let t193 = f64x8::splat(1.0) / t89 / t191;
            let t197 = -t181 * t189 - f64x8::splat(5.0) / f64x8::splat(27.0) * t52 * v_sigma2 * t193;
            let t202 = ((t65).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t175 * t95 + t154 + f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t75 * t197));
            let tvrho1 = t64 + t99 + t7 * (t169 + t202);
            acc_vrho_1 = tvrho1;
            let t205 = f64x8::splat(1.0) / t38;
            let t209 = ((t45).select(t37 * t205 * t41 / f64x8::splat(24.0), f64x8::splat(0.0)));
            let t210 = t209 * t126;
            let t214 = -t119 * t210 + f64x8::splat(5.0) / f64x8::splat(72.0) * t52 * t56;
            let t218 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t31 * t214));
            let tvsigma0 = t7 * t218;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t219 = f64x8::splat(1.0) / t76;
            let t223 = ((t83).select(t37 * t219 * t79 / f64x8::splat(24.0), f64x8::splat(0.0)));
            let t224 = t223 * t188;
            let t228 = -t181 * t224 + f64x8::splat(5.0) / f64x8::splat(72.0) * t52 * t91;
            let t232 = ((t65).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t75 * t228));
            let tvsigma2 = t7 * t232;
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

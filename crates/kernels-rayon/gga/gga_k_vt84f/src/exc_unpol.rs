//! GGA_K_VT84F exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_vt84f.c`
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
pub fn gga_k_vt84f_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_alpha: f64,
    param_mu: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_alpha = f64x8::splat(param_alpha);
    let param_mu = f64x8::splat(param_mu);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let mut acc_zk = V_ZERO;
        {
            let t2 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = t3 * t3;
            let t5 = f64x8::splat(M_CBRTPI);
            let t7 = t4 * t5 * f64x8::splat(M_PI);
            let t8 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t9 = zeta_threshold - f64x8::splat(1.0);
            let t11 = ((t8).select(t9, (t8).select(-t9, f64x8::splat(0.0))));
            let t12 = f64x8::splat(1.0) + t11;
            let t14 = (simd::cbrt(zeta_threshold));
            let t15 = t14 * t14;
            let t17 = (simd::cbrt(t12));
            let t18 = t17 * t17;
            let t20 = (((t12).simd_le(zeta_threshold)).select(t15 * zeta_threshold, t18 * t12));
            let t21 = (simd::cbrt(v_rho));
            let t22 = t21 * t21;
            let t23 = t20 * t22;
            let t24 = f64x8::splat(M_CBRT6);
            let t25 = t24 * t24;
            let t26 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t27 = (simd::cbrt(t26));
            let t29 = t25 / t27;
            let t30 = ((v_sigma).sqrt());
            let t31 = f64x8::splat(M_CBRT2);
            let t32 = t30 * t31;
            let t34 = f64x8::splat(1.0) / t21 / v_rho;
            let t37 = t29 * t32 * t34 / f64x8::splat(12.0);
            let t38 = ((f64x8::splat(f64::EPSILON)).sqrt());
            let t39 = (t37).simd_le(t38);
            let t41 = (-param_mu + param_alpha + f64x8::splat(5.0) / f64x8::splat(3.0)) * t24;
            let t42 = t27 * t27;
            let t43 = f64x8::splat(1.0) / t42;
            let t44 = t41 * t43;
            let t45 = t31 * t31;
            let t46 = v_sigma * t45;
            let t47 = v_rho * v_rho;
            let t49 = f64x8::splat(1.0) / t22 / t47;
            let t53 = param_mu * param_alpha;
            let t54 = param_mu * param_mu;
            let t56 = (t53 + t54 - param_alpha) * t25;
            let t58 = f64x8::splat(1.0) / t27 / t26;
            let t59 = t56 * t58;
            let t60 = v_sigma * v_sigma;
            let t61 = t60 * t31;
            let t62 = t47 * t47;
            let t63 = t62 * v_rho;
            let t65 = f64x8::splat(1.0) / t21 / t63;
            let t69 = param_alpha * param_alpha;
            let t71 = param_mu * t69 / f64x8::splat(2.0);
            let t74 = t69 / f64x8::splat(2.0);
            let t76 = t26 * t26;
            let t78 = (-t71 - (t53 + t54) * param_mu - t74) / t76;
            let t79 = t60 * v_sigma;
            let t80 = t62 * t62;
            let t81 = f64x8::splat(1.0) / t80;
            let t85 = t69 * param_alpha;
            let t89 = t54 * param_mu;
            let t93 = (param_mu * t85 / f64x8::splat(6.0) - (-param_alpha * t54 - t71 - t89) * param_mu + t74) * t24;
            let t95 = f64x8::splat(1.0) / t42 / t76;
            let t96 = t93 * t95;
            let t97 = t60 * t60;
            let t98 = t97 * t45;
            let t99 = t80 * t47;
            let t101 = f64x8::splat(1.0) / t22 / t99;
            let t106 = (t38).simd_lt(t37);
            let t107 = ((t106).select(t37, t38));
            let t108 = t107 * t107;
            let t109 = param_mu * t108;
            let t110 = param_alpha * t108;
            let t111 = (simd::exp(-t110));
            let t112 = f64x8::splat(1.0) + t109;
            let t113 = f64x8::splat(1.0) / t112;
            let t114 = t111 * t113;
            let t116 = t108 * t108;
            let t118 = (simd::exp(-param_alpha * t116));
            let t119 = f64x8::splat(1.0) - t118;
            let t120 = f64x8::splat(1.0) / t108;
            let t121 = t120 - f64x8::splat(1.0);
            let t125 = ((t39).select(f64x8::splat(1.0) + t44 * t46 * t49 / f64x8::splat(24.0) + t59 * t61 * t65 / f64x8::splat(288.0) + t78 * t79 * t81 / f64x8::splat(576.0) + t96 * t98 * t101 / f64x8::splat(13824.0), f64x8::splat(1.0) - t109 * t114 + t119 * t121 + f64x8::splat(5.0) / f64x8::splat(3.0) * t108));
            let t129 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t125));
            let tzk0 = f64x8::splat(2.0) * t129;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}

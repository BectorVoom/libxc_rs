//! MGGA_X_RSCAN exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_rscan.c`
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
pub fn mgga_x_rscan_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_alphar: f64,
    param_c2: f64,
    param_d: f64,
    param_k1: f64,
    param_taur: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_alphar = f64x8::splat(param_alphar);
    let param_c2 = f64x8::splat(param_c2);
    let param_d = f64x8::splat(param_d);
    let param_k1 = f64x8::splat(param_k1);
    let param_taur = f64x8::splat(param_taur);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let v_lapl = load(lapl, ip, np);
        let v_tau = load(tau, ip, np);
        let mut acc_zk = V_ZERO;
        {
            let t3 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t4 = f64x8::splat(M_CBRT3);
            let t5 = f64x8::splat(M_CBRTPI);
            let t8 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t9 = zeta_threshold - f64x8::splat(1.0);
            let t11 = ((t8).select(t9, (t8).select(-t9, f64x8::splat(0.0))));
            let t12 = f64x8::splat(1.0) + t11;
            let t14 = (simd::cbrt(zeta_threshold));
            let t16 = (simd::cbrt(t12));
            let t18 = (((t12).simd_le(zeta_threshold)).select(t14 * zeta_threshold, t16 * t12));
            let t19 = t4 / t5 * t18;
            let t20 = (simd::cbrt(v_rho));
            let t21 = f64x8::splat(M_CBRT6);
            let t22 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t23 = (simd::cbrt(t22));
            let t24 = t23 * t23;
            let t25 = f64x8::splat(1.0) / t24;
            let t26 = t21 * t25;
            let t27 = f64x8::splat(M_CBRT2);
            let t28 = t27 * t27;
            let t29 = v_sigma * t28;
            let t30 = v_rho * v_rho;
            let t31 = t20 * t20;
            let t33 = f64x8::splat(1.0) / t31 / t30;
            let t34 = t29 * t33;
            let t35 = t26 * t34;
            let t39 = f64x8::splat(100.0) / f64x8::splat(6561.0) / param_k1 - f64x8::splat(73.0) / f64x8::splat(648.0);
            let t40 = t21 * t21;
            let t42 = t23 * t22;
            let t43 = f64x8::splat(1.0) / t42;
            let t44 = t39 * t40 * t43;
            let t45 = v_sigma * v_sigma;
            let t46 = t45 * t27;
            let t47 = t30 * t30;
            let t48 = t47 * v_rho;
            let t50 = f64x8::splat(1.0) / t20 / t48;
            let t55 = (simd::exp(-f64x8::splat(27.0) / f64x8::splat(80.0) * t39 * t21 * t25 * t34));
            let t56 = t50 * t55;
            let t60 = ((f64x8::splat(146.0)).sqrt());
            let t61 = t60 * t21;
            let t62 = t61 * t25;
            let t65 = t12 * t12;
            let t66 = t65 * t65;
            let t67 = t66 * t12;
            let t68 = t67 * t48;
            let t69 = v_tau * t28;
            let t70 = t31 * v_rho;
            let t71 = f64x8::splat(1.0) / t70;
            let t74 = t69 * t71 - t34 / f64x8::splat(8.0);
            let t75 = (f64x8::splat(0.0)).simd_lt(t74);
            let t76 = ((t75).select(t74, f64x8::splat(0.0)));
            let t77 = t76 * t76;
            let t78 = t77 * t76;
            let t79 = t12 * v_rho;
            let t80 = (simd::cbrt(t79));
            let t81 = t80 * t80;
            let t84 = t40 * t24;
            let t88 = f64x8::splat(3.0) / f64x8::splat(40.0) * t27 * t81 * t79 * t84 + param_taur / f64x8::splat(2.0);
            let t89 = t88 * t88;
            let t90 = t89 * t88;
            let t91 = f64x8::splat(1.0) / t90;
            let t93 = t65 * t12;
            let t94 = t30 * v_rho;
            let t96 = t80 * t93 * t94;
            let t97 = t28 * t96;
            let t98 = f64x8::splat(1.0) / t89;
            let t99 = t77 * t98;
            let t102 = t97 * t99 / f64x8::splat(16.0) + param_alphar;
            let t103 = f64x8::splat(1.0) / t102;
            let t104 = t78 * t91 * t103;
            let t106 = t68 * t104 / f64x8::splat(32.0);
            let t107 = f64x8::splat(1.0) - t106;
            let t109 = t107 * t107;
            let t111 = (simd::exp(-t109 / f64x8::splat(2.0)));
            let t114 = f64x8::splat(7.0) / f64x8::splat(12960.0) * t62 * t34 + t60 * t107 * t111 / f64x8::splat(100.0);
            let t115 = t114 * t114;
            let t116 = param_k1 + f64x8::splat(5.0) / f64x8::splat(972.0) * t35 + t44 * t46 * t56 / f64x8::splat(288.0) + t115;
            let t121 = f64x8::splat(1.0) + param_k1 * (f64x8::splat(1.0) - param_k1 / t116);
            let t122 = (t106).simd_le(f64x8::splat(2.5));
            let t123 = (f64x8::splat(2.5)).simd_lt(t106);
            let t124 = ((t123).select(f64x8::splat(2.5), t106));
            let t126 = t124 * t124;
            let t128 = t126 * t124;
            let t130 = t126 * t126;
            let t132 = t130 * t124;
            let t134 = t130 * t126;
            let t139 = ((t123).select(t106, f64x8::splat(2.5)));
            let t140 = f64x8::splat(1.0) - t139;
            let t143 = (simd::exp(param_c2 / t140));
            let t145 = ((t122).select(f64x8::splat(1.0) - f64x8::splat(0.667) * t124 - f64x8::splat(0.4445555) * t126 - f64x8::splat(0.663086601049) * t128 + f64x8::splat(1.45129704449) * t130 - f64x8::splat(0.887998041597) * t132 + f64x8::splat(0.234528941479) * t134 - f64x8::splat(0.023185843322) * t130 * t128, -param_d * t143));
            let t146 = f64x8::splat(1.0) - t145;
            let t149 = t121 * t146 + f64x8::splat(1.174) * t145;
            let t151 = ((f64x8::splat(3.0)).sqrt());
            let t152 = f64x8::splat(1.0) / t23;
            let t153 = t40 * t152;
            let t154 = ((v_sigma).sqrt());
            let t155 = t154 * t27;
            let t157 = f64x8::splat(1.0) / t20 / v_rho;
            let t159 = t153 * t155 * t157;
            let t160 = ((t159).sqrt());
            let t164 = (simd::exp(-f64x8::splat(9.8958) * t151 / t160));
            let t165 = f64x8::splat(1.0) - t164;
            let t169 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t149 * t165));
            let tzk0 = f64x8::splat(2.0) * t169;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}

//! MGGA_X_REGTPSS exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_regtpss.c`
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
pub fn mgga_x_regtpss_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
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
            let t7 = t4 / t5;
            let t8 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t9 = zeta_threshold - f64x8::splat(1.0);
            let t11 = ((t8).select(t9, (t8).select(-t9, f64x8::splat(0.0))));
            let t12 = f64x8::splat(1.0) + t11;
            let t14 = (simd::cbrt(zeta_threshold));
            let t16 = (simd::cbrt(t12));
            let t18 = (((t12).simd_le(zeta_threshold)).select(t14 * zeta_threshold, t16 * t12));
            let t19 = (simd::cbrt(v_rho));
            let t20 = t18 * t19;
            let t21 = f64x8::splat(1.0) / v_rho;
            let t22 = v_sigma * t21;
            let t23 = f64x8::splat(1.0) / v_tau;
            let t24 = t22 * t23;
            let t25 = ((t24) * (t24) * (t24));
            let t26 = v_sigma * v_sigma;
            let t27 = v_rho * v_rho;
            let t28 = f64x8::splat(1.0) / t27;
            let t29 = t26 * t28;
            let t30 = v_tau * v_tau;
            let t31 = f64x8::splat(1.0) / t30;
            let t32 = t29 * t31;
            let t34 = f64x8::splat(1.0) + t32 / f64x8::splat(64.0);
            let t35 = t34 * t34;
            let t36 = f64x8::splat(1.0) / t35;
            let t40 = f64x8::splat(M_CBRT6);
            let t41 = (f64x8::splat(10.0) / f64x8::splat(81.0) + f64x8::splat(0.0045938270703125) * t25 * t36) * t40;
            let t42 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t43 = (simd::cbrt(t42));
            let t44 = t43 * t43;
            let t45 = f64x8::splat(1.0) / t44;
            let t46 = t41 * t45;
            let t47 = f64x8::splat(M_CBRT2);
            let t48 = t47 * t47;
            let t49 = v_sigma * t48;
            let t50 = t19 * t19;
            let t52 = f64x8::splat(1.0) / t50 / t27;
            let t53 = t49 * t52;
            let t56 = v_tau * t48;
            let t58 = f64x8::splat(1.0) / t50 / v_rho;
            let t61 = t56 * t58 - t53 / f64x8::splat(8.0);
            let t62 = t61 * t40;
            let t63 = t62 * t45;
            let t65 = f64x8::splat(5.0) / f64x8::splat(9.0) * t63 - f64x8::splat(1.0);
            let t66 = t45 * t65;
            let t69 = f64x8::splat(1.0) + f64x8::splat(0.2222222222222222) * t62 * t66;
            let t70 = ((t69).sqrt());
            let t71 = f64x8::splat(1.0) / t70;
            let t74 = t40 * t45;
            let t75 = t74 * t53;
            let t76 = t75 / f64x8::splat(36.0);
            let t77 = f64x8::splat(9.0) / f64x8::splat(20.0) * t65 * t71 + t76;
            let t78 = t77 * t77;
            let t81 = t40 * t40;
            let t83 = f64x8::splat(1.0) / t43 / t42;
            let t84 = t81 * t83;
            let t85 = t26 * t47;
            let t86 = t27 * t27;
            let t87 = t86 * v_rho;
            let t89 = f64x8::splat(1.0) / t19 / t87;
            let t91 = t84 * t85 * t89;
            let t93 = f64x8::splat(162.0) * t32 + f64x8::splat(100.0) * t91;
            let t94 = ((t93).sqrt());
            let t97 = f64x8::splat(6.582356890714508e-05) * t91;
            let t99 = t26 * v_sigma;
            let t100 = t86 * t86;
            let t101 = f64x8::splat(1.0) / t100;
            let t103 = f64x8::splat(5.408850610708026e-06) * t99 * t101;
            let t104 = t46 * t53 / f64x8::splat(24.0) + f64x8::splat(146.0) / f64x8::splat(2025.0) * t78 - f64x8::splat(73.0) / f64x8::splat(97200.0) * t77 * t94 + t97 + f64x8::splat(0.0020448759451792767) * t32 + t103;
            let t106 = f64x8::splat(1.0) + f64x8::splat(0.06134627835537829) * t75;
            let t107 = t106 * t106;
            let t108 = f64x8::splat(1.0) / t107;
            let t110 = f64x8::splat(0.804) + t104 * t108;
            let t112 = f64x8::splat(0.646416) / t110;
            let t113 = -t65;
            let t114 = t113 * t113;
            let t115 = t114 * t113;
            let t116 = t61 * t61;
            let t117 = t116 * t81;
            let t118 = t117 * t83;
            let t120 = f64x8::splat(1.0) + f64x8::splat(0.6714891975308642) * t118;
            let t121 = ((t120).sqrt());
            let t123 = f64x8::splat(1.0) / t121 / t120;
            let t124 = t115 * t123;
            let t126 = (simd::exp(-t75 / f64x8::splat(8.0)));
            let t128 = -f64x8::splat(0.45) + t76;
            let t129 = t128 * t128;
            let t132 = f64x8::splat(2592.0) + f64x8::splat(25.0) * t91;
            let t133 = ((t132).sqrt());
            let t136 = f64x8::splat(0.029644443963477367) * t75 + f64x8::splat(146.0) / f64x8::splat(2025.0) * t129 - f64x8::splat(73.0) / f64x8::splat(48600.0) * t128 * t133 + t97 + f64x8::splat(0.1308720604914737) + t103;
            let t138 = f64x8::splat(0.804) + t136 * t108;
            let t141 = -f64x8::splat(0.646416) / t138 + t112;
            let t142 = t126 * t141;
            let t144 = f64x8::splat(1.804) - t112 + t124 * t142;
            let t148 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t144));
            let tzk0 = f64x8::splat(2.0) * t148;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}

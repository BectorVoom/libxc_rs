//! MGGA_X_RPPSCAN exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_rppscan.c`
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

#[allow(unused_variables, non_snake_case)]
pub fn mgga_x_rppscan_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_c2: f64,
    param_d: f64,
    param_eta: f64,
    param_k1: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_c2 = f64x8::splat(param_c2);
    let param_d = f64x8::splat(param_d);
    let param_eta = f64x8::splat(param_eta);
    let param_k1 = f64x8::splat(param_k1);
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
            let t65 = v_tau * t28;
            let t66 = t31 * v_rho;
            let t67 = f64x8::splat(1.0) / t66;
            let t70 = t65 * t67 - t34 / f64x8::splat(8.0);
            let t73 = param_eta * v_sigma;
            let t74 = t28 * t33;
            let t77 = f64x8::splat(3.0) / f64x8::splat(10.0) * t40 * t24 + t73 * t74 / f64x8::splat(8.0);
            let t78 = f64x8::splat(1.0) / t77;
            let t79 = t70 * t78;
            let t80 = f64x8::splat(1.0) - t79;
            let t82 = t80 * t80;
            let t84 = (simd::exp(-t82 / f64x8::splat(2.0)));
            let t87 = f64x8::splat(7.0) / f64x8::splat(12960.0) * t62 * t34 + t60 * t80 * t84 / f64x8::splat(100.0);
            let t88 = t87 * t87;
            let t89 = param_k1 + f64x8::splat(5.0) / f64x8::splat(972.0) * t35 + t44 * t46 * t56 / f64x8::splat(288.0) + t88;
            let t94 = f64x8::splat(1.0) + param_k1 * (f64x8::splat(1.0) - param_k1 / t89);
            let t95 = (t79).simd_le(f64x8::splat(2.5));
            let t96 = (f64x8::splat(2.5)).simd_lt(t79);
            let t97 = ((t96).select(f64x8::splat(2.5), t79));
            let t99 = t97 * t97;
            let t101 = t99 * t97;
            let t103 = t99 * t99;
            let t105 = t103 * t97;
            let t107 = t103 * t99;
            let t112 = ((t96).select(t79, f64x8::splat(2.5)));
            let t113 = f64x8::splat(1.0) - t112;
            let t116 = (simd::exp(param_c2 / t113));
            let t118 = ((t95).select(f64x8::splat(1.0) - f64x8::splat(0.667) * t97 - f64x8::splat(0.4445555) * t99 - f64x8::splat(0.663086601049) * t101 + f64x8::splat(1.45129704449) * t103 - f64x8::splat(0.887998041597) * t105 + f64x8::splat(0.234528941479) * t107 - f64x8::splat(0.023185843322) * t103 * t101, -param_d * t116));
            let t119 = f64x8::splat(1.0) - t118;
            let t122 = t94 * t119 + f64x8::splat(1.174) * t118;
            let t124 = ((f64x8::splat(3.0)).sqrt());
            let t125 = f64x8::splat(1.0) / t23;
            let t126 = t40 * t125;
            let t127 = ((v_sigma).sqrt());
            let t128 = t127 * t27;
            let t130 = f64x8::splat(1.0) / t20 / v_rho;
            let t132 = t126 * t128 * t130;
            let t133 = ((t132).sqrt());
            let t137 = (simd::exp(-f64x8::splat(9.8958) * t124 / t133));
            let t138 = f64x8::splat(1.0) - t137;
            let t142 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t122 * t138));
            let tzk0 = f64x8::splat(2.0) * t142;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}

//! MGGA_X_PKZB vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_pkzb.c`
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
pub fn mgga_x_pkzb_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
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
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
        let mut acc_vlapl = V_ZERO;
        let mut acc_vtau = V_ZERO;
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
            let t31 = t19 * t19;
            let t33 = f64x8::splat(1.0) / t31 / t30;
            let t34 = t29 * t33;
            let t35 = t26 * t34;
            let t37 = v_tau * t28;
            let t39 = f64x8::splat(1.0) / t31 / v_rho;
            let t44 = t26 * t37 * t39 / f64x8::splat(4.0) - f64x8::splat(9.0) / f64x8::splat(20.0) - t35 / f64x8::splat(288.0);
            let t45 = t44 * t44;
            let t47 = t44 * t21;
            let t48 = t47 * t25;
            let t51 = t21 * t21;
            let t53 = f64x8::splat(1.0) / t23 / t22;
            let t54 = t51 * t53;
            let t55 = v_sigma * v_sigma;
            let t56 = t55 * t27;
            let t57 = t30 * t30;
            let t58 = t57 * v_rho;
            let t60 = f64x8::splat(1.0) / t19 / t58;
            let t64 = f64x8::splat(0.804) + f64x8::splat(5.0) / f64x8::splat(972.0) * t35 + f64x8::splat(146.0) / f64x8::splat(2025.0) * t45 - f64x8::splat(73.0) / f64x8::splat(9720.0) * t48 * t34 + f64x8::splat(0.0004581846800182562) * t54 * t56 * t60;
            let t67 = f64x8::splat(1.804) - f64x8::splat(0.646416) / t64;
            let t71 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t18 * t19 * t67));
            let tzk0 = f64x8::splat(2.0) * t71;
            acc_zk = tzk0;
            let t72 = f64x8::splat(1.0) / t31;
            let t77 = t4 * t18;
            let t78 = t64 * t64;
            let t79 = f64x8::splat(1.0) / t78;
            let t80 = t19 * t79;
            let t81 = t30 * v_rho;
            let t83 = f64x8::splat(1.0) / t31 / t81;
            let t84 = t29 * t83;
            let t85 = t26 * t84;
            let t91 = -f64x8::splat(5.0) / f64x8::splat(12.0) * t26 * t37 * t33 + t85 / f64x8::splat(108.0);
            let t94 = t91 * t21;
            let t95 = t94 * t25;
            let t100 = t57 * t30;
            let t102 = f64x8::splat(1.0) / t19 / t100;
            let t106 = -f64x8::splat(10.0) / f64x8::splat(729.0) * t85 + f64x8::splat(292.0) / f64x8::splat(2025.0) * t44 * t91 - f64x8::splat(73.0) / f64x8::splat(9720.0) * t95 * t34 + f64x8::splat(73.0) / f64x8::splat(3645.0) * t48 * t84 - f64x8::splat(0.002443651626764033) * t54 * t56 * t102;
            let t111 = ((t3).select(f64x8::splat(0.0), -t7 * t18 * t72 * t67 / f64x8::splat(8.0) - f64x8::splat(0.1655109536374632) * t77 * t80 * t106));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t111 + f64x8::splat(2.0) * t71;
            acc_vrho = tvrho0;
            let t117 = t25 * t28;
            let t118 = t117 * t33;
            let t119 = t47 * t118;
            let t123 = t54 * t27 * t60 * v_sigma;
            let t125 = f64x8::splat(5.0) / f64x8::splat(972.0) * t26 * t28 * t33 - f64x8::splat(146.0) / f64x8::splat(18225.0) * t119 + f64x8::splat(0.0009685241382715376) * t123;
            let t129 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(0.1655109536374632) * t77 * t80 * t125));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t129;
            acc_vsigma = tvsigma0;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl = tvlapl0;
            let t131 = t117 * t39;
            let t140 = f64x8::splat(73.0) / f64x8::splat(2025.0) * t47 * t131 - f64x8::splat(73.0) / f64x8::splat(19440.0) * t54 * t27 / t19 / t57 * v_sigma;
            let t144 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(0.1655109536374632) * t77 * t80 * t140));
            let tvtau0 = f64x8::splat(2.0) * v_rho * t144;
            acc_vtau = tvtau0;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        store_add(vlapl, ip, m, acc_vlapl);
        store_add(vtau, ip, m, acc_vtau);
        ip += 8;
    }
}

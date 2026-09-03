//! MGGA_X_MBEEFVDW exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_mbeefvdw.c`
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
pub fn mgga_x_mbeefvdw_exc_unpol(
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
            let t12 = t11 + f64x8::splat(1.0);
            let t14 = (simd::cbrt(zeta_threshold));
            let t16 = (simd::cbrt(t12));
            let t18 = (((t12).simd_le(zeta_threshold)).select(t14 * zeta_threshold, t16 * t12));
            let t19 = (simd::cbrt(v_rho));
            let t20 = t18 * t19;
            let t21 = f64x8::splat(M_CBRT6);
            let t22 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t23 = (simd::cbrt(t22));
            let t24 = t23 * t23;
            let t25 = f64x8::splat(1.0) / t24;
            let t26 = t21 * t25;
            let t27 = t26 * v_sigma;
            let t28 = f64x8::splat(M_CBRT2);
            let t29 = t28 * t28;
            let t30 = v_rho * v_rho;
            let t31 = t19 * t19;
            let t33 = f64x8::splat(1.0) / t31 / t30;
            let t34 = t29 * t33;
            let t35 = v_sigma * t29;
            let t36 = t35 * t33;
            let t39 = f64x8::splat(6.5124) + t26 * t36 / f64x8::splat(24.0);
            let t40 = f64x8::splat(1.0) / t39;
            let t41 = t34 * t40;
            let t42 = t27 * t41;
            let t44 = t42 / f64x8::splat(12.0) - f64x8::splat(1.0);
            let t45 = v_tau * t29;
            let t47 = f64x8::splat(1.0) / t31 / v_rho;
            let t53 = f64x8::splat(5.0) / f64x8::splat(9.0) * (t45 * t47 - t36 / f64x8::splat(8.0)) * t21 * t25;
            let t54 = (f64x8::splat(10000.0)).simd_le(t53);
            let t55 = (f64x8::splat(10000.0)).simd_lt(t53);
            let t56 = ((t55).select(t53, f64x8::splat(10000.0)));
            let t57 = t56 * t56;
            let t60 = t57 * t56;
            let t61 = f64x8::splat(1.0) / t60;
            let t62 = t57 * t57;
            let t63 = f64x8::splat(1.0) / t62;
            let t66 = ((t55).select(f64x8::splat(10000.0), t53));
            let t67 = t66 * t66;
            let t68 = f64x8::splat(1.0) - t67;
            let t69 = t68 * t68;
            let t70 = t69 * t68;
            let t71 = t67 * t66;
            let t72 = f64x8::splat(1.0) + t71;
            let t74 = t71 * t72 + f64x8::splat(1.0);
            let t75 = f64x8::splat(1.0) / t74;
            let t77 = ((t54).select(f64x8::splat(1.0) - f64x8::splat(3.0) / t57 - t61 + f64x8::splat(3.0) * t63, -t70 * t75));
            let t78 = t77 * t77;
            let t79 = t78 * t78;
            let t82 = f64x8::splat(3.0) / f64x8::splat(8.0) + f64x8::splat(35.0) / f64x8::splat(8.0) * t79 - f64x8::splat(15.0) / f64x8::splat(4.0) * t78;
            let t85 = t78 * t77;
            let t88 = f64x8::splat(5.0) / f64x8::splat(2.0) * t85 - f64x8::splat(3.0) / f64x8::splat(2.0) * t77;
            let t92 = -f64x8::splat(1.0) / f64x8::splat(2.0) + f64x8::splat(3.0) / f64x8::splat(2.0) * t78;
            let t95 = t44 * t77;
            let t99 = t44 * t44;
            let t100 = t99 * t99;
            let t106 = f64x8::splat(3.0) / f64x8::splat(8.0) + f64x8::splat(35.0) / f64x8::splat(8.0) * t100 - f64x8::splat(15.0) / f64x8::splat(4.0) * t99;
            let t113 = -f64x8::splat(1.00478906e-07) * t44 * t82 - f64x8::splat(0.00608338264) * t44 * t88 + f64x8::splat(0.0318024096) * t44 * t92 + f64x8::splat(0.0453837246) * t95 - f64x8::splat(0.06972770593) * t77 + f64x8::splat(0.0217681859775) * t78 + f64x8::splat(0.00618699843125) * t100 + f64x8::splat(0.01214700985) * t42 - f64x8::splat(0.0851282539125) * t99 - f64x8::splat(3.40722258e-09) * t106 * t82 + f64x8::splat(5.74317889e-08) * t106 * t88 - f64x8::splat(5.00749348e-07) * t106 * t92;
            let t114 = t106 * t77;
            let t116 = t99 * t44;
            let t119 = f64x8::splat(5.0) / f64x8::splat(2.0) * t116 - t42 / f64x8::splat(8.0) + f64x8::splat(3.0) / f64x8::splat(2.0);
            let t126 = t119 * t77;
            let t129 = -f64x8::splat(1.0) / f64x8::splat(2.0) + f64x8::splat(3.0) / f64x8::splat(2.0) * t99;
            let t136 = t129 * t77;
            let t141 = f64x8::splat(1.0451438955835) + f64x8::splat(9.19317034e-07) * t114 + f64x8::splat(3.97324768e-09) * t119 * t82 - f64x8::splat(5.49909413e-08) * t119 * t88 + f64x8::splat(1.33707403e-07) * t119 * t92 + f64x8::splat(0.0192374554) * t126 + f64x8::splat(2.01895739e-07) * t129 * t82 - f64x8::splat(6.57949254e-07) * t129 * t88 - f64x8::splat(0.00521818079) * t129 * t92 - f64x8::splat(0.0222650139) * t136 + f64x8::splat(0.00061919587625) * t79 - f64x8::splat(0.050282912) * t116 + f64x8::splat(0.00351985355) * t85;
            let t142 = t113 + t141;
            let t146 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t7 * t20 * t142));
            let tzk0 = f64x8::splat(2.0) * t146;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}

//! GGA_X_SFAT_PBE exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_sfat_pbe.c`
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
pub fn gga_x_sfat_pbe_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_hyb_omega_0 = f64x8::splat(param_hyb_omega_0);
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
            let t4 = f64x8::splat(M_CBRTPI);
            let t7 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t8 = zeta_threshold - f64x8::splat(1.0);
            let t10 = ((t7).select(t8, (t7).select(-t8, f64x8::splat(0.0))));
            let t11 = f64x8::splat(1.0) + t10;
            let t13 = (simd::cbrt(zeta_threshold));
            let t15 = (simd::cbrt(t11));
            let t17 = (((t11).simd_le(zeta_threshold)).select(t13 * zeta_threshold, t15 * t11));
            let t18 = t17 / t4 * t3;
            let t19 = (simd::cbrt(v_rho));
            let t20 = t3 * t3;
            let t22 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t23 = (simd::cbrt(t22));
            let t24 = f64x8::splat(1.0) / t23;
            let t25 = f64x8::splat(M_CBRT4);
            let t27 = f64x8::splat(M_CBRT6);
            let t28 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t29 = (simd::cbrt(t28));
            let t30 = t29 * t29;
            let t31 = f64x8::splat(1.0) / t30;
            let t32 = t31 * t27;
            let t33 = f64x8::splat(M_CBRT2);
            let t34 = t33 * t33;
            let t35 = t34 * v_sigma;
            let t36 = v_rho * v_rho;
            let t37 = t19 * t19;
            let t39 = f64x8::splat(1.0) / t37 / t36;
            let t43 = f64x8::splat(0.804) + f64x8::splat(0.009146457198521547) * t39 * t35 * t32;
            let t46 = f64x8::splat(1.804) - f64x8::splat(0.646416) / t43;
            let t49 = f64x8::splat(1.0) / t46 * t25 * t24 * t20 * f64x8::splat(M_PI);
            let t50 = ((t49).sqrt());
            let t52 = f64x8::splat(1.0) / t50 * param_hyb_omega_0;
            let t53 = v_rho * t11;
            let t54 = (simd::cbrt(t53));
            let t55 = f64x8::splat(1.0) / t54;
            let t58 = t55 * t33 * t52 / f64x8::splat(2.0);
            let t59 = (f64x8::splat(1.92)).simd_le(t58);
            let t60 = (f64x8::splat(1.92)).simd_lt(t58);
            let t61 = ((t60).select(t58, f64x8::splat(1.92)));
            let t62 = t61 * t61;
            let t63 = t62 * t62;
            let t64 = f64x8::splat(1.0) / t63;
            let t66 = t63 * t62;
            let t67 = f64x8::splat(1.0) / t66;
            let t69 = t63 * t63;
            let t70 = f64x8::splat(1.0) / t69;
            let t72 = t69 * t62;
            let t73 = f64x8::splat(1.0) / t72;
            let t75 = t69 * t63;
            let t76 = f64x8::splat(1.0) / t75;
            let t78 = t69 * t66;
            let t79 = f64x8::splat(1.0) / t78;
            let t81 = t69 * t69;
            let t82 = f64x8::splat(1.0) / t81;
            let t85 = f64x8::splat(1.0) / t81 / t62;
            let t88 = f64x8::splat(1.0) / t81 / t63;
            let t91 = f64x8::splat(1.0) / t81 / t66;
            let t94 = f64x8::splat(1.0) / t81 / t69;
            let t97 = f64x8::splat(1.0) / t81 / t72;
            let t100 = f64x8::splat(1.0) / t81 / t75;
            let t103 = f64x8::splat(1.0) / t81 / t78;
            let t105 = t81 * t81;
            let t106 = f64x8::splat(1.0) / t105;
            let t109 = f64x8::splat(1.0) / t105 / t62;
            let t112 = f64x8::splat(1.0) / t105 / t63;
            let t116 = -t64 / f64x8::splat(30.0) + t67 / f64x8::splat(70.0) - t70 / f64x8::splat(135.0) + t73 / f64x8::splat(231.0) - t76 / f64x8::splat(364.0) + t79 / f64x8::splat(540.0) - t82 / f64x8::splat(765.0) + t85 / f64x8::splat(1045.0) - t88 / f64x8::splat(1386.0) + t91 / f64x8::splat(1794.0) - t94 / f64x8::splat(2275.0) + t97 / f64x8::splat(2835.0) - t100 / f64x8::splat(3480.0) + t103 / f64x8::splat(4216.0) - t106 / f64x8::splat(5049.0) + t109 / f64x8::splat(5985.0) - t112 / f64x8::splat(7030.0) + f64x8::splat(1.0) / t62 / f64x8::splat(9.0);
            let t117 = ((t60).select(f64x8::splat(1.92), t58));
            let t118 = (simd::atan2(f64x8::splat(1.0), t117));
            let t119 = t117 * t117;
            let t120 = t119 + f64x8::splat(3.0);
            let t121 = f64x8::splat(1.0) / t119;
            let t122 = f64x8::splat(1.0) + t121;
            let t123 = (simd::ln(t122));
            let t125 = -t123 * t120 + f64x8::splat(1.0);
            let t128 = t118 + t125 * t117 / f64x8::splat(4.0);
            let t132 = ((t59).select(t116, f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t128 * t117));
            let t137 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t46 * t132 * t19 * t18));
            let tzk0 = f64x8::splat(2.0) * t137;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}

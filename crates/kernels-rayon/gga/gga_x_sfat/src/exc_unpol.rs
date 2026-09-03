//! GGA_X_SFAT exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_sfat.c`
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
pub fn gga_x_sfat_exc_unpol(
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
            let t26 = t25 * t24;
            let t27 = t24 * t20;
            let t28 = t25 * t27;
            let t29 = f64x8::splat(M_CBRT2);
            let t30 = t29 * t29;
            let t31 = t30 * v_sigma;
            let t32 = v_rho * v_rho;
            let t33 = t19 * t19;
            let t35 = f64x8::splat(1.0) / t33 / t32;
            let t36 = ((v_sigma).sqrt());
            let t37 = t29 * t36;
            let t39 = f64x8::splat(1.0) / t19 / v_rho;
            let t41 = (simd::ln(t39 * t37 + ((((t39 * t37) * (t39 * t37)) + f64x8::splat(1.0)).sqrt())));
            let t42 = t41 * t39;
            let t45 = f64x8::splat(1.0) + f64x8::splat(0.0252) * t42 * t37;
            let t46 = f64x8::splat(1.0) / t45;
            let t51 = f64x8::splat(1.0) + f64x8::splat(0.0009333333333333333) * t46 * t35 * t31 * t28;
            let t54 = f64x8::splat(1.0) / t51 * t26 * t20 * f64x8::splat(M_PI);
            let t55 = ((t54).sqrt());
            let t57 = f64x8::splat(1.0) / t55 * param_hyb_omega_0;
            let t58 = v_rho * t11;
            let t59 = (simd::cbrt(t58));
            let t60 = f64x8::splat(1.0) / t59;
            let t61 = t60 * t29;
            let t63 = t61 * t57 / f64x8::splat(2.0);
            let t64 = (f64x8::splat(1.92)).simd_le(t63);
            let t65 = (f64x8::splat(1.92)).simd_lt(t63);
            let t66 = ((t65).select(t63, f64x8::splat(1.92)));
            let t67 = t66 * t66;
            let t68 = t67 * t67;
            let t69 = t68 * t68;
            let t70 = t69 * t69;
            let t71 = t70 * t70;
            let t73 = f64x8::splat(1.0) / t71 / t67;
            let t76 = f64x8::splat(1.0) / t71 / t68;
            let t78 = f64x8::splat(1.0) / t68;
            let t80 = t68 * t67;
            let t81 = f64x8::splat(1.0) / t80;
            let t83 = f64x8::splat(1.0) / t69;
            let t85 = t69 * t67;
            let t86 = f64x8::splat(1.0) / t85;
            let t88 = t69 * t68;
            let t89 = f64x8::splat(1.0) / t88;
            let t91 = t69 * t80;
            let t92 = f64x8::splat(1.0) / t91;
            let t94 = f64x8::splat(1.0) / t70;
            let t97 = f64x8::splat(1.0) / t70 / t67;
            let t100 = f64x8::splat(1.0) / t70 / t68;
            let t103 = f64x8::splat(1.0) / t70 / t80;
            let t106 = f64x8::splat(1.0) / t70 / t69;
            let t109 = f64x8::splat(1.0) / t70 / t85;
            let t112 = f64x8::splat(1.0) / t70 / t88;
            let t115 = f64x8::splat(1.0) / t70 / t91;
            let t117 = f64x8::splat(1.0) / t71;
            let t121 = t73 / f64x8::splat(5985.0) - t76 / f64x8::splat(7030.0) - t78 / f64x8::splat(30.0) + t81 / f64x8::splat(70.0) - t83 / f64x8::splat(135.0) + t86 / f64x8::splat(231.0) - t89 / f64x8::splat(364.0) + t92 / f64x8::splat(540.0) - t94 / f64x8::splat(765.0) + t97 / f64x8::splat(1045.0) - t100 / f64x8::splat(1386.0) + t103 / f64x8::splat(1794.0) - t106 / f64x8::splat(2275.0) + t109 / f64x8::splat(2835.0) - t112 / f64x8::splat(3480.0) + t115 / f64x8::splat(4216.0) - t117 / f64x8::splat(5049.0) + f64x8::splat(1.0) / t67 / f64x8::splat(9.0);
            let t122 = ((t65).select(f64x8::splat(1.92), t63));
            let t123 = (simd::atan2(f64x8::splat(1.0), t122));
            let t124 = t122 * t122;
            let t125 = t124 + f64x8::splat(3.0);
            let t126 = f64x8::splat(1.0) / t124;
            let t127 = f64x8::splat(1.0) + t126;
            let t128 = (simd::ln(t127));
            let t130 = -t125 * t128 + f64x8::splat(1.0);
            let t133 = t123 + t130 * t122 / f64x8::splat(4.0);
            let t137 = ((t64).select(t121, f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t133 * t122));
            let t138 = t137 * t19;
            let t142 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t51 * t138 * t18));
            let tzk0 = f64x8::splat(2.0) * t142;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}

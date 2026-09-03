//! GGA_X_ITYH exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_ityh.c`
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
pub fn gga_x_ityh_exc_unpol(
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
            let t18 = t3 / t4 * t17;
            let t19 = (simd::cbrt(v_rho));
            let t20 = t3 * t3;
            let t22 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t23 = (simd::cbrt(t22));
            let t24 = f64x8::splat(1.0) / t23;
            let t25 = f64x8::splat(M_CBRT4);
            let t26 = t24 * t25;
            let t27 = t20 * t24;
            let t28 = t27 * t25;
            let t29 = f64x8::splat(M_CBRT2);
            let t30 = t29 * t29;
            let t31 = v_sigma * t30;
            let t32 = v_rho * v_rho;
            let t33 = t19 * t19;
            let t35 = f64x8::splat(1.0) / t33 / t32;
            let t36 = ((v_sigma).sqrt());
            let t37 = t36 * t29;
            let t39 = f64x8::splat(1.0) / t19 / v_rho;
            let t41 = (simd::ln(t37 * t39 + ((((t37 * t39) * (t37 * t39)) + f64x8::splat(1.0)).sqrt())));
            let t42 = t39 * t41;
            let t45 = f64x8::splat(1.0) + f64x8::splat(0.0252) * t37 * t42;
            let t46 = f64x8::splat(1.0) / t45;
            let t51 = f64x8::splat(1.0) + f64x8::splat(0.0009333333333333333) * t28 * t31 * t35 * t46;
            let t54 = f64x8::splat(M_PI) * t20 * t26 / t51;
            let t55 = ((t54).sqrt());
            let t57 = param_hyb_omega_0 / t55;
            let t58 = t11 * v_rho;
            let t59 = (simd::cbrt(t58));
            let t60 = f64x8::splat(1.0) / t59;
            let t61 = t29 * t60;
            let t63 = t57 * t61 / f64x8::splat(2.0);
            let t64 = (f64x8::splat(1.35)).simd_le(t63);
            let t65 = (f64x8::splat(1.35)).simd_lt(t63);
            let t66 = ((t65).select(t63, f64x8::splat(1.35)));
            let t67 = t66 * t66;
            let t70 = t67 * t67;
            let t71 = f64x8::splat(1.0) / t70;
            let t73 = t70 * t67;
            let t74 = f64x8::splat(1.0) / t73;
            let t76 = t70 * t70;
            let t77 = f64x8::splat(1.0) / t76;
            let t80 = f64x8::splat(1.0) / t76 / t67;
            let t83 = f64x8::splat(1.0) / t76 / t70;
            let t86 = f64x8::splat(1.0) / t76 / t73;
            let t88 = t76 * t76;
            let t89 = f64x8::splat(1.0) / t88;
            let t92 = ((t65).select(f64x8::splat(1.35), t63));
            let t93 = ((f64x8::splat(M_PI)).sqrt());
            let t94 = f64x8::splat(1.0) / t92;
            let t96 = (simd::erf(t94 / f64x8::splat(2.0)));
            let t98 = t92 * t92;
            let t99 = f64x8::splat(1.0) / t98;
            let t101 = (simd::exp(-t99 / f64x8::splat(4.0)));
            let t102 = t101 - f64x8::splat(1.0);
            let t105 = t101 - f64x8::splat(3.0) / f64x8::splat(2.0) - f64x8::splat(2.0) * t98 * t102;
            let t108 = f64x8::splat(2.0) * t92 * t105 + t93 * t96;
            let t112 = ((t64).select(f64x8::splat(1.0) / t67 / f64x8::splat(36.0) - t71 / f64x8::splat(960.0) + t74 / f64x8::splat(26880.0) - t77 / f64x8::splat(829440.0) + t80 / f64x8::splat(28385280.0) - t83 / f64x8::splat(1073479680.0) + t86 / f64x8::splat(44590694400.0) - t89 / f64x8::splat(2021444812800.0), f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t92 * t108));
            let t113 = t19 * t112;
            let t117 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t113 * t51));
            let tzk0 = f64x8::splat(2.0) * t117;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}

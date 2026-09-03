//! GGA_X_CHACHIYO vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_chachiyo.c`
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
pub fn gga_x_chachiyo_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
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
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
        {
            let t2 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = f64x8::splat(M_CBRTPI);
            let t7 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t8 = zeta_threshold - f64x8::splat(1.0);
            let t10 = ((t7).select(t8, (t7).select(-t8, f64x8::splat(0.0))));
            let t11 = t10 + f64x8::splat(1.0);
            let t13 = (simd::cbrt(zeta_threshold));
            let t15 = (simd::cbrt(t11));
            let t17 = (((t11).simd_le(zeta_threshold)).select(t13 * zeta_threshold, t15 * t11));
            let t18 = t3 / t4 * t17;
            let t19 = (simd::cbrt(v_rho));
            let t20 = t4 * t4;
            let t21 = t3 * t20;
            let t22 = v_rho * v_rho;
            let t23 = t19 * t19;
            let t25 = f64x8::splat(1.0) / t23 / t22;
            let t29 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t30 = t3 * t3;
            let t31 = t30 * t4;
            let t32 = ((v_sigma).sqrt());
            let t34 = f64x8::splat(1.0) / t19 / v_rho;
            let t36 = t31 * t32 * t34;
            let t38 = f64x8::splat(2.0) / f64x8::splat(27.0) * t36 + f64x8::splat(1.0);
            let t39 = (simd::ln(t38));
            let t41 = f64x8::splat(4.0) / f64x8::splat(81.0) * t21 * v_sigma * t25 + t29 * t39;
            let t44 = f64x8::splat(2.0) / f64x8::splat(9.0) * t36 + t29;
            let t45 = f64x8::splat(1.0) / t44;
            let t46 = f64x8::splat(1.0) / t39;
            let t47 = t45 * t46;
            let t51 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t19 * t41 * t47));
            let tzk0 = f64x8::splat(2.0) * t51;
            acc_zk = tzk0;
            let t52 = f64x8::splat(1.0) / t23;
            let t57 = t22 * v_rho;
            let t59 = f64x8::splat(1.0) / t23 / t57;
            let t64 = t4 * t29 * t30;
            let t66 = f64x8::splat(1.0) / t19 / t22;
            let t68 = f64x8::splat(1.0) / t38;
            let t72 = -f64x8::splat(32.0) / f64x8::splat(243.0) * t21 * v_sigma * t59 - f64x8::splat(8.0) / f64x8::splat(81.0) * t64 * t32 * t66 * t68;
            let t78 = t17 / t22;
            let t79 = t78 * t41;
            let t80 = t44 * t44;
            let t81 = f64x8::splat(1.0) / t80;
            let t82 = t81 * t46;
            let t83 = t82 * t32;
            let t86 = t39 * t39;
            let t87 = f64x8::splat(1.0) / t86;
            let t88 = t45 * t87;
            let t90 = t88 * t32 * t68;
            let t94 = ((t2).select(f64x8::splat(0.0), -t18 * t52 * t41 * t47 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t19 * t72 * t47 - t79 * t83 / f64x8::splat(3.0) - t79 * t90 / f64x8::splat(9.0)));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t94 + f64x8::splat(2.0) * t51;
            acc_vrho = tvrho0;
            let t99 = f64x8::splat(1.0) / t32;
            let t104 = f64x8::splat(4.0) / f64x8::splat(81.0) * t21 * t25 + t64 * t99 * t34 * t68 / f64x8::splat(27.0);
            let t110 = t17 / v_rho;
            let t111 = t110 * t41;
            let t112 = t82 * t99;
            let t116 = t88 * t99 * t68;
            let t120 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t19 * t104 * t47 + t111 * t112 / f64x8::splat(8.0) + t111 * t116 / f64x8::splat(24.0)));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t120;
            acc_vsigma = tvsigma0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}

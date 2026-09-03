//! GGA_X_BPCCAC exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_bpccac.c`
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
pub fn gga_x_bpccac_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
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
        let mut acc_zk = V_ZERO;
        {
            let t2 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = f64x8::splat(M_CBRTPI);
            let t6 = t3 / t4;
            let t7 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t8 = zeta_threshold - f64x8::splat(1.0);
            let t10 = ((t7).select(t8, (t7).select(-t8, f64x8::splat(0.0))));
            let t11 = f64x8::splat(1.0) + t10;
            let t13 = (simd::cbrt(zeta_threshold));
            let t15 = (simd::cbrt(t11));
            let t17 = (((t11).simd_le(zeta_threshold)).select(t13 * zeta_threshold, t15 * t11));
            let t18 = (simd::cbrt(v_rho));
            let t19 = t17 * t18;
            let t20 = ((v_sigma).sqrt());
            let t21 = f64x8::splat(M_CBRT2);
            let t24 = f64x8::splat(1.0) / t18 / v_rho;
            let t25 = t20 * t21 * t24;
            let t27 = (simd::exp(-t25 + f64x8::splat(19.0)));
            let t28 = f64x8::splat(1.0) + t27;
            let t29 = f64x8::splat(1.0) / t28;
            let t30 = f64x8::splat(1.0) - t29;
            let t31 = f64x8::splat(M_CBRT6);
            let t32 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t33 = (simd::cbrt(t32));
            let t34 = t33 * t33;
            let t35 = f64x8::splat(1.0) / t34;
            let t36 = t31 * t35;
            let t37 = t21 * t21;
            let t38 = v_sigma * t37;
            let t39 = v_rho * v_rho;
            let t40 = t18 * t18;
            let t42 = f64x8::splat(1.0) / t40 / t39;
            let t43 = t38 * t42;
            let t44 = t36 * t43;
            let t46 = f64x8::splat(1.227) + f64x8::splat(0.009146457198521547) * t44;
            let t49 = f64x8::splat(2.227) - f64x8::splat(1.505529) / t46;
            let t52 = (simd::exp(-f64x8::splat(25.0) / f64x8::splat(6.0) * t44));
            let t55 = (f64x8::splat(0.2743) - f64x8::splat(0.1508) * t52) * t31;
            let t56 = t55 * t35;
            let t59 = t31 * t31;
            let t61 = f64x8::splat(1.0) / t33 / t32;
            let t62 = t59 * t61;
            let t63 = v_sigma * v_sigma;
            let t64 = t63 * t21;
            let t65 = t39 * t39;
            let t66 = t65 * v_rho;
            let t68 = f64x8::splat(1.0) / t18 / t66;
            let t71 = f64x8::splat(1.388888888888889e-05) * t62 * t64 * t68;
            let t72 = t56 * t43 / f64x8::splat(24.0) - t71;
            let t74 = t59 / t33;
            let t75 = t74 * t20;
            let t76 = t21 * t24;
            let t79 = (simd::ln(f64x8::splat(0.6496333333333333) * t74 * t25 + ((((f64x8::splat(0.6496333333333333) * t74 * t25) * (f64x8::splat(0.6496333333333333) * t74 * t25)) + f64x8::splat(1.0)).sqrt())));
            let t80 = t76 * t79;
            let t83 = f64x8::splat(1.0) + f64x8::splat(0.016370833333333334) * t75 * t80 + t71;
            let t84 = f64x8::splat(1.0) / t83;
            let t86 = t72 * t84 + f64x8::splat(1.0);
            let t88 = t29 * t86 + t30 * t49;
            let t92 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t88));
            let tzk0 = f64x8::splat(2.0) * t92;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}

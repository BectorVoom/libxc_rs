//! GGA_K_APBE fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_apbe.c`
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
pub fn gga_k_apbe_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    param_kappa: f64,
    param_mu: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_kappa = f64x8::splat(param_kappa);
    let param_mu = f64x8::splat(param_mu);
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
        let mut acc_v2rho2 = V_ZERO;
        let mut acc_v2rhosigma = V_ZERO;
        let mut acc_v2sigma2 = V_ZERO;
        {
            let t2 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = t3 * t3;
            let t5 = f64x8::splat(M_CBRTPI);
            let t7 = t4 * t5 * f64x8::splat(M_PI);
            let t8 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t9 = zeta_threshold - f64x8::splat(1.0);
            let t11 = ((t8).select(t9, (t8).select(-t9, f64x8::splat(0.0))));
            let t12 = f64x8::splat(1.0) + t11;
            let t14 = (simd::cbrt(zeta_threshold));
            let t15 = t14 * t14;
            let t17 = (simd::cbrt(t12));
            let t18 = t17 * t17;
            let t20 = (((t12).simd_le(zeta_threshold)).select(t15 * zeta_threshold, t18 * t12));
            let t21 = (simd::cbrt(v_rho));
            let t22 = t21 * t21;
            let t24 = f64x8::splat(M_CBRT6);
            let t26 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t27 = (simd::cbrt(t26));
            let t28 = t27 * t27;
            let t29 = f64x8::splat(1.0) / t28;
            let t31 = f64x8::splat(M_CBRT2);
            let t32 = t31 * t31;
            let t34 = v_rho * v_rho;
            let t40 = param_kappa + param_mu * t24 * t29 * v_sigma * t32 / t22 / t34 / f64x8::splat(24.0);
            let t45 = f64x8::splat(1.0) + param_kappa * (f64x8::splat(1.0) - param_kappa / t40);
            let t49 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t20 * t22 * t45));
            let tzk0 = f64x8::splat(2.0) * t49;
            acc_zk = tzk0;
            let t55 = t34 * v_rho;
            let t58 = param_kappa * param_kappa;
            let t60 = t7 * t20 / t55 * t58;
            let t61 = t40 * t40;
            let t63 = f64x8::splat(1.0) / t61 * param_mu;
            let t66 = t29 * v_sigma * t32;
            let t67 = t63 * t24 * t66;
            let t71 = ((t2).select(f64x8::splat(0.0), t7 * t20 / t21 * t45 / f64x8::splat(10.0) - t60 * t67 / f64x8::splat(60.0)));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t71 + f64x8::splat(2.0) * t49;
            acc_vrho = tvrho0;
            let t79 = t24 * t29 * t32;
            let t80 = t63 * t79;
            let t83 = ((t2).select(f64x8::splat(0.0), t7 * t20 / t34 * t58 * t80 / f64x8::splat(160.0)));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t83;
            acc_vsigma = tvsigma0;
            let t92 = t34 * t34;
            let t96 = t7 * t20 / t92 * t58;
            let t99 = t92 * t34;
            let t104 = t7 * t20 / t22 / t99 * t58;
            let t107 = param_mu * param_mu;
            let t108 = f64x8::splat(1.0) / t61 / t40 * t107;
            let t109 = t24 * t24;
            let t110 = t108 * t109;
            let t112 = f64x8::splat(1.0) / t27 / t26;
            let t113 = v_sigma * v_sigma;
            let t116 = t110 * t112 * t113 * t31;
            let t120 = ((t2).select(f64x8::splat(0.0), -t7 * t20 / t21 / v_rho * t45 / f64x8::splat(30.0) + f64x8::splat(7.0) / f64x8::splat(180.0) * t96 * t67 - t104 * t116 / f64x8::splat(135.0)));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t120 + f64x8::splat(4.0) * t71;
            acc_v2rho2 = tv2rho20;
            let t125 = t92 * v_rho;
            let t130 = t7 * t20 / t22 / t125 * t58;
            let t133 = t110 * t112 * t31 * v_sigma;
            let t137 = ((t2).select(f64x8::splat(0.0), -t60 * t80 / f64x8::splat(80.0) + t130 * t133 / f64x8::splat(360.0)));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t137 + f64x8::splat(2.0) * t83;
            acc_v2rhosigma = tv2rhosigma0;
            let t147 = t108 * t109 * t112 * t31;
            let t150 = ((t2).select(f64x8::splat(0.0), -t7 * t20 / t22 / t92 * t58 * t147 / f64x8::splat(960.0)));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t150;
            acc_v2sigma2 = tv2sigma20;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rho2.into(); v2rho2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rhosigma.into(); v2rhosigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2sigma2.into(); v2sigma2[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}

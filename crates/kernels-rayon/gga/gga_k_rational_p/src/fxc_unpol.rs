//! GGA_K_RATIONAL_P fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_rational_p.c`
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
pub fn gga_k_rational_p_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    param_p: f64,
    param_C2: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_p = f64x8::splat(param_p);
    let param_C2 = f64x8::splat(param_C2);
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
            let t24 = f64x8::splat(1.0) / param_p;
            let t26 = f64x8::splat(M_CBRT6);
            let t28 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t29 = (simd::cbrt(t28));
            let t30 = t29 * t29;
            let t31 = f64x8::splat(1.0) / t30;
            let t32 = t31 * v_sigma;
            let t33 = f64x8::splat(M_CBRT2);
            let t34 = t33 * t33;
            let t35 = v_rho * v_rho;
            let t42 = f64x8::splat(1.0) + param_C2 * t24 * t26 * t32 * t34 / t22 / t35 / f64x8::splat(24.0);
            let t43 = (simd::pow(t42, -param_p));
            let t47 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t20 * t22 * t43));
            let tzk0 = f64x8::splat(2.0) * t47;
            acc_zk = tzk0;
            let t53 = t35 * v_rho;
            let t57 = t7 * t20 / t53 * t43;
            let t58 = param_C2 * t26;
            let t60 = v_sigma * t34;
            let t61 = f64x8::splat(1.0) / t42;
            let t63 = t58 * t31 * t60 * t61;
            let t67 = ((t2).select(f64x8::splat(0.0), t7 * t20 / t21 * t43 / f64x8::splat(10.0) + t57 * t63 / f64x8::splat(60.0)));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t67 + f64x8::splat(2.0) * t47;
            acc_vrho = tvrho0;
            let t74 = t31 * t34;
            let t76 = t58 * t74 * t61;
            let t79 = ((t2).select(f64x8::splat(0.0), -t7 * t20 / t35 * t43 * t76 / f64x8::splat(160.0)));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t79;
            acc_vsigma = tvsigma0;
            let t88 = t35 * t35;
            let t92 = t7 * t20 / t88 * t43;
            let t95 = t88 * t35;
            let t97 = f64x8::splat(1.0) / t22 / t95;
            let t100 = t7 * t20 * t97 * t43;
            let t101 = param_C2 * param_C2;
            let t102 = t26 * t26;
            let t103 = t101 * t102;
            let t105 = f64x8::splat(1.0) / t29 / t28;
            let t106 = t103 * t105;
            let t107 = v_sigma * v_sigma;
            let t109 = t42 * t42;
            let t110 = f64x8::splat(1.0) / t109;
            let t112 = t106 * t107 * t33 * t110;
            let t115 = t7 * t20;
            let t118 = t115 * t97 * t43 * t101;
            let t119 = t102 * t105;
            let t122 = t33 * t110 * t24;
            let t123 = t119 * t107 * t122;
            let t127 = ((t2).select(f64x8::splat(0.0), -t7 * t20 / t21 / v_rho * t43 / f64x8::splat(30.0) - f64x8::splat(7.0) / f64x8::splat(180.0) * t92 * t63 + t100 * t112 / f64x8::splat(270.0) + t118 * t123 / f64x8::splat(270.0)));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t127 + f64x8::splat(4.0) * t67;
            acc_v2rho2 = tv2rho20;
            let t132 = t88 * v_rho;
            let t134 = f64x8::splat(1.0) / t22 / t132;
            let t137 = t7 * t20 * t134 * t43;
            let t140 = t106 * v_sigma * t33 * t110;
            let t149 = t119 * t33 * t110 * t24 * v_sigma;
            let t153 = ((t2).select(f64x8::splat(0.0), t57 * t76 / f64x8::splat(80.0) - t137 * t140 / f64x8::splat(720.0) - t115 * t134 * t43 * t101 * t149 / f64x8::splat(720.0)));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t153 + f64x8::splat(2.0) * t79;
            acc_v2rhosigma = tv2rhosigma0;
            let t160 = t7 * t20 / t22 / t88 * t43;
            let t163 = t103 * t105 * t33 * t110;
            let t165 = t106 * t122;
            let t169 = ((t2).select(f64x8::splat(0.0), t160 * t163 / f64x8::splat(1920.0) + t160 * t165 / f64x8::splat(1920.0)));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t169;
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

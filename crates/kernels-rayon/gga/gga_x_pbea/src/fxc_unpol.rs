//! GGA_X_PBEA fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_pbea.c`
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
pub fn gga_x_pbea_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
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
        let mut acc_v2rho2 = V_ZERO;
        let mut acc_v2rhosigma = V_ZERO;
        let mut acc_v2sigma2 = V_ZERO;
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
            let t20 = f64x8::splat(M_CBRT2);
            let t21 = t20 * t20;
            let t23 = v_rho * v_rho;
            let t24 = t18 * t18;
            let t26 = f64x8::splat(1.0) / t24 / t23;
            let t29 = f64x8::splat(1.0) + f64x8::splat(0.008639940809536326) * v_sigma * t21 * t26;
            let t30 = (simd::pow(t29, -f64x8::splat(0.52)));
            let t32 = f64x8::splat(1.804) - f64x8::splat(0.804) * t30;
            let t36 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t17 * t18 * t32));
            let tzk0 = f64x8::splat(2.0) * t36;
            acc_zk = tzk0;
            let t42 = t3 * t17;
            let t43 = t23 * v_rho;
            let t45 = f64x8::splat(1.0) / t18 / t43;
            let t47 = (simd::pow(t29, -f64x8::splat(1.52)));
            let t49 = t47 * v_sigma * t21;
            let t53 = ((t2).select(f64x8::splat(0.0), -t6 * t17 / t24 * t32 / f64x8::splat(8.0) + f64x8::splat(0.00246634334405953) * t42 * t45 * t49));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t53 + f64x8::splat(2.0) * t36;
            acc_vrho = tvrho0;
            let t62 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(0.0009248787540223239) * t42 / t18 / t23 * t47 * t21));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t62;
            acc_vsigma = tvsigma0;
            let t71 = t23 * t23;
            let t73 = f64x8::splat(1.0) / t18 / t71;
            let t77 = t71 * t43;
            let t78 = f64x8::splat(1.0) / t77;
            let t79 = t42 * t78;
            let t80 = (simd::pow(t29, -f64x8::splat(2.52)));
            let t81 = v_sigma * v_sigma;
            let t83 = t80 * t81 * t20;
            let t87 = ((t2).select(f64x8::splat(0.0), t6 * t17 / t24 / v_rho * t32 / f64x8::splat(12.0) - f64x8::splat(0.007399030032178591) * t42 * t73 * t49 + f64x8::splat(0.00017274545052360375) * t79 * t83));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t87 + f64x8::splat(4.0) * t53;
            acc_v2rho2 = tv2rho20;
            let t94 = t71 * t23;
            let t95 = f64x8::splat(1.0) / t94;
            let t98 = t80 * t20 * v_sigma;
            let t102 = ((t2).select(f64x8::splat(0.0), f64x8::splat(0.002158050426052089) * t42 * t45 * t47 * t21 - f64x8::splat(6.47795439463514e-05) * t42 * t95 * t98));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t102 + f64x8::splat(2.0) * t62;
            acc_v2rhosigma = tv2rhosigma0;
            let t105 = t71 * v_rho;
            let t111 = ((t2).select(f64x8::splat(0.0), f64x8::splat(2.429232897988178e-05) * t42 / t105 * t80 * t20));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t111;
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

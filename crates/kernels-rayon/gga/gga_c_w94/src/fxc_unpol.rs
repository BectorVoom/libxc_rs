//! GGA_C_W94 fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_w94.c`
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
pub fn gga_c_w94_fxc_unpol(
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
            let t1 = ((v_sigma).sqrt());
            let t2 = t1 * v_sigma;
            let t3 = v_rho * v_rho;
            let t4 = t3 * t3;
            let t5 = f64x8::splat(1.0) / t4;
            let t7 = (simd::cbrt(v_rho));
            let t9 = f64x8::splat(1.0) / t7 / v_rho;
            let t10 = t1 * t9;
            let t11 = (simd::pow(t10, f64x8::splat(1.0) / f64x8::splat(16.0)));
            let t12 = t11 * t11;
            let t13 = t12 * t11;
            let t16 = t3 * v_rho;
            let t17 = f64x8::splat(1.0) / t16;
            let t20 = f64x8::splat(M_CBRT3);
            let t22 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t23 = t20 * t22;
            let t24 = f64x8::splat(M_CBRT4);
            let t25 = t24 * t24;
            let t30 = f64x8::splat(11.8) + f64x8::splat(0.15067) * t13 * t2 * t5 + f64x8::splat(0.01102) * v_sigma * t17 + t23 * t25 / t7 / f64x8::splat(4.0);
            let tzk0 = -f64x8::splat(1.0) / t30;
            acc_zk = tzk0;
            let t32 = t30 * t30;
            let t33 = f64x8::splat(1.0) / t32;
            let t34 = v_rho * t33;
            let t35 = t7 * t7;
            let t37 = f64x8::splat(1.0) / t35 / t3;
            let t39 = t13 * v_sigma * t37;
            let t40 = t39 * t1;
            let t42 = f64x8::splat(1.0) / t7 / t3;
            let t50 = -f64x8::splat(0.6403475) * t40 * t42 - f64x8::splat(0.03306) * v_sigma * t5 - t23 * t25 * t9 / f64x8::splat(12.0);
            let tvrho0 = t34 * t50 + tzk0;
            acc_vrho = tvrho0;
            let t52 = f64x8::splat(1.0) / t1;
            let t53 = t39 * t52;
            let t57 = f64x8::splat(0.2401303125) * t53 * t9 + f64x8::splat(0.01102) * t17;
            let tvsigma0 = t34 * t57;
            acc_vsigma = tvsigma0;
            let t61 = f64x8::splat(1.0) / t32 / t30;
            let t62 = v_rho * t61;
            let t63 = t50 * t50;
            let t66 = t13 * t10;
            let t67 = t66 * v_sigma;
            let t69 = f64x8::splat(1.0) / t35 / t4;
            let t73 = f64x8::splat(1.0) / t7 / t16;
            let t76 = t4 * v_rho;
            let t77 = f64x8::splat(1.0) / t76;
            let t83 = f64x8::splat(1.8676802083333333) * t67 * t69 + f64x8::splat(1.4941441666666666) * t40 * t73 + f64x8::splat(0.13224) * v_sigma * t77 + t23 * t25 * t42 / f64x8::splat(9.0);
            let tv2rho20 = f64x8::splat(2.0) * t33 * t50 + t34 * t83 - f64x8::splat(2.0) * t62 * t63;
            acc_v2rho2 = tv2rho20;
            let t86 = t57 * t50;
            let t90 = f64x8::splat(1.0) / t35 / t16;
            let t96 = -f64x8::splat(0.700380078125) * t66 * t90 - f64x8::splat(0.32017375) * t53 * t42 - f64x8::splat(0.03306) * t5;
            let tv2rhosigma0 = t33 * t57 + t34 * t96 - f64x8::splat(2.0) * t62 * t86;
            acc_v2rhosigma = tv2rhosigma0;
            let t98 = t57 * t57;
            let t101 = f64x8::splat(1.0) / v_sigma;
            let t102 = t66 * t101;
            let t105 = f64x8::splat(1.0) / t2;
            let t106 = t39 * t105;
            let t109 = f64x8::splat(0.262642529296875) * t102 * t37 - f64x8::splat(0.12006515625) * t106 * t9;
            let tv2sigma20 = t34 * t109 - f64x8::splat(2.0) * t62 * t98;
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

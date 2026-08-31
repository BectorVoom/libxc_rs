//! GGA_C_LYP vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_lyp.c`
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
pub fn gga_c_lyp_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_a: f64,
    param_b: f64,
    param_c: f64,
    param_d: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_a = f64x8::splat(param_a);
    let param_b = f64x8::splat(param_b);
    let param_c = f64x8::splat(param_c);
    let param_d = f64x8::splat(param_d);
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
            let t1 = (simd::cbrt(v_rho));
            let t2 = f64x8::splat(1.0) / t1;
            let t4 = param_d * t2 + f64x8::splat(1.0);
            let t5 = f64x8::splat(1.0) / t4;
            let t7 = (simd::exp(-param_c * t2));
            let t8 = param_b * t7;
            let t9 = v_rho * v_rho;
            let t10 = t1 * t1;
            let t12 = f64x8::splat(1.0) / t10 / t9;
            let t13 = v_sigma * t12;
            let t15 = param_d * t5 + param_c;
            let t16 = t15 * t2;
            let t18 = -f64x8::splat(1.0) / f64x8::splat(72.0) - f64x8::splat(7.0) / f64x8::splat(72.0) * t16;
            let t20 = f64x8::splat(M_CBRT3);
            let t21 = t20 * t20;
            let t22 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t23 = (simd::cbrt(t22));
            let t24 = t23 * t23;
            let t26 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t27 = zeta_threshold * zeta_threshold;
            let t28 = (simd::cbrt(zeta_threshold));
            let t29 = t28 * t28;
            let t31 = ((t26).select(t29 * t27, f64x8::splat(1.0)));
            let t35 = f64x8::splat(5.0) / f64x8::splat(2.0) - t16 / f64x8::splat(18.0);
            let t36 = t35 * v_sigma;
            let t37 = t12 * t31;
            let t40 = t16 - f64x8::splat(11.0);
            let t41 = t40 * v_sigma;
            let t44 = ((t26).select(t29 * t27 * zeta_threshold, f64x8::splat(1.0)));
            let t45 = t12 * t44;
            let t48 = f64x8::splat(M_CBRT2);
            let t49 = t48 * t48;
            let t50 = v_sigma * t49;
            let t53 = ((t26).select(t27, f64x8::splat(1.0)));
            let t54 = t53 * v_sigma;
            let t56 = t49 * t12 * t31;
            let t62 = -t13 * t18 - f64x8::splat(3.0) / f64x8::splat(10.0) * t21 * t24 * t31 + t36 * t37 / f64x8::splat(8.0) + t41 * t45 / f64x8::splat(144.0) - t48 * (f64x8::splat(4.0) / f64x8::splat(3.0) * t50 * t37 - t54 * t56 / f64x8::splat(2.0)) / f64x8::splat(8.0);
            let tzk0 = param_a * (t8 * t5 * t62 - t5);
            acc_zk = tzk0;
            let t66 = v_rho * param_a;
            let t67 = t4 * t4;
            let t68 = f64x8::splat(1.0) / t67;
            let t69 = t68 * param_d;
            let t71 = f64x8::splat(1.0) / t1 / v_rho;
            let t74 = param_b * param_c;
            let t75 = t74 * t71;
            let t76 = t7 * t5;
            let t77 = t76 * t62;
            let t80 = t8 * t68;
            let t81 = t62 * param_d;
            let t85 = t9 * v_rho;
            let t87 = f64x8::splat(1.0) / t10 / t85;
            let t88 = v_sigma * t87;
            let t91 = param_d * param_d;
            let t92 = t91 * t68;
            let t94 = f64x8::splat(1.0) / t10 / v_rho;
            let t97 = t15 * t71 - t92 * t94;
            let t98 = f64x8::splat(7.0) / f64x8::splat(216.0) * t97;
            let t100 = t97 / f64x8::splat(54.0);
            let t101 = t100 * v_sigma;
            let t104 = t87 * t31;
            let t108 = -t97 / f64x8::splat(3.0);
            let t109 = t108 * v_sigma;
            let t112 = t87 * t44;
            let t118 = t49 * t87 * t31;
            let t124 = f64x8::splat(8.0) / f64x8::splat(3.0) * t88 * t18 - t13 * t98 + t101 * t37 / f64x8::splat(8.0) - t36 * t104 / f64x8::splat(3.0) + t109 * t45 / f64x8::splat(144.0) - t41 * t112 / f64x8::splat(54.0) - t48 * (-f64x8::splat(32.0) / f64x8::splat(9.0) * t50 * t104 + f64x8::splat(4.0) / f64x8::splat(3.0) * t54 * t118) / f64x8::splat(8.0);
            let t127 = -t69 * t71 / f64x8::splat(3.0) + t75 * t77 / f64x8::splat(3.0) + t80 * t81 * t71 / f64x8::splat(3.0) + t8 * t5 * t124;
            let tvrho0 = t66 * t127 + tzk0;
            acc_vrho = tvrho0;
            let t129 = t66 * param_b;
            let t138 = t53 * t49;
            let t144 = -t12 * t18 + t35 * t12 * t31 / f64x8::splat(8.0) + t40 * t12 * t44 / f64x8::splat(144.0) - t48 * (f64x8::splat(4.0) / f64x8::splat(3.0) * t56 - t138 * t37 / f64x8::splat(2.0)) / f64x8::splat(8.0);
            let t145 = t76 * t144;
            let tvsigma0 = t129 * t145;
            acc_vsigma = tvsigma0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}

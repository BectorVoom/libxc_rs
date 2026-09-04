//! GGA_K_LKT fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_lkt.c`
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
pub fn gga_k_lkt_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    param_a: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_a = f64x8::splat(param_a);
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
            let t23 = t20 * t22;
            let t24 = f64x8::splat(M_CBRT6);
            let t25 = t24 * t24;
            let t26 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t27 = (simd::cbrt(t26));
            let t29 = t25 / t27;
            let t30 = ((v_sigma).sqrt());
            let t31 = f64x8::splat(M_CBRT2);
            let t32 = t30 * t31;
            let t34 = f64x8::splat(1.0) / t21 / v_rho;
            let t37 = t29 * t32 * t34 / f64x8::splat(12.0);
            let t38 = (t37).simd_lt(f64x8::splat(200.0));
            let t39 = ((t38).select(t37, f64x8::splat(200.0)));
            let t40 = param_a * t39;
            let t41 = (simd::cosh(t40));
            let t42 = f64x8::splat(1.0) / t41;
            let t43 = t27 * t27;
            let t45 = t24 / t43;
            let t46 = t31 * t31;
            let t47 = v_sigma * t46;
            let t48 = v_rho * v_rho;
            let t50 = f64x8::splat(1.0) / t22 / t48;
            let t54 = t42 + f64x8::splat(5.0) / f64x8::splat(72.0) * t45 * t47 * t50;
            let t58 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t54));
            let tzk0 = f64x8::splat(2.0) * t58;
            acc_zk = tzk0;
            let t60 = t20 / t21;
            let t64 = t41 * t41;
            let t65 = f64x8::splat(1.0) / t64;
            let t66 = t65 * param_a;
            let t68 = f64x8::splat(1.0) / t21 / t48;
            let t72 = ((t38).select(-t29 * t32 * t68 / f64x8::splat(9.0), f64x8::splat(0.0)));
            let t73 = (simd::sinh(t40));
            let t74 = t72 * t73;
            let t76 = t48 * v_rho;
            let t78 = f64x8::splat(1.0) / t22 / t76;
            let t82 = -t66 * t74 - f64x8::splat(5.0) / f64x8::splat(27.0) * t45 * t47 * t78;
            let t87 = ((t2).select(f64x8::splat(0.0), t7 * t60 * t54 / f64x8::splat(10.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t82));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t87 + f64x8::splat(2.0) * t58;
            acc_vrho = tvrho0;
            let t91 = f64x8::splat(1.0) / t30 * t31;
            let t95 = ((t38).select(t29 * t91 * t34 / f64x8::splat(24.0), f64x8::splat(0.0)));
            let t96 = t95 * t73;
            let t101 = -t66 * t96 + f64x8::splat(5.0) / f64x8::splat(72.0) * t45 * t46 * t50;
            let t105 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t101));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t105;
            acc_vsigma = tvsigma0;
            let t108 = t20 * t34;
            let t116 = f64x8::splat(1.0) / t64 / t41;
            let t117 = param_a * param_a;
            let t118 = t116 * t117;
            let t119 = t72 * t72;
            let t120 = t73 * t73;
            let t125 = f64x8::splat(1.0) / t21 / t76;
            let t129 = ((t38).select(f64x8::splat(7.0) / f64x8::splat(27.0) * t29 * t32 * t125, f64x8::splat(0.0)));
            let t132 = t42 * t117;
            let t134 = t48 * t48;
            let t136 = f64x8::splat(1.0) / t22 / t134;
            let t140 = f64x8::splat(2.0) * t118 * t119 * t120 - t66 * t129 * t73 - t132 * t119 + f64x8::splat(55.0) / f64x8::splat(81.0) * t45 * t47 * t136;
            let t145 = ((t2).select(f64x8::splat(0.0), -t7 * t108 * t54 / f64x8::splat(30.0) + t7 * t60 * t82 / f64x8::splat(5.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t140));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t145 + f64x8::splat(4.0) * t87;
            acc_v2rho2 = tv2rho20;
            let t151 = t95 * t120;
            let t158 = ((t38).select(-t29 * t91 * t68 / f64x8::splat(18.0), f64x8::splat(0.0)));
            let t159 = t158 * t73;
            let t166 = f64x8::splat(2.0) * t118 * t151 * t72 - t66 * t159 - t132 * t95 * t72 - f64x8::splat(5.0) / f64x8::splat(27.0) * t45 * t46 * t78;
            let t171 = ((t2).select(f64x8::splat(0.0), t7 * t60 * t101 / f64x8::splat(10.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t166));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t171 + f64x8::splat(2.0) * t105;
            acc_v2rhosigma = tv2rhosigma0;
            let t174 = t95 * t95;
            let t175 = t174 * t120;
            let t180 = f64x8::splat(1.0) / t30 / v_sigma * t31;
            let t184 = ((t38).select(-t29 * t180 * t34 / f64x8::splat(48.0), f64x8::splat(0.0)));
            let t185 = t184 * t73;
            let t188 = f64x8::splat(2.0) * t118 * t175 - t132 * t174 - t66 * t185;
            let t192 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t188));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t192;
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

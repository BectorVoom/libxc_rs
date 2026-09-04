//! GGA_XC_TH1 fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_xc_th1.c`
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
pub fn gga_xc_th1_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    param_omega_0: f64,
    param_omega_1: f64,
    param_omega_2: f64,
    param_omega_3: f64,
    param_omega_4: f64,
    param_omega_5: f64,
    param_omega_6: f64,
    param_omega_7: f64,
    param_omega_8: f64,
    param_omega_9: f64,
    param_omega_10: f64,
    param_omega_11: f64,
    param_omega_12: f64,
    param_omega_13: f64,
    param_omega_14: f64,
    param_omega_15: f64,
    param_omega_20: f64,
    param_omega_16: f64,
    param_omega_17: f64,
    param_omega_18: f64,
    param_omega_19: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_omega_0 = f64x8::splat(param_omega_0);
    let param_omega_1 = f64x8::splat(param_omega_1);
    let param_omega_2 = f64x8::splat(param_omega_2);
    let param_omega_3 = f64x8::splat(param_omega_3);
    let param_omega_4 = f64x8::splat(param_omega_4);
    let param_omega_5 = f64x8::splat(param_omega_5);
    let param_omega_6 = f64x8::splat(param_omega_6);
    let param_omega_7 = f64x8::splat(param_omega_7);
    let param_omega_8 = f64x8::splat(param_omega_8);
    let param_omega_9 = f64x8::splat(param_omega_9);
    let param_omega_10 = f64x8::splat(param_omega_10);
    let param_omega_11 = f64x8::splat(param_omega_11);
    let param_omega_12 = f64x8::splat(param_omega_12);
    let param_omega_13 = f64x8::splat(param_omega_13);
    let param_omega_14 = f64x8::splat(param_omega_14);
    let param_omega_15 = f64x8::splat(param_omega_15);
    let param_omega_20 = f64x8::splat(param_omega_20);
    let param_omega_16 = f64x8::splat(param_omega_16);
    let param_omega_17 = f64x8::splat(param_omega_17);
    let param_omega_18 = f64x8::splat(param_omega_18);
    let param_omega_19 = f64x8::splat(param_omega_19);
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
            let t2 = (simd::pow(f64x8::splat(2.0), f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t3 = t2 * t2;
            let t4 = t3 * t3;
            let t6 = param_omega_0 * t4 * t2;
            let t7 = (simd::pow(v_rho, f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t8 = t7 * v_rho;
            let t12 = f64x8::splat(M_CBRT2);
            let t13 = t12 * t12;
            let t14 = param_omega_1 * t13;
            let t15 = (simd::cbrt(v_rho));
            let t16 = t15 * v_rho;
            let t20 = f64x8::splat(M_SQRT2);
            let t21 = param_omega_2 * t20;
            let t22 = ((v_rho).sqrt());
            let t23 = t22 * v_rho;
            let t27 = param_omega_3 * t12;
            let t28 = t15 * t15;
            let t29 = t28 * v_rho;
            let t33 = param_omega_4 * t13;
            let t34 = ((v_sigma).sqrt());
            let t36 = (simd::cbrt(zeta_threshold));
            let t38 = (((f64x8::splat(1.0)).simd_le(zeta_threshold)).select(t36 * zeta_threshold, f64x8::splat(1.0)));
            let t43 = param_omega_5 * t20;
            let t49 = param_omega_6 * t12;
            let t55 = param_omega_7 * t2;
            let t61 = param_omega_8 * t20;
            let t62 = f64x8::splat(1.0) / t8;
            let t64 = t38 * t38;
            let t69 = param_omega_9 * t12;
            let t70 = f64x8::splat(1.0) / v_rho;
            let t76 = param_omega_10 * t2;
            let t77 = t7 * t7;
            let t78 = t77 * t77;
            let t79 = t78 * t7;
            let t80 = f64x8::splat(1.0) / t79;
            let t85 = param_omega_11;
            let t86 = f64x8::splat(1.0) / t28;
            let t87 = t85 * t86;
            let t88 = v_sigma * t64;
            let t92 = param_omega_12 * t20;
            let t93 = v_rho * v_rho;
            let t95 = f64x8::splat(1.0) / t28 / t93;
            let t96 = v_sigma * t95;
            let t98 = t96 * t64 - t96;
            let t103 = param_omega_13 * t12;
            let t108 = param_omega_14 * t2;
            let t109 = t79 * v_rho;
            let t113 = param_omega_15;
            let t114 = t113 * t93;
            let t117 = param_omega_20;
            let t119 = t6 * t8 / f64x8::splat(2.0) + t14 * t16 / f64x8::splat(2.0) + t21 * t23 / f64x8::splat(2.0) + t27 * t29 / f64x8::splat(2.0) + t33 * t34 * t38 / f64x8::splat(4.0) + t43 * t7 * t34 * t38 / f64x8::splat(4.0) + t49 * t15 * t34 * t38 / f64x8::splat(4.0) + t55 * t22 * t34 * t38 / f64x8::splat(4.0) + t61 * t62 * v_sigma * t64 / f64x8::splat(8.0) + t69 * t70 * v_sigma * t64 / f64x8::splat(8.0) + t76 * t80 * v_sigma * t64 / f64x8::splat(8.0) + t87 * t88 / f64x8::splat(8.0) + t92 * t23 * t98 / f64x8::splat(2.0) + t103 * t29 * t98 / f64x8::splat(2.0) + t108 * t109 * t98 / f64x8::splat(2.0) + t114 * t98 / f64x8::splat(2.0) + t117 * v_rho;
            let tzk0 = t119 * t70;
            acc_zk = tzk0;
            let t136 = f64x8::splat(1.0) / t22;
            let t142 = f64x8::splat(1.0) / t7 / t93;
            let t147 = f64x8::splat(1.0) / t93;
            let t152 = f64x8::splat(1.0) / t109;
            let t157 = f64x8::splat(1.0) / t29;
            let t158 = t85 * t157;
            let t164 = t93 * v_rho;
            let t166 = f64x8::splat(1.0) / t28 / t164;
            let t167 = v_sigma * t166;
            let t170 = -f64x8::splat(8.0) / f64x8::splat(3.0) * t167 * t64 + f64x8::splat(8.0) / f64x8::splat(3.0) * t167;
            let t186 = t113 * v_rho;
            let tvrho0 = f64x8::splat(7.0) / f64x8::splat(12.0) * t6 * t7 + f64x8::splat(2.0) / f64x8::splat(3.0) * t14 * t15 + f64x8::splat(3.0) / f64x8::splat(4.0) * t21 * t22 + f64x8::splat(5.0) / f64x8::splat(6.0) * t27 * t28 + t43 * t80 * t34 * t38 / f64x8::splat(24.0) + t49 * t86 * t34 * t38 / f64x8::splat(12.0) + t55 * t136 * t34 * t38 / f64x8::splat(8.0) - f64x8::splat(7.0) / f64x8::splat(48.0) * t61 * t142 * v_sigma * t64 - t69 * t147 * v_sigma * t64 / f64x8::splat(8.0) - f64x8::splat(5.0) / f64x8::splat(48.0) * t76 * t152 * v_sigma * t64 - t158 * t88 / f64x8::splat(12.0) + f64x8::splat(3.0) / f64x8::splat(4.0) * t92 * t22 * t98 + t92 * t23 * t170 / f64x8::splat(2.0) + f64x8::splat(5.0) / f64x8::splat(6.0) * t103 * t28 * t98 + t103 * t29 * t170 / f64x8::splat(2.0) + f64x8::splat(11.0) / f64x8::splat(12.0) * t108 * t79 * t98 + t108 * t109 * t170 / f64x8::splat(2.0) + t186 * t98 + t114 * t170 / f64x8::splat(2.0) + t117;
            acc_vrho = tvrho0;
            let t190 = f64x8::splat(1.0) / t34;
            let t218 = t95 * t64 - t95;
            let tvsigma0 = t33 * t190 * t38 / f64x8::splat(8.0) + t43 * t7 * t190 * t38 / f64x8::splat(8.0) + t49 * t15 * t190 * t38 / f64x8::splat(8.0) + t55 * t22 * t190 * t38 / f64x8::splat(8.0) + t61 * t62 * t64 / f64x8::splat(8.0) + t69 * t70 * t64 / f64x8::splat(8.0) + t76 * t80 * t64 / f64x8::splat(8.0) + t87 * t64 / f64x8::splat(8.0) + t92 * t23 * t218 / f64x8::splat(2.0) + t103 * t29 * t218 / f64x8::splat(2.0) + t108 * t109 * t218 / f64x8::splat(2.0) + t114 * t218 / f64x8::splat(2.0);
            acc_vsigma = tvsigma0;
            let t230 = t85 * t95;
            let t236 = t93 * t93;
            let t238 = f64x8::splat(1.0) / t28 / t236;
            let t239 = v_sigma * t238;
            let t242 = f64x8::splat(88.0) / f64x8::splat(9.0) * t239 * t64 - f64x8::splat(88.0) / f64x8::splat(9.0) * t239;
            let t266 = f64x8::splat(1.0) / t23;
            let t272 = f64x8::splat(1.0) / t7 / t164;
            let t277 = f64x8::splat(5.0) / f64x8::splat(36.0) * t230 * t88 + f64x8::splat(3.0) / f64x8::splat(2.0) * t92 * t22 * t170 + t92 * t23 * t242 / f64x8::splat(2.0) + f64x8::splat(5.0) / f64x8::splat(3.0) * t103 * t28 * t170 + t103 * t29 * t242 / f64x8::splat(2.0) + f64x8::splat(11.0) / f64x8::splat(6.0) * t108 * t79 * t170 + t108 * t109 * t242 / f64x8::splat(2.0) - f64x8::splat(5.0) / f64x8::splat(144.0) * t43 * t152 * t34 * t38 - t49 * t157 * t34 * t38 / f64x8::splat(18.0) - t55 * t266 * t34 * t38 / f64x8::splat(16.0) + f64x8::splat(91.0) / f64x8::splat(288.0) * t61 * t272 * v_sigma * t64;
            let t278 = f64x8::splat(1.0) / t164;
            let t284 = f64x8::splat(1.0) / t79 / t93;
            let t292 = f64x8::splat(1.0) / t15;
            let t296 = f64x8::splat(1.0) / t7;
            let t313 = t69 * t278 * v_sigma * t64 / f64x8::splat(4.0) + f64x8::splat(55.0) / f64x8::splat(288.0) * t76 * t284 * v_sigma * t64 + f64x8::splat(3.0) / f64x8::splat(8.0) * t92 * t136 * t98 + f64x8::splat(5.0) / f64x8::splat(9.0) * t103 * t292 * t98 + f64x8::splat(55.0) / f64x8::splat(72.0) * t108 * t296 * t98 + f64x8::splat(7.0) / f64x8::splat(72.0) * t6 * t80 + f64x8::splat(2.0) / f64x8::splat(9.0) * t14 * t86 + f64x8::splat(3.0) / f64x8::splat(8.0) * t21 * t136 + f64x8::splat(5.0) / f64x8::splat(9.0) * t27 * t292 + t113 * t98 + f64x8::splat(2.0) * t186 * t170 + t114 * t242 / f64x8::splat(2.0);
            let tv2rho20 = t277 + t313;
            acc_v2rho2 = tv2rho20;
            let t342 = -f64x8::splat(8.0) / f64x8::splat(3.0) * t166 * t64 + f64x8::splat(8.0) / f64x8::splat(3.0) * t166;
            let tv2rhosigma0 = t43 * t80 * t190 * t38 / f64x8::splat(48.0) + t49 * t86 * t190 * t38 / f64x8::splat(24.0) + t55 * t136 * t190 * t38 / f64x8::splat(16.0) - f64x8::splat(7.0) / f64x8::splat(48.0) * t61 * t142 * t64 - t69 * t147 * t64 / f64x8::splat(8.0) - f64x8::splat(5.0) / f64x8::splat(48.0) * t76 * t152 * t64 - t158 * t64 / f64x8::splat(12.0) + f64x8::splat(3.0) / f64x8::splat(4.0) * t92 * t22 * t218 + t92 * t23 * t342 / f64x8::splat(2.0) + f64x8::splat(5.0) / f64x8::splat(6.0) * t103 * t28 * t218 + t103 * t29 * t342 / f64x8::splat(2.0) + f64x8::splat(11.0) / f64x8::splat(12.0) * t108 * t79 * t218 + t108 * t109 * t342 / f64x8::splat(2.0) + t186 * t218 + t114 * t342 / f64x8::splat(2.0);
            acc_v2rhosigma = tv2rhosigma0;
            let t362 = f64x8::splat(1.0) / t34 / v_sigma;
            let tv2sigma20 = -t49 * t15 * t362 * t38 / f64x8::splat(16.0) - t55 * t22 * t362 * t38 / f64x8::splat(16.0) - t43 * t7 * t362 * t38 / f64x8::splat(16.0) - t33 * t362 * t38 / f64x8::splat(16.0);
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

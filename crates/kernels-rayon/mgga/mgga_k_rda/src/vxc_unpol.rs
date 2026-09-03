//! MGGA_K_RDA vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_k_rda.c`
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
pub fn mgga_k_rda_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_A0: f64,
    param_A1: f64,
    param_A2: f64,
    param_A3: f64,
    param_a: f64,
    param_b: f64,
    param_beta1: f64,
    param_beta2: f64,
    param_beta3: f64,
    param_c: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_A0 = f64x8::splat(param_A0);
    let param_A1 = f64x8::splat(param_A1);
    let param_A2 = f64x8::splat(param_A2);
    let param_A3 = f64x8::splat(param_A3);
    let param_a = f64x8::splat(param_a);
    let param_b = f64x8::splat(param_b);
    let param_beta1 = f64x8::splat(param_beta1);
    let param_beta2 = f64x8::splat(param_beta2);
    let param_beta3 = f64x8::splat(param_beta3);
    let param_c = f64x8::splat(param_c);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let v_lapl = load(lapl, ip, np);
        let v_tau = load(tau, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
        let mut acc_vlapl = V_ZERO;
        let mut acc_vtau = V_ZERO;
        {
            let t3 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t4 = f64x8::splat(M_CBRT3);
            let t5 = t4 * t4;
            let t6 = f64x8::splat(M_CBRTPI);
            let t8 = t5 * t6 * f64x8::splat(M_PI);
            let t9 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t10 = zeta_threshold - f64x8::splat(1.0);
            let t12 = ((t9).select(t10, (t9).select(-t10, f64x8::splat(0.0))));
            let t13 = f64x8::splat(1.0) + t12;
            let t15 = (simd::cbrt(zeta_threshold));
            let t16 = t15 * t15;
            let t18 = (simd::cbrt(t13));
            let t19 = t18 * t18;
            let t21 = (((t13).simd_le(zeta_threshold)).select(t16 * zeta_threshold, t19 * t13));
            let t22 = (simd::cbrt(v_rho));
            let t23 = t22 * t22;
            let t24 = t21 * t23;
            let t25 = f64x8::splat(M_CBRT6);
            let t26 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t27 = (simd::cbrt(t26));
            let t28 = t27 * t27;
            let t29 = f64x8::splat(1.0) / t28;
            let t30 = t25 * t29;
            let t31 = f64x8::splat(M_CBRT2);
            let t32 = t31 * t31;
            let t33 = v_sigma * t32;
            let t34 = v_rho * v_rho;
            let t36 = f64x8::splat(1.0) / t23 / t34;
            let t38 = t30 * t33 * t36;
            let t40 = t25 * t25;
            let t42 = f64x8::splat(1.0) / t27 / t26;
            let t43 = t40 * t42;
            let t44 = v_sigma * v_sigma;
            let t45 = t44 * t31;
            let t46 = t34 * t34;
            let t47 = t46 * v_rho;
            let t49 = f64x8::splat(1.0) / t22 / t47;
            let t51 = t43 * t45 * t49;
            let t52 = param_a * t40;
            let t53 = t52 * t42;
            let t54 = v_lapl * v_lapl;
            let t55 = t54 * t31;
            let t56 = t34 * v_rho;
            let t58 = f64x8::splat(1.0) / t22 / t56;
            let t59 = t55 * t58;
            let t62 = f64x8::splat(2.0) * t53 * t59 + f64x8::splat(2.0) * t51;
            let t64 = ((t62).sqrt());
            let t67 = f64x8::splat(1.0) + param_beta1 * t64 / f64x8::splat(24.0);
            let t68 = t67 * t67;
            let t69 = f64x8::splat(1.0) / t68;
            let t72 = param_b * t40;
            let t73 = t72 * t42;
            let t76 = f64x8::splat(2.0) * t73 * t59 + f64x8::splat(2.0) * t51;
            let t77 = t76 * t76;
            let t79 = ((t76).sqrt());
            let t82 = f64x8::splat(1.0) + param_beta2 * t79 / f64x8::splat(24.0);
            let t83 = t82 * t82;
            let t84 = t83 * t83;
            let t85 = f64x8::splat(1.0) / t84;
            let t88 = param_c * t25;
            let t89 = t88 * t29;
            let t90 = v_lapl * t32;
            let t92 = f64x8::splat(1.0) / t23 / v_rho;
            let t96 = t89 * t90 * t92 / f64x8::splat(24.0) + t38 / f64x8::splat(24.0);
            let t97 = param_A3 * t96;
            let t99 = param_beta3 * t96 + f64x8::splat(1.0);
            let t100 = f64x8::splat(1.0) / t99;
            let t102 = f64x8::splat(5.0) / f64x8::splat(72.0) * t38 + param_A0 + param_A1 * t62 * t69 / f64x8::splat(576.0) + param_A2 * t77 * t85 / f64x8::splat(331776.0) + t97 * t100;
            let t106 = ((t3).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t8 * t24 * t102));
            let tzk0 = f64x8::splat(2.0) * t106;
            acc_zk = tzk0;
            let t108 = t21 / t22;
            let t113 = f64x8::splat(1.0) / t23 / t56;
            let t115 = t30 * t33 * t113;
            let t117 = t46 * t34;
            let t119 = f64x8::splat(1.0) / t22 / t117;
            let t122 = f64x8::splat(32.0) / f64x8::splat(3.0) * t43 * t45 * t119;
            let t124 = f64x8::splat(1.0) / t22 / t46;
            let t125 = t55 * t124;
            let t128 = -t122 - f64x8::splat(20.0) / f64x8::splat(3.0) * t53 * t125;
            let t132 = param_A1 * t64;
            let t134 = f64x8::splat(1.0) / t68 / t67;
            let t135 = t134 * param_beta1;
            let t139 = param_A2 * t76;
            let t142 = -t122 - f64x8::splat(20.0) / f64x8::splat(3.0) * t73 * t125;
            let t146 = t79 * t76;
            let t147 = param_A2 * t146;
            let t149 = f64x8::splat(1.0) / t84 / t82;
            let t150 = t149 * param_beta2;
            let t158 = -t115 / f64x8::splat(9.0) - f64x8::splat(5.0) / f64x8::splat(72.0) * t89 * t90 * t36;
            let t159 = param_A3 * t158;
            let t161 = t99 * t99;
            let t162 = f64x8::splat(1.0) / t161;
            let t163 = t162 * param_beta3;
            let t164 = t163 * t158;
            let t166 = -f64x8::splat(5.0) / f64x8::splat(27.0) * t115 + param_A1 * t128 * t69 / f64x8::splat(576.0) - t132 * t135 * t128 / f64x8::splat(13824.0) + t139 * t85 * t142 / f64x8::splat(165888.0) - t147 * t150 * t142 / f64x8::splat(3981312.0) + t159 * t100 - t97 * t164;
            let t171 = ((t3).select(f64x8::splat(0.0), t8 * t108 * t102 / f64x8::splat(10.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t8 * t24 * t166));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t171 + f64x8::splat(2.0) * t106;
            acc_vrho = tvrho0;
            let t174 = t32 * t36;
            let t175 = t30 * t174;
            let t177 = param_A1 * t40;
            let t178 = t177 * t42;
            let t179 = v_sigma * t31;
            let t180 = t49 * t69;
            let t184 = t132 * t135;
            let t185 = t179 * t49;
            let t186 = t43 * t185;
            let t189 = t85 * t40;
            let t190 = t139 * t189;
            let t191 = t42 * v_sigma;
            let t192 = t31 * t49;
            let t193 = t191 * t192;
            let t196 = t147 * t150;
            let t199 = param_A3 * t25;
            let t200 = t199 * t29;
            let t204 = t97 * t163;
            let t207 = f64x8::splat(5.0) / f64x8::splat(72.0) * t175 + t178 * t179 * t180 / f64x8::splat(144.0) - t184 * t186 / f64x8::splat(3456.0) + t190 * t193 / f64x8::splat(41472.0) - t196 * t186 / f64x8::splat(995328.0) + t200 * t174 * t100 / f64x8::splat(24.0) - t204 * t175 / f64x8::splat(24.0);
            let t211 = ((t3).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t8 * t24 * t207));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t211;
            acc_vsigma = tvsigma0;
            let t213 = param_A1 * param_a;
            let t214 = t213 * t43;
            let t215 = v_lapl * t31;
            let t216 = t58 * t69;
            let t220 = t135 * param_a;
            let t221 = t132 * t220;
            let t222 = t215 * t58;
            let t223 = t43 * t222;
            let t226 = t85 * param_b;
            let t227 = t139 * t226;
            let t231 = t147 * t150 * param_b;
            let t234 = param_A3 * param_c;
            let t235 = t234 * t25;
            let t236 = t29 * t32;
            let t242 = t88 * t236 * t92;
            let t245 = t214 * t215 * t216 / f64x8::splat(144.0) - t221 * t223 / f64x8::splat(3456.0) + t227 * t223 / f64x8::splat(41472.0) - t231 * t223 / f64x8::splat(995328.0) + t235 * t236 * t92 * t100 / f64x8::splat(24.0) - t204 * t242 / f64x8::splat(24.0);
            let t249 = ((t3).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t8 * t24 * t245));
            let tvlapl0 = f64x8::splat(2.0) * v_rho * t249;
            acc_vlapl = tvlapl0;
            let tvtau0 = f64x8::splat(0.0);
            acc_vtau = tvtau0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vlapl.into(); vlapl[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vtau.into(); vtau[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}

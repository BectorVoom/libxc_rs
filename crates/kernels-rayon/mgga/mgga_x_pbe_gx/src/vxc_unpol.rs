//! MGGA_X_PBE_GX vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_pbe_gx.c`
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
pub fn mgga_x_pbe_gx_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
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
            let t5 = f64x8::splat(M_CBRTPI);
            let t8 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t9 = zeta_threshold - f64x8::splat(1.0);
            let t11 = ((t8).select(t9, (t8).select(-t9, f64x8::splat(0.0))));
            let t12 = f64x8::splat(1.0) + t11;
            let t14 = (simd::cbrt(zeta_threshold));
            let t16 = (simd::cbrt(t12));
            let t18 = (((t12).simd_le(zeta_threshold)).select(t14 * zeta_threshold, t16 * t12));
            let t19 = t4 / t5 * t18;
            let t20 = (simd::cbrt(v_rho));
            let t21 = f64x8::splat(M_CBRT2);
            let t22 = t4 * t4;
            let t24 = f64x8::splat(M_CBRT4);
            let t26 = f64x8::splat(8.0) / f64x8::splat(27.0) * t21 * t22 * t24;
            let t27 = t21 * t21;
            let t28 = v_tau * t27;
            let t29 = t20 * t20;
            let t31 = f64x8::splat(1.0) / t29 / v_rho;
            let t33 = v_sigma * t27;
            let t34 = v_rho * v_rho;
            let t36 = f64x8::splat(1.0) / t29 / t34;
            let t37 = t33 * t36;
            let t39 = t28 * t31 - t37 / f64x8::splat(8.0);
            let t40 = f64x8::splat(M_CBRT6);
            let t42 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t43 = (simd::cbrt(t42));
            let t44 = t43 * t43;
            let t45 = f64x8::splat(1.0) / t44;
            let t46 = t39 * t40 * t45;
            let t48 = f64x8::splat(0.827411) - f64x8::splat(0.3575333333333333) * t46;
            let t50 = f64x8::splat(1.0) - f64x8::splat(0.45341611111111113) * t46;
            let t51 = f64x8::splat(1.0) / t50;
            let t53 = f64x8::splat(1.0) - t26;
            let t54 = t48 * t51 * t53;
            let t57 = t26 + f64x8::splat(5.0) / f64x8::splat(9.0) * t46 * t54;
            let t58 = f64x8::splat(5.0) / f64x8::splat(9.0) * t46;
            let t59 = f64x8::splat(1.0) - t58;
            let t60 = ((t59).simd_ge(V_ZERO).select(V_ONE, V_ZERO));
            let t62 = f64x8::splat(1.0) + t58;
            let t63 = f64x8::splat(1.0) / t62;
            let t66 = f64x8::splat(1.0) + f64x8::splat(0.148) * t59 * t63;
            let t67 = -t59;
            let t68 = ((t67).simd_ge(V_ZERO).select(V_ONE, V_ZERO));
            let t70 = t57 * t60 + t66 * t68;
            let t73 = f64x8::splat(1.0) + f64x8::splat(0.001015549) * t37;
            let t74 = f64x8::splat(1.0) / t73;
            let t78 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t70 * t74));
            let tzk0 = f64x8::splat(2.0) * t78;
            acc_zk = tzk0;
            let t79 = f64x8::splat(1.0) / t29;
            let t86 = t34 * v_rho;
            let t88 = f64x8::splat(1.0) / t29 / t86;
            let t91 = -f64x8::splat(5.0) / f64x8::splat(3.0) * t28 * t36 + t33 * t88 / f64x8::splat(3.0);
            let t92 = t91 * t40;
            let t93 = t92 * t45;
            let t96 = t40 * t40;
            let t97 = t39 * t96;
            let t99 = f64x8::splat(1.0) / t43 / t42;
            let t100 = t97 * t99;
            let t102 = t91 * t51 * t53;
            let t105 = t50 * t50;
            let t106 = f64x8::splat(1.0) / t105;
            let t107 = t48 * t106;
            let t108 = t53 * t91;
            let t109 = t107 * t108;
            let t112 = f64x8::splat(5.0) / f64x8::splat(9.0) * t93 * t54 - f64x8::splat(0.19862962962962963) * t100 * t102 + f64x8::splat(0.25189783950617284) * t100 * t109;
            let t114 = f64x8::splat(0.0);
            let t115 = t57 * t114;
            let t118 = t45 * t63;
            let t121 = t62 * t62;
            let t122 = f64x8::splat(1.0) / t121;
            let t123 = t59 * t122;
            let t126 = -f64x8::splat(0.08222222222222222) * t92 * t118 - f64x8::splat(0.08222222222222222) * t123 * t93;
            let t128 = t66 * t114;
            let t131 = t112 * t60 - f64x8::splat(5.0) / f64x8::splat(9.0) * t115 * t93 + t126 * t68 + f64x8::splat(5.0) / f64x8::splat(9.0) * t128 * t93;
            let t136 = t4 * t18;
            let t138 = f64x8::splat(1.0) / t20 / t86;
            let t139 = t136 * t138;
            let t140 = t73 * t73;
            let t141 = f64x8::splat(1.0) / t140;
            let t142 = t70 * t141;
            let t143 = t142 * t33;
            let t147 = ((t3).select(f64x8::splat(0.0), -t19 * t79 * t70 * t74 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t131 * t74 - f64x8::splat(0.0006934006726548522) * t139 * t143));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t147 + f64x8::splat(2.0) * t78;
            acc_vrho = tvrho0;
            let t150 = t27 * t36;
            let t153 = t51 * t53;
            let t154 = t45 * t48 * t153;
            let t155 = t150 * t40 * t154;
            let t158 = t100 * t150 * t153;
            let t160 = t99 * t48;
            let t161 = t97 * t160;
            let t162 = t106 * t53;
            let t164 = t161 * t162 * t150;
            let t166 = -f64x8::splat(5.0) / f64x8::splat(72.0) * t155 + f64x8::splat(0.024828703703703704) * t158 - f64x8::splat(0.031487229938271605) * t164;
            let t168 = t115 * t27;
            let t170 = t36 * t40 * t45;
            let t171 = t168 * t170;
            let t173 = t40 * t45;
            let t174 = t173 * t63;
            let t175 = t150 * t174;
            let t177 = t123 * t27;
            let t178 = t177 * t170;
            let t180 = f64x8::splat(0.010277777777777778) * t175 + f64x8::splat(0.010277777777777778) * t178;
            let t182 = t128 * t27;
            let t183 = t182 * t170;
            let t185 = t166 * t60 + f64x8::splat(5.0) / f64x8::splat(72.0) * t171 + t180 * t68 - f64x8::splat(5.0) / f64x8::splat(72.0) * t183;
            let t192 = t136 / t20 / t34;
            let t193 = t142 * t27;
            let t197 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t185 * t74 + f64x8::splat(0.0002600252522455696) * t192 * t193));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t197;
            acc_vsigma = tvsigma0;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl = tvlapl0;
            let t199 = t27 * t31;
            let t209 = f64x8::splat(5.0) / f64x8::splat(9.0) * t199 * t40 * t154 - f64x8::splat(0.19862962962962963) * t100 * t199 * t153 + f64x8::splat(0.25189783950617284) * t161 * t162 * t199;
            let t212 = t31 * t40 * t45;
            let t219 = -f64x8::splat(0.08222222222222222) * t199 * t174 - f64x8::splat(0.08222222222222222) * t177 * t212;
            let t223 = t209 * t60 - f64x8::splat(5.0) / f64x8::splat(9.0) * t168 * t212 + t219 * t68 + f64x8::splat(5.0) / f64x8::splat(9.0) * t182 * t212;
            let t228 = ((t3).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t19 * t20 * t223 * t74));
            let tvtau0 = f64x8::splat(2.0) * v_rho * t228;
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

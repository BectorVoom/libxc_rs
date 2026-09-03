//! GGA_X_AK13 vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_ak13.c`
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

/// Load 8 elements with a given stride and offset.
#[inline(always)]
fn load_strided(s: &[f64], ip: usize, np: usize, stride: usize, offset: usize) -> f64x8 {
    let mut b = [0.0f64; 8];
    if ip + 8 <= np {
        let base = ip * stride + offset;
        b[0] = s[base];
        b[1] = s[base + stride];
        b[2] = s[base + 2 * stride];
        b[3] = s[base + 3 * stride];
        b[4] = s[base + 4 * stride];
        b[5] = s[base + 5 * stride];
        b[6] = s[base + 6 * stride];
        b[7] = s[base + 7 * stride];
    } else {
        for k in 0..8 {
            let p = (ip + k).min(np - 1);
            b[k] = s[p * stride + offset];
        }
    }
    f64x8::new(b)
}

/// Store 8 elements with a given stride and offset.
#[inline(always)]
fn store_strided(s: &mut [f64], ip: usize, m: usize, stride: usize, offset: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let base = ip * stride + offset;
        s[base] = a[0];
        s[base + stride] = a[1];
        s[base + 2 * stride] = a[2];
        s[base + 3 * stride] = a[3];
        s[base + 4 * stride] = a[4];
        s[base + 5 * stride] = a[5];
        s[base + 6 * stride] = a[6];
        s[base + 7 * stride] = a[7];
    } else {
        for k in 0..m {
            s[(ip + k) * stride + offset] = a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_ak13_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_B1: f64,
    param_B2: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_B1 = f64x8::splat(param_B1);
    let param_B2 = f64x8::splat(param_B2);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho0 = load_strided(rho, ip, np, 2, 0);
        let v_rho1 = load_strided(rho, ip, np, 2, 1);
        let v_sigma0 = load_strided(sigma, ip, np, 3, 0);
        let v_sigma1 = load_strided(sigma, ip, np, 3, 1);
        let v_sigma2 = load_strided(sigma, ip, np, 3, 2);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        let mut acc_vsigma_0 = V_ZERO;
        let mut acc_vsigma_1 = V_ZERO;
        let mut acc_vsigma_2 = V_ZERO;
        {
            let t1 = (v_rho0).simd_le(dens_threshold);
            let t2 = f64x8::splat(M_CBRT3);
            let t3 = f64x8::splat(M_CBRTPI);
            let t5 = t2 / t3;
            let t6 = v_rho0 + v_rho1;
            let t7 = f64x8::splat(1.0) / t6;
            let t10 = (f64x8::splat(2.0) * v_rho0 * t7).simd_le(zeta_threshold);
            let t11 = zeta_threshold - f64x8::splat(1.0);
            let t14 = (f64x8::splat(2.0) * v_rho1 * t7).simd_le(zeta_threshold);
            let t15 = -t11;
            let t16 = v_rho0 - v_rho1;
            let t18 = ((t10).select(t11, (t14).select(t15, t16 * t7)));
            let t19 = f64x8::splat(1.0) + t18;
            let t20 = (t19).simd_le(zeta_threshold);
            let t21 = (simd::cbrt(zeta_threshold));
            let t22 = t21 * zeta_threshold;
            let t23 = (simd::cbrt(t19));
            let t25 = ((t20).select(t22, t23 * t19));
            let t26 = (simd::cbrt(t6));
            let t27 = t25 * t26;
            let t28 = f64x8::splat(M_CBRT6);
            let t29 = t28 * t28;
            let t31 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t32 = (simd::cbrt(t31));
            let t33 = f64x8::splat(1.0) / t32;
            let t34 = param_B1 * t29 * t33;
            let t35 = ((v_sigma0).sqrt());
            let t36 = (simd::cbrt(v_rho0));
            let t38 = f64x8::splat(1.0) / t36 / v_rho0;
            let t39 = t35 * t38;
            let t40 = t29 * t33;
            let t43 = f64x8::splat(1.0) + t40 * t39 / f64x8::splat(12.0);
            let t44 = (simd::ln(t43));
            let t49 = param_B2 * t29 * t33;
            let t50 = f64x8::splat(1.0) + t44;
            let t51 = (simd::ln(t50));
            let t55 = f64x8::splat(1.0) + t34 * t39 * t44 / f64x8::splat(12.0) + t49 * t39 * t51 / f64x8::splat(12.0);
            let t59 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t55));
            let t60 = (v_rho1).simd_le(dens_threshold);
            let t61 = -t16;
            let t63 = ((t14).select(t11, (t10).select(t15, t61 * t7)));
            let t64 = f64x8::splat(1.0) + t63;
            let t65 = (t64).simd_le(zeta_threshold);
            let t66 = (simd::cbrt(t64));
            let t68 = ((t65).select(t22, t66 * t64));
            let t69 = t68 * t26;
            let t70 = ((v_sigma2).sqrt());
            let t71 = (simd::cbrt(v_rho1));
            let t73 = f64x8::splat(1.0) / t71 / v_rho1;
            let t74 = t70 * t73;
            let t77 = f64x8::splat(1.0) + t40 * t74 / f64x8::splat(12.0);
            let t78 = (simd::ln(t77));
            let t82 = f64x8::splat(1.0) + t78;
            let t83 = (simd::ln(t82));
            let t87 = f64x8::splat(1.0) + t34 * t74 * t78 / f64x8::splat(12.0) + t49 * t74 * t83 / f64x8::splat(12.0);
            let t91 = ((t60).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t69 * t87));
            let tzk0 = t59 + t91;
            acc_zk = tzk0;
            let t92 = t6 * t6;
            let t93 = f64x8::splat(1.0) / t92;
            let t94 = t16 * t93;
            let t96 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t7 - t94)));
            let t99 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t96));
            let t100 = t99 * t26;
            let t104 = t26 * t26;
            let t105 = f64x8::splat(1.0) / t104;
            let t106 = t25 * t105;
            let t109 = t5 * t106 * t55 / f64x8::splat(8.0);
            let t110 = v_rho0 * v_rho0;
            let t112 = f64x8::splat(1.0) / t36 / t110;
            let t113 = t35 * t112;
            let t117 = param_B1 * t28;
            let t118 = t32 * t32;
            let t119 = f64x8::splat(1.0) / t118;
            let t120 = t117 * t119;
            let t121 = t110 * v_rho0;
            let t122 = t36 * t36;
            let t124 = f64x8::splat(1.0) / t122 / t121;
            let t125 = v_sigma0 * t124;
            let t126 = f64x8::splat(1.0) / t43;
            let t134 = param_B2 * t28 * t119;
            let t135 = f64x8::splat(1.0) / t50;
            let t136 = t126 * t135;
            let t140 = -t34 * t113 * t44 / f64x8::splat(9.0) - t120 * t125 * t126 / f64x8::splat(18.0) - t49 * t113 * t51 / f64x8::splat(9.0) - t134 * t125 * t136 / f64x8::splat(18.0);
            let t145 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t100 * t55 - t109 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t140));
            let t146 = t61 * t93;
            let t148 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -t7 - t146)));
            let t151 = ((t65).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t66 * t148));
            let t152 = t151 * t26;
            let t156 = t68 * t105;
            let t159 = t5 * t156 * t87 / f64x8::splat(8.0);
            let t161 = ((t60).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t152 * t87 - t159));
            let tvrho0 = t59 + t91 + t6 * (t145 + t161);
            acc_vrho_0 = tvrho0;
            let t165 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -t7 - t94)));
            let t168 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t165));
            let t169 = t168 * t26;
            let t174 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t169 * t55 - t109));
            let t176 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), t7 - t146)));
            let t179 = ((t65).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t66 * t176));
            let t180 = t179 * t26;
            let t184 = v_rho1 * v_rho1;
            let t186 = f64x8::splat(1.0) / t71 / t184;
            let t187 = t70 * t186;
            let t191 = t184 * v_rho1;
            let t192 = t71 * t71;
            let t194 = f64x8::splat(1.0) / t192 / t191;
            let t195 = v_sigma2 * t194;
            let t196 = f64x8::splat(1.0) / t77;
            let t203 = f64x8::splat(1.0) / t82;
            let t204 = t196 * t203;
            let t208 = -t34 * t187 * t78 / f64x8::splat(9.0) - t120 * t195 * t196 / f64x8::splat(18.0) - t49 * t187 * t83 / f64x8::splat(9.0) - t134 * t195 * t204 / f64x8::splat(18.0);
            let t213 = ((t60).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t180 * t87 - t159 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t69 * t208));
            let tvrho1 = t59 + t91 + t6 * (t174 + t213);
            acc_vrho_1 = tvrho1;
            let t216 = f64x8::splat(1.0) / t35;
            let t217 = t216 * t38;
            let t222 = f64x8::splat(1.0) / t122 / t110;
            let t234 = t34 * t217 * t44 / f64x8::splat(24.0) + t117 * t119 * t222 * t126 / f64x8::splat(48.0) + t49 * t217 * t51 / f64x8::splat(24.0) + t134 * t222 * t126 * t135 / f64x8::splat(48.0);
            let t238 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t234));
            let tvsigma0 = t6 * t238;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t239 = f64x8::splat(1.0) / t70;
            let t240 = t239 * t73;
            let t245 = f64x8::splat(1.0) / t192 / t184;
            let t257 = t34 * t240 * t78 / f64x8::splat(24.0) + t117 * t119 * t245 * t196 / f64x8::splat(48.0) + t49 * t240 * t83 / f64x8::splat(24.0) + t134 * t245 * t196 * t203 / f64x8::splat(48.0);
            let t261 = ((t60).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t69 * t257));
            let tvsigma2 = t6 * t261;
            acc_vsigma_2 = tvsigma2;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(vsigma, ip, m, 3, 0, acc_vsigma_0);
        store_strided(vsigma, ip, m, 3, 1, acc_vsigma_1);
        store_strided(vsigma, ip, m, 3, 2, acc_vsigma_2);
        ip += 8;
    }
}

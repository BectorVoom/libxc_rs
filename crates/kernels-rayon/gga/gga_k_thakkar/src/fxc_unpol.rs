//! GGA_K_THAKKAR fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_thakkar.c`
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
pub fn gga_k_thakkar_fxc_unpol(
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
            let t24 = f64x8::splat(M_CBRT2);
            let t25 = t24 * t24;
            let t26 = v_sigma * t25;
            let t27 = v_rho * v_rho;
            let t29 = f64x8::splat(1.0) / t22 / t27;
            let t30 = ((v_sigma).sqrt());
            let t31 = t30 * t24;
            let t33 = f64x8::splat(1.0) / t21 / v_rho;
            let t35 = (simd::ln(t31 * t33 + ((((t31 * t33) * (t31 * t33)) + f64x8::splat(1.0)).sqrt())));
            let t36 = t33 * t35;
            let t39 = f64x8::splat(1.0) + f64x8::splat(0.0253) * t31 * t36;
            let t40 = f64x8::splat(1.0) / t39;
            let t44 = f64x8::splat(M_CBRT4);
            let t49 = f64x8::splat(2.0) * t44 * t30 * t24 * t33 + f64x8::splat(1.0);
            let t50 = f64x8::splat(1.0) / t49;
            let t51 = t33 * t50;
            let t54 = f64x8::splat(1.0) + f64x8::splat(0.0055) * t26 * t29 * t40 - f64x8::splat(0.072) * t31 * t51;
            let t58 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t54));
            let tzk0 = f64x8::splat(2.0) * t58;
            acc_zk = tzk0;
            let t60 = t20 / t21;
            let t64 = t27 * v_rho;
            let t66 = f64x8::splat(1.0) / t22 / t64;
            let t70 = t39 * t39;
            let t71 = f64x8::splat(1.0) / t70;
            let t72 = t29 * t71;
            let t74 = f64x8::splat(1.0) / t21 / t27;
            let t75 = t74 * t35;
            let t78 = t26 * t29;
            let t79 = t78 + f64x8::splat(1.0);
            let t80 = ((t79).sqrt());
            let t81 = f64x8::splat(1.0) / t80;
            let t82 = t66 * t81;
            let t85 = -f64x8::splat(0.03373333333333333) * t31 * t75 - f64x8::splat(0.03373333333333333) * t26 * t82;
            let t89 = t74 * t50;
            let t92 = t49 * t49;
            let t93 = f64x8::splat(1.0) / t92;
            let t95 = t66 * t93 * t44;
            let t98 = -f64x8::splat(0.014666666666666666) * t26 * t66 * t40 - f64x8::splat(0.0055) * t26 * t72 * t85 + f64x8::splat(0.096) * t31 * t89 - f64x8::splat(0.192) * t26 * t95;
            let t103 = ((t2).select(f64x8::splat(0.0), t7 * t60 * t54 / f64x8::splat(10.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t98));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t103 + f64x8::splat(2.0) * t58;
            acc_vrho = tvrho0;
            let t106 = t25 * t29;
            let t109 = f64x8::splat(1.0) / t30;
            let t110 = t109 * t24;
            let t115 = f64x8::splat(0.01265) * t110 * t36 + f64x8::splat(0.01265) * t106 * t81;
            let t121 = t93 * t44;
            let t124 = f64x8::splat(0.0055) * t106 * t40 - f64x8::splat(0.0055) * t26 * t72 * t115 - f64x8::splat(0.036) * t110 * t51 + f64x8::splat(0.072) * t106 * t121;
            let t128 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t124));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t128;
            acc_vsigma = tvsigma0;
            let t131 = t20 * t33;
            let t138 = t27 * t27;
            let t140 = f64x8::splat(1.0) / t22 / t138;
            let t144 = t66 * t71;
            let t149 = f64x8::splat(1.0) / t70 / t39;
            let t150 = t29 * t149;
            let t151 = t85 * t85;
            let t156 = f64x8::splat(1.0) / t21 / t64;
            let t157 = t156 * t35;
            let t160 = t140 * t81;
            let t163 = v_sigma * v_sigma;
            let t164 = t163 * t24;
            let t165 = t138 * t64;
            let t167 = f64x8::splat(1.0) / t21 / t165;
            let t169 = f64x8::splat(1.0) / t80 / t79;
            let t173 = f64x8::splat(0.0787111111111111) * t31 * t157 + f64x8::splat(0.16866666666666666) * t26 * t160 - f64x8::splat(0.08995555555555555) * t164 * t167 * t169;
            let t177 = t156 * t50;
            let t181 = t140 * t93 * t44;
            let t184 = t30 * v_sigma;
            let t185 = t138 * t27;
            let t186 = f64x8::splat(1.0) / t185;
            let t189 = f64x8::splat(1.0) / t92 / t49;
            let t190 = t44 * t44;
            let t191 = t189 * t190;
            let t194 = f64x8::splat(0.05377777777777778) * t26 * t140 * t40 + f64x8::splat(0.029333333333333333) * t26 * t144 * t85 + f64x8::splat(0.011) * t26 * t150 * t151 - f64x8::splat(0.0055) * t26 * t72 * t173 - f64x8::splat(0.224) * t31 * t177 + f64x8::splat(0.96) * t26 * t181 - f64x8::splat(2.048) * t184 * t186 * t191;
            let t199 = ((t2).select(f64x8::splat(0.0), -t7 * t131 * t54 / f64x8::splat(30.0) + t7 * t60 * t98 / f64x8::splat(5.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t194));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t199 + f64x8::splat(4.0) * t103;
            acc_v2rho2 = tv2rho20;
            let t205 = t25 * t66;
            let t208 = t71 * t85;
            let t214 = t149 * t115;
            let t215 = t214 * t85;
            let t223 = f64x8::splat(1.0) / t21 / t185;
            let t224 = t24 * t223;
            let t225 = t169 * v_sigma;
            let t228 = -f64x8::splat(0.016866666666666665) * t110 * t75 - f64x8::splat(0.0506) * t205 * t81 + f64x8::splat(0.03373333333333333) * t224 * t225;
            let t236 = t138 * v_rho;
            let t237 = f64x8::splat(1.0) / t236;
            let t239 = t190 * t30;
            let t242 = -f64x8::splat(0.014666666666666666) * t205 * t40 - f64x8::splat(0.0055) * t106 * t208 + f64x8::splat(0.014666666666666666) * t26 * t144 * t115 + f64x8::splat(0.011) * t78 * t215 - f64x8::splat(0.0055) * t26 * t72 * t228 + f64x8::splat(0.048) * t110 * t89 - f64x8::splat(0.288) * t205 * t121 + f64x8::splat(0.768) * t237 * t189 * t239;
            let t247 = ((t2).select(f64x8::splat(0.0), t7 * t60 * t124 / f64x8::splat(10.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t242));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t247 + f64x8::splat(2.0) * t128;
            acc_v2rhosigma = tv2rhosigma0;
            let t250 = t71 * t115;
            let t253 = t115 * t115;
            let t257 = f64x8::splat(1.0) / t184;
            let t258 = t257 * t24;
            let t261 = f64x8::splat(1.0) / v_sigma;
            let t262 = t261 * t25;
            let t263 = t29 * t81;
            let t267 = f64x8::splat(1.0) / t21 / t236;
            let t271 = -f64x8::splat(0.006325) * t258 * t36 + f64x8::splat(0.006325) * t262 * t263 - f64x8::splat(0.01265) * t24 * t267 * t169;
            let t278 = t29 * t93 * t44;
            let t281 = f64x8::splat(1.0) / t138;
            let t286 = -f64x8::splat(0.011) * t106 * t250 + f64x8::splat(0.011) * t26 * t150 * t253 - f64x8::splat(0.0055) * t26 * t72 * t271 + f64x8::splat(0.018) * t258 * t51 + f64x8::splat(0.036) * t262 * t278 - f64x8::splat(0.288) * t281 * t189 * t190 * t109;
            let t290 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t286));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t290;
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

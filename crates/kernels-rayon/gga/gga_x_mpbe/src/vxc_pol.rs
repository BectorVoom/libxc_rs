//! GGA_X_MPBE vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_mpbe.c`
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
pub fn gga_x_mpbe_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_c1: f64,
    param_a: f64,
    param_c2: f64,
    param_c3: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_c1 = f64x8::splat(param_c1);
    let param_a = f64x8::splat(param_a);
    let param_c2 = f64x8::splat(param_c2);
    let param_c3 = f64x8::splat(param_c3);
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
            let t29 = param_c1 * t28;
            let t30 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t31 = (simd::cbrt(t30));
            let t32 = t31 * t31;
            let t33 = f64x8::splat(1.0) / t32;
            let t34 = t29 * t33;
            let t35 = v_rho0 * v_rho0;
            let t36 = (simd::cbrt(v_rho0));
            let t37 = t36 * t36;
            let t39 = f64x8::splat(1.0) / t37 / t35;
            let t41 = param_a * t28;
            let t42 = t33 * v_sigma0;
            let t46 = f64x8::splat(1.0) + t41 * t42 * t39 / f64x8::splat(24.0);
            let t47 = f64x8::splat(1.0) / t46;
            let t51 = t28 * t28;
            let t52 = param_c2 * t51;
            let t54 = f64x8::splat(1.0) / t31 / t30;
            let t55 = t52 * t54;
            let t56 = v_sigma0 * v_sigma0;
            let t57 = t35 * t35;
            let t58 = t57 * v_rho0;
            let t60 = f64x8::splat(1.0) / t36 / t58;
            let t62 = t46 * t46;
            let t63 = f64x8::splat(1.0) / t62;
            let t67 = t30 * t30;
            let t68 = f64x8::splat(1.0) / t67;
            let t69 = param_c3 * t68;
            let t70 = t56 * v_sigma0;
            let t71 = t57 * t57;
            let t72 = f64x8::splat(1.0) / t71;
            let t74 = t62 * t46;
            let t75 = f64x8::splat(1.0) / t74;
            let t79 = f64x8::splat(1.0) + t34 * v_sigma0 * t39 * t47 / f64x8::splat(24.0) + t55 * t56 * t60 * t63 / f64x8::splat(576.0) + t69 * t70 * t72 * t75 / f64x8::splat(2304.0);
            let t83 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t79));
            let t84 = (v_rho1).simd_le(dens_threshold);
            let t85 = -t16;
            let t87 = ((t14).select(t11, (t10).select(t15, t85 * t7)));
            let t88 = f64x8::splat(1.0) + t87;
            let t89 = (t88).simd_le(zeta_threshold);
            let t90 = (simd::cbrt(t88));
            let t92 = ((t89).select(t22, t90 * t88));
            let t93 = t92 * t26;
            let t94 = v_rho1 * v_rho1;
            let t95 = (simd::cbrt(v_rho1));
            let t96 = t95 * t95;
            let t98 = f64x8::splat(1.0) / t96 / t94;
            let t100 = t33 * v_sigma2;
            let t104 = f64x8::splat(1.0) + t41 * t100 * t98 / f64x8::splat(24.0);
            let t105 = f64x8::splat(1.0) / t104;
            let t109 = v_sigma2 * v_sigma2;
            let t110 = t94 * t94;
            let t111 = t110 * v_rho1;
            let t113 = f64x8::splat(1.0) / t95 / t111;
            let t115 = t104 * t104;
            let t116 = f64x8::splat(1.0) / t115;
            let t120 = t109 * v_sigma2;
            let t121 = t110 * t110;
            let t122 = f64x8::splat(1.0) / t121;
            let t124 = t115 * t104;
            let t125 = f64x8::splat(1.0) / t124;
            let t129 = f64x8::splat(1.0) + t34 * v_sigma2 * t98 * t105 / f64x8::splat(24.0) + t55 * t109 * t113 * t116 / f64x8::splat(576.0) + t69 * t120 * t122 * t125 / f64x8::splat(2304.0);
            let t133 = ((t84).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t93 * t129));
            let tzk0 = t83 + t133;
            acc_zk = tzk0;
            let t134 = t6 * t6;
            let t135 = f64x8::splat(1.0) / t134;
            let t136 = t16 * t135;
            let t138 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t7 - t136)));
            let t141 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t138));
            let t142 = t141 * t26;
            let t146 = t26 * t26;
            let t147 = f64x8::splat(1.0) / t146;
            let t148 = t25 * t147;
            let t151 = t5 * t148 * t79 / f64x8::splat(8.0);
            let t152 = t35 * v_rho0;
            let t154 = f64x8::splat(1.0) / t37 / t152;
            let t160 = param_c1 * t51 * t54;
            let t161 = t57 * t35;
            let t163 = f64x8::splat(1.0) / t36 / t161;
            let t164 = t56 * t163;
            let t165 = t63 * param_a;
            let t172 = param_c2 * t68;
            let t173 = t172 * t70;
            let t174 = t71 * v_rho0;
            let t175 = f64x8::splat(1.0) / t174;
            let t176 = t175 * t75;
            let t177 = t176 * param_a;
            let t184 = t56 * t56;
            let t185 = t71 * t152;
            let t187 = f64x8::splat(1.0) / t37 / t185;
            let t190 = t62 * t62;
            let t191 = f64x8::splat(1.0) / t190;
            let t193 = t28 * t33;
            let t194 = t191 * param_a * t193;
            let t197 = -t34 * v_sigma0 * t154 * t47 / f64x8::splat(9.0) + t160 * t164 * t165 / f64x8::splat(216.0) - t55 * t164 * t63 / f64x8::splat(108.0) + t173 * t177 / f64x8::splat(432.0) - t69 * t70 * t175 * t75 / f64x8::splat(288.0) + t69 * t184 * t187 * t194 / f64x8::splat(6912.0);
            let t202 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t142 * t79 - t151 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t197));
            let t203 = t85 * t135;
            let t205 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -t7 - t203)));
            let t208 = ((t89).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t90 * t205));
            let t209 = t208 * t26;
            let t213 = t92 * t147;
            let t216 = t5 * t213 * t129 / f64x8::splat(8.0);
            let t218 = ((t84).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t209 * t129 - t216));
            let tvrho0 = t83 + t133 + t6 * (t202 + t218);
            acc_vrho_0 = tvrho0;
            let t222 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -t7 - t136)));
            let t225 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t222));
            let t226 = t225 * t26;
            let t231 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t226 * t79 - t151));
            let t233 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), t7 - t203)));
            let t236 = ((t89).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t90 * t233));
            let t237 = t236 * t26;
            let t241 = t94 * v_rho1;
            let t243 = f64x8::splat(1.0) / t96 / t241;
            let t248 = t110 * t94;
            let t250 = f64x8::splat(1.0) / t95 / t248;
            let t251 = t109 * t250;
            let t252 = t116 * param_a;
            let t259 = t172 * t120;
            let t260 = t121 * v_rho1;
            let t261 = f64x8::splat(1.0) / t260;
            let t262 = t261 * t125;
            let t263 = t262 * param_a;
            let t270 = t109 * t109;
            let t271 = t121 * t241;
            let t273 = f64x8::splat(1.0) / t96 / t271;
            let t276 = t115 * t115;
            let t277 = f64x8::splat(1.0) / t276;
            let t279 = t277 * param_a * t193;
            let t282 = -t34 * v_sigma2 * t243 * t105 / f64x8::splat(9.0) + t160 * t251 * t252 / f64x8::splat(216.0) - t55 * t251 * t116 / f64x8::splat(108.0) + t259 * t263 / f64x8::splat(432.0) - t69 * t120 * t261 * t125 / f64x8::splat(288.0) + t69 * t270 * t273 * t279 / f64x8::splat(6912.0);
            let t287 = ((t84).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t237 * t129 - t216 - f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t93 * t282));
            let tvrho1 = t83 + t133 + t6 * (t231 + t287);
            acc_vrho_1 = tvrho1;
            let t294 = v_sigma0 * t60;
            let t301 = t172 * t56;
            let t302 = t72 * t75;
            let t303 = t302 * param_a;
            let t310 = t71 * t35;
            let t312 = f64x8::splat(1.0) / t37 / t310;
            let t317 = t29 * t33 * t39 * t47 / f64x8::splat(24.0) - t160 * t294 * t165 / f64x8::splat(576.0) + t55 * t294 * t63 / f64x8::splat(288.0) - t301 * t303 / f64x8::splat(1152.0) + t69 * t56 * t72 * t75 / f64x8::splat(768.0) - t69 * t70 * t312 * t194 / f64x8::splat(18432.0);
            let t321 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t27 * t317));
            let tvsigma0 = t6 * t321;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t326 = v_sigma2 * t113;
            let t333 = t172 * t109;
            let t334 = t122 * t125;
            let t335 = t334 * param_a;
            let t342 = t121 * t94;
            let t344 = f64x8::splat(1.0) / t96 / t342;
            let t349 = t29 * t33 * t98 * t105 / f64x8::splat(24.0) - t160 * t326 * t252 / f64x8::splat(576.0) + t55 * t326 * t116 / f64x8::splat(288.0) - t333 * t335 / f64x8::splat(1152.0) + t69 * t109 * t122 * t125 / f64x8::splat(768.0) - t69 * t120 * t344 * t279 / f64x8::splat(18432.0);
            let t353 = ((t84).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t93 * t349));
            let tvsigma2 = t6 * t353;
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

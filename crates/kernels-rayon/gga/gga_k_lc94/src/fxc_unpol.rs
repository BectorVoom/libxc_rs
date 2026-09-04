//! GGA_K_LC94 fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_lc94.c`
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
pub fn gga_k_lc94_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    param_a: f64,
    param_alpha: f64,
    param_b: f64,
    param_c: f64,
    param_d: f64,
    param_expo: f64,
    param_f: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_a = f64x8::splat(param_a);
    let param_alpha = f64x8::splat(param_alpha);
    let param_b = f64x8::splat(param_b);
    let param_c = f64x8::splat(param_c);
    let param_d = f64x8::splat(param_d);
    let param_expo = f64x8::splat(param_expo);
    let param_f = f64x8::splat(param_f);
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
            let t26 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t27 = (simd::cbrt(t26));
            let t28 = t27 * t27;
            let t29 = f64x8::splat(1.0) / t28;
            let t31 = f64x8::splat(M_CBRT2);
            let t32 = t31 * t31;
            let t33 = v_sigma * t32;
            let t34 = v_rho * v_rho;
            let t36 = f64x8::splat(1.0) / t22 / t34;
            let t37 = t33 * t36;
            let t40 = (simd::exp(-param_alpha * t24 * t29 * t37 / f64x8::splat(24.0)));
            let t43 = (t40 * param_d + param_c) * t24;
            let t44 = t43 * t29;
            let t47 = t24 * t24;
            let t48 = f64x8::splat(1.0) / t27;
            let t49 = t47 * t48;
            let t50 = ((v_sigma).sqrt());
            let t53 = f64x8::splat(1.0) / t21 / v_rho;
            let t54 = t50 * t31 * t53;
            let t57 = (simd::pow(t49 * t54 / f64x8::splat(12.0), param_expo));
            let t58 = param_f * t57;
            let t59 = t44 * t37 / f64x8::splat(24.0) - t58;
            let t60 = t49 * t50;
            let t66 = (simd::ln(param_b * t47 * t48 * t54 / f64x8::splat(12.0) + ((((param_b * t47 * t48 * t54 / f64x8::splat(12.0)) * (param_b * t47 * t48 * t54 / f64x8::splat(12.0))) + f64x8::splat(1.0)).sqrt())));
            let t67 = param_a * t66;
            let t68 = t31 * t53 * t67;
            let t71 = f64x8::splat(1.0) + t60 * t68 / f64x8::splat(12.0) + t58;
            let t72 = f64x8::splat(1.0) / t71;
            let t74 = t59 * t72 + f64x8::splat(1.0);
            let t78 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t74));
            let tzk0 = f64x8::splat(2.0) * t78;
            acc_zk = tzk0;
            let t80 = t20 / t21;
            let t84 = param_d * param_alpha;
            let t86 = f64x8::splat(1.0) / t27 / t26;
            let t87 = t47 * t86;
            let t88 = t84 * t87;
            let t89 = v_sigma * v_sigma;
            let t90 = t89 * t31;
            let t91 = t34 * t34;
            let t92 = t91 * t34;
            let t94 = f64x8::splat(1.0) / t21 / t92;
            let t95 = t94 * t40;
            let t99 = t34 * v_rho;
            let t101 = f64x8::splat(1.0) / t22 / t99;
            let t105 = f64x8::splat(1.0) / v_rho;
            let t108 = f64x8::splat(4.0) / f64x8::splat(3.0) * t58 * param_expo * t105;
            let t109 = t88 * t90 * t95 / f64x8::splat(108.0) - t44 * t33 * t101 / f64x8::splat(9.0) + t108;
            let t111 = t71 * t71;
            let t112 = f64x8::splat(1.0) / t111;
            let t113 = t59 * t112;
            let t115 = f64x8::splat(1.0) / t21 / t34;
            let t117 = t31 * t115 * t67;
            let t120 = t24 * t29;
            let t121 = t120 * t33;
            let t123 = param_b * param_b;
            let t128 = f64x8::splat(6.0) * t123 * t24 * t29 * t37 + f64x8::splat(144.0);
            let t129 = ((t128).sqrt());
            let t131 = param_b / t129;
            let t132 = t101 * param_a * t131;
            let t135 = -t60 * t117 / f64x8::splat(9.0) - f64x8::splat(2.0) / f64x8::splat(3.0) * t121 * t132 - t108;
            let t137 = t109 * t72 - t113 * t135;
            let t142 = ((t2).select(f64x8::splat(0.0), t7 * t80 * t74 / f64x8::splat(10.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t137));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t142 + f64x8::splat(2.0) * t78;
            acc_vrho = tvrho0;
            let t145 = t91 * v_rho;
            let t147 = f64x8::splat(1.0) / t21 / t145;
            let t148 = t31 * t147;
            let t149 = t40 * v_sigma;
            let t153 = t29 * t32;
            let t157 = f64x8::splat(1.0) / v_sigma;
            let t160 = t58 * param_expo * t157 / f64x8::splat(2.0);
            let t161 = -t88 * t148 * t149 / f64x8::splat(288.0) + t43 * t153 * t36 / f64x8::splat(24.0) - t160;
            let t164 = t49 / t50;
            let t167 = t120 * t32;
            let t169 = t36 * param_a * t131;
            let t172 = t164 * t68 / f64x8::splat(24.0) + t167 * t169 / f64x8::splat(4.0) + t160;
            let t174 = -t113 * t172 + t161 * t72;
            let t178 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t174));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t178;
            acc_vsigma = tvsigma0;
            let t181 = t20 * t53;
            let t188 = t91 * t99;
            let t190 = f64x8::splat(1.0) / t21 / t188;
            let t191 = t190 * t40;
            let t195 = param_alpha * param_alpha;
            let t196 = param_d * t195;
            let t197 = t26 * t26;
            let t198 = f64x8::splat(1.0) / t197;
            let t199 = t196 * t198;
            let t200 = t89 * v_sigma;
            let t201 = t91 * t91;
            let t202 = t201 * t34;
            let t203 = f64x8::splat(1.0) / t202;
            let t209 = f64x8::splat(1.0) / t22 / t91;
            let t213 = param_expo * param_expo;
            let t214 = f64x8::splat(1.0) / t34;
            let t215 = t213 * t214;
            let t217 = f64x8::splat(16.0) / f64x8::splat(9.0) * t58 * t215;
            let t220 = f64x8::splat(4.0) / f64x8::splat(3.0) * t58 * param_expo * t214;
            let t221 = -t88 * t90 * t191 / f64x8::splat(12.0) + t199 * t200 * t203 * t40 / f64x8::splat(81.0) + f64x8::splat(11.0) / f64x8::splat(27.0) * t44 * t33 * t209 - t217 - t220;
            let t223 = t109 * t112;
            let t227 = f64x8::splat(1.0) / t111 / t71;
            let t228 = t59 * t227;
            let t229 = t135 * t135;
            let t233 = f64x8::splat(1.0) / t21 / t99;
            let t235 = t31 * t233 * t67;
            let t239 = t209 * param_a * t131;
            let t242 = t87 * t90;
            let t244 = t123 * param_b;
            let t246 = f64x8::splat(1.0) / t129 / t128;
            let t247 = t244 * t246;
            let t248 = t190 * param_a * t247;
            let t251 = f64x8::splat(7.0) / f64x8::splat(27.0) * t60 * t235 + f64x8::splat(10.0) / f64x8::splat(3.0) * t121 * t239 - f64x8::splat(32.0) / f64x8::splat(3.0) * t242 * t248 + t217 + t220;
            let t253 = -t113 * t251 - f64x8::splat(2.0) * t135 * t223 + t221 * t72 + f64x8::splat(2.0) * t228 * t229;
            let t258 = ((t2).select(f64x8::splat(0.0), -t7 * t181 * t74 / f64x8::splat(30.0) + t7 * t80 * t137 / f64x8::splat(5.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t253));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t258 + f64x8::splat(4.0) * t142;
            acc_v2rho2 = tv2rho20;
            let t264 = t31 * t94;
            let t268 = t201 * v_rho;
            let t269 = f64x8::splat(1.0) / t268;
            let t277 = t213 * t105;
            let t280 = f64x8::splat(2.0) / f64x8::splat(3.0) * t58 * t277 * t157;
            let t281 = t88 * t264 * t149 / f64x8::splat(36.0) - t199 * t269 * t89 * t40 / f64x8::splat(216.0) - t43 * t153 * t101 / f64x8::splat(9.0) + t280;
            let t283 = t161 * t112;
            let t286 = t172 * t135;
            let t295 = param_a * t244 * t246 * v_sigma;
            let t298 = -t164 * t117 / f64x8::splat(18.0) - t167 * t132 + f64x8::splat(4.0) * t87 * t264 * t295 - t280;
            let t300 = -t113 * t298 - t135 * t283 - t172 * t223 + f64x8::splat(2.0) * t228 * t286 + t281 * t72;
            let t305 = ((t2).select(f64x8::splat(0.0), t7 * t80 * t174 / f64x8::splat(10.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t300));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t305 + f64x8::splat(2.0) * t178;
            acc_v2rhosigma = tv2rhosigma0;
            let t308 = f64x8::splat(1.0) / t201;
            let t313 = t84 * t47;
            let t314 = t86 * t31;
            let t319 = f64x8::splat(1.0) / t89;
            let t322 = t58 * t213 * t319 / f64x8::splat(4.0);
            let t325 = t58 * param_expo * t319 / f64x8::splat(2.0);
            let t326 = t199 * t308 * t40 * v_sigma / f64x8::splat(576.0) - t313 * t314 * t147 * t40 / f64x8::splat(144.0) - t322 + t325;
            let t330 = t172 * t172;
            let t335 = t49 / t50 / v_sigma;
            let t339 = t120 * t157 * t32;
            let t342 = t87 * t31;
            let t344 = t147 * param_a * t247;
            let t347 = -t335 * t68 / f64x8::splat(48.0) + t339 * t169 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(2.0) * t342 * t344 + t322 - t325;
            let t349 = -t113 * t347 - f64x8::splat(2.0) * t172 * t283 + f64x8::splat(2.0) * t228 * t330 + t326 * t72;
            let t353 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t349));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t353;
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

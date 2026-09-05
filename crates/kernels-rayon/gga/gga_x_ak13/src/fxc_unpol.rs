//! GGA_X_AK13 fxc unpol kernel — explicit SIMD (bit-exact).
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

/// Accumulate 8 consecutive grid points into an output array.
///
/// `+=`, not `=`. The scalar kernel writes `out[ip] += v`; a plain store is a
/// different operation in two ways. It keeps the sign of a negative zero where
/// `0.0 + -0.0` gives `+0.0` -- a bit difference the fingerprint gate reports
/// as a rejection even though no value changed (`gga_x_pbepow fxc` was
/// rejected on exactly this, 273 of 200,000 `v2sigma2` elements) -- and it
/// would discard whatever a caller had already put in the buffer.
#[inline(always)]
fn store_add(s: &mut [f64], ip: usize, m: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        let r: [f64; 8] = (f64x8::new(b) + acc).into();
        s[ip..ip + 8].copy_from_slice(&r);
    } else {
        for k in 0..m {
            s[ip + k] += a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_ak13_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
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
            let t4 = f64x8::splat(M_CBRTPI);
            let t6 = t3 / t4;
            let t7 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t8 = zeta_threshold - f64x8::splat(1.0);
            let t10 = ((t7).select(t8, (t7).select(-t8, f64x8::splat(0.0))));
            let t11 = f64x8::splat(1.0) + t10;
            let t13 = (simd::cbrt(zeta_threshold));
            let t15 = (simd::cbrt(t11));
            let t17 = (((t11).simd_le(zeta_threshold)).select(t13 * zeta_threshold, t15 * t11));
            let t18 = (simd::cbrt(v_rho));
            let t19 = t17 * t18;
            let t20 = f64x8::splat(M_CBRT6);
            let t21 = t20 * t20;
            let t23 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t24 = (simd::cbrt(t23));
            let t25 = f64x8::splat(1.0) / t24;
            let t26 = param_B1 * t21 * t25;
            let t27 = ((v_sigma).sqrt());
            let t28 = f64x8::splat(M_CBRT2);
            let t29 = t27 * t28;
            let t31 = f64x8::splat(1.0) / t18 / v_rho;
            let t32 = t21 * t25;
            let t36 = f64x8::splat(1.0) + t32 * t29 * t31 / f64x8::splat(12.0);
            let t37 = (simd::ln(t36));
            let t38 = t31 * t37;
            let t43 = param_B2 * t21 * t25;
            let t44 = f64x8::splat(1.0) + t37;
            let t45 = (simd::ln(t44));
            let t46 = t31 * t45;
            let t50 = f64x8::splat(1.0) + t26 * t29 * t38 / f64x8::splat(12.0) + t43 * t29 * t46 / f64x8::splat(12.0);
            let t54 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t50));
            let tzk0 = f64x8::splat(2.0) * t54;
            acc_zk = tzk0;
            let t55 = t18 * t18;
            let t57 = t17 / t55;
            let t61 = v_rho * v_rho;
            let t63 = f64x8::splat(1.0) / t18 / t61;
            let t64 = t63 * t37;
            let t69 = t24 * t24;
            let t70 = f64x8::splat(1.0) / t69;
            let t71 = param_B1 * t20 * t70;
            let t72 = t28 * t28;
            let t73 = v_sigma * t72;
            let t74 = t61 * v_rho;
            let t76 = f64x8::splat(1.0) / t55 / t74;
            let t77 = f64x8::splat(1.0) / t36;
            let t78 = t76 * t77;
            let t82 = t63 * t45;
            let t86 = param_B2 * t20;
            let t88 = t86 * t70 * v_sigma;
            let t89 = t72 * t76;
            let t90 = f64x8::splat(1.0) / t44;
            let t91 = t77 * t90;
            let t92 = t89 * t91;
            let t95 = -t26 * t29 * t64 / f64x8::splat(9.0) - t71 * t73 * t78 / f64x8::splat(18.0) - t43 * t29 * t82 / f64x8::splat(9.0) - t88 * t92 / f64x8::splat(18.0);
            let t100 = ((t2).select(f64x8::splat(0.0), -t6 * t57 * t50 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t95));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t100 + f64x8::splat(2.0) * t54;
            acc_vrho = tvrho0;
            let t103 = f64x8::splat(1.0) / t27;
            let t104 = t103 * t28;
            let t109 = f64x8::splat(1.0) / t55 / t61;
            let t110 = t72 * t109;
            let t117 = t86 * t70;
            let t118 = t110 * t91;
            let t121 = t26 * t104 * t38 / f64x8::splat(24.0) + t71 * t110 * t77 / f64x8::splat(48.0) + t43 * t104 * t46 / f64x8::splat(24.0) + t117 * t118 / f64x8::splat(48.0);
            let t125 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t121));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t125;
            acc_vsigma = tvsigma0;
            let t130 = t17 / t55 / v_rho;
            let t138 = f64x8::splat(1.0) / t18 / t74;
            let t139 = t138 * t37;
            let t143 = t61 * t61;
            let t145 = f64x8::splat(1.0) / t55 / t143;
            let t146 = t145 * t77;
            let t150 = f64x8::splat(1.0) / t23;
            let t151 = param_B1 * t150;
            let t152 = t27 * v_sigma;
            let t153 = t143 * t61;
            let t154 = f64x8::splat(1.0) / t153;
            let t156 = t36 * t36;
            let t157 = f64x8::splat(1.0) / t156;
            let t161 = t138 * t45;
            let t165 = t72 * t145;
            let t166 = t165 * t91;
            let t169 = param_B2 * t150;
            let t170 = t169 * t152;
            let t171 = t154 * t157;
            let t172 = t171 * t90;
            let t175 = t44 * t44;
            let t176 = f64x8::splat(1.0) / t175;
            let t177 = t171 * t176;
            let t180 = f64x8::splat(7.0) / f64x8::splat(27.0) * t26 * t29 * t139 + f64x8::splat(5.0) / f64x8::splat(18.0) * t71 * t73 * t146 - f64x8::splat(2.0) / f64x8::splat(27.0) * t151 * t152 * t154 * t157 + f64x8::splat(7.0) / f64x8::splat(27.0) * t43 * t29 * t161 + f64x8::splat(5.0) / f64x8::splat(18.0) * t88 * t166 - f64x8::splat(2.0) / f64x8::splat(27.0) * t170 * t172 - f64x8::splat(2.0) / f64x8::splat(27.0) * t170 * t177;
            let t185 = ((t2).select(f64x8::splat(0.0), t6 * t130 * t50 / f64x8::splat(12.0) - t6 * t57 * t95 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t180));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t185 + f64x8::splat(4.0) * t100;
            acc_v2rho2 = tv2rho20;
            let t197 = t143 * v_rho;
            let t198 = f64x8::splat(1.0) / t197;
            let t199 = t198 * t157;
            let t208 = t169 * t198;
            let t209 = t157 * t90;
            let t210 = t209 * t27;
            let t213 = t157 * t176;
            let t214 = t213 * t27;
            let t217 = -t26 * t104 * t64 / f64x8::splat(18.0) - t71 * t89 * t77 / f64x8::splat(12.0) + t151 * t199 * t27 / f64x8::splat(36.0) - t43 * t104 * t82 / f64x8::splat(18.0) - t117 * t92 / f64x8::splat(12.0) + t208 * t210 / f64x8::splat(36.0) + t208 * t214 / f64x8::splat(36.0);
            let t222 = ((t2).select(f64x8::splat(0.0), -t6 * t57 * t121 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t217));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t222 + f64x8::splat(2.0) * t125;
            acc_v2rhosigma = tv2rhosigma0;
            let t225 = f64x8::splat(1.0) / t152;
            let t226 = t225 * t28;
            let t230 = f64x8::splat(1.0) / v_sigma;
            let t231 = t230 * t72;
            let t232 = t109 * t77;
            let t236 = f64x8::splat(1.0) / t143;
            let t237 = t236 * t157;
            let t245 = t86 * t70 * t230;
            let t248 = t169 * t236;
            let t255 = -t26 * t226 * t38 / f64x8::splat(48.0) + t71 * t231 * t232 / f64x8::splat(96.0) - t151 * t237 * t103 / f64x8::splat(96.0) - t43 * t226 * t46 / f64x8::splat(48.0) + t245 * t118 / f64x8::splat(96.0) - t248 * t209 * t103 / f64x8::splat(96.0) - t248 * t213 * t103 / f64x8::splat(96.0);
            let t259 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t255));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t259;
            acc_v2sigma2 = tv2sigma20;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        store_add(v2rho2, ip, m, acc_v2rho2);
        store_add(v2rhosigma, ip, m, acc_v2rhosigma);
        store_add(v2sigma2, ip, m, acc_v2sigma2);
        ip += 8;
    }
}

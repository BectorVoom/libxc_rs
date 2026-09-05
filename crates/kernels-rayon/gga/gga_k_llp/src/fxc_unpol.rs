//! GGA_K_LLP fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_llp.c`
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
pub fn gga_k_llp_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    param_beta: f64,
    param_gamma: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_beta = f64x8::splat(param_beta);
    let param_gamma = f64x8::splat(param_gamma);
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
            let t24 = param_beta * t4;
            let t26 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t27 = f64x8::splat(1.0) / t26;
            let t28 = f64x8::splat(M_CBRT4);
            let t29 = t27 * t28;
            let t30 = t24 * t29;
            let t31 = f64x8::splat(M_CBRT2);
            let t32 = t31 * t31;
            let t33 = v_sigma * t32;
            let t34 = v_rho * v_rho;
            let t36 = f64x8::splat(1.0) / t22 / t34;
            let t37 = param_gamma * param_beta;
            let t38 = ((v_sigma).sqrt());
            let t39 = t37 * t38;
            let t41 = f64x8::splat(1.0) / t21 / v_rho;
            let t45 = (simd::ln(t38 * t31 * t41 + ((((t38 * t31 * t41) * (t38 * t31 * t41)) + f64x8::splat(1.0)).sqrt())));
            let t46 = t31 * t41 * t45;
            let t48 = f64x8::splat(1.0) + t39 * t46;
            let t49 = f64x8::splat(1.0) / t48;
            let t50 = t36 * t49;
            let t54 = f64x8::splat(1.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t30 * t33 * t50;
            let t58 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t54));
            let tzk0 = f64x8::splat(2.0) * t58;
            acc_zk = tzk0;
            let t60 = t20 / t21;
            let t64 = t34 * v_rho;
            let t66 = f64x8::splat(1.0) / t22 / t64;
            let t67 = t66 * t49;
            let t71 = t48 * t48;
            let t72 = f64x8::splat(1.0) / t71;
            let t73 = t36 * t72;
            let t75 = f64x8::splat(1.0) / t21 / t34;
            let t77 = t31 * t75 * t45;
            let t79 = t37 * v_sigma;
            let t80 = t32 * t66;
            let t82 = t33 * t36 + f64x8::splat(1.0);
            let t83 = ((t82).sqrt());
            let t84 = f64x8::splat(1.0) / t83;
            let t85 = t80 * t84;
            let t88 = -f64x8::splat(4.0) / f64x8::splat(3.0) * t39 * t77 - f64x8::splat(4.0) / f64x8::splat(3.0) * t79 * t85;
            let t93 = -f64x8::splat(16.0) / f64x8::splat(27.0) * t30 * t33 * t67 - f64x8::splat(2.0) / f64x8::splat(9.0) * t30 * t33 * t73 * t88;
            let t98 = ((t2).select(f64x8::splat(0.0), t7 * t60 * t54 / f64x8::splat(10.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t93));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t98 + f64x8::splat(2.0) * t58;
            acc_vrho = tvrho0;
            let t101 = t24 * t27;
            let t102 = t28 * t32;
            let t106 = t37 / t38;
            let t108 = t32 * t36;
            let t109 = t108 * t84;
            let t112 = t106 * t46 / f64x8::splat(2.0) + t37 * t109 / f64x8::splat(2.0);
            let t117 = -f64x8::splat(2.0) / f64x8::splat(9.0) * t30 * t33 * t73 * t112 + f64x8::splat(2.0) / f64x8::splat(9.0) * t101 * t102 * t50;
            let t121 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t117));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t121;
            acc_vsigma = tvsigma0;
            let t124 = t20 * t41;
            let t131 = t34 * t34;
            let t133 = f64x8::splat(1.0) / t22 / t131;
            let t134 = t133 * t49;
            let t138 = t66 * t72;
            let t144 = f64x8::splat(1.0) / t71 / t48;
            let t145 = t36 * t144;
            let t146 = t88 * t88;
            let t152 = f64x8::splat(1.0) / t21 / t64;
            let t154 = t31 * t152 * t45;
            let t157 = t32 * t133;
            let t158 = t157 * t84;
            let t161 = v_sigma * v_sigma;
            let t162 = t37 * t161;
            let t165 = f64x8::splat(1.0) / t21 / t131 / t64;
            let t168 = f64x8::splat(1.0) / t83 / t82;
            let t169 = t31 * t165 * t168;
            let t172 = f64x8::splat(28.0) / f64x8::splat(9.0) * t39 * t154 + f64x8::splat(20.0) / f64x8::splat(3.0) * t79 * t158 - f64x8::splat(32.0) / f64x8::splat(9.0) * t162 * t169;
            let t177 = f64x8::splat(176.0) / f64x8::splat(81.0) * t30 * t33 * t134 + f64x8::splat(32.0) / f64x8::splat(27.0) * t30 * t33 * t138 * t88 + f64x8::splat(4.0) / f64x8::splat(9.0) * t30 * t33 * t145 * t146 - f64x8::splat(2.0) / f64x8::splat(9.0) * t30 * t33 * t73 * t172;
            let t182 = ((t2).select(f64x8::splat(0.0), -t7 * t124 * t54 / f64x8::splat(30.0) + t7 * t60 * t93 / f64x8::splat(5.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t177));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t182 + f64x8::splat(4.0) * t98;
            acc_v2rho2 = tv2rho20;
            let t191 = t72 * t88;
            let t200 = t24 * t29 * v_sigma;
            let t201 = t144 * t112;
            let t202 = t201 * t88;
            let t203 = t108 * t202;
            let t210 = t37 * t31;
            let t211 = t131 * t34;
            let t213 = f64x8::splat(1.0) / t21 / t211;
            let t218 = -f64x8::splat(2.0) / f64x8::splat(3.0) * t106 * t77 - f64x8::splat(2.0) * t37 * t85 + f64x8::splat(4.0) / f64x8::splat(3.0) * t210 * t213 * t168 * v_sigma;
            let t223 = -f64x8::splat(16.0) / f64x8::splat(27.0) * t101 * t102 * t67 - f64x8::splat(2.0) / f64x8::splat(9.0) * t30 * t108 * t191 + f64x8::splat(16.0) / f64x8::splat(27.0) * t30 * t33 * t138 * t112 + f64x8::splat(4.0) / f64x8::splat(9.0) * t200 * t203 - f64x8::splat(2.0) / f64x8::splat(9.0) * t30 * t33 * t73 * t218;
            let t228 = ((t2).select(f64x8::splat(0.0), t7 * t60 * t117 / f64x8::splat(10.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t223));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t228 + f64x8::splat(2.0) * t121;
            acc_v2rhosigma = tv2rhosigma0;
            let t231 = t72 * t112;
            let t235 = t112 * t112;
            let t242 = t37 / t38 / v_sigma;
            let t245 = f64x8::splat(1.0) / v_sigma;
            let t246 = t37 * t245;
            let t249 = t131 * v_rho;
            let t252 = t31 / t21 / t249;
            let t253 = t252 * t168;
            let t256 = -t242 * t46 / f64x8::splat(4.0) + t246 * t109 / f64x8::splat(4.0) - t37 * t253 / f64x8::splat(2.0);
            let t261 = -f64x8::splat(4.0) / f64x8::splat(9.0) * t30 * t108 * t231 + f64x8::splat(4.0) / f64x8::splat(9.0) * t30 * t33 * t145 * t235 - f64x8::splat(2.0) / f64x8::splat(9.0) * t30 * t33 * t73 * t256;
            let t265 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t23 * t261));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t265;
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

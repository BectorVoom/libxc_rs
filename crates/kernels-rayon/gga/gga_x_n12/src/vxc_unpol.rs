//! GGA_X_N12 vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_n12.c`
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
pub fn gga_x_n12_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_CC_0_1: f64,
    param_CC_0_2: f64,
    param_CC_0_3: f64,
    param_CC_1_1: f64,
    param_CC_1_2: f64,
    param_CC_1_3: f64,
    param_CC_1_0: f64,
    param_CC_2_1: f64,
    param_CC_2_2: f64,
    param_CC_2_3: f64,
    param_CC_2_0: f64,
    param_CC_3_1: f64,
    param_CC_3_2: f64,
    param_CC_3_3: f64,
    param_CC_3_0: f64,
    param_CC_0_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_CC_0_1 = f64x8::splat(param_CC_0_1);
    let param_CC_0_2 = f64x8::splat(param_CC_0_2);
    let param_CC_0_3 = f64x8::splat(param_CC_0_3);
    let param_CC_1_1 = f64x8::splat(param_CC_1_1);
    let param_CC_1_2 = f64x8::splat(param_CC_1_2);
    let param_CC_1_3 = f64x8::splat(param_CC_1_3);
    let param_CC_1_0 = f64x8::splat(param_CC_1_0);
    let param_CC_2_1 = f64x8::splat(param_CC_2_1);
    let param_CC_2_2 = f64x8::splat(param_CC_2_2);
    let param_CC_2_3 = f64x8::splat(param_CC_2_3);
    let param_CC_2_0 = f64x8::splat(param_CC_2_0);
    let param_CC_3_1 = f64x8::splat(param_CC_3_1);
    let param_CC_3_2 = f64x8::splat(param_CC_3_2);
    let param_CC_3_3 = f64x8::splat(param_CC_3_3);
    let param_CC_3_0 = f64x8::splat(param_CC_3_0);
    let param_CC_0_0 = f64x8::splat(param_CC_0_0);
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
        {
            let t2 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = f64x8::splat(M_CBRTPI);
            let t6 = t3 / t4;
            let t7 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t8 = zeta_threshold - f64x8::splat(1.0);
            let t10 = ((t7).select(t8, (t7).select(-t8, f64x8::splat(0.0))));
            let t11 = f64x8::splat(1.0) + t10;
            let t12 = (t11).simd_le(zeta_threshold);
            let t13 = (simd::cbrt(zeta_threshold));
            let t15 = (simd::cbrt(t11));
            let t17 = ((t12).select(t13 * zeta_threshold, t15 * t11));
            let t18 = (simd::cbrt(v_rho));
            let t19 = t17 * t18;
            let t21 = param_CC_0_1;
            let t22 = t21 * v_sigma;
            let t23 = f64x8::splat(M_CBRT2);
            let t24 = t23 * t23;
            let t25 = v_rho * v_rho;
            let t26 = t18 * t18;
            let t28 = f64x8::splat(1.0) / t26 / t25;
            let t29 = t24 * t28;
            let t33 = f64x8::splat(1.0) + f64x8::splat(0.004) * v_sigma * t24 * t28;
            let t34 = f64x8::splat(1.0) / t33;
            let t35 = t29 * t34;
            let t38 = param_CC_0_2;
            let t39 = v_sigma * v_sigma;
            let t40 = t38 * t39;
            let t41 = t25 * t25;
            let t42 = t41 * v_rho;
            let t44 = f64x8::splat(1.0) / t18 / t42;
            let t46 = t33 * t33;
            let t47 = f64x8::splat(1.0) / t46;
            let t48 = t23 * t44 * t47;
            let t51 = param_CC_0_3;
            let t52 = t39 * v_sigma;
            let t53 = t51 * t52;
            let t54 = t41 * t41;
            let t55 = f64x8::splat(1.0) / t54;
            let t56 = t46 * t33;
            let t57 = f64x8::splat(1.0) / t56;
            let t58 = t55 * t57;
            let t62 = param_CC_1_1;
            let t63 = t62 * v_sigma;
            let t66 = param_CC_1_2;
            let t67 = t66 * t39;
            let t70 = param_CC_1_3;
            let t71 = t70 * t52;
            let t74 = param_CC_1_0 + f64x8::splat(0.004) * t63 * t35 + f64x8::splat(3.2e-05) * t67 * t48 + f64x8::splat(2.56e-07) * t71 * t58;
            let t79 = ((t12).select(f64x8::splat(1.0) / t13, f64x8::splat(1.0) / t15));
            let t82 = f64x8::splat(1.0) + f64x8::splat(0.4) / t18 * t23 * t79;
            let t83 = f64x8::splat(1.0) / t82;
            let t86 = param_CC_2_1;
            let t87 = t86 * v_sigma;
            let t90 = param_CC_2_2;
            let t91 = t90 * t39;
            let t94 = param_CC_2_3;
            let t95 = t94 * t52;
            let t98 = param_CC_2_0 + f64x8::splat(0.004) * t87 * t35 + f64x8::splat(3.2e-05) * t91 * t48 + f64x8::splat(2.56e-07) * t95 * t58;
            let t99 = t82 * t82;
            let t100 = f64x8::splat(1.0) / t99;
            let t103 = param_CC_3_1;
            let t104 = t103 * v_sigma;
            let t107 = param_CC_3_2;
            let t108 = t107 * t39;
            let t111 = param_CC_3_3;
            let t112 = t111 * t52;
            let t115 = param_CC_3_0 + f64x8::splat(0.004) * t104 * t35 + f64x8::splat(3.2e-05) * t108 * t48 + f64x8::splat(2.56e-07) * t112 * t58;
            let t116 = t99 * t82;
            let t117 = f64x8::splat(1.0) / t116;
            let t119 = param_CC_0_0 + f64x8::splat(0.004) * t22 * t35 + f64x8::splat(3.2e-05) * t40 * t48 + f64x8::splat(2.56e-07) * t53 * t58 + t74 * t83 + t98 * t100 + t115 * t117;
            let t123 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t119));
            let tzk0 = f64x8::splat(2.0) * t123;
            acc_zk = tzk0;
            let t125 = t17 / t26;
            let t129 = t25 * v_rho;
            let t131 = f64x8::splat(1.0) / t26 / t129;
            let t132 = t24 * t131;
            let t133 = t132 * t34;
            let t136 = t21 * t39;
            let t137 = t41 * t25;
            let t139 = f64x8::splat(1.0) / t18 / t137;
            let t141 = t23 * t139 * t47;
            let t146 = t38 * t52;
            let t147 = t54 * v_rho;
            let t148 = f64x8::splat(1.0) / t147;
            let t149 = t148 * t57;
            let t154 = t39 * t39;
            let t155 = t51 * t154;
            let t156 = t54 * t129;
            let t158 = f64x8::splat(1.0) / t26 / t156;
            let t159 = t46 * t46;
            let t160 = f64x8::splat(1.0) / t159;
            let t162 = t158 * t160 * t24;
            let t167 = t62 * t39;
            let t172 = t66 * t52;
            let t177 = t70 * t154;
            let t180 = -f64x8::splat(0.010666666666666666) * t63 * t133 + f64x8::splat(8.533333333333334e-05) * t167 * t141 - f64x8::splat(0.00017066666666666668) * t67 * t141 + f64x8::splat(1.3653333333333333e-06) * t172 * t149 - f64x8::splat(2.048e-06) * t71 * t149 + f64x8::splat(8.192e-09) * t177 * t162;
            let t182 = t74 * t100;
            let t186 = f64x8::splat(1.0) / t18 / v_rho * t23 * t79;
            let t191 = t86 * t39;
            let t196 = t90 * t52;
            let t201 = t94 * t154;
            let t204 = -f64x8::splat(0.010666666666666666) * t87 * t133 + f64x8::splat(8.533333333333334e-05) * t191 * t141 - f64x8::splat(0.00017066666666666668) * t91 * t141 + f64x8::splat(1.3653333333333333e-06) * t196 * t149 - f64x8::splat(2.048e-06) * t95 * t149 + f64x8::splat(8.192e-09) * t201 * t162;
            let t206 = t98 * t117;
            let t211 = t103 * t39;
            let t216 = t107 * t52;
            let t221 = t111 * t154;
            let t224 = -f64x8::splat(0.010666666666666666) * t104 * t133 + f64x8::splat(8.533333333333334e-05) * t211 * t141 - f64x8::splat(0.00017066666666666668) * t108 * t141 + f64x8::splat(1.3653333333333333e-06) * t216 * t149 - f64x8::splat(2.048e-06) * t112 * t149 + f64x8::splat(8.192e-09) * t221 * t162;
            let t226 = t99 * t99;
            let t227 = f64x8::splat(1.0) / t226;
            let t228 = t115 * t227;
            let t231 = -f64x8::splat(0.010666666666666666) * t22 * t133 + f64x8::splat(8.533333333333334e-05) * t136 * t141 - f64x8::splat(0.00017066666666666668) * t40 * t141 + f64x8::splat(1.3653333333333333e-06) * t146 * t149 - f64x8::splat(2.048e-06) * t53 * t149 + f64x8::splat(8.192e-09) * t155 * t162 + t180 * t83 + f64x8::splat(0.13333333333333333) * t182 * t186 + t204 * t100 + f64x8::splat(0.26666666666666666) * t206 * t186 + t224 * t117 + f64x8::splat(0.4) * t228 * t186;
            let t236 = ((t2).select(f64x8::splat(0.0), -t6 * t125 * t119 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t231));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t236 + f64x8::splat(2.0) * t123;
            acc_vrho = tvrho0;
            let t239 = t21 * t24;
            let t240 = t28 * t34;
            let t245 = t38 * v_sigma;
            let t250 = t51 * t39;
            let t253 = t54 * t25;
            let t255 = f64x8::splat(1.0) / t26 / t253;
            let t257 = t255 * t160 * t24;
            let t260 = t62 * t24;
            let t265 = t66 * v_sigma;
            let t270 = t70 * t39;
            let t275 = f64x8::splat(0.004) * t260 * t240 - f64x8::splat(3.2e-05) * t63 * t48 + f64x8::splat(6.4e-05) * t265 * t48 - f64x8::splat(5.12e-07) * t67 * t58 + f64x8::splat(7.68e-07) * t270 * t58 - f64x8::splat(3.072e-09) * t71 * t257;
            let t277 = t86 * t24;
            let t282 = t90 * v_sigma;
            let t287 = t94 * t39;
            let t292 = f64x8::splat(0.004) * t277 * t240 - f64x8::splat(3.2e-05) * t87 * t48 + f64x8::splat(6.4e-05) * t282 * t48 - f64x8::splat(5.12e-07) * t91 * t58 + f64x8::splat(7.68e-07) * t287 * t58 - f64x8::splat(3.072e-09) * t95 * t257;
            let t294 = t103 * t24;
            let t299 = t107 * v_sigma;
            let t304 = t111 * t39;
            let t309 = f64x8::splat(0.004) * t294 * t240 - f64x8::splat(3.2e-05) * t104 * t48 + f64x8::splat(6.4e-05) * t299 * t48 - f64x8::splat(5.12e-07) * t108 * t58 + f64x8::splat(7.68e-07) * t304 * t58 - f64x8::splat(3.072e-09) * t112 * t257;
            let t311 = f64x8::splat(0.004) * t239 * t240 - f64x8::splat(3.2e-05) * t22 * t48 + f64x8::splat(6.4e-05) * t245 * t48 - f64x8::splat(5.12e-07) * t40 * t58 + f64x8::splat(7.68e-07) * t250 * t58 - f64x8::splat(3.072e-09) * t53 * t257 + t275 * t83 + t292 * t100 + t309 * t117;
            let t315 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t311));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t315;
            acc_vsigma = tvsigma0;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        ip += 8;
    }
}

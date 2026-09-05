//! GGA_C_LYPR vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_lypr.c`
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
pub fn gga_c_lypr_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_m1: f64,
    param_omega: f64,
    param_d: f64,
    param_m2: f64,
    param_b: f64,
    param_c: f64,
    param_a: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_m1 = f64x8::splat(param_m1);
    let param_omega = f64x8::splat(param_omega);
    let param_d = f64x8::splat(param_d);
    let param_m2 = f64x8::splat(param_m2);
    let param_b = f64x8::splat(param_b);
    let param_c = f64x8::splat(param_c);
    let param_a = f64x8::splat(param_a);
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
            let t2 = (simd::cbrt(v_rho));
            let t3 = f64x8::splat(1.0) / t2;
            let t5 = (simd::erfc(param_m1 * param_omega * t3));
            let t7 = param_d * t3 + f64x8::splat(1.0);
            let t8 = f64x8::splat(1.0) / t7;
            let t10 = param_m2 * param_omega;
            let t12 = (simd::erfc(t10 * t3));
            let t13 = t12 * param_b;
            let t15 = (simd::exp(-param_c * t3));
            let t16 = t15 * t8;
            let t17 = v_rho * v_rho;
            let t18 = t2 * t2;
            let t20 = f64x8::splat(1.0) / t18 / t17;
            let t21 = v_sigma * t20;
            let t23 = param_d * t8 + param_c;
            let t24 = t23 * t3;
            let t26 = -f64x8::splat(1.0) / f64x8::splat(72.0) - f64x8::splat(7.0) / f64x8::splat(72.0) * t24;
            let t28 = f64x8::splat(M_CBRT3);
            let t29 = t28 * t28;
            let t30 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t31 = (simd::cbrt(t30));
            let t32 = t31 * t31;
            let t34 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t35 = zeta_threshold * zeta_threshold;
            let t36 = (simd::cbrt(zeta_threshold));
            let t37 = t36 * t36;
            let t39 = ((t34).select(t37 * t35, f64x8::splat(1.0)));
            let t43 = f64x8::splat(5.0) / f64x8::splat(2.0) - t24 / f64x8::splat(18.0);
            let t44 = t43 * v_sigma;
            let t45 = t20 * t39;
            let t48 = t24 - f64x8::splat(11.0);
            let t49 = t48 * v_sigma;
            let t52 = ((t34).select(t37 * t35 * zeta_threshold, f64x8::splat(1.0)));
            let t53 = t20 * t52;
            let t56 = f64x8::splat(M_CBRT2);
            let t57 = t56 * t56;
            let t58 = v_sigma * t57;
            let t61 = ((t34).select(t35, f64x8::splat(1.0)));
            let t62 = t61 * v_sigma;
            let t64 = t57 * t20 * t39;
            let t70 = -t21 * t26 - f64x8::splat(3.0) / f64x8::splat(10.0) * t29 * t32 * t39 + t44 * t45 / f64x8::splat(8.0) + t49 * t53 / f64x8::splat(144.0) - t56 * (f64x8::splat(4.0) / f64x8::splat(3.0) * t58 * t45 - t62 * t64 / f64x8::splat(2.0)) / f64x8::splat(8.0);
            let t71 = t16 * t70;
            let t73 = param_b * t15;
            let t74 = ((f64x8::splat(M_PI)).sqrt());
            let t75 = f64x8::splat(1.0) / t74;
            let t76 = t8 * t75;
            let t77 = t73 * t76;
            let t78 = param_m2 * param_m2;
            let t79 = param_omega * param_omega;
            let t81 = f64x8::splat(1.0) / t18;
            let t83 = (simd::exp(-t78 * t79 * t81));
            let t84 = t17 * v_rho;
            let t85 = f64x8::splat(1.0) / t84;
            let t86 = t83 * t85;
            let tzk0 = param_a * (-t5 * t8 + t13 * t71 + f64x8::splat(7.0) / f64x8::splat(36.0) * t77 * t10 * t86 * v_sigma);
            acc_zk = tzk0;
            let t92 = v_rho * param_a;
            let t93 = param_m1 * param_m1;
            let t96 = (simd::exp(-t93 * t79 * t81));
            let t98 = t75 * t96 * param_m1;
            let t100 = f64x8::splat(1.0) / t2 / v_rho;
            let t105 = t7 * t7;
            let t106 = f64x8::splat(1.0) / t105;
            let t107 = t5 * t106;
            let t108 = param_d * t100;
            let t111 = t75 * t83;
            let t112 = t111 * t10;
            let t113 = t100 * param_b;
            let t117 = t13 * param_c;
            let t118 = t100 * t15;
            let t119 = t8 * t70;
            let t123 = t13 * t15;
            let t124 = t106 * t70;
            let t129 = f64x8::splat(1.0) / t18 / t84;
            let t130 = v_sigma * t129;
            let t133 = param_d * param_d;
            let t134 = t133 * t106;
            let t139 = -t134 / t18 / v_rho + t23 * t100;
            let t140 = f64x8::splat(7.0) / f64x8::splat(216.0) * t139;
            let t142 = t139 / f64x8::splat(54.0);
            let t143 = t142 * v_sigma;
            let t146 = t129 * t39;
            let t150 = -t139 / f64x8::splat(3.0);
            let t151 = t150 * v_sigma;
            let t154 = t129 * t52;
            let t160 = t57 * t129 * t39;
            let t166 = f64x8::splat(8.0) / f64x8::splat(3.0) * t130 * t26 - t21 * t140 + t143 * t45 / f64x8::splat(8.0) - t44 * t146 / f64x8::splat(3.0) + t151 * t53 / f64x8::splat(144.0) - t49 * t154 / f64x8::splat(54.0) - t56 * (-f64x8::splat(32.0) / f64x8::splat(9.0) * t58 * t146 + f64x8::splat(4.0) / f64x8::splat(3.0) * t62 * t160) / f64x8::splat(8.0);
            let t167 = t16 * t166;
            let t169 = param_b * param_c;
            let t170 = t17 * t17;
            let t172 = f64x8::splat(1.0) / t2 / t170;
            let t173 = t172 * t15;
            let t176 = t75 * param_m2;
            let t177 = param_omega * t83;
            let t179 = t176 * t177 * v_sigma;
            let t182 = t106 * t75;
            let t184 = t73 * t182 * param_m2;
            let t190 = t78 * param_m2;
            let t191 = t79 * param_omega;
            let t192 = t190 * t191;
            let t194 = f64x8::splat(1.0) / t18 / t170;
            let t195 = t194 * t83;
            let t200 = f64x8::splat(1.0) / t170;
            let t201 = t83 * t200;
            let t206 = -f64x8::splat(2.0) / f64x8::splat(3.0) * t98 * param_omega * t100 * t8 - t107 * t108 / f64x8::splat(3.0) + f64x8::splat(2.0) / f64x8::splat(3.0) * t112 * t113 * t71 + t117 * t118 * t119 / f64x8::splat(3.0) + t123 * t124 * t108 / f64x8::splat(3.0) + t13 * t167 + f64x8::splat(7.0) / f64x8::splat(108.0) * t169 * t173 * t8 * t179 + f64x8::splat(7.0) / f64x8::splat(108.0) * t184 * t177 * t172 * v_sigma * param_d + f64x8::splat(7.0) / f64x8::splat(54.0) * t77 * t192 * t195 * v_sigma - f64x8::splat(7.0) / f64x8::splat(12.0) * t77 * t10 * t201 * v_sigma;
            let tvrho0 = t92 * t206 + tzk0;
            acc_vrho = tvrho0;
            let t216 = t61 * t57;
            let t222 = -t20 * t26 + t43 * t20 * t39 / f64x8::splat(8.0) + t48 * t20 * t52 / f64x8::splat(144.0) - t56 * (f64x8::splat(4.0) / f64x8::splat(3.0) * t64 - t216 * t45 / f64x8::splat(2.0)) / f64x8::splat(8.0);
            let t223 = t16 * t222;
            let t228 = t13 * t223 + f64x8::splat(7.0) / f64x8::splat(36.0) * t77 * t10 * t86;
            let tvsigma0 = t92 * t228;
            acc_vsigma = tvsigma0;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        ip += 8;
    }
}

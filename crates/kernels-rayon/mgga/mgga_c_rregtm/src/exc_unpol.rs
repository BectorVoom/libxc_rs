//! MGGA_C_RREGTM exc unpol kernel — explicit SIMD (exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_rregtm.c`
//! by tools/translate_rayon/from_maple.py, then rewritten to
//! `wide::f64x8` by simd.py (exact math). Eight grid points per step; every lane runs maple2c's expression
//! sequence in its original order.
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]
use libxc_rkernel_math::constants::*;
use libxc_rkernel_math::simd;
use libxc_rkernel_math::wide::{f64x8, CmpEq, CmpGe, CmpGt, CmpLe, CmpLt, CmpNe};

const V_ZERO: f64x8 = f64x8::new([0.0; 8]);
const V_ONE: f64x8 = f64x8::new([1.0; 8]);

// `exp`, `ln` and the cube-root family come from `libxc_rkernel_math::simd`,
// which is bit-identical per lane to the scalar calls the scalar kernel makes
// (exp/ln to glibc's `_fma` ifuncs, cbrt to `powers::cbrt_f64`). Only
// `atan`/`tanh`-class calls still use `wide`'s ~1 ulp forms; a kernel with
// none of those produces output bit-identical to its scalar form.

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
pub fn mgga_c_rregtm_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
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
        {
            let t2 = f64x8::splat(M_CBRT3);
            let t3 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t4 = (simd::cbrt(t3));
            let t5 = t2 * t4;
            let t6 = f64x8::splat(M_CBRT4);
            let t7 = t6 * t6;
            let t8 = (simd::cbrt(v_rho));
            let t11 = t5 * t7 / t8;
            let t13 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t11;
            let t14 = ((t11).sqrt());
            let t17 = ((t11) * (t11).sqrt());
            let t19 = t2 * t2;
            let t20 = t4 * t4;
            let t21 = t19 * t20;
            let t22 = t8 * t8;
            let t25 = t21 * t6 / t22;
            let t27 = f64x8::splat(3.79785) * t14 + f64x8::splat(0.8969) * t11 + f64x8::splat(0.204775) * t17 + f64x8::splat(0.123235) * t25;
            let t30 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t27;
            let t31 = (simd::ln(t30));
            let t33 = f64x8::splat(0.0621814) * t13 * t31;
            let t34 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t35 = (simd::cbrt(zeta_threshold));
            let t37 = ((t34).select(t35 * zeta_threshold, f64x8::splat(1.0)));
            let t39 = f64x8::splat(2.0) * t37 - f64x8::splat(2.0);
            let t40 = f64x8::splat(M_CBRT2);
            let t41 = t40 - f64x8::splat(1.0);
            let t43 = f64x8::splat(1.0) / t41 / f64x8::splat(2.0);
            let t44 = t39 * t43;
            let t46 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t11;
            let t51 = f64x8::splat(5.1785) * t14 + f64x8::splat(0.905775) * t11 + f64x8::splat(0.1100325) * t17 + f64x8::splat(0.1241775) * t25;
            let t54 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t51;
            let t55 = (simd::ln(t54));
            let t58 = f64x8::splat(0.0197516734986138) * t44 * t46 * t55;
            let t59 = (simd::ln(f64x8::splat(2.0)));
            let t60 = f64x8::splat(1.0) - t59;
            let t61 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t63 = t60 / t61;
            let t64 = t35 * t35;
            let t65 = ((t34).select(t64, f64x8::splat(1.0)));
            let t66 = t65 * t65;
            let t67 = t66 * t65;
            let t69 = f64x8::splat(1.0) + f64x8::splat(0.025) * t11;
            let t71 = f64x8::splat(1.0) + f64x8::splat(0.04445) * t11;
            let t72 = f64x8::splat(1.0) / t71;
            let t73 = t69 * t72;
            let t74 = f64x8::splat(1.0) / t60;
            let t77 = f64x8::splat(1.0) / t67;
            let t78 = t61 * t77;
            let t80 = (simd::exp(-(-t33 + t58) * t74 * t78));
            let t81 = t80 - f64x8::splat(1.0);
            let t82 = f64x8::splat(1.0) / t81;
            let t83 = t74 * t82;
            let t84 = t83 * v_sigma;
            let t85 = t73 * t84;
            let t86 = v_rho * v_rho;
            let t88 = f64x8::splat(1.0) / t8 / t86;
            let t89 = t88 * t40;
            let t90 = f64x8::splat(1.0) / t66;
            let t92 = f64x8::splat(1.0) / t4;
            let t94 = t19 * t92 * t6;
            let t98 = f64x8::splat(1.0) + f64x8::splat(0.027439371595564633) * t85 * t89 * t90 * t94;
            let t99 = ((t98).sqrt().sqrt());
            let t101 = f64x8::splat(1.0) - f64x8::splat(1.0) / t99;
            let t104 = f64x8::splat(1.0) + f64x8::splat(1.0) * t101 * t81;
            let t105 = (simd::ln(t104));
            let t107 = t63 * t67 * t105;
            let t109 = f64x8::splat(1.0) / t22 / v_rho;
            let t112 = f64x8::splat(1.0) / t22 / t86;
            let t116 = f64x8::splat(M_CBRT6);
            let t118 = (simd::cbrt(t61));
            let t119 = t118 * t118;
            let t120 = f64x8::splat(1.0) / t119;
            let t121 = t40 * t40;
            let t122 = t120 * t121;
            let t124 = f64x8::splat(5.0) / f64x8::splat(9.0) * (v_tau * t109 - v_sigma * t112 / f64x8::splat(8.0)) * t116 * t122;
            let t125 = (t124).simd_le(f64x8::splat(1.0));
            let t126 = (simd::ln(f64x8::splat(f64::EPSILON)));
            let t129 = t126 / (-t126 + f64x8::splat(0.64));
            let t130 = (-t129).simd_lt(t124);
            let t131 = (t124).simd_lt(-t129);
            let t132 = ((t131).select(t124, -t129));
            let t133 = f64x8::splat(1.0) - t132;
            let t134 = f64x8::splat(1.0) / t133;
            let t137 = (simd::exp(-f64x8::splat(0.64) * t132 * t134));
            let t138 = ((t130).select(f64x8::splat(0.0), t137));
            let t140 = (simd::ln(f64x8::splat(1.4285714285714286) * f64x8::splat(f64::EPSILON)));
            let t143 = (-t140 + f64x8::splat(1.5)) / t140;
            let t144 = (t124).simd_lt(-t143);
            let t145 = ((t144).select(-t143, t124));
            let t146 = f64x8::splat(1.0) - t145;
            let t149 = (simd::exp(f64x8::splat(1.5) / t146));
            let t151 = ((t144).select(f64x8::splat(0.0), -f64x8::splat(0.7) * t149));
            let t152 = ((t125).select(t138, t151));
            let t155 = f64x8::splat(1.0) + f64x8::splat(0.04445) * t14 + f64x8::splat(0.03138525) * t11;
            let t156 = f64x8::splat(1.0) / t155;
            let t159 = (simd::exp(f64x8::splat(1.0) * t156));
            let t160 = t159 - f64x8::splat(1.0);
            let t161 = t116 * t120;
            let t162 = t121 * v_sigma;
            let t166 = f64x8::splat(1.0) + f64x8::splat(0.02133764210437636) * t161 * t162 * t112;
            let t167 = ((t166).sqrt().sqrt());
            let t169 = f64x8::splat(1.0) - f64x8::splat(1.0) / t167;
            let t171 = t160 * t169 + f64x8::splat(1.0);
            let t172 = (simd::ln(t171));
            let t178 = f64x8::splat(1.0) - f64x8::splat(2.363) * t41 * t39 * t43;
            let t180 = (-f64x8::splat(0.0285764) * t156 + f64x8::splat(0.0285764) * t172) * t178 + t33 - t58 - t107;
            let t181 = t152 * t180;
            let tzk0 = -t33 + t58 + t107 + t181;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}

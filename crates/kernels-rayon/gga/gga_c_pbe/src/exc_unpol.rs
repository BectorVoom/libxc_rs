//! GGA_C_PBE exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_pbe.c`
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
pub fn gga_c_pbe_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_gamma: f64,
    param_BB: f64,
    param_beta: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_gamma = f64x8::splat(param_gamma);
    let param_BB = f64x8::splat(param_BB);
    let param_beta = f64x8::splat(param_beta);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let mut acc_zk = V_ZERO;
        {
            let t1 = f64x8::splat(M_CBRT3);
            let t2 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t3 = (simd::cbrt(t2));
            let t4 = t1 * t3;
            let t5 = f64x8::splat(M_CBRT4);
            let t6 = t5 * t5;
            let t7 = (simd::cbrt(v_rho));
            let t10 = t4 * t6 / t7;
            let t12 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t10;
            let t13 = ((t10).sqrt());
            let t16 = ((t10) * (t10).sqrt());
            let t18 = t1 * t1;
            let t19 = t3 * t3;
            let t20 = t18 * t19;
            let t21 = t7 * t7;
            let t24 = t20 * t5 / t21;
            let t26 = f64x8::splat(3.79785) * t13 + f64x8::splat(0.8969) * t10 + f64x8::splat(0.204775) * t16 + f64x8::splat(0.123235) * t24;
            let t29 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t26;
            let t30 = (simd::ln(t29));
            let t32 = f64x8::splat(0.0621814) * t12 * t30;
            let t33 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t34 = (simd::cbrt(zeta_threshold));
            let t36 = ((t33).select(t34 * zeta_threshold, f64x8::splat(1.0)));
            let t39 = f64x8::splat(M_CBRT2);
            let t43 = (f64x8::splat(2.0) * t36 - f64x8::splat(2.0)) / (f64x8::splat(2.0) * t39 - f64x8::splat(2.0));
            let t45 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t10;
            let t50 = f64x8::splat(5.1785) * t13 + f64x8::splat(0.905775) * t10 + f64x8::splat(0.1100325) * t16 + f64x8::splat(0.1241775) * t24;
            let t53 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t50;
            let t54 = (simd::ln(t53));
            let t57 = f64x8::splat(0.0197516734986138) * t43 * t45 * t54;
            let t58 = t34 * t34;
            let t59 = ((t33).select(t58, f64x8::splat(1.0)));
            let t60 = t59 * t59;
            let t61 = t60 * t59;
            let t62 = param_gamma * t61;
            let t63 = v_rho * v_rho;
            let t65 = f64x8::splat(1.0) / t7 / t63;
            let t68 = f64x8::splat(1.0) / t60;
            let t70 = f64x8::splat(1.0) / t3;
            let t72 = t68 * t18 * t70 * t5;
            let t75 = param_BB * param_beta;
            let t76 = f64x8::splat(1.0) / param_gamma;
            let t79 = f64x8::splat(1.0) / t61;
            let t81 = (simd::exp(-(-t32 + t57) * t76 * t79));
            let t82 = t81 - f64x8::splat(1.0);
            let t83 = f64x8::splat(1.0) / t82;
            let t84 = t76 * t83;
            let t85 = v_sigma * v_sigma;
            let t87 = t75 * t84 * t85;
            let t88 = t63 * t63;
            let t90 = f64x8::splat(1.0) / t21 / t88;
            let t91 = t39 * t39;
            let t92 = t90 * t91;
            let t93 = t60 * t60;
            let t94 = f64x8::splat(1.0) / t93;
            let t95 = t92 * t94;
            let t96 = f64x8::splat(1.0) / t19;
            let t97 = t1 * t96;
            let t98 = t97 * t6;
            let t99 = t95 * t98;
            let t102 = v_sigma * t65 * t39 * t72 / f64x8::splat(96.0) + t87 * t99 / f64x8::splat(3072.0);
            let t103 = param_beta * t102;
            let t104 = param_beta * t76;
            let t107 = t104 * t83 * t102 + f64x8::splat(1.0);
            let t108 = f64x8::splat(1.0) / t107;
            let t109 = t76 * t108;
            let t111 = t103 * t109 + f64x8::splat(1.0);
            let t112 = (simd::ln(t111));
            let t113 = t62 * t112;
            let tzk0 = -t32 + t57 + t113;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}

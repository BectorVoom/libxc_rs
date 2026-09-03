//! GGA_C_AM05 vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_am05.c`
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
pub fn gga_c_am05_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_alpha: f64,
    param_gamma: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_alpha = f64x8::splat(param_alpha);
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
            let t33 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t34 = (simd::cbrt(zeta_threshold));
            let t36 = ((t33).select(t34 * zeta_threshold, f64x8::splat(1.0)));
            let t39 = f64x8::splat(M_CBRT2);
            let t43 = (f64x8::splat(2.0) * t36 - f64x8::splat(2.0)) / (f64x8::splat(2.0) * t39 - f64x8::splat(2.0));
            let t45 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t10;
            let t50 = f64x8::splat(5.1785) * t13 + f64x8::splat(0.905775) * t10 + f64x8::splat(0.1100325) * t16 + f64x8::splat(0.1241775) * t24;
            let t53 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t50;
            let t54 = (simd::ln(t53));
            let t58 = -f64x8::splat(0.0621814) * t12 * t30 + f64x8::splat(0.0197516734986138) * t43 * t45 * t54;
            let t59 = ((t33).select(zeta_threshold, f64x8::splat(1.0)));
            let t60 = t58 * t59;
            let t61 = f64x8::splat(M_CBRT6);
            let t62 = param_alpha * t61;
            let t63 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t64 = (simd::cbrt(t63));
            let t65 = t64 * t64;
            let t66 = f64x8::splat(1.0) / t65;
            let t68 = t39 * t39;
            let t69 = v_sigma * t68;
            let t70 = v_rho * v_rho;
            let t72 = f64x8::splat(1.0) / t21 / t70;
            let t76 = f64x8::splat(1.0) + t62 * t66 * t69 * t72 / f64x8::splat(24.0);
            let t77 = f64x8::splat(1.0) / t76;
            let t80 = t77 + param_gamma * (f64x8::splat(1.0) - t77);
            let tzk0 = t60 * t80;
            acc_zk = tzk0;
            let t82 = f64x8::splat(1.0) / t7 / v_rho;
            let t83 = t6 * t82;
            let t87 = t26 * t26;
            let t88 = f64x8::splat(1.0) / t87;
            let t89 = t12 * t88;
            let t91 = f64x8::splat(1.0) / t13 * t1;
            let t92 = t3 * t6;
            let t93 = t92 * t82;
            let t94 = t91 * t93;
            let t96 = t4 * t83;
            let t98 = ((t10).sqrt());
            let t99 = t98 * t1;
            let t100 = t99 * t93;
            let t105 = t20 * t5 / t21 / v_rho;
            let t107 = -f64x8::splat(0.632975) * t94 - f64x8::splat(0.29896666666666666) * t96 - f64x8::splat(0.1023875) * t100 - f64x8::splat(0.08215666666666667) * t105;
            let t108 = f64x8::splat(1.0) / t29;
            let t109 = t107 * t108;
            let t112 = t43 * t1;
            let t117 = t43 * t45;
            let t118 = t50 * t50;
            let t119 = f64x8::splat(1.0) / t118;
            let t124 = -f64x8::splat(0.8630833333333333) * t94 - f64x8::splat(0.301925) * t96 - f64x8::splat(0.05501625) * t100 - f64x8::splat(0.082785) * t105;
            let t126 = f64x8::splat(1.0) / t53;
            let t127 = t119 * t124 * t126;
            let t130 = f64x8::splat(0.0011073470983333333) * t4 * t83 * t30 + f64x8::splat(1.0) * t89 * t109 - f64x8::splat(0.00018311447306006544) * t112 * t92 * t82 * t54 - f64x8::splat(0.5848223622634646) * t117 * t127;
            let t131 = v_rho * t130;
            let t132 = t59 * t80;
            let t134 = v_rho * t58;
            let t135 = t76 * t76;
            let t136 = f64x8::splat(1.0) / t135;
            let t138 = t136 * param_alpha * t61;
            let t139 = t66 * v_sigma;
            let t140 = t70 * v_rho;
            let t142 = f64x8::splat(1.0) / t21 / t140;
            let t143 = t68 * t142;
            let t144 = t139 * t143;
            let t146 = param_gamma * t136;
            let t147 = t146 * t62;
            let t150 = t138 * t144 / f64x8::splat(9.0) - t147 * t144 / f64x8::splat(9.0);
            let t151 = t59 * t150;
            let tvrho0 = t131 * t132 + t134 * t151 + tzk0;
            acc_vrho = tvrho0;
            let t153 = t66 * t68;
            let t156 = t146 * param_alpha;
            let t157 = t61 * t66;
            let t162 = t156 * t157 * t68 * t72 / f64x8::splat(24.0) - t138 * t153 * t72 / f64x8::splat(24.0);
            let t163 = t59 * t162;
            let tvsigma0 = t134 * t163;
            acc_vsigma = tvsigma0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}

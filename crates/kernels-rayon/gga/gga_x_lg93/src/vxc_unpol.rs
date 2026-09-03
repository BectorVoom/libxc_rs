//! GGA_X_LG93 vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_lg93.c`
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
pub fn gga_x_lg93_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
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
            let t18 = t6 * t17;
            let t19 = (simd::cbrt(v_rho));
            let t20 = f64x8::splat(M_CBRT6);
            let t21 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t22 = (simd::cbrt(t21));
            let t23 = t22 * t22;
            let t24 = f64x8::splat(1.0) / t23;
            let t25 = t20 * t24;
            let t26 = f64x8::splat(M_CBRT2);
            let t27 = t26 * t26;
            let t28 = v_sigma * t27;
            let t29 = v_rho * v_rho;
            let t30 = t19 * t19;
            let t32 = f64x8::splat(1.0) / t30 / t29;
            let t34 = t25 * t28 * t32;
            let t36 = t20 * t20;
            let t38 = f64x8::splat(1.0) / t22 / t21;
            let t39 = t36 * t38;
            let t40 = v_sigma * v_sigma;
            let t41 = t40 * t26;
            let t42 = t29 * t29;
            let t43 = t42 * v_rho;
            let t45 = f64x8::splat(1.0) / t19 / t43;
            let t49 = t40 * v_sigma;
            let t50 = t42 * t42;
            let t51 = f64x8::splat(1.0) / t50;
            let t54 = t21 * t21;
            let t57 = t20 / t23 / t54;
            let t58 = t40 * t40;
            let t59 = t58 * t27;
            let t60 = t50 * t29;
            let t62 = f64x8::splat(1.0) / t30 / t60;
            let t69 = t36 / t22 / t54 / t21;
            let t70 = t58 * v_sigma;
            let t71 = t70 * t26;
            let t72 = t50 * t43;
            let t74 = f64x8::splat(1.0) / t19 / t72;
            let t78 = t58 * t40;
            let t79 = t50 * t50;
            let t80 = f64x8::splat(1.0) / t79;
            let t83 = f64x8::splat(1.0) + f64x8::splat(0.2058807993646726) * t34 + f64x8::splat(0.1034375) * t39 * t41 * t45 + f64x8::splat(0.0003995356322973242) * t49 * t51 + f64x8::splat(0.0008766637731481481) * t57 * t59 * t62 + f64x8::splat(0.009464819637345679) * t69 * t71 * t74 + f64x8::splat(1.7770905884280507e-08) * t78 * t80;
            let t84 = (simd::pow(t83, f64x8::splat(0.024974)));
            let t87 = f64x8::splat(1.0) + f64x8::splat(4.166666666666667e-10) * t34;
            let t88 = f64x8::splat(1.0) / t87;
            let t92 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t19 * t84 * t88));
            let tzk0 = f64x8::splat(2.0) * t92;
            acc_zk = tzk0;
            let t93 = f64x8::splat(1.0) / t30;
            let t98 = (simd::pow(t83, -f64x8::splat(0.975026)));
            let t99 = t19 * t98;
            let t100 = t29 * v_rho;
            let t102 = f64x8::splat(1.0) / t30 / t100;
            let t106 = t42 * t29;
            let t108 = f64x8::splat(1.0) / t19 / t106;
            let t112 = t50 * v_rho;
            let t113 = f64x8::splat(1.0) / t112;
            let t116 = t50 * t100;
            let t118 = f64x8::splat(1.0) / t30 / t116;
            let t122 = t50 * t106;
            let t124 = f64x8::splat(1.0) / t19 / t122;
            let t128 = t79 * v_rho;
            let t129 = f64x8::splat(1.0) / t128;
            let t132 = -f64x8::splat(0.5490154649724602) * t25 * t28 * t102 - f64x8::splat(0.5516666666666666) * t39 * t41 * t108 - f64x8::splat(0.0031962850583785937) * t49 * t113 - f64x8::splat(0.009351080246913581) * t57 * t59 * t118 - f64x8::splat(0.12619759516460904) * t69 * t71 * t124 - f64x8::splat(2.843344941484881e-07) * t78 * t129;
            let t133 = t88 * t132;
            let t137 = t3 * t17;
            let t139 = f64x8::splat(1.0) / t19 / t100;
            let t141 = t137 * t139 * t84;
            let t142 = t87 * t87;
            let t143 = f64x8::splat(1.0) / t142;
            let t144 = t143 * t20;
            let t146 = t24 * v_sigma * t27;
            let t147 = t144 * t146;
            let t151 = ((t2).select(f64x8::splat(0.0), -t18 * t93 * t84 * t88 / f64x8::splat(8.0) - f64x8::splat(0.00936525) * t18 * t99 * t133 - f64x8::splat(2.8449335968970655e-10) * t141 * t147));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t151 + f64x8::splat(2.0) * t92;
            acc_vrho = tvrho0;
            let t157 = v_sigma * t26;
            let t163 = t49 * t27;
            let t167 = t58 * t26;
            let t173 = f64x8::splat(0.2058807993646726) * t25 * t27 * t32 + f64x8::splat(0.206875) * t39 * t157 * t45 + f64x8::splat(0.0011986068968919726) * t40 * t51 + f64x8::splat(0.0035066550925925925) * t57 * t163 * t62 + f64x8::splat(0.04732409818672839) * t69 * t167 * t74 + f64x8::splat(1.0662543530568304e-07) * t70 * t80;
            let t174 = t88 * t173;
            let t179 = f64x8::splat(1.0) / t19 / t29;
            let t182 = t24 * t27;
            let t183 = t144 * t182;
            let t187 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(0.00936525) * t18 * t99 * t174 + f64x8::splat(1.0668500988363994e-10) * t137 * t179 * t84 * t183));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t187;
            acc_vsigma = tvsigma0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}

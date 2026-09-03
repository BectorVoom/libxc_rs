//! GGA_X_HTBS vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_htbs.c`
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
pub fn gga_x_htbs_vxc_unpol(
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
            let t18 = (simd::cbrt(v_rho));
            let t19 = t17 * t18;
            let t20 = f64x8::splat(M_CBRT6);
            let t21 = t20 * t20;
            let t22 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t23 = (simd::cbrt(t22));
            let t25 = t21 / t23;
            let t26 = ((v_sigma).sqrt());
            let t27 = f64x8::splat(M_CBRT2);
            let t28 = t26 * t27;
            let t30 = f64x8::splat(1.0) / t18 / v_rho;
            let t32 = t25 * t28 * t30;
            let t33 = t32 / f64x8::splat(12.0);
            let t34 = (t33).simd_le(f64x8::splat(0.6));
            let t35 = t23 * t23;
            let t36 = f64x8::splat(1.0) / t35;
            let t37 = t20 * t36;
            let t38 = t27 * t27;
            let t39 = v_sigma * t38;
            let t40 = v_rho * v_rho;
            let t41 = t18 * t18;
            let t43 = f64x8::splat(1.0) / t41 / t40;
            let t45 = t37 * t39 * t43;
            let t47 = t37 * v_sigma;
            let t48 = t38 * t43;
            let t50 = (simd::exp(-t45 / f64x8::splat(24.0)));
            let t51 = t48 * t50;
            let t55 = f64x8::splat(1.0) / t23 / t22;
            let t56 = t21 * t55;
            let t57 = v_sigma * v_sigma;
            let t58 = t57 * t27;
            let t59 = t40 * t40;
            let t60 = t59 * v_rho;
            let t62 = f64x8::splat(1.0) / t18 / t60;
            let t64 = t56 * t58 * t62;
            let t66 = f64x8::splat(1.0) + f64x8::splat(2.7560657413756314e-05) * t64;
            let t67 = (simd::ln(t66));
            let t68 = f64x8::splat(0.804) + f64x8::splat(5.0) / f64x8::splat(972.0) * t45 + f64x8::splat(0.004002424276710846) * t47 * t51 + t67;
            let t71 = f64x8::splat(1.804) - f64x8::splat(0.646416) / t68;
            let t72 = (f64x8::splat(2.6)).simd_le(t33);
            let t74 = (simd::exp(-f64x8::splat(0.011376190545424806) * t45));
            let t76 = f64x8::splat(1.804) - f64x8::splat(0.804) * t74;
            let t77 = f64x8::splat(0.190125) * t32;
            let t78 = f64x8::splat(0.195) * t45;
            let t79 = t26 * v_sigma;
            let t80 = f64x8::splat(1.0) / t59;
            let t82 = f64x8::splat(0.017625664237781676) * t79 * t80;
            let t83 = f64x8::splat(0.005208333333333333) * t64;
            let t86 = t20 / t35 / t22;
            let t87 = t26 * t57;
            let t88 = t87 * t38;
            let t89 = t59 * t40;
            let t91 = f64x8::splat(1.0) / t41 / t89;
            let t94 = f64x8::splat(0.0003255208333333333) * t86 * t88 * t91;
            let t95 = -f64x8::splat(0.40608) + t77 - t78 + t82 - t83 + t94;
            let t97 = f64x8::splat(1.40608) - t77 + t78 - t82 + t83 - t94;
            let t100 = ((t34).select(t71, (t72).select(t76, t97 * t71 + t95 * t76)));
            let t104 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t100));
            let tzk0 = f64x8::splat(2.0) * t104;
            acc_zk = tzk0;
            let t106 = t17 / t41;
            let t110 = t68 * t68;
            let t111 = f64x8::splat(1.0) / t110;
            let t112 = t40 * v_rho;
            let t114 = f64x8::splat(1.0) / t41 / t112;
            let t116 = t37 * t39 * t114;
            let t118 = t38 * t114;
            let t119 = t118 * t50;
            let t122 = t56 * t57;
            let t124 = f64x8::splat(1.0) / t18 / t89;
            let t125 = t27 * t124;
            let t126 = t125 * t50;
            let t129 = f64x8::splat(1.0) / t66;
            let t130 = t125 * t129;
            let t133 = -f64x8::splat(10.0) / f64x8::splat(729.0) * t116 - f64x8::splat(0.010673131404562256) * t47 * t119 + f64x8::splat(0.0008894276170468547) * t122 * t126 - f64x8::splat(0.00014699017287336702) * t122 * t130;
            let t136 = t118 * t74;
            let t140 = f64x8::splat(1.0) / t18 / t40;
            let t145 = f64x8::splat(1.0) / t60;
            let t151 = t59 * t112;
            let t153 = f64x8::splat(1.0) / t41 / t151;
            let t157 = -f64x8::splat(0.2535) * t25 * t28 * t140 + f64x8::splat(0.52) * t116 - f64x8::splat(0.0705026569511267) * t79 * t145 + f64x8::splat(0.027777777777777776) * t56 * t58 * t124 - f64x8::splat(0.002170138888888889) * t86 * t88 * t153;
            let t160 = t95 * t20 * t36;
            let t162 = t39 * t114 * t74;
            let t165 = -t157;
            let t167 = t97 * t111;
            let t171 = ((t34).select(f64x8::splat(0.646416) * t111 * t133, (t72).select(-f64x8::splat(0.024390552529390784) * t47 * t136, t157 * t76 - f64x8::splat(0.024390552529390784) * t160 * t162 + t165 * t71 + f64x8::splat(0.646416) * t167 * t133)));
            let t176 = ((t2).select(f64x8::splat(0.0), -t6 * t106 * t100 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t171));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t176 + f64x8::splat(2.0) * t104;
            acc_vrho = tvrho0;
            let t179 = t37 * t48;
            let t183 = t56 * v_sigma;
            let t184 = t27 * t62;
            let t185 = t184 * t50;
            let t188 = t184 * t129;
            let t191 = f64x8::splat(5.0) / f64x8::splat(972.0) * t179 + f64x8::splat(0.004002424276710846) * t37 * t51 - f64x8::splat(0.0003335353563925705) * t183 * t185 + f64x8::splat(5.512131482751263e-05) * t183 * t188;
            let t194 = t48 * t74;
            let t197 = f64x8::splat(1.0) / t26;
            let t198 = t197 * t27;
            let t205 = v_sigma * t27;
            let t209 = t79 * t38;
            let t213 = f64x8::splat(0.0950625) * t25 * t198 * t30 - f64x8::splat(0.195) * t179 + f64x8::splat(0.026438496356672513) * t26 * t80 - f64x8::splat(0.010416666666666666) * t56 * t205 * t62 + f64x8::splat(0.0008138020833333334) * t86 * t209 * t91;
            let t217 = -t213;
            let t222 = ((t34).select(f64x8::splat(0.646416) * t111 * t191, (t72).select(f64x8::splat(0.009146457198521543) * t37 * t194, t213 * t76 + f64x8::splat(0.009146457198521543) * t160 * t194 + t217 * t71 + f64x8::splat(0.646416) * t167 * t191)));
            let t226 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t222));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t226;
            acc_vsigma = tvsigma0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}

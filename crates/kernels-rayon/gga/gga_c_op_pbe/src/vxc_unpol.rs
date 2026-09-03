//! GGA_C_OP_PBE vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_op_pbe.c`
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
pub fn gga_c_op_pbe_vxc_unpol(
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
            let t1 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t4 = (t1) | ((v_rho / f64x8::splat(2.0)).simd_le(dens_threshold));
            let t5 = zeta_threshold - f64x8::splat(1.0);
            let t6 = -t5;
            let t7 = ((t1).select(t5, (t1).select(t6, f64x8::splat(0.0))));
            let t8 = t7 * t7;
            let t9 = f64x8::splat(1.0) - t8;
            let t10 = t9 * v_rho;
            let t11 = f64x8::splat(1.0) + t7;
            let t14 = (t11 * v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t15 = f64x8::splat(M_CBRT3);
            let t16 = t15 * t15;
            let t18 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t20 = t16 / t18;
            let t21 = f64x8::splat(M_CBRT4);
            let t22 = t20 * t21;
            let t23 = f64x8::splat(M_CBRT2);
            let t24 = (t11).simd_le(zeta_threshold);
            let t25 = f64x8::splat(1.0) - t7;
            let t26 = (t25).simd_le(zeta_threshold);
            let t27 = ((t24).select(t5, (t26).select(t6, t7)));
            let t28 = f64x8::splat(1.0) + t27;
            let t29 = t28 * v_rho;
            let t30 = (simd::cbrt(t29));
            let t31 = f64x8::splat(1.0) / t30;
            let t33 = f64x8::splat(M_CBRT6);
            let t34 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t35 = (simd::cbrt(t34));
            let t36 = t35 * t35;
            let t37 = f64x8::splat(1.0) / t36;
            let t38 = t33 * t37;
            let t39 = t23 * t23;
            let t41 = v_rho * v_rho;
            let t42 = (simd::cbrt(v_rho));
            let t43 = t42 * t42;
            let t45 = f64x8::splat(1.0) / t43 / t41;
            let t49 = f64x8::splat(0.804) + f64x8::splat(0.009146457198521547) * t38 * v_sigma * t39 * t45;
            let t52 = f64x8::splat(1.804) - f64x8::splat(0.646416) / t49;
            let t53 = f64x8::splat(1.0) / t52;
            let t57 = ((t14).select(f64x8::splat(0.0), t22 * t23 * t31 * t53 / f64x8::splat(9.0)));
            let t61 = (t25 * v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t62 = ((t26).select(t5, (t24).select(t6, -t7)));
            let t63 = f64x8::splat(1.0) + t62;
            let t64 = t63 * v_rho;
            let t65 = (simd::cbrt(t64));
            let t66 = f64x8::splat(1.0) / t65;
            let t71 = ((t61).select(f64x8::splat(0.0), t22 * t23 * t66 * t53 / f64x8::splat(9.0)));
            let t72 = t57 + t71;
            let t73 = (t72).simd_eq(f64x8::splat(0.0));
            let t74 = ((t73).select(f64x8::splat(f64::EPSILON), t72));
            let t77 = f64x8::splat(3.61925846) / t74 + f64x8::splat(0.5764);
            let t78 = t74 * t74;
            let t79 = t78 * t78;
            let t80 = f64x8::splat(1.0) / t79;
            let t82 = t78 * t74;
            let t83 = f64x8::splat(1.0) / t82;
            let t85 = f64x8::splat(1.0) / t78;
            let t87 = f64x8::splat(32.02615087407435) * t80 + f64x8::splat(15.19118443242906) * t83 + f64x8::splat(1.801312286343) * t85;
            let t88 = f64x8::splat(1.0) / t87;
            let tzk0 = ((t4).select(f64x8::splat(0.0), -f64x8::splat(0.25) * t10 * t77 * t88));
            acc_zk = tzk0;
            let t92 = t9 * t77;
            let t96 = f64x8::splat(1.0) / t30 / t29;
            let t102 = t21 * t31;
            let t103 = t52 * t52;
            let t104 = f64x8::splat(1.0) / t103;
            let t106 = t20 * t102 * t104;
            let t107 = t49 * t49;
            let t108 = f64x8::splat(1.0) / t107;
            let t109 = t108 * t33;
            let t110 = t37 * v_sigma;
            let t111 = t41 * v_rho;
            let t113 = f64x8::splat(1.0) / t43 / t111;
            let t114 = t110 * t113;
            let t115 = t109 * t114;
            let t119 = ((t14).select(f64x8::splat(0.0), -t22 * t23 * t96 * t53 * t28 / f64x8::splat(27.0) + f64x8::splat(0.003503654089741928) * t106 * t115));
            let t121 = f64x8::splat(1.0) / t65 / t64;
            let t127 = t21 * t66;
            let t129 = t20 * t127 * t104;
            let t133 = ((t61).select(f64x8::splat(0.0), -t22 * t23 * t121 * t53 * t63 / f64x8::splat(27.0) + f64x8::splat(0.003503654089741928) * t129 * t115));
            let t135 = ((t73).select(f64x8::splat(0.0), t119 + t133));
            let t140 = t87 * t87;
            let t141 = f64x8::splat(1.0) / t140;
            let t142 = t77 * t141;
            let t144 = f64x8::splat(1.0) / t79 / t74;
            let t145 = t144 * t135;
            let t147 = t80 * t135;
            let t151 = -f64x8::splat(128.1046034962974) * t145 - f64x8::splat(45.57355329728718) * t147 - f64x8::splat(3.602624572686) * t83 * t135;
            let t156 = ((t4).select(f64x8::splat(0.0), -f64x8::splat(0.25) * t92 * t88 + f64x8::splat(0.904814615) * t10 * t85 * t135 * t88 + f64x8::splat(0.25) * t10 * t142 * t151));
            let tvrho0 = v_rho * t156 + tzk0;
            acc_vrho = tvrho0;
            let t158 = t20 * t102;
            let t159 = t104 * t108;
            let t161 = t159 * t38 * t45;
            let t164 = ((t14).select(f64x8::splat(0.0), -f64x8::splat(0.001313870283653223) * t158 * t161));
            let t165 = t20 * t127;
            let t168 = ((t61).select(f64x8::splat(0.0), -f64x8::splat(0.001313870283653223) * t165 * t161));
            let t170 = ((t73).select(f64x8::splat(0.0), t164 + t168));
            let t175 = t144 * t170;
            let t177 = t80 * t170;
            let t179 = t83 * t170;
            let t181 = -f64x8::splat(128.1046034962974) * t175 - f64x8::splat(45.57355329728718) * t177 - f64x8::splat(3.602624572686) * t179;
            let t186 = ((t4).select(f64x8::splat(0.0), f64x8::splat(0.904814615) * t10 * t85 * t170 * t88 + f64x8::splat(0.25) * t10 * t142 * t181));
            let tvsigma0 = v_rho * t186;
            acc_vsigma = tvsigma0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}

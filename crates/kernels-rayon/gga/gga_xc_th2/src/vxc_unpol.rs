//! GGA_XC_TH2 vxc unpol kernel — explicit SIMD (exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_xc_th2.c`
//! by tools/translate_rayon/from_maple.py, then rewritten to
//! `wide::f64x8` by simd.py (exact math). Eight grid points per step; every lane runs maple2c's expression
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
pub fn gga_xc_th2_vxc_unpol(
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
            let t1 = (simd::pow(f64x8::splat(2.0), f64x8::splat(1.0) / f64x8::splat(12.0)));
            let t2 = t1 * t1;
            let t3 = t2 * t1;
            let t4 = t2 * t2;
            let t5 = t4 * t4;
            let t6 = t5 * t3;
            let t7 = (simd::pow(v_rho, f64x8::splat(1.0) / f64x8::splat(12.0)));
            let t11 = (simd::pow(f64x8::splat(2.0), f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t12 = t11 * t11;
            let t13 = t12 * t12;
            let t14 = t13 * t11;
            let t15 = (simd::pow(v_rho, f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t16 = t15 * v_rho;
            let t19 = f64x8::splat(M_CBRT2);
            let t20 = t19 * t19;
            let t21 = (simd::cbrt(v_rho));
            let t22 = t21 * v_rho;
            let t25 = f64x8::splat(M_SQRT2);
            let t26 = ((v_rho).sqrt());
            let t27 = t26 * v_rho;
            let t30 = t21 * t21;
            let t31 = t30 * v_rho;
            let t32 = t19 * t31;
            let t34 = t4 * t3;
            let t35 = t34 * t7;
            let t36 = ((v_sigma).sqrt());
            let t38 = (simd::cbrt(zeta_threshold));
            let t40 = (((f64x8::splat(1.0)).simd_le(zeta_threshold)).select(t38 * zeta_threshold, f64x8::splat(1.0)));
            let t41 = t36 * t40;
            let t44 = t25 * t15;
            let t47 = t19 * t21;
            let t50 = t11 * t26;
            let t53 = f64x8::splat(1.0) / v_rho;
            let t54 = t19 * t53;
            let t55 = t40 * t40;
            let t56 = v_sigma * t55;
            let t59 = t15 * t15;
            let t60 = t59 * t59;
            let t61 = t60 * t15;
            let t62 = f64x8::splat(1.0) / t61;
            let t63 = t11 * t62;
            let t66 = f64x8::splat(1.0) / t30;
            let t70 = v_rho * v_rho;
            let t72 = f64x8::splat(1.0) / t30 / t70;
            let t73 = v_sigma * t72;
            let t74 = t73 * t55;
            let t75 = t74 - t73;
            let t78 = t61 * v_rho;
            let t79 = t11 * t78;
            let t84 = f64x8::splat(0.3394155) * t6 * t7 * v_rho - f64x8::splat(0.879105) * t14 * t16 + f64x8::splat(0.63838) * t20 * t22 - f64x8::splat(0.803945) * t25 * t27 + f64x8::splat(0.182805) * t32 - f64x8::splat(0.04533175) * t35 * t41 + f64x8::splat(0.03674325) * t44 * t41 + f64x8::splat(0.03678525) * t47 * t41 - f64x8::splat(0.017922925) * t50 * t41 - f64x8::splat(0.0050895875) * t54 * t56 + f64x8::splat(0.0026828125) * t63 * t56 - f64x8::splat(9.60195e-05) * t66 * v_sigma * t55 + f64x8::splat(0.01551885) * t32 * t75 - f64x8::splat(0.0360163) * t79 * t75 + f64x8::splat(0.0223281) * t70 * t75;
            let tzk0 = t84 * t53;
            acc_zk = tzk0;
            let t93 = t19 * t30;
            let t95 = t7 * t7;
            let t97 = t95 * t95;
            let t98 = t97 * t97;
            let t99 = t98 * t95 * t7;
            let t100 = f64x8::splat(1.0) / t99;
            let t101 = t34 * t100;
            let t104 = t25 * t62;
            let t107 = t19 * t66;
            let t110 = f64x8::splat(1.0) / t26;
            let t111 = t11 * t110;
            let t115 = t19 / t70;
            let t118 = f64x8::splat(1.0) / t78;
            let t119 = t11 * t118;
            let t122 = f64x8::splat(1.0) / t31;
            let t128 = t70 * v_rho;
            let t130 = f64x8::splat(1.0) / t30 / t128;
            let t131 = v_sigma * t130;
            let t132 = t131 * t55;
            let t134 = -f64x8::splat(8.0) / f64x8::splat(3.0) * t132 + f64x8::splat(8.0) / f64x8::splat(3.0) * t131;
            let t137 = t11 * t61;
            let tvrho0 = f64x8::splat(0.367700125) * t6 * t7 - f64x8::splat(1.0256225) * t14 * t15 + f64x8::splat(0.8511733333333333) * t20 * t21 - f64x8::splat(1.2059175) * t25 * t26 + f64x8::splat(0.304675) * t93 - f64x8::splat(0.0037776458333333334) * t101 * t41 + f64x8::splat(0.006123875) * t104 * t41 + f64x8::splat(0.01226175) * t107 * t41 - f64x8::splat(0.0089614625) * t111 * t41 + f64x8::splat(0.0050895875) * t115 * t56 - f64x8::splat(0.0022356770833333334) * t119 * t56 + f64x8::splat(6.4013e-05) * t122 * v_sigma * t55 + f64x8::splat(0.02586475) * t93 * t75 + f64x8::splat(0.01551885) * t32 * t134 - f64x8::splat(0.06602988333333333) * t137 * t75 - f64x8::splat(0.0360163) * t79 * t134 + f64x8::splat(0.0446562) * v_rho * t75 + f64x8::splat(0.0223281) * t70 * t134;
            acc_vrho = tvrho0;
            let t147 = f64x8::splat(1.0) / t36 * t40;
            let t162 = t72 * t55;
            let t163 = t162 - t72;
            let tvsigma0 = -f64x8::splat(0.022665875) * t35 * t147 + f64x8::splat(0.018371625) * t44 * t147 + f64x8::splat(0.018392625) * t47 * t147 - f64x8::splat(0.0089614625) * t50 * t147 - f64x8::splat(0.0050895875) * t54 * t55 + f64x8::splat(0.0026828125) * t63 * t55 - f64x8::splat(9.60195e-05) * t66 * t55 + f64x8::splat(0.01551885) * t32 * t163 - f64x8::splat(0.0360163) * t79 * t163 + f64x8::splat(0.0223281) * t70 * t163;
            acc_vsigma = tvsigma0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}

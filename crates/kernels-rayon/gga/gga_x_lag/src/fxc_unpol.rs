//! GGA_X_LAG fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_lag.c`
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
pub fn gga_x_lag_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
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
        let mut acc_v2rho2 = V_ZERO;
        let mut acc_v2rhosigma = V_ZERO;
        let mut acc_v2sigma2 = V_ZERO;
        {
            let t2 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t5 = zeta_threshold - f64x8::splat(1.0);
            let t7 = ((t4).select(t5, (t4).select(-t5, f64x8::splat(0.0))));
            let t8 = f64x8::splat(1.0) + t7;
            let t10 = (simd::cbrt(zeta_threshold));
            let t12 = (simd::cbrt(t8));
            let t14 = (((t8).simd_le(zeta_threshold)).select(t10 * zeta_threshold, t12 * t8));
            let t15 = t3 * t14;
            let t16 = (simd::cbrt(v_rho));
            let t17 = f64x8::splat(M_CBRT6);
            let t18 = t17 * t17;
            let t19 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t20 = (simd::cbrt(t19));
            let t21 = f64x8::splat(1.0) / t20;
            let t22 = t18 * t21;
            let t23 = ((v_sigma).sqrt());
            let t24 = f64x8::splat(M_CBRT2);
            let t29 = t22 * t23 * t24 / t16 / v_rho;
            let t30 = (simd::pow(t29, f64x8::splat(2.626712)));
            let t33 = f64x8::splat(1.0) + f64x8::splat(0.00013471619689594795) * t30;
            let t34 = (simd::pow(t33, -f64x8::splat(0.657946)));
            let t38 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(1.540002877192757e-05) * t15 * t16 * t30 * t34));
            let tzk0 = f64x8::splat(2.0) * t38;
            acc_zk = tzk0;
            let t39 = t16 * t16;
            let t45 = v_rho * v_rho;
            let t46 = f64x8::splat(1.0) / t45;
            let t47 = (simd::pow(t29, f64x8::splat(1.626712)));
            let t49 = t15 * t46 * t47;
            let t50 = t34 * t18;
            let t52 = t21 * t23 * t24;
            let t53 = t50 * t52;
            let t56 = (simd::pow(t29, f64x8::splat(4.253424)));
            let t58 = t15 * t46 * t56;
            let t59 = (simd::pow(t33, -f64x8::splat(1.657946)));
            let t60 = t59 * t18;
            let t61 = t60 * t52;
            let t65 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(5.133342923975857e-06) * t15 / t39 * t30 * t34 + f64x8::splat(5.393525383408988e-05) * t49 * t53 - f64x8::splat(4.780604235623332e-09) * t58 * t61));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t65 + f64x8::splat(2.0) * t38;
            acc_vrho = tvrho0;
            let t68 = f64x8::splat(1.0) / v_rho;
            let t70 = t15 * t68 * t47;
            let t71 = f64x8::splat(1.0) / t23;
            let t73 = t21 * t71 * t24;
            let t74 = t50 * t73;
            let t78 = t15 * t68 * t56;
            let t79 = t60 * t73;
            let t83 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(2.0225720187783704e-05) * t70 * t74 + f64x8::splat(1.7927265883587494e-09) * t78 * t79));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t83;
            acc_vsigma = tvsigma0;
            let t92 = t45 * v_rho;
            let t93 = f64x8::splat(1.0) / t92;
            let t95 = t15 * t93 * t47;
            let t99 = t15 * t93 * t56;
            let t102 = t45 * t45;
            let t104 = f64x8::splat(1.0) / t16 / t102;
            let t105 = (simd::pow(t29, f64x8::splat(0.626712)));
            let t107 = t15 * t104 * t105;
            let t108 = t34 * t17;
            let t109 = t20 * t20;
            let t110 = f64x8::splat(1.0) / t109;
            let t112 = t24 * t24;
            let t113 = t110 * v_sigma * t112;
            let t114 = t108 * t113;
            let t117 = (simd::pow(t29, f64x8::splat(3.253424)));
            let t119 = t15 * t104 * t117;
            let t120 = t59 * t17;
            let t121 = t120 * t113;
            let t124 = (simd::pow(t29, f64x8::splat(5.880136)));
            let t126 = t15 * t104 * t124;
            let t127 = (simd::pow(t33, -f64x8::splat(2.657946)));
            let t128 = t127 * t17;
            let t129 = t128 * t113;
            let t133 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.4222286159839043e-06) * t15 / t39 / v_rho * t30 * t34 - f64x8::splat(8.989208972348313e-05) * t95 * t53 + f64x8::splat(7.967673726038885e-09) * t99 * t61 - f64x8::splat(0.0007018969970796801) * t107 * t114 + f64x8::splat(2.631296584261165e-07) * t119 * t121 - f64x8::splat(2.2437549929142988e-11) * t126 * t129));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t133 + f64x8::splat(4.0) * t65;
            acc_v2rho2 = tv2rho20;
            let t139 = f64x8::splat(1.0) / t16 / t92;
            let t141 = t15 * t139 * t105;
            let t142 = t110 * t112;
            let t143 = t108 * t142;
            let t147 = t15 * t139 * t117;
            let t148 = t120 * t142;
            let t154 = t15 * t139 * t124;
            let t155 = t128 * t142;
            let t159 = ((t2).select(f64x8::splat(0.0), f64x8::splat(2.0225720187783704e-05) * t49 * t74 + f64x8::splat(0.00026321137390488005) * t141 * t143 - f64x8::splat(9.86736219097937e-08) * t147 * t148 - f64x8::splat(1.7927265883587494e-09) * t58 * t79 + f64x8::splat(8.414081223428621e-12) * t154 * t155));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t159 + f64x8::splat(2.0) * t83;
            acc_v2rhosigma = tv2rhosigma0;
            let t163 = f64x8::splat(1.0) / t16 / t45;
            let t165 = t15 * t163 * t105;
            let t166 = f64x8::splat(1.0) / v_sigma;
            let t168 = t110 * t166 * t112;
            let t169 = t108 * t168;
            let t173 = t15 * t163 * t117;
            let t174 = t120 * t168;
            let t177 = t23 * v_sigma;
            let t178 = f64x8::splat(1.0) / t177;
            let t180 = t21 * t178 * t24;
            let t181 = t50 * t180;
            let t185 = t15 * t163 * t124;
            let t186 = t128 * t168;
            let t189 = t60 * t180;
            let t193 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(9.870426521433003e-05) * t165 * t169 + f64x8::splat(3.700260821617263e-08) * t173 * t174 + f64x8::splat(1.0112860093891852e-05) * t70 * t181 - f64x8::splat(3.1552804587857326e-12) * t185 * t186 - f64x8::splat(8.963632941793747e-10) * t78 * t189));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t193;
            acc_v2sigma2 = tv2sigma20;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rho2.into(); v2rho2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rhosigma.into(); v2rhosigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2sigma2.into(); v2sigma2[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}

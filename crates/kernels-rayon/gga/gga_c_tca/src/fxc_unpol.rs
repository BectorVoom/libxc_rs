//! GGA_C_TCA fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_tca.c`
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
pub fn gga_c_tca_fxc_unpol(
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
            let t2 = (simd::cbrt(zeta_threshold));
            let t3 = t2 * t2;
            let t4 = (((f64x8::splat(1.0)).simd_le(zeta_threshold)).select(t3, f64x8::splat(1.0)));
            let t5 = t4 * t4;
            let t6 = t5 * t4;
            let t7 = f64x8::splat(M_CBRT3);
            let t9 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t10 = t7 * t9;
            let t11 = f64x8::splat(M_CBRT4);
            let t12 = t11 * t11;
            let t13 = (simd::cbrt(v_rho));
            let t18 = f64x8::splat(4.88827) + f64x8::splat(0.79425925) * t10 * t12 / t13;
            let t19 = (simd::atan(t18));
            let t21 = -f64x8::splat(0.655868) * t19 + f64x8::splat(0.897889);
            let t22 = t6 * t21;
            let t23 = t7 * t7;
            let t24 = t22 * t23;
            let t25 = f64x8::splat(1.0) / t9;
            let t26 = t25 * t11;
            let t27 = f64x8::splat(M_CBRT6);
            let t28 = t27 * t27;
            let t29 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t30 = (simd::cbrt(t29));
            let t31 = f64x8::splat(1.0) / t30;
            let t32 = t28 * t31;
            let t33 = f64x8::splat(M_CBRT2);
            let t34 = ((v_sigma).sqrt());
            let t35 = t33 * t34;
            let t37 = f64x8::splat(1.0) / t13 / v_rho;
            let t39 = t32 * t35 * t37;
            let t40 = (simd::pow(t39, f64x8::splat(2.3)));
            let t42 = f64x8::splat(1.0) + f64x8::splat(0.004712150703442276) * t40;
            let t43 = f64x8::splat(1.0) / t42;
            let t46 = t24 * t26 * t13 * t43;
            let tzk0 = t46 / f64x8::splat(3.0);
            acc_zk = tzk0;
            let t48 = t18 * t18;
            let t49 = t48 + f64x8::splat(1.0);
            let t50 = f64x8::splat(1.0) / t49;
            let t51 = t6 * t50;
            let t55 = f64x8::splat(1.0) / v_rho * t6;
            let t57 = t23 * t25;
            let t58 = t57 * t11;
            let t60 = t42 * t42;
            let t61 = f64x8::splat(1.0) / t60;
            let t62 = (simd::pow(t39, f64x8::splat(1.3)));
            let t63 = t61 * t62;
            let t64 = t63 * t28;
            let t65 = t31 * t33;
            let t66 = t65 * t34;
            let t67 = t64 * t66;
            let tvrho0 = f64x8::splat(4.0) / f64x8::splat(9.0) * t46 + f64x8::splat(0.6945723010386666) * t51 * t43 + f64x8::splat(0.004816865163518771) * t55 * t21 * t58 * t67;
            acc_vrho = tvrho0;
            let t70 = t22 * t58;
            let t71 = f64x8::splat(1.0) / t34;
            let t72 = t65 * t71;
            let tvsigma0 = -f64x8::splat(0.001806324436319539) * t70 * t64 * t72;
            acc_vsigma = tvsigma0;
            let t76 = t50 * t43;
            let t79 = t13 * t13;
            let t85 = v_rho * v_rho;
            let t86 = f64x8::splat(1.0) / t85;
            let t91 = t49 * t49;
            let t92 = f64x8::splat(1.0) / t91;
            let t93 = t6 * t92;
            let t94 = t43 * t18;
            let t102 = f64x8::splat(1.0) / t13 / t85;
            let t107 = t85 * v_rho;
            let t109 = f64x8::splat(1.0) / t13 / t107;
            let t110 = t109 * t6;
            let t112 = t110 * t21 * t58;
            let t114 = f64x8::splat(1.0) / t60 / t42;
            let t115 = (simd::pow(t39, f64x8::splat(2.6)));
            let t116 = t114 * t115;
            let t117 = t116 * t27;
            let t118 = t30 * t30;
            let t119 = f64x8::splat(1.0) / t118;
            let t120 = t33 * t33;
            let t121 = t119 * t120;
            let t122 = t121 * v_sigma;
            let t123 = t117 * t122;
            let t126 = (simd::pow(t39, f64x8::splat(0.3)));
            let t127 = t61 * t126;
            let t128 = t127 * t27;
            let t129 = t128 * t122;
            let tv2rho20 = f64x8::splat(0.9260964013848889) * t55 * t76 + f64x8::splat(4.0) / f64x8::splat(27.0) * t24 * t26 / t79 * t43 + f64x8::splat(0.0016056217211729237) * t24 * t26 * t86 * t67 + f64x8::splat(0.3677803165958304) * t93 * t94 * t10 * t12 * t37 + f64x8::splat(0.020073966722509357) * t51 * t63 * t32 * t35 * t102 + f64x8::splat(0.0008352788401267458) * t112 * t123 - f64x8::splat(0.05009539770059522) * t112 * t129;
            acc_v2rho2 = tv2rho20;
            let t133 = t50 * t61;
            let t134 = t6 * t37 * t133;
            let t135 = t62 * t28;
            let t136 = t135 * t72;
            let t139 = t121 * t102;
            let tv2rhosigma0 = -f64x8::splat(0.0037638687604705044) * t134 * t136 - f64x8::splat(0.0003132295650475297) * t70 * t117 * t139 + f64x8::splat(0.018785774137723206) * t70 * t128 * t139;
            acc_v2rhosigma = tv2rhosigma0;
            let t147 = t24 * t26 * t114;
            let t148 = t115 * t27;
            let t149 = t148 * t119;
            let t150 = f64x8::splat(1.0) / v_sigma;
            let t151 = t120 * t150;
            let t152 = t151 * t37;
            let t157 = t24 * t26 * t61;
            let t158 = t126 * t27;
            let t159 = t158 * t119;
            let t163 = t34 * v_sigma;
            let t164 = f64x8::splat(1.0) / t163;
            let t165 = t65 * t164;
            let tv2sigma20 = f64x8::splat(0.00011746108689282363) * t147 * t149 * t152 - f64x8::splat(0.007044665301646202) * t157 * t159 * t152 + f64x8::splat(0.0009031622181597695) * t70 * t64 * t165;
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

//! MGGA_C_CC fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_cc.c`
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
pub fn mgga_c_cc_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2rholapl: &mut [f64],
    v2rhotau: &mut [f64],
    v2sigma2: &mut [f64],
    v2sigmalapl: &mut [f64],
    v2sigmatau: &mut [f64],
    v2lapl2: &mut [f64],
    v2lapltau: &mut [f64],
    v2tau2: &mut [f64],
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
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
        let mut acc_vlapl = V_ZERO;
        let mut acc_vtau = V_ZERO;
        let mut acc_v2rho2 = V_ZERO;
        let mut acc_v2rhosigma = V_ZERO;
        let mut acc_v2rholapl = V_ZERO;
        let mut acc_v2rhotau = V_ZERO;
        let mut acc_v2sigma2 = V_ZERO;
        let mut acc_v2sigmalapl = V_ZERO;
        let mut acc_v2sigmatau = V_ZERO;
        let mut acc_v2lapl2 = V_ZERO;
        let mut acc_v2lapltau = V_ZERO;
        let mut acc_v2tau2 = V_ZERO;
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
            let t35 = (simd::cbrt(zeta_threshold));
            let t37 = (((f64x8::splat(1.0)).simd_le(zeta_threshold)).select(t35 * zeta_threshold, f64x8::splat(1.0)));
            let t40 = f64x8::splat(M_CBRT2);
            let t44 = (f64x8::splat(2.0) * t37 - f64x8::splat(2.0)) / (f64x8::splat(2.0) * t40 - f64x8::splat(2.0));
            let t46 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t11;
            let t51 = f64x8::splat(5.1785) * t14 + f64x8::splat(0.905775) * t11 + f64x8::splat(0.1100325) * t17 + f64x8::splat(0.1241775) * t25;
            let t54 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t51;
            let t55 = (simd::ln(t54));
            let t58 = f64x8::splat(0.0197516734986138) * t44 * t46 * t55;
            let tzk0 = -t33 + t58;
            acc_zk = tzk0;
            let t60 = f64x8::splat(1.0) / t8 / v_rho;
            let t61 = t7 * t60;
            let t63 = t5 * t61 * t31;
            let t65 = t27 * t27;
            let t66 = f64x8::splat(1.0) / t65;
            let t67 = t13 * t66;
            let t69 = f64x8::splat(1.0) / t14 * t2;
            let t70 = t4 * t7;
            let t71 = t70 * t60;
            let t72 = t69 * t71;
            let t74 = t5 * t61;
            let t76 = ((t11).sqrt());
            let t77 = t76 * t2;
            let t78 = t77 * t71;
            let t83 = t21 * t6 / t22 / v_rho;
            let t85 = -f64x8::splat(0.632975) * t72 - f64x8::splat(0.29896666666666666) * t74 - f64x8::splat(0.1023875) * t78 - f64x8::splat(0.08215666666666667) * t83;
            let t86 = f64x8::splat(1.0) / t30;
            let t87 = t85 * t86;
            let t88 = t67 * t87;
            let t90 = t44 * t2;
            let t93 = t90 * t70 * t60 * t55;
            let t95 = t44 * t46;
            let t96 = t51 * t51;
            let t97 = f64x8::splat(1.0) / t96;
            let t102 = -f64x8::splat(0.8630833333333333) * t72 - f64x8::splat(0.301925) * t74 - f64x8::splat(0.05501625) * t78 - f64x8::splat(0.082785) * t83;
            let t104 = f64x8::splat(1.0) / t54;
            let t105 = t97 * t102 * t104;
            let t106 = t95 * t105;
            let tvrho0 = -t33 + t58 + v_rho * (f64x8::splat(0.0011073470983333333) * t63 + f64x8::splat(1.0) * t88 - f64x8::splat(0.00018311447306006544) * t93 - f64x8::splat(0.5848223622634646) * t106);
            acc_vrho = tvrho0;
            let tvsigma0 = f64x8::splat(0.0);
            acc_vsigma = tvsigma0;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl = tvlapl0;
            let tvtau0 = f64x8::splat(0.0);
            acc_vtau = tvtau0;
            let t114 = v_rho * v_rho;
            let t116 = f64x8::splat(1.0) / t8 / t114;
            let t117 = t7 * t116;
            let t119 = t5 * t117 * t31;
            let t121 = t5 * t7;
            let t122 = t60 * t66;
            let t124 = t121 * t122 * t87;
            let t126 = t65 * t27;
            let t127 = f64x8::splat(1.0) / t126;
            let t128 = t13 * t127;
            let t129 = t85 * t85;
            let t130 = t129 * t86;
            let t131 = t128 * t130;
            let t135 = f64x8::splat(1.0) / t14 / t11 * t19;
            let t136 = t20 * t6;
            let t138 = f64x8::splat(1.0) / t22 / t114;
            let t139 = t136 * t138;
            let t140 = t135 * t139;
            let t142 = t70 * t116;
            let t143 = t69 * t142;
            let t145 = t5 * t117;
            let t147 = f64x8::splat(1.0)/((t11).sqrt());
            let t148 = t147 * t19;
            let t149 = t148 * t139;
            let t151 = t77 * t142;
            let t154 = t21 * t6 * t138;
            let t156 = -f64x8::splat(0.4219833333333333) * t140 + f64x8::splat(0.8439666666666666) * t143 + f64x8::splat(0.3986222222222222) * t145 + f64x8::splat(0.06825833333333334) * t149 + f64x8::splat(0.13651666666666668) * t151 + f64x8::splat(0.1369277777777778) * t154;
            let t157 = t156 * t86;
            let t158 = t67 * t157;
            let t160 = t65 * t65;
            let t161 = f64x8::splat(1.0) / t160;
            let t162 = t13 * t161;
            let t163 = t30 * t30;
            let t164 = f64x8::splat(1.0) / t163;
            let t165 = t129 * t164;
            let t166 = t162 * t165;
            let t170 = t90 * t70 * t116 * t55;
            let t172 = t44 * t5;
            let t174 = t172 * t61 * t105;
            let t176 = t96 * t51;
            let t177 = f64x8::splat(1.0) / t176;
            let t178 = t102 * t102;
            let t180 = t177 * t178 * t104;
            let t181 = t95 * t180;
            let t189 = -f64x8::splat(0.5753888888888888) * t140 + f64x8::splat(1.1507777777777777) * t143 + f64x8::splat(0.4025666666666667) * t145 + f64x8::splat(0.0366775) * t149 + f64x8::splat(0.073355) * t151 + f64x8::splat(0.137975) * t154;
            let t191 = t97 * t189 * t104;
            let t192 = t95 * t191;
            let t194 = t96 * t96;
            let t195 = f64x8::splat(1.0) / t194;
            let t196 = t195 * t178;
            let t197 = t54 * t54;
            let t198 = f64x8::splat(1.0) / t197;
            let t199 = t196 * t198;
            let t200 = t95 * t199;
            let tv2rho20 = f64x8::splat(0.0022146941966666666) * t63 + f64x8::splat(2.0) * t88 - f64x8::splat(0.0003662289461201309) * t93 - f64x8::splat(1.1696447245269292) * t106 + v_rho * (-f64x8::splat(0.0014764627977777779) * t119 - f64x8::splat(0.035616666666666665) * t124 - f64x8::splat(2.0) * t131 + f64x8::splat(1.0) * t158 + f64x8::splat(16.081979498692537) * t166 + f64x8::splat(0.00024415263074675396) * t170 + f64x8::splat(0.01084358130030174) * t174 + f64x8::splat(1.1696447245269292) * t181 - f64x8::splat(0.5848223622634646) * t192 - f64x8::splat(17.315859105681465) * t200);
            acc_v2rho2 = tv2rho20;
            let tv2rhosigma0 = f64x8::splat(0.0);
            acc_v2rhosigma = tv2rhosigma0;
            let tv2rholapl0 = f64x8::splat(0.0);
            acc_v2rholapl = tv2rholapl0;
            let tv2rhotau0 = f64x8::splat(0.0);
            acc_v2rhotau = tv2rhotau0;
            let tv2sigma20 = f64x8::splat(0.0);
            acc_v2sigma2 = tv2sigma20;
            let tv2sigmalapl0 = f64x8::splat(0.0);
            acc_v2sigmalapl = tv2sigmalapl0;
            let tv2sigmatau0 = f64x8::splat(0.0);
            acc_v2sigmatau = tv2sigmatau0;
            let tv2lapl20 = f64x8::splat(0.0);
            acc_v2lapl2 = tv2lapl20;
            let tv2lapltau0 = f64x8::splat(0.0);
            acc_v2lapltau = tv2lapltau0;
            let tv2tau20 = f64x8::splat(0.0);
            acc_v2tau2 = tv2tau20;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vlapl.into(); vlapl[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vtau.into(); vtau[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rho2.into(); v2rho2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rhosigma.into(); v2rhosigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rholapl.into(); v2rholapl[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rhotau.into(); v2rhotau[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2sigma2.into(); v2sigma2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2sigmalapl.into(); v2sigmalapl[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2sigmatau.into(); v2sigmatau[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2lapl2.into(); v2lapl2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2lapltau.into(); v2lapltau[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2tau2.into(); v2tau2[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}

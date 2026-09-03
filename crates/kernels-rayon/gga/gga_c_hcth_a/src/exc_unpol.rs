//! GGA_C_HCTH_A exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_hcth_a.c`
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
pub fn gga_c_hcth_a_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
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
        let mut acc_zk = V_ZERO;
        {
            let t3 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t4 = ((v_rho / f64x8::splat(2.0)).simd_le(dens_threshold)) | (t3);
            let t5 = ((t3).select(zeta_threshold, f64x8::splat(1.0)));
            let t6 = f64x8::splat(M_CBRT3);
            let t7 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t8 = (simd::cbrt(t7));
            let t9 = t6 * t8;
            let t10 = f64x8::splat(M_CBRT4);
            let t11 = t10 * t10;
            let t12 = t9 * t11;
            let t13 = (simd::cbrt(v_rho));
            let t14 = f64x8::splat(1.0) / t13;
            let t15 = f64x8::splat(M_CBRT2);
            let t16 = t14 * t15;
            let t17 = (simd::cbrt(zeta_threshold));
            let t19 = ((t3).select(f64x8::splat(1.0) / t17, f64x8::splat(1.0)));
            let t21 = t12 * t16 * t19;
            let t22 = t21 / f64x8::splat(4.0);
            let t23 = ((t21).sqrt());
            let t25 = t22 + f64x8::splat(1.86372) * t23 + f64x8::splat(12.9352);
            let t26 = f64x8::splat(1.0) / t25;
            let t27 = t19 * t26;
            let t31 = (simd::ln(t12 * t16 * t27 / f64x8::splat(4.0)));
            let t32 = f64x8::splat(0.0310907) * t31;
            let t33 = t23 + f64x8::splat(3.72744);
            let t36 = (simd::atan(f64x8::splat(6.15199081975908) / t33));
            let t37 = f64x8::splat(0.038783294878113016) * t36;
            let t38 = t23 / f64x8::splat(2.0);
            let t39 = t38 + f64x8::splat(0.10498);
            let t40 = t39 * t39;
            let t42 = (simd::ln(t40 * t26));
            let t43 = f64x8::splat(0.0009690227711544374) * t42;
            let t45 = t22 + f64x8::splat(3.53021) * t23 + f64x8::splat(18.0578);
            let t46 = f64x8::splat(1.0) / t45;
            let t47 = t19 * t46;
            let t51 = (simd::ln(t12 * t16 * t47 / f64x8::splat(4.0)));
            let t53 = t23 + f64x8::splat(7.06042);
            let t56 = (simd::atan(f64x8::splat(4.730926909560113) / t53));
            let t58 = t38 + f64x8::splat(0.325);
            let t59 = t58 * t58;
            let t61 = (simd::ln(t59 * t46));
            let t65 = t17 * zeta_threshold;
            let t67 = (((f64x8::splat(2.0)).simd_le(zeta_threshold)).select(t65, f64x8::splat(2.0) * t15));
            let t69 = (((f64x8::splat(0.0)).simd_le(zeta_threshold)).select(t65, f64x8::splat(0.0)));
            let t70 = t67 + t69 - f64x8::splat(2.0);
            let t72 = t15 - f64x8::splat(1.0);
            let t74 = f64x8::splat(1.0) / t72 / f64x8::splat(2.0);
            let t79 = ((t4).select(f64x8::splat(0.0), t5 * (t32 + t37 + t43 + (f64x8::splat(0.01554535) * t51 + f64x8::splat(0.05249139316978094) * t56 + f64x8::splat(0.0022478670955426118) * t61 - t32 - t37 - t43) * t70 * t74) / f64x8::splat(2.0)));
            let t80 = t15 * t15;
            let t81 = v_sigma * t80;
            let t82 = v_rho * v_rho;
            let t83 = t13 * t13;
            let t85 = f64x8::splat(1.0) / t83 / t82;
            let t86 = t81 * t85;
            let t88 = f64x8::splat(1.0) + f64x8::splat(0.2) * t86;
            let t89 = f64x8::splat(1.0) / t88;
            let t93 = v_sigma * v_sigma;
            let t94 = t93 * t15;
            let t95 = t82 * t82;
            let t96 = t95 * v_rho;
            let t98 = f64x8::splat(1.0) / t13 / t96;
            let t99 = t88 * t88;
            let t100 = f64x8::splat(1.0) / t99;
            let t101 = t98 * t100;
            let t104 = t93 * v_sigma;
            let t105 = t95 * t95;
            let t106 = f64x8::splat(1.0) / t105;
            let t107 = t104 * t106;
            let t108 = t99 * t88;
            let t109 = f64x8::splat(1.0) / t108;
            let t112 = f64x8::splat(0.0136823) + f64x8::splat(0.053784) * t81 * t85 * t89 - f64x8::splat(0.04406152) * t94 * t101 + f64x8::splat(0.03326304) * t107 * t109;
            let t114 = f64x8::splat(2.0) * t79 * t112;
            let t115 = t11 * t14;
            let t116 = t9 * t115;
            let t117 = t116 / f64x8::splat(4.0);
            let t118 = ((t116).sqrt());
            let t120 = t117 + f64x8::splat(1.86372) * t118 + f64x8::splat(12.9352);
            let t121 = f64x8::splat(1.0) / t120;
            let t125 = (simd::ln(t9 * t115 * t121 / f64x8::splat(4.0)));
            let t127 = t118 + f64x8::splat(3.72744);
            let t130 = (simd::atan(f64x8::splat(6.15199081975908) / t127));
            let t132 = t118 / f64x8::splat(2.0);
            let t133 = t132 + f64x8::splat(0.10498);
            let t134 = t133 * t133;
            let t136 = (simd::ln(t134 * t121));
            let t138 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t139 = f64x8::splat(1.0) / t138;
            let t141 = t117 + f64x8::splat(0.565535) * t118 + f64x8::splat(13.0045);
            let t142 = f64x8::splat(1.0) / t141;
            let t146 = (simd::ln(t9 * t115 * t142 / f64x8::splat(4.0)));
            let t147 = t118 + f64x8::splat(1.13107);
            let t150 = (simd::atan(f64x8::splat(7.123108917818118) / t147));
            let t152 = t132 + f64x8::splat(0.0047584);
            let t153 = t152 * t152;
            let t155 = (simd::ln(t153 * t142));
            let t159 = ((t3).select(t65, f64x8::splat(1.0)));
            let t164 = f64x8::splat(9.0) * (f64x8::splat(2.0) * t159 - f64x8::splat(2.0)) * t74 * t72;
            let t168 = f64x8::splat(0.0310907) * t125 + f64x8::splat(0.038783294878113016) * t130 + f64x8::splat(0.0009690227711544374) * t136 - t139 * (t146 + f64x8::splat(0.31770800474394145) * t150 + f64x8::splat(0.00041403379428206277) * t155) * t164 / f64x8::splat(24.0) - f64x8::splat(2.0) * t79;
            let t170 = f64x8::splat(1.0) + f64x8::splat(0.006) * t86;
            let t171 = f64x8::splat(1.0) / t170;
            let t175 = t170 * t170;
            let t176 = f64x8::splat(1.0) / t175;
            let t177 = t98 * t176;
            let t180 = t175 * t170;
            let t181 = f64x8::splat(1.0) / t180;
            let t184 = f64x8::splat(0.836897) + f64x8::splat(0.01032306) * t81 * t85 * t171 - f64x8::splat(0.00020051856) * t94 * t177 - f64x8::splat(3.95283456e-06) * t107 * t181;
            let t185 = t168 * t184;
            let tzk0 = t114 + t185;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}

//! LDA_X_YUKAWA fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_x_yukawa.c`
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
pub fn lda_x_yukawa_fxc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    param_hyb_omega_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_hyb_omega_0 = f64x8::splat(param_hyb_omega_0);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_v2rho2 = V_ZERO;
        {
            let t1 = f64x8::splat(M_CBRT3);
            let t3 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t5 = f64x8::splat(M_CBRT4);
            let t6 = t5 * t5;
            let t7 = t6 * t3 * t1;
            let t8 = f64x8::splat(M_CBRT2);
            let t9 = t8 * t8;
            let t10 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t11 = (simd::cbrt(zeta_threshold));
            let t13 = ((t10).select(t11 * zeta_threshold, f64x8::splat(1.0)));
            let t14 = t13 * t9;
            let t15 = (simd::cbrt(v_rho));
            let t16 = (simd::cbrt(f64x8::splat(9.0)));
            let t17 = t16 * t16;
            let t18 = t3 * t3;
            let t20 = param_hyb_omega_0 * t18 * t17;
            let t23 = ((t10).select(t11, f64x8::splat(1.0)));
            let t24 = f64x8::splat(1.0) / t23;
            let t27 = t24 / t15 * t1 * t20 / f64x8::splat(18.0);
            let t28 = (f64x8::splat(1.92)).simd_le(t27);
            let t29 = (f64x8::splat(1.92)).simd_lt(t27);
            let t30 = ((t29).select(t27, f64x8::splat(1.92)));
            let t31 = t30 * t30;
            let t34 = t31 * t31;
            let t35 = f64x8::splat(1.0) / t34;
            let t37 = t34 * t31;
            let t38 = f64x8::splat(1.0) / t37;
            let t40 = t34 * t34;
            let t41 = f64x8::splat(1.0) / t40;
            let t43 = t40 * t31;
            let t44 = f64x8::splat(1.0) / t43;
            let t46 = t40 * t34;
            let t47 = f64x8::splat(1.0) / t46;
            let t49 = t40 * t37;
            let t50 = f64x8::splat(1.0) / t49;
            let t52 = t40 * t40;
            let t53 = f64x8::splat(1.0) / t52;
            let t56 = f64x8::splat(1.0) / t52 / t31;
            let t59 = f64x8::splat(1.0) / t52 / t34;
            let t62 = f64x8::splat(1.0) / t52 / t37;
            let t65 = f64x8::splat(1.0) / t52 / t40;
            let t68 = f64x8::splat(1.0) / t52 / t43;
            let t71 = f64x8::splat(1.0) / t52 / t46;
            let t74 = f64x8::splat(1.0) / t52 / t49;
            let t76 = t52 * t52;
            let t77 = f64x8::splat(1.0) / t76;
            let t80 = f64x8::splat(1.0) / t76 / t31;
            let t83 = f64x8::splat(1.0) / t76 / t34;
            let t85 = f64x8::splat(1.0) / t31 / f64x8::splat(9.0) - t35 / f64x8::splat(30.0) + t38 / f64x8::splat(70.0) - t41 / f64x8::splat(135.0) + t44 / f64x8::splat(231.0) - t47 / f64x8::splat(364.0) + t50 / f64x8::splat(540.0) - t53 / f64x8::splat(765.0) + t56 / f64x8::splat(1045.0) - t59 / f64x8::splat(1386.0) + t62 / f64x8::splat(1794.0) - t65 / f64x8::splat(2275.0) + t68 / f64x8::splat(2835.0) - t71 / f64x8::splat(3480.0) + t74 / f64x8::splat(4216.0) - t77 / f64x8::splat(5049.0) + t80 / f64x8::splat(5985.0) - t83 / f64x8::splat(7030.0);
            let t86 = ((t29).select(f64x8::splat(1.92), t27));
            let t87 = (simd::atan2(f64x8::splat(1.0), t86));
            let t88 = t86 * t86;
            let t89 = t88 + f64x8::splat(3.0);
            let t90 = f64x8::splat(1.0) / t88;
            let t91 = f64x8::splat(1.0) + t90;
            let t92 = (simd::ln(t91));
            let t94 = -t92 * t89 + f64x8::splat(1.0);
            let t97 = t87 + t94 * t86 / f64x8::splat(4.0);
            let t101 = ((t28).select(t85, f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t97 * t86));
            let t104 = t101 * t15 * t14 * t7;
            let tzk0 = -f64x8::splat(3.0) / f64x8::splat(16.0) * t104;
            acc_zk = tzk0;
            let t107 = t15 * v_rho;
            let t109 = t3 * t1 * t107;
            let t110 = t9 * t6;
            let t111 = t31 * t30;
            let t112 = f64x8::splat(1.0) / t111;
            let t117 = t24 / t107 * t1 * t20 / f64x8::splat(54.0);
            let t118 = ((t29).select(-t117, f64x8::splat(0.0)));
            let t121 = t34 * t30;
            let t122 = f64x8::splat(1.0) / t121;
            let t125 = t34 * t111;
            let t126 = f64x8::splat(1.0) / t125;
            let t129 = t40 * t30;
            let t130 = f64x8::splat(1.0) / t129;
            let t133 = t40 * t111;
            let t134 = f64x8::splat(1.0) / t133;
            let t137 = t40 * t121;
            let t138 = f64x8::splat(1.0) / t137;
            let t141 = t40 * t125;
            let t142 = f64x8::splat(1.0) / t141;
            let t146 = f64x8::splat(1.0) / t52 / t30;
            let t150 = f64x8::splat(1.0) / t52 / t111;
            let t154 = f64x8::splat(1.0) / t52 / t121;
            let t158 = f64x8::splat(1.0) / t52 / t125;
            let t162 = f64x8::splat(1.0) / t52 / t129;
            let t166 = f64x8::splat(1.0) / t52 / t133;
            let t170 = f64x8::splat(1.0) / t52 / t137;
            let t174 = f64x8::splat(1.0) / t52 / t141;
            let t178 = f64x8::splat(1.0) / t76 / t30;
            let t182 = f64x8::splat(1.0) / t76 / t111;
            let t186 = f64x8::splat(1.0) / t76 / t121;
            let t189 = -f64x8::splat(2.0) / f64x8::splat(9.0) * t118 * t112 + f64x8::splat(2.0) / f64x8::splat(15.0) * t118 * t122 - f64x8::splat(3.0) / f64x8::splat(35.0) * t118 * t126 + f64x8::splat(8.0) / f64x8::splat(135.0) * t118 * t130 - f64x8::splat(10.0) / f64x8::splat(231.0) * t118 * t134 + f64x8::splat(3.0) / f64x8::splat(91.0) * t118 * t138 - f64x8::splat(7.0) / f64x8::splat(270.0) * t118 * t142 + f64x8::splat(16.0) / f64x8::splat(765.0) * t118 * t146 - f64x8::splat(18.0) / f64x8::splat(1045.0) * t118 * t150 + f64x8::splat(10.0) / f64x8::splat(693.0) * t118 * t154 - f64x8::splat(11.0) / f64x8::splat(897.0) * t118 * t158 + f64x8::splat(24.0) / f64x8::splat(2275.0) * t118 * t162 - f64x8::splat(26.0) / f64x8::splat(2835.0) * t118 * t166 + f64x8::splat(7.0) / f64x8::splat(870.0) * t118 * t170 - f64x8::splat(15.0) / f64x8::splat(2108.0) * t118 * t174 + f64x8::splat(32.0) / f64x8::splat(5049.0) * t118 * t178 - f64x8::splat(34.0) / f64x8::splat(5985.0) * t118 * t182 + f64x8::splat(18.0) / f64x8::splat(3515.0) * t118 * t186;
            let t190 = ((t29).select(f64x8::splat(0.0), -t117));
            let t193 = f64x8::splat(1.0) / t91;
            let t199 = t88 * t86;
            let t200 = f64x8::splat(1.0) / t199;
            let t201 = t200 * t89;
            let t202 = t193 * t190;
            let t205 = -f64x8::splat(2.0) * t92 * t190 * t86 + f64x8::splat(2.0) * t202 * t201;
            let t208 = -t193 * t90 * t190 + t94 * t190 / f64x8::splat(4.0) + t205 * t86 / f64x8::splat(4.0);
            let t212 = ((t28).select(t189, -f64x8::splat(8.0) / f64x8::splat(3.0) * t97 * t190 - f64x8::splat(8.0) / f64x8::splat(3.0) * t208 * t86));
            let tvrho0 = -t104 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(16.0) * t212 * t13 * t110 * t109;
            acc_vrho = tvrho0;
            let t217 = t15 * t15;
            let t218 = f64x8::splat(1.0) / t217;
            let t227 = t118 * t118;
            let t263 = f64x8::splat(1.0) / t76 / t37;
            let t266 = f64x8::splat(2.0) / f64x8::splat(3.0) * t227 * t35 - f64x8::splat(2.0) / f64x8::splat(3.0) * t227 * t38 + f64x8::splat(3.0) / f64x8::splat(5.0) * t227 * t41 - f64x8::splat(8.0) / f64x8::splat(15.0) * t227 * t44 + f64x8::splat(10.0) / f64x8::splat(21.0) * t227 * t47 - f64x8::splat(3.0) / f64x8::splat(7.0) * t227 * t50 + f64x8::splat(7.0) / f64x8::splat(18.0) * t227 * t53 - f64x8::splat(16.0) / f64x8::splat(45.0) * t227 * t56 + f64x8::splat(18.0) / f64x8::splat(55.0) * t227 * t59 - f64x8::splat(10.0) / f64x8::splat(33.0) * t227 * t62 + f64x8::splat(11.0) / f64x8::splat(39.0) * t227 * t65 - f64x8::splat(24.0) / f64x8::splat(91.0) * t227 * t68 + f64x8::splat(26.0) / f64x8::splat(105.0) * t227 * t71 - f64x8::splat(7.0) / f64x8::splat(30.0) * t227 * t74 + f64x8::splat(15.0) / f64x8::splat(68.0) * t227 * t77 - f64x8::splat(32.0) / f64x8::splat(153.0) * t227 * t80 + f64x8::splat(34.0) / f64x8::splat(171.0) * t227 * t83 - f64x8::splat(18.0) / f64x8::splat(95.0) * t227 * t263;
            let t267 = v_rho * v_rho;
            let t273 = f64x8::splat(2.0) / f64x8::splat(81.0) * t24 / t15 / t267 * t1 * t20;
            let t274 = ((t29).select(t273, f64x8::splat(0.0)));
            let t311 = -f64x8::splat(2.0) / f64x8::splat(9.0) * t274 * t112 + f64x8::splat(2.0) / f64x8::splat(15.0) * t274 * t122 - f64x8::splat(3.0) / f64x8::splat(35.0) * t274 * t126 + f64x8::splat(8.0) / f64x8::splat(135.0) * t274 * t130 - f64x8::splat(10.0) / f64x8::splat(231.0) * t274 * t134 + f64x8::splat(3.0) / f64x8::splat(91.0) * t274 * t138 - f64x8::splat(7.0) / f64x8::splat(270.0) * t274 * t142 + f64x8::splat(16.0) / f64x8::splat(765.0) * t274 * t146 - f64x8::splat(18.0) / f64x8::splat(1045.0) * t274 * t150 + f64x8::splat(10.0) / f64x8::splat(693.0) * t274 * t154 - f64x8::splat(11.0) / f64x8::splat(897.0) * t274 * t158 + f64x8::splat(24.0) / f64x8::splat(2275.0) * t274 * t162 - f64x8::splat(26.0) / f64x8::splat(2835.0) * t274 * t166 + f64x8::splat(7.0) / f64x8::splat(870.0) * t274 * t170 - f64x8::splat(15.0) / f64x8::splat(2108.0) * t274 * t174 + f64x8::splat(32.0) / f64x8::splat(5049.0) * t274 * t178 - f64x8::splat(34.0) / f64x8::splat(5985.0) * t274 * t182 + f64x8::splat(18.0) / f64x8::splat(3515.0) * t274 * t186;
            let t313 = ((t29).select(f64x8::splat(0.0), t273));
            let t318 = t90 * t313;
            let t320 = t190 * t190;
            let t321 = t200 * t320;
            let t324 = t88 * t88;
            let t326 = f64x8::splat(1.0) / t324 / t86;
            let t328 = t91 * t91;
            let t329 = f64x8::splat(1.0) / t328;
            let t344 = f64x8::splat(1.0) / t324;
            let t345 = t344 * t89;
            let t346 = t193 * t320;
            let t349 = t193 * t313;
            let t353 = f64x8::splat(1.0) / t324 / t88;
            let t354 = t353 * t89;
            let t355 = t329 * t320;
            let t358 = f64x8::splat(8.0) * t193 * t320 * t90 - f64x8::splat(2.0) * t92 * t313 * t86 + f64x8::splat(2.0) * t349 * t201 - f64x8::splat(2.0) * t92 * t320 - f64x8::splat(6.0) * t346 * t345 + f64x8::splat(4.0) * t355 * t354;
            let t361 = -t193 * t318 + f64x8::splat(2.0) * t193 * t321 - f64x8::splat(2.0) * t329 * t326 * t320 + t94 * t313 / f64x8::splat(4.0) + t205 * t190 / f64x8::splat(2.0) + t358 * t86 / f64x8::splat(4.0);
            let t365 = ((t28).select(t266 + t311, -f64x8::splat(8.0) / f64x8::splat(3.0) * t97 * t313 - f64x8::splat(16.0) / f64x8::splat(3.0) * t208 * t190 - f64x8::splat(8.0) / f64x8::splat(3.0) * t361 * t86));
            let tv2rho20 = -t101 * t218 * t14 * t7 / f64x8::splat(12.0) - t212 * t15 * t14 * t7 / f64x8::splat(2.0) - f64x8::splat(3.0) / f64x8::splat(16.0) * t365 * t13 * t110 * t109;
            acc_v2rho2 = tv2rho20;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rho2.into(); v2rho2[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}

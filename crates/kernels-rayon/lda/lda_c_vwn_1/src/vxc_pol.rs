//! LDA_C_VWN_1 vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_vwn_1.c`
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

/// Load 8 elements with a given stride and offset.
#[inline(always)]
fn load_strided(s: &[f64], ip: usize, np: usize, stride: usize, offset: usize) -> f64x8 {
    let mut b = [0.0f64; 8];
    if ip + 8 <= np {
        let base = ip * stride + offset;
        b[0] = s[base];
        b[1] = s[base + stride];
        b[2] = s[base + 2 * stride];
        b[3] = s[base + 3 * stride];
        b[4] = s[base + 4 * stride];
        b[5] = s[base + 5 * stride];
        b[6] = s[base + 6 * stride];
        b[7] = s[base + 7 * stride];
    } else {
        for k in 0..8 {
            let p = (ip + k).min(np - 1);
            b[k] = s[p * stride + offset];
        }
    }
    f64x8::new(b)
}

/// Store 8 elements with a given stride and offset.
#[inline(always)]
fn store_strided(s: &mut [f64], ip: usize, m: usize, stride: usize, offset: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let base = ip * stride + offset;
        s[base] = a[0];
        s[base + stride] = a[1];
        s[base + 2 * stride] = a[2];
        s[base + 3 * stride] = a[3];
        s[base + 4 * stride] = a[4];
        s[base + 5 * stride] = a[5];
        s[base + 6 * stride] = a[6];
        s[base + 7 * stride] = a[7];
    } else {
        for k in 0..m {
            s[(ip + k) * stride + offset] = a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn lda_c_vwn_1_vxc_pol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho0 = load_strided(rho, ip, np, 2, 0);
        let v_rho1 = load_strided(rho, ip, np, 2, 1);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        {
            let t1 = f64x8::splat(M_CBRT3);
            let t2 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t3 = (simd::cbrt(t2));
            let t4 = t1 * t3;
            let t5 = f64x8::splat(M_CBRT4);
            let t6 = t5 * t5;
            let t7 = v_rho0 + v_rho1;
            let t8 = (simd::cbrt(t7));
            let t9 = f64x8::splat(1.0) / t8;
            let t10 = t6 * t9;
            let t11 = t4 * t10;
            let t12 = t11 / f64x8::splat(4.0);
            let t13 = ((t11).sqrt());
            let t15 = t12 + f64x8::splat(1.86372) * t13 + f64x8::splat(12.9352);
            let t16 = f64x8::splat(1.0) / t15;
            let t20 = (simd::ln(t4 * t10 * t16 / f64x8::splat(4.0)));
            let t22 = t13 + f64x8::splat(3.72744);
            let t25 = (simd::atan(f64x8::splat(6.15199081975908) / t22));
            let t27 = t13 / f64x8::splat(2.0);
            let t28 = t27 + f64x8::splat(0.10498);
            let t29 = t28 * t28;
            let t31 = (simd::ln(t29 * t16));
            let t33 = f64x8::splat(0.0310907) * t20 + f64x8::splat(0.038783294878113016) * t25 + f64x8::splat(0.0009690227711544374) * t31;
            let t34 = v_rho0 - v_rho1;
            let t35 = f64x8::splat(1.0) / t7;
            let t36 = t34 * t35;
            let t37 = f64x8::splat(1.0) + t36;
            let t38 = (t37).simd_le(zeta_threshold);
            let t39 = (simd::cbrt(zeta_threshold));
            let t40 = t39 * zeta_threshold;
            let t41 = (simd::cbrt(t37));
            let t43 = ((t38).select(t40, t41 * t37));
            let t44 = f64x8::splat(1.0) - t36;
            let t45 = (t44).simd_le(zeta_threshold);
            let t46 = (simd::cbrt(t44));
            let t48 = ((t45).select(t40, t46 * t44));
            let t49 = t43 + t48 - f64x8::splat(2.0);
            let t50 = f64x8::splat(M_CBRT2);
            let t53 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t50 - f64x8::splat(2.0));
            let t55 = -t49 * t53 + f64x8::splat(1.0);
            let t56 = t33 * t55;
            let t58 = t12 + f64x8::splat(3.53021) * t13 + f64x8::splat(18.0578);
            let t59 = f64x8::splat(1.0) / t58;
            let t63 = (simd::ln(t4 * t10 * t59 / f64x8::splat(4.0)));
            let t65 = t13 + f64x8::splat(7.06042);
            let t68 = (simd::atan(f64x8::splat(4.730926909560113) / t65));
            let t70 = t27 + f64x8::splat(0.325);
            let t71 = t70 * t70;
            let t73 = (simd::ln(t71 * t59));
            let t75 = f64x8::splat(0.01554535) * t63 + f64x8::splat(0.05249139316978094) * t68 + f64x8::splat(0.0022478670955426118) * t73;
            let t77 = t75 * t49 * t53;
            let tzk0 = t56 + t77;
            acc_zk = tzk0;
            let t79 = f64x8::splat(1.0) / t8 / t7;
            let t80 = t6 * t79;
            let t84 = t4 * t6;
            let t85 = t15 * t15;
            let t86 = f64x8::splat(1.0) / t85;
            let t87 = t9 * t86;
            let t88 = t4 * t80;
            let t89 = t88 / f64x8::splat(12.0);
            let t90 = f64x8::splat(1.0) / t13;
            let t91 = t90 * t1;
            let t92 = t3 * t6;
            let t94 = t91 * t92 * t79;
            let t96 = -t89 - f64x8::splat(0.31062) * t94;
            let t101 = t1 * t1;
            let t103 = f64x8::splat(1.0) / t3;
            let t104 = (-t4 * t80 * t16 / f64x8::splat(12.0) - t84 * t87 * t96 / f64x8::splat(4.0)) * t101 * t103;
            let t105 = t5 * t8;
            let t106 = t105 * t15;
            let t109 = t22 * t22;
            let t110 = f64x8::splat(1.0) / t109;
            let t112 = t110 * t90 * t1;
            let t114 = f64x8::splat(37.8469910464) * t110 + f64x8::splat(1.0);
            let t115 = f64x8::splat(1.0) / t114;
            let t120 = t28 * t16;
            let t121 = t120 * t90;
            let t124 = t29 * t86;
            let t126 = -t121 * t88 / f64x8::splat(6.0) - t124 * t96;
            let t127 = f64x8::splat(1.0) / t29;
            let t128 = t126 * t127;
            let t131 = f64x8::splat(0.010363566666666667) * t104 * t106 + f64x8::splat(0.03976574567502677) * t112 * t92 * t79 * t115 + f64x8::splat(0.0009690227711544374) * t128 * t15;
            let t132 = t131 * t55;
            let t133 = t7 * t7;
            let t134 = f64x8::splat(1.0) / t133;
            let t135 = t34 * t134;
            let t136 = t35 - t135;
            let t139 = ((t38).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t41 * t136));
            let t140 = -t136;
            let t143 = ((t45).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t46 * t140));
            let t144 = t139 + t143;
            let t146 = t33 * t144 * t53;
            let t150 = t58 * t58;
            let t151 = f64x8::splat(1.0) / t150;
            let t152 = t9 * t151;
            let t154 = -t89 - f64x8::splat(0.5883683333333334) * t94;
            let t160 = (-t4 * t80 * t59 / f64x8::splat(12.0) - t84 * t152 * t154 / f64x8::splat(4.0)) * t101 * t103;
            let t161 = t105 * t58;
            let t164 = t65 * t65;
            let t165 = f64x8::splat(1.0) / t164;
            let t167 = t165 * t90 * t1;
            let t169 = f64x8::splat(22.3816694236) * t165 + f64x8::splat(1.0);
            let t170 = f64x8::splat(1.0) / t169;
            let t175 = t70 * t59;
            let t176 = t175 * t90;
            let t179 = t71 * t151;
            let t181 = -t176 * t88 / f64x8::splat(6.0) - t179 * t154;
            let t182 = f64x8::splat(1.0) / t71;
            let t183 = t181 * t182;
            let t186 = f64x8::splat(0.005181783333333334) * t160 * t161 + f64x8::splat(0.041388824077869424) * t167 * t92 * t79 * t170 + f64x8::splat(0.0022478670955426118) * t183 * t58;
            let t188 = t186 * t49 * t53;
            let t190 = t75 * t144 * t53;
            let tvrho0 = t56 + t77 + t7 * (t132 - t146 + t188 + t190);
            acc_vrho_0 = tvrho0;
            let t193 = -t35 - t135;
            let t196 = ((t38).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t41 * t193));
            let t197 = -t193;
            let t200 = ((t45).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t46 * t197));
            let t201 = t196 + t200;
            let t203 = t33 * t201 * t53;
            let t205 = t75 * t201 * t53;
            let tvrho1 = t56 + t77 + t7 * (t132 - t203 + t188 + t205);
            acc_vrho_1 = tvrho1;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        ip += 8;
    }
}

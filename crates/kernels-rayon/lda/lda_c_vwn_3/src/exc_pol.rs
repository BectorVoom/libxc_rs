//! LDA_C_VWN_3 exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_vwn_3.c`
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
pub fn lda_c_vwn_3_exc_pol(
    rho: &[f64],
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
        let v_rho0 = load_strided(rho, ip, np, 2, 0);
        let v_rho1 = load_strided(rho, ip, np, 2, 1);
        let mut acc_zk = V_ZERO;
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
            let t21 = f64x8::splat(0.0310907) * t20;
            let t22 = t13 + f64x8::splat(3.72744);
            let t25 = (simd::atan(f64x8::splat(6.15199081975908) / t22));
            let t26 = f64x8::splat(0.038783294878113016) * t25;
            let t27 = t13 / f64x8::splat(2.0);
            let t28 = t27 + f64x8::splat(0.10498);
            let t29 = t28 * t28;
            let t31 = (simd::ln(t29 * t16));
            let t32 = f64x8::splat(0.0009690227711544374) * t31;
            let t34 = t12 + f64x8::splat(3.53021) * t13 + f64x8::splat(18.0578);
            let t35 = f64x8::splat(1.0) / t34;
            let t39 = (simd::ln(t4 * t10 * t35 / f64x8::splat(4.0)));
            let t41 = t13 + f64x8::splat(7.06042);
            let t44 = (simd::atan(f64x8::splat(4.730926909560113) / t41));
            let t46 = t27 + f64x8::splat(0.325);
            let t47 = t46 * t46;
            let t49 = (simd::ln(t47 * t35));
            let t51 = f64x8::splat(0.01554535) * t39 + f64x8::splat(0.05249139316978094) * t44 + f64x8::splat(0.0022478670955426118) * t49 - t21 - t26 - t32;
            let t53 = t12 + f64x8::splat(10.06155) * t13 + f64x8::splat(101.578);
            let t54 = f64x8::splat(1.0) / t53;
            let t58 = (simd::ln(t4 * t10 * t54 / f64x8::splat(4.0)));
            let t60 = t13 + f64x8::splat(20.1231);
            let t63 = (simd::atan(f64x8::splat(1.171685277708993) / t60));
            let t65 = t27 + f64x8::splat(0.743294);
            let t66 = t65 * t65;
            let t68 = (simd::ln(t66 * t54));
            let t71 = t12 + f64x8::splat(6.536) * t13 + f64x8::splat(42.7198);
            let t72 = f64x8::splat(1.0) / t71;
            let t76 = (simd::ln(t4 * t10 * t72 / f64x8::splat(4.0)));
            let t78 = t13 + f64x8::splat(13.072);
            let t81 = (simd::atan(f64x8::splat(0.0448998886412873) / t78));
            let t83 = t27 + f64x8::splat(0.409286);
            let t84 = t83 * t83;
            let t86 = (simd::ln(t84 * t72));
            let t88 = f64x8::splat(0.01554535) * t58 + f64x8::splat(0.6188180297906063) * t63 + f64x8::splat(0.002667310007273315) * t68 - f64x8::splat(0.0310907) * t76 - f64x8::splat(20.521972937837504) * t81 - f64x8::splat(0.004431373767749538) * t86;
            let t89 = f64x8::splat(1.0) / t88;
            let t90 = t51 * t89;
            let t91 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t92 = f64x8::splat(1.0) / t91;
            let t94 = t12 + f64x8::splat(0.534175) * t13 + f64x8::splat(11.4813);
            let t95 = f64x8::splat(1.0) / t94;
            let t99 = (simd::ln(t4 * t10 * t95 / f64x8::splat(4.0)));
            let t100 = t13 + f64x8::splat(1.06835);
            let t103 = (simd::atan(f64x8::splat(6.692072046645942) / t100));
            let t105 = t27 + f64x8::splat(0.228344);
            let t106 = t105 * t105;
            let t108 = (simd::ln(t106 * t95));
            let t111 = t92 * (t99 + f64x8::splat(0.32323836906055065) * t103 + f64x8::splat(0.021608710360898266) * t108);
            let t112 = t90 * t111;
            let t113 = v_rho0 - v_rho1;
            let t114 = f64x8::splat(1.0) / t7;
            let t115 = t113 * t114;
            let t116 = f64x8::splat(1.0) + t115;
            let t117 = (t116).simd_le(zeta_threshold);
            let t118 = (simd::cbrt(zeta_threshold));
            let t119 = t118 * zeta_threshold;
            let t120 = (simd::cbrt(t116));
            let t122 = ((t117).select(t119, t120 * t116));
            let t123 = f64x8::splat(1.0) - t115;
            let t124 = (t123).simd_le(zeta_threshold);
            let t125 = (simd::cbrt(t123));
            let t127 = ((t124).select(t119, t125 * t123));
            let t128 = t122 + t127 - f64x8::splat(2.0);
            let t129 = f64x8::splat(M_CBRT2);
            let t130 = t129 - f64x8::splat(1.0);
            let t132 = f64x8::splat(1.0) / t130 / f64x8::splat(2.0);
            let t133 = t128 * t132;
            let t134 = t113 * t113;
            let t135 = t134 * t134;
            let t136 = t7 * t7;
            let t137 = t136 * t136;
            let t138 = f64x8::splat(1.0) / t137;
            let t140 = -t135 * t138 + f64x8::splat(1.0);
            let t141 = f64x8::splat(9.0) * t130;
            let t142 = t140 * t141;
            let t143 = t133 * t142;
            let t145 = t112 * t143 / f64x8::splat(24.0);
            let t146 = t51 * t128;
            let t147 = t132 * t135;
            let t148 = t147 * t138;
            let t149 = t146 * t148;
            let tzk0 = t21 + t26 + t32 - t145 + t149;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}

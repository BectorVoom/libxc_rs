//! LDA_C_VWN vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_vwn.c`
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

/// Accumulate 8 consecutive grid points into an output array.
///
/// `+=`, not `=`. The scalar kernel writes `out[ip] += v`; a plain store is a
/// different operation in two ways. It keeps the sign of a negative zero where
/// `0.0 + -0.0` gives `+0.0` -- a bit difference the fingerprint gate reports
/// as a rejection even though no value changed (`gga_x_pbepow fxc` was
/// rejected on exactly this, 273 of 200,000 `v2sigma2` elements) -- and it
/// would discard whatever a caller had already put in the buffer.
#[inline(always)]
fn store_add(s: &mut [f64], ip: usize, m: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        let r: [f64; 8] = (f64x8::new(b) + acc).into();
        s[ip..ip + 8].copy_from_slice(&r);
    } else {
        for k in 0..m {
            s[ip + k] += a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn lda_c_vwn_vxc_unpol(
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
        let v_rho = load(rho, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        {
            let t1 = f64x8::splat(M_CBRT3);
            let t2 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t3 = (simd::cbrt(t2));
            let t4 = t1 * t3;
            let t5 = f64x8::splat(M_CBRT4);
            let t6 = t5 * t5;
            let t7 = (simd::cbrt(v_rho));
            let t8 = f64x8::splat(1.0) / t7;
            let t9 = t6 * t8;
            let t10 = t4 * t9;
            let t11 = t10 / f64x8::splat(4.0);
            let t12 = ((t10).sqrt());
            let t14 = t11 + f64x8::splat(1.86372) * t12 + f64x8::splat(12.9352);
            let t15 = f64x8::splat(1.0) / t14;
            let t19 = (simd::ln(t4 * t9 * t15 / f64x8::splat(4.0)));
            let t20 = f64x8::splat(0.0310907) * t19;
            let t21 = t12 + f64x8::splat(3.72744);
            let t24 = (simd::atan(f64x8::splat(6.15199081975908) / t21));
            let t25 = f64x8::splat(0.038783294878113016) * t24;
            let t26 = t12 / f64x8::splat(2.0);
            let t27 = t26 + f64x8::splat(0.10498);
            let t28 = t27 * t27;
            let t30 = (simd::ln(t28 * t15));
            let t31 = f64x8::splat(0.0009690227711544374) * t30;
            let t32 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t33 = f64x8::splat(1.0) / t32;
            let t35 = t11 + f64x8::splat(0.565535) * t12 + f64x8::splat(13.0045);
            let t36 = f64x8::splat(1.0) / t35;
            let t40 = (simd::ln(t4 * t9 * t36 / f64x8::splat(4.0)));
            let t41 = t12 + f64x8::splat(1.13107);
            let t44 = (simd::atan(f64x8::splat(7.123108917818118) / t41));
            let t46 = t26 + f64x8::splat(0.0047584);
            let t47 = t46 * t46;
            let t49 = (simd::ln(t47 * t36));
            let t54 = (simd::cbrt(zeta_threshold));
            let t56 = (((f64x8::splat(1.0)).simd_le(zeta_threshold)).select(t54 * zeta_threshold, f64x8::splat(1.0)));
            let t59 = f64x8::splat(M_CBRT2);
            let t60 = t59 - f64x8::splat(1.0);
            let t65 = f64x8::splat(9.0) * t56 - f64x8::splat(9.0);
            let t67 = t33 * (t40 + f64x8::splat(0.31770800474394145) * t44 + f64x8::splat(0.00041403379428206277) * t49) * t65 / f64x8::splat(24.0);
            let tzk0 = t20 + t25 + t31 - t67;
            acc_zk = tzk0;
            let t69 = f64x8::splat(1.0) / t7 / v_rho;
            let t70 = t6 * t69;
            let t74 = t4 * t6;
            let t75 = t14 * t14;
            let t76 = f64x8::splat(1.0) / t75;
            let t77 = t8 * t76;
            let t78 = t4 * t70;
            let t79 = t78 / f64x8::splat(12.0);
            let t80 = f64x8::splat(1.0) / t12;
            let t81 = t80 * t1;
            let t82 = t3 * t6;
            let t84 = t81 * t82 * t69;
            let t86 = -t79 - f64x8::splat(0.31062) * t84;
            let t91 = t1 * t1;
            let t93 = f64x8::splat(1.0) / t3;
            let t94 = (-t4 * t70 * t15 / f64x8::splat(12.0) - t74 * t77 * t86 / f64x8::splat(4.0)) * t91 * t93;
            let t95 = t5 * t7;
            let t96 = t95 * t14;
            let t97 = t94 * t96;
            let t99 = t21 * t21;
            let t100 = f64x8::splat(1.0) / t99;
            let t102 = t100 * t80 * t1;
            let t104 = f64x8::splat(37.8469910464) * t100 + f64x8::splat(1.0);
            let t105 = f64x8::splat(1.0) / t104;
            let t108 = t102 * t82 * t69 * t105;
            let t110 = t27 * t15;
            let t111 = t110 * t80;
            let t114 = t28 * t76;
            let t116 = -t111 * t78 / f64x8::splat(6.0) - t114 * t86;
            let t117 = f64x8::splat(1.0) / t28;
            let t118 = t116 * t117;
            let t119 = t118 * t14;
            let t124 = t35 * t35;
            let t125 = f64x8::splat(1.0) / t124;
            let t126 = t8 * t125;
            let t128 = -t79 - f64x8::splat(0.09425583333333333) * t84;
            let t134 = (-t4 * t70 * t36 / f64x8::splat(12.0) - t74 * t126 * t128 / f64x8::splat(4.0)) * t91 * t93;
            let t135 = t95 * t35;
            let t138 = t41 * t41;
            let t139 = f64x8::splat(1.0) / t138;
            let t141 = t139 * t80 * t1;
            let t143 = f64x8::splat(50.7386806551) * t139 + f64x8::splat(1.0);
            let t144 = f64x8::splat(1.0) / t143;
            let t149 = t46 * t36;
            let t150 = t149 * t80;
            let t153 = t47 * t125;
            let t155 = -t150 * t78 / f64x8::splat(6.0) - t153 * t128;
            let t156 = f64x8::splat(1.0) / t47;
            let t157 = t155 * t156;
            let t162 = t33 * (t134 * t135 / f64x8::splat(3.0) + f64x8::splat(0.37717812030896175) * t141 * t82 * t69 * t144 + f64x8::splat(0.00041403379428206277) * t157 * t35) * t65;
            let tvrho0 = t20 + t25 + t31 - t67 + v_rho * (f64x8::splat(0.010363566666666667) * t97 + f64x8::splat(0.03976574567502677) * t108 + f64x8::splat(0.0009690227711544374) * t119 - t162 / f64x8::splat(24.0));
            acc_vrho = tvrho0;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        ip += 8;
    }
}

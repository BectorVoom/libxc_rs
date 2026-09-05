//! LDA_C_VWN_RPA vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_vwn_rpa.c`
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
pub fn lda_c_vwn_rpa_vxc_unpol(
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
            let t14 = t11 + f64x8::splat(6.536) * t12 + f64x8::splat(42.7198);
            let t15 = f64x8::splat(1.0) / t14;
            let t19 = (simd::ln(t4 * t9 * t15 / f64x8::splat(4.0)));
            let t21 = t12 + f64x8::splat(13.072);
            let t24 = (simd::atan(f64x8::splat(0.0448998886412873) / t21));
            let t26 = t12 / f64x8::splat(2.0);
            let t27 = t26 + f64x8::splat(0.409286);
            let t28 = t27 * t27;
            let t30 = (simd::ln(t28 * t15));
            let t34 = (simd::cbrt(zeta_threshold));
            let t36 = (((f64x8::splat(1.0)).simd_le(zeta_threshold)).select(t34 * zeta_threshold, f64x8::splat(1.0)));
            let t38 = f64x8::splat(2.0) * t36 - f64x8::splat(2.0);
            let t39 = f64x8::splat(M_CBRT2);
            let t42 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t39 - f64x8::splat(2.0));
            let t44 = -t38 * t42 + f64x8::splat(1.0);
            let t45 = (f64x8::splat(0.0310907) * t19 + f64x8::splat(20.521972937837504) * t24 + f64x8::splat(0.004431373767749538) * t30) * t44;
            let t47 = t11 + f64x8::splat(10.06155) * t12 + f64x8::splat(101.578);
            let t48 = f64x8::splat(1.0) / t47;
            let t52 = (simd::ln(t4 * t9 * t48 / f64x8::splat(4.0)));
            let t54 = t12 + f64x8::splat(20.1231);
            let t57 = (simd::atan(f64x8::splat(1.171685277708993) / t54));
            let t59 = t26 + f64x8::splat(0.743294);
            let t60 = t59 * t59;
            let t62 = (simd::ln(t60 * t48));
            let t66 = (f64x8::splat(0.01554535) * t52 + f64x8::splat(0.6188180297906063) * t57 + f64x8::splat(0.002667310007273315) * t62) * t38 * t42;
            let tzk0 = t45 + t66;
            acc_zk = tzk0;
            let t68 = f64x8::splat(1.0) / t7 / v_rho;
            let t69 = t6 * t68;
            let t73 = t4 * t6;
            let t74 = t14 * t14;
            let t75 = f64x8::splat(1.0) / t74;
            let t76 = t8 * t75;
            let t77 = t4 * t69;
            let t78 = t77 / f64x8::splat(12.0);
            let t79 = f64x8::splat(1.0) / t12;
            let t80 = t79 * t1;
            let t81 = t3 * t6;
            let t83 = t80 * t81 * t68;
            let t85 = -t78 - f64x8::splat(1.0893333333333333) * t83;
            let t90 = t1 * t1;
            let t92 = f64x8::splat(1.0) / t3;
            let t93 = (-t4 * t69 * t15 / f64x8::splat(12.0) - t73 * t76 * t85 / f64x8::splat(4.0)) * t90 * t92;
            let t94 = t5 * t7;
            let t95 = t94 * t14;
            let t98 = t21 * t21;
            let t99 = f64x8::splat(1.0) / t98;
            let t101 = t99 * t79 * t1;
            let t103 = f64x8::splat(0.002016) * t99 + f64x8::splat(1.0);
            let t104 = f64x8::splat(1.0) / t103;
            let t109 = t27 * t15;
            let t110 = t109 * t79;
            let t113 = t28 * t75;
            let t115 = -t110 * t77 / f64x8::splat(6.0) - t113 * t85;
            let t116 = f64x8::splat(1.0) / t28;
            let t117 = t115 * t116;
            let t121 = (f64x8::splat(0.010363566666666667) * t93 * t95 + f64x8::splat(0.15357238326806924) * t101 * t81 * t68 * t104 + f64x8::splat(0.004431373767749538) * t117 * t14) * t44;
            let t125 = t47 * t47;
            let t126 = f64x8::splat(1.0) / t125;
            let t127 = t8 * t126;
            let t129 = -t78 - f64x8::splat(1.676925) * t83;
            let t135 = (-t4 * t69 * t48 / f64x8::splat(12.0) - t73 * t127 * t129 / f64x8::splat(4.0)) * t90 * t92;
            let t136 = t94 * t47;
            let t139 = t54 * t54;
            let t140 = f64x8::splat(1.0) / t139;
            let t142 = t140 * t79 * t1;
            let t144 = f64x8::splat(1.37284639) * t140 + f64x8::splat(1.0);
            let t145 = f64x8::splat(1.0) / t144;
            let t150 = t59 * t48;
            let t151 = t150 * t79;
            let t154 = t60 * t126;
            let t156 = -t151 * t77 / f64x8::splat(6.0) - t154 * t129;
            let t157 = f64x8::splat(1.0) / t60;
            let t158 = t156 * t157;
            let t163 = (f64x8::splat(0.005181783333333334) * t135 * t136 + f64x8::splat(0.12084332918108974) * t142 * t81 * t68 * t145 + f64x8::splat(0.002667310007273315) * t158 * t47) * t38 * t42;
            let tvrho0 = t45 + t66 + v_rho * (t121 + t163);
            acc_vrho = tvrho0;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        ip += 8;
    }
}

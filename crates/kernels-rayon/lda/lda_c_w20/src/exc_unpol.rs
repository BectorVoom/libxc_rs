//! LDA_C_W20 exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_w20.c`
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
pub fn lda_c_w20_exc_unpol(
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
        let v_rho = load(rho, ip, np);
        let mut acc_zk = V_ZERO;
        {
            let t1 = (simd::ln(f64x8::splat(2.0)));
            let t2 = f64x8::splat(1.0) - t1;
            let t3 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t4 = f64x8::splat(1.0) / t3;
            let t5 = t2 * t4;
            let t6 = t1 / f64x8::splat(6.0);
            let t8 = f64x8::splat(1.0) / t2;
            let t12 = (simd::exp(-f64x8::splat(2.0) * (-f64x8::splat(0.16244537117517982) + t6) * t8 * t3));
            let t13 = f64x8::splat(M_CBRT3);
            let t14 = t13 * t13;
            let t15 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t16 = (simd::cbrt(t15));
            let t17 = t16 * t16;
            let t18 = t14 * t17;
            let t19 = f64x8::splat(M_CBRT4);
            let t20 = (simd::cbrt(v_rho));
            let t21 = t20 * t20;
            let t22 = f64x8::splat(1.0) / t21;
            let t24 = t18 * t19 * t22;
            let t26 = (simd::exp(-t24 / f64x8::splat(40000.0)));
            let t27 = f64x8::splat(1.0) - t26;
            let t28 = f64x8::splat(M_CBRTPI);
            let t29 = t28 * t28;
            let t31 = (simd::cbrt(f64x8::splat(9.0)));
            let t32 = f64x8::splat(1.0) / t29 * t31;
            let t33 = t19 * t19;
            let t39 = t12 / f64x8::splat(2.0);
            let t40 = (-f64x8::splat(0.9) + f64x8::splat(3.0) / f64x8::splat(16.0) * t32 * t33) * t8 * t3 + t39;
            let t44 = (-f64x8::splat(2.0) * t27 * t40 + t12) * t14;
            let t45 = f64x8::splat(1.0) / t16;
            let t46 = t45 * t19;
            let t47 = t46 * t20;
            let t50 = t27 * t8;
            let t51 = ((f64x8::splat(4.0)).sqrt());
            let t52 = t13 * t16;
            let t53 = f64x8::splat(1.0) / t20;
            let t55 = t52 * t33 * t53;
            let t56 = ((t55).sqrt());
            let t58 = f64x8::splat(1.0) / t56 / t55;
            let t60 = t50 * t51 * t58;
            let t62 = t31 * t31;
            let t63 = t62 * t19;
            let t64 = t29 * t3;
            let t68 = -f64x8::splat(3.0) / f64x8::splat(40.0) * t63 * t64 * t8 + t39;
            let t72 = (-f64x8::splat(2.0) * t27 * t68 + t12) * t13;
            let t73 = f64x8::splat(1.0) / t17;
            let t74 = t73 * t33;
            let t75 = t74 * t21;
            let t78 = f64x8::splat(1.0) + t44 * t47 / f64x8::splat(3.0) - f64x8::splat(118.43525281307231) * t60 + t72 * t75 / f64x8::splat(3.0);
            let t79 = (simd::ln(t78));
            let t81 = t5 * t79 / f64x8::splat(2.0);
            let t82 = t52 * t33;
            let t83 = t53 * t26;
            let t84 = ((f64x8::splat(4.0)).sqrt().sqrt());
            let t85 = t84 * t84;
            let t86 = t85 * t84;
            let t87 = ((t55).sqrt().sqrt());
            let t91 = t26 + f64x8::splat(5.0) / f64x8::splat(8.0) * t86 * t87 * t55;
            let t92 = f64x8::splat(1.0) / t91;
            let t93 = t3 * f64x8::splat(M_PI);
            let t95 = f64x8::splat(1.0) / t28 / t93;
            let t97 = f64x8::splat(12.0) * t1;
            let t98 = f64x8::splat(7.0) / f64x8::splat(6.0) * t3 - t97 - f64x8::splat(1.0);
            let t99 = t95 * t98;
            let t100 = t14 * t45;
            let t104 = f64x8::splat(1.0) + t100 * t19 * t20 / f64x8::splat(3.0);
            let t105 = (simd::ln(t104));
            let t109 = -t63 * t99 * t105 / f64x8::splat(36.0) - f64x8::splat(0.01);
            let t110 = t92 * t109;
            let t113 = t82 * t83 * t110 / f64x8::splat(4.0);
            let t118 = (simd::exp(-f64x8::splat(4.0) * (-f64x8::splat(0.1412623711751798) + t6) * t8 * t3));
            let t119 = f64x8::splat(M_CBRT2);
            let t127 = t118 / f64x8::splat(2.0);
            let t128 = f64x8::splat(2.0) * (-f64x8::splat(0.9) + f64x8::splat(3.0) / f64x8::splat(16.0) * t32 * t33 * t119) * t8 * t3 + t127;
            let t132 = (-f64x8::splat(2.0) * t27 * t128 + t118) * t14;
            let t136 = t119 * t119;
            let t141 = -f64x8::splat(3.0) / f64x8::splat(20.0) * t63 * t64 * t136 * t8 + t127;
            let t145 = (-f64x8::splat(2.0) * t27 * t141 + t118) * t13;
            let t148 = f64x8::splat(1.0) + t132 * t47 / f64x8::splat(3.0) - f64x8::splat(236.87050562614462) * t60 + t145 * t75 / f64x8::splat(3.0);
            let t149 = (simd::ln(t148));
            let t154 = t136 * t62;
            let t156 = f64x8::splat(13.0) / f64x8::splat(12.0) * t3 - t97 + f64x8::splat(1.0) / f64x8::splat(2.0);
            let t157 = t95 * t156;
            let t159 = t154 * t157 * t105;
            let t164 = (simd::cbrt(zeta_threshold));
            let t166 = (((f64x8::splat(1.0)).simd_le(zeta_threshold)).select(t164 * zeta_threshold, f64x8::splat(1.0)));
            let t168 = f64x8::splat(2.0) * t166 - f64x8::splat(2.0);
            let t172 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t119 - f64x8::splat(2.0));
            let t173 = (-t5 * t149 / f64x8::splat(4.0) - t52 * t83 * t92 * t159 / f64x8::splat(144.0) + t81 - t113) * t168 * t172;
            let tzk0 = -t81 + t113 + t173;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}

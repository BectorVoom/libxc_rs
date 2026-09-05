//! LDA_XC_TETER93 fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_xc_teter93.c`
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
pub fn lda_xc_teter93_fxc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
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
        let mut acc_v2rho2 = V_ZERO;
        {
            let t2 = (simd::cbrt(zeta_threshold));
            let t4 = (((f64x8::splat(1.0)).simd_le(zeta_threshold)).select(t2 * zeta_threshold, f64x8::splat(1.0)));
            let t7 = f64x8::splat(M_CBRT2);
            let t11 = (f64x8::splat(2.0) * t4 - f64x8::splat(2.0)) / (f64x8::splat(2.0) * t7 - f64x8::splat(2.0));
            let t15 = f64x8::splat(M_CBRT3);
            let t16 = (f64x8::splat(2.217058676663745) + f64x8::splat(0.6157402568883344) * t11) * t15;
            let t17 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t18 = (simd::cbrt(t17));
            let t19 = f64x8::splat(M_CBRT4);
            let t20 = t19 * t19;
            let t21 = t18 * t20;
            let t22 = (simd::cbrt(v_rho));
            let t23 = f64x8::splat(1.0) / t22;
            let t29 = t15 * t15;
            let t30 = (f64x8::splat(0.7405551735357053) + f64x8::splat(0.1574201515892867) * t11) * t29;
            let t31 = t18 * t18;
            let t32 = t31 * t19;
            let t33 = t22 * t22;
            let t35 = t32 / t33;
            let t40 = (f64x8::splat(0.01968227878617998) + f64x8::splat(0.003532336663397157) * t11) * t17;
            let t41 = f64x8::splat(1.0) / v_rho;
            let t44 = f64x8::splat(0.4581652932831429) + f64x8::splat(0.119086804055547) * t11 + t16 * t21 * t23 / f64x8::splat(4.0) + t30 * t35 / f64x8::splat(4.0) + f64x8::splat(3.0) / f64x8::splat(4.0) * t40 * t41;
            let t45 = t15 * t18;
            let t51 = (f64x8::splat(4.504130959426697) + f64x8::splat(0.2673612973836267) * t11) * t29;
            let t56 = (f64x8::splat(1.110667363742916) + f64x8::splat(0.2052004607777787) * t11) * t17;
            let t61 = (f64x8::splat(0.02359291751427506) + f64x8::splat(0.004200005045691381) * t11) * t15;
            let t63 = t18 * t17 * t20;
            let t65 = f64x8::splat(1.0) / t22 / v_rho;
            let t69 = f64x8::splat(0.25) * t45 * t20 * t23 + t51 * t35 / f64x8::splat(4.0) + f64x8::splat(3.0) / f64x8::splat(4.0) * t56 * t41 + f64x8::splat(3.0) / f64x8::splat(16.0) * t61 * t63 * t65;
            let t70 = f64x8::splat(1.0) / t69;
            let tzk0 = -t44 * t70;
            acc_zk = tzk0;
            let t77 = t32 / t33 / v_rho;
            let t80 = v_rho * v_rho;
            let t81 = f64x8::splat(1.0) / t80;
            let t84 = -t16 * t21 * t65 / f64x8::splat(12.0) - t30 * t77 / f64x8::splat(6.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t40 * t81;
            let t85 = v_rho * t84;
            let t87 = v_rho * t44;
            let t88 = t69 * t69;
            let t89 = f64x8::splat(1.0) / t88;
            let t98 = f64x8::splat(1.0) / t22 / t80;
            let t102 = -f64x8::splat(0.08333333333333333) * t45 * t20 * t65 - t51 * t77 / f64x8::splat(6.0) - f64x8::splat(3.0) / f64x8::splat(4.0) * t56 * t81 - t61 * t63 * t98 / f64x8::splat(4.0);
            let t103 = t89 * t102;
            let tvrho0 = t87 * t103 - t85 * t70 + tzk0;
            acc_vrho = tvrho0;
            let t107 = t44 * t89;
            let t115 = t32 / t33 / t80;
            let t118 = t80 * v_rho;
            let t119 = f64x8::splat(1.0) / t118;
            let t122 = t16 * t21 * t98 / f64x8::splat(9.0) + f64x8::splat(5.0) / f64x8::splat(18.0) * t30 * t115 + f64x8::splat(3.0) / f64x8::splat(2.0) * t40 * t119;
            let t123 = v_rho * t122;
            let t128 = f64x8::splat(1.0) / t88 / t69;
            let t129 = t102 * t102;
            let t130 = t128 * t129;
            let t141 = f64x8::splat(1.0) / t22 / t118;
            let t145 = f64x8::splat(0.1111111111111111) * t45 * t20 * t98 + f64x8::splat(5.0) / f64x8::splat(18.0) * t51 * t115 + f64x8::splat(3.0) / f64x8::splat(2.0) * t56 * t119 + f64x8::splat(7.0) / f64x8::splat(12.0) * t61 * t63 * t141;
            let t146 = t89 * t145;
            let tv2rho20 = f64x8::splat(2.0) * t107 * t102 + f64x8::splat(2.0) * t85 * t103 - t123 * t70 - f64x8::splat(2.0) * t87 * t130 + t87 * t146 - f64x8::splat(2.0) * t84 * t70;
            acc_v2rho2 = tv2rho20;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(v2rho2, ip, m, acc_v2rho2);
        ip += 8;
    }
}

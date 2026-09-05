//! GGA_X_LV_RPW86 vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_lv_rpw86.c`
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
pub fn gga_x_lv_rpw86_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
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
        {
            let t2 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = f64x8::splat(M_CBRTPI);
            let t6 = t3 / t4;
            let t7 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t8 = zeta_threshold - f64x8::splat(1.0);
            let t10 = ((t7).select(t8, (t7).select(-t8, f64x8::splat(0.0))));
            let t11 = f64x8::splat(1.0) + t10;
            let t13 = (simd::cbrt(zeta_threshold));
            let t15 = (simd::cbrt(t11));
            let t17 = (((t11).simd_le(zeta_threshold)).select(t13 * zeta_threshold, t15 * t11));
            let t18 = (simd::cbrt(v_rho));
            let t19 = t17 * t18;
            let t20 = f64x8::splat(M_CBRT6);
            let t21 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t22 = (simd::cbrt(t21));
            let t23 = t22 * t22;
            let t25 = t20 / t23;
            let t26 = f64x8::splat(M_CBRT2);
            let t27 = t26 * t26;
            let t28 = v_sigma * t27;
            let t29 = v_rho * v_rho;
            let t30 = t18 * t18;
            let t32 = f64x8::splat(1.0) / t30 / t29;
            let t34 = t25 * t28 * t32;
            let t36 = f64x8::splat(1.0) + f64x8::splat(0.003931018518518519) * t34;
            let t37 = v_sigma * v_sigma;
            let t38 = t37 * v_sigma;
            let t39 = t29 * t29;
            let t40 = t39 * t39;
            let t41 = f64x8::splat(1.0) / t40;
            let t42 = t38 * t41;
            let t43 = f64x8::splat(3.881824540052514e-07) * t42;
            let t44 = f64x8::splat(1.0) + t43;
            let t45 = f64x8::splat(1.0) / t44;
            let t48 = t20 * t20;
            let t51 = t48 / t22 / t21;
            let t52 = t37 * t26;
            let t53 = t39 * v_rho;
            let t55 = f64x8::splat(1.0) / t18 / t53;
            let t60 = f64x8::splat(1.0) + f64x8::splat(0.077125) * t34 + f64x8::splat(0.06017361111111111) * t51 * t52 * t55 + f64x8::splat(2.905130394988796e-06) * t42;
            let t61 = (simd::pow(t60, f64x8::splat(1.0) / f64x8::splat(15.0)));
            let t62 = f64x8::splat(1.15) + t43;
            let t63 = f64x8::splat(1.0) / t62;
            let t64 = t61 * t63;
            let t67 = t36 * t45 + f64x8::splat(3.881824540052514e-07) * t42 * t64;
            let t71 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t67));
            let tzk0 = f64x8::splat(2.0) * t71;
            acc_zk = tzk0;
            let t73 = t17 / t30;
            let t77 = t25 * v_sigma;
            let t78 = t29 * v_rho;
            let t80 = f64x8::splat(1.0) / t30 / t78;
            let t81 = t27 * t80;
            let t82 = t81 * t45;
            let t85 = t44 * t44;
            let t86 = f64x8::splat(1.0) / t85;
            let t87 = t36 * t86;
            let t88 = t40 * v_rho;
            let t89 = f64x8::splat(1.0) / t88;
            let t90 = t38 * t89;
            let t95 = t61 * t61;
            let t96 = t95 * t95;
            let t98 = t96 * t96;
            let t99 = t98 * t96 * t95;
            let t100 = f64x8::splat(1.0) / t99;
            let t101 = t100 * t63;
            let t105 = t39 * t29;
            let t107 = f64x8::splat(1.0) / t18 / t105;
            let t112 = -f64x8::splat(0.20566666666666666) * t25 * t28 * t80 - f64x8::splat(0.32092592592592595) * t51 * t52 * t107 - f64x8::splat(2.324104315991037e-05) * t90;
            let t113 = t101 * t112;
            let t116 = t37 * t37;
            let t117 = t116 * t37;
            let t118 = t40 * t40;
            let t120 = f64x8::splat(1.0) / t118 / v_rho;
            let t121 = t117 * t120;
            let t122 = t62 * t62;
            let t123 = f64x8::splat(1.0) / t122;
            let t124 = t61 * t123;
            let t127 = -f64x8::splat(0.010482716049382716) * t77 * t82 + f64x8::splat(3.1054596320420114e-06) * t87 * t90 - f64x8::splat(3.1054596320420114e-06) * t90 * t64 + f64x8::splat(2.5878830267016762e-08) * t42 * t113 + f64x8::splat(1.205484940780313e-12) * t121 * t124;
            let t132 = ((t2).select(f64x8::splat(0.0), -t6 * t73 * t67 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t127));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t132 + f64x8::splat(2.0) * t71;
            acc_vrho = tvrho0;
            let t135 = t27 * t32;
            let t139 = t37 * t41;
            let t146 = v_sigma * t26;
            let t151 = f64x8::splat(0.077125) * t25 * t135 + f64x8::splat(0.12034722222222222) * t51 * t146 * t55 + f64x8::splat(8.715391184966388e-06) * t139;
            let t152 = t101 * t151;
            let t155 = t116 * v_sigma;
            let t156 = f64x8::splat(1.0) / t118;
            let t157 = t155 * t156;
            let t160 = f64x8::splat(0.003931018518518519) * t25 * t135 * t45 - f64x8::splat(1.1645473620157543e-06) * t87 * t139 + f64x8::splat(1.1645473620157543e-06) * t139 * t64 + f64x8::splat(2.5878830267016762e-08) * t42 * t152 - f64x8::splat(4.5205685279261743e-13) * t157 * t124;
            let t164 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t160));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t164;
            acc_vsigma = tvsigma0;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        ip += 8;
    }
}

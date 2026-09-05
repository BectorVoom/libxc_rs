//! LDA_C_HL vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_hl.c`
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

/// Accumulate 8 elements with a given stride and offset.
///
/// `+=`, not `=`: the scalar kernel this was translated from writes
/// `out[ip * stride + offset] += v`, and a plain store is not the same
/// operation. It differs on the sign of zero -- `0.0 + -0.0` is `+0.0`
/// while a store of `-0.0` keeps the sign -- which is a bit difference
/// the fingerprint gate sees, and it would silently drop a caller's
/// existing contribution if one were ever there.
///
/// The read is not free on this path: a polarized `kxc`/`lxc` kernel
/// writes many strided outputs per point, and `lda_c_pw_erf kxc pol`
/// measured 84 -> 114 ns/pt (1.36x). It is charged anyway, because the
/// scalar kernel this is compared against does the same read. Gathering
/// into a vector, adding once and scattering back was tried and is no
/// faster (117 ns/pt), so the cost is the load itself, not scheduling.
#[inline(always)]
fn store_strided(s: &mut [f64], ip: usize, m: usize, stride: usize, offset: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let base = ip * stride + offset;
        s[base] += a[0];
        s[base + stride] += a[1];
        s[base + 2 * stride] += a[2];
        s[base + 3 * stride] += a[3];
        s[base + 4 * stride] += a[4];
        s[base + 5 * stride] += a[5];
        s[base + 6 * stride] += a[6];
        s[base + 7 * stride] += a[7];
    } else {
        for k in 0..m {
            s[(ip + k) * stride + offset] += a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn lda_c_hl_vxc_pol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    param_hl_c_0: f64,
    param_hl_r_0: f64,
    param_hl_c_1: f64,
    param_hl_r_1: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_hl_c_0 = f64x8::splat(param_hl_c_0);
    let param_hl_r_0 = f64x8::splat(param_hl_r_0);
    let param_hl_c_1 = f64x8::splat(param_hl_c_1);
    let param_hl_r_1 = f64x8::splat(param_hl_r_1);
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
            let t1 = param_hl_c_0;
            let t2 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t3 = v_rho0 + v_rho1;
            let t4 = f64x8::splat(1.0) / t3;
            let t5 = t2 * t4;
            let t6 = param_hl_r_0;
            let t7 = t6 * t6;
            let t8 = t7 * t6;
            let t9 = f64x8::splat(1.0) / t8;
            let t12 = f64x8::splat(1.0) + f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t9;
            let t13 = f64x8::splat(M_CBRT3);
            let t14 = t13 * t13;
            let t15 = (simd::cbrt(t2));
            let t16 = f64x8::splat(1.0) / t15;
            let t17 = t14 * t16;
            let t18 = f64x8::splat(M_CBRT4);
            let t19 = (simd::cbrt(t3));
            let t20 = t18 * t19;
            let t24 = f64x8::splat(1.0) + t17 * t20 * t6 / f64x8::splat(3.0);
            let t25 = (simd::ln(t24));
            let t27 = t15 * t15;
            let t28 = t14 * t27;
            let t29 = t19 * t19;
            let t31 = t18 / t29;
            let t32 = f64x8::splat(1.0) / t7;
            let t36 = t13 * t15;
            let t37 = t18 * t18;
            let t39 = t37 / t19;
            let t40 = f64x8::splat(1.0) / t6;
            let t45 = t1 * (t12 * t25 - t28 * t31 * t32 / f64x8::splat(4.0) + t36 * t39 * t40 / f64x8::splat(8.0) - f64x8::splat(1.0) / f64x8::splat(3.0));
            let t46 = v_rho0 - v_rho1;
            let t47 = t46 * t4;
            let t48 = f64x8::splat(1.0) + t47;
            let t49 = (t48).simd_le(zeta_threshold);
            let t50 = (simd::cbrt(zeta_threshold));
            let t51 = t50 * zeta_threshold;
            let t52 = (simd::cbrt(t48));
            let t54 = ((t49).select(t51, t52 * t48));
            let t55 = f64x8::splat(1.0) - t47;
            let t56 = (t55).simd_le(zeta_threshold);
            let t57 = (simd::cbrt(t55));
            let t59 = ((t56).select(t51, t57 * t55));
            let t61 = f64x8::splat(M_CBRT2);
            let t64 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t61 - f64x8::splat(2.0));
            let t65 = (t54 + t59 - f64x8::splat(2.0)) * t64;
            let t66 = param_hl_c_1;
            let t67 = param_hl_r_1;
            let t68 = t67 * t67;
            let t69 = t68 * t67;
            let t70 = f64x8::splat(1.0) / t69;
            let t73 = f64x8::splat(1.0) + f64x8::splat(3.0) / f64x8::splat(4.0) * t5 * t70;
            let t77 = f64x8::splat(1.0) + t17 * t20 * t67 / f64x8::splat(3.0);
            let t78 = (simd::ln(t77));
            let t80 = f64x8::splat(1.0) / t68;
            let t84 = f64x8::splat(1.0) / t67;
            let t90 = -t66 * (t73 * t78 - t28 * t31 * t80 / f64x8::splat(4.0) + t36 * t39 * t84 / f64x8::splat(8.0) - f64x8::splat(1.0) / f64x8::splat(3.0)) + t45;
            let t91 = t65 * t90;
            let tzk0 = -t45 + t91;
            acc_zk = tzk0;
            let t92 = t3 * t3;
            let t93 = f64x8::splat(1.0) / t92;
            let t94 = t2 * t93;
            let t95 = t9 * t25;
            let t99 = t12 * t14 * t16;
            let t100 = f64x8::splat(1.0) / t24;
            let t101 = t6 * t100;
            let t107 = t18 / t29 / t3;
            let t113 = t37 / t19 / t3;
            let t118 = t1 * (-f64x8::splat(3.0) / f64x8::splat(4.0) * t94 * t95 + t99 * t31 * t101 / f64x8::splat(9.0) + t28 * t107 * t32 / f64x8::splat(6.0) - t36 * t113 * t40 / f64x8::splat(24.0));
            let t119 = t46 * t93;
            let t120 = t4 - t119;
            let t123 = ((t49).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t52 * t120));
            let t124 = -t120;
            let t127 = ((t56).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t57 * t124));
            let t129 = (t123 + t127) * t64;
            let t130 = t129 * t90;
            let t131 = t70 * t78;
            let t135 = t73 * t14 * t16;
            let t136 = f64x8::splat(1.0) / t77;
            let t137 = t67 * t136;
            let t149 = -t66 * (-f64x8::splat(3.0) / f64x8::splat(4.0) * t94 * t131 + t135 * t31 * t137 / f64x8::splat(9.0) + t28 * t107 * t80 / f64x8::splat(6.0) - t36 * t113 * t84 / f64x8::splat(24.0)) + t118;
            let t150 = t65 * t149;
            let tvrho0 = -t45 + t91 + t3 * (-t118 + t130 + t150);
            acc_vrho_0 = tvrho0;
            let t153 = -t4 - t119;
            let t156 = ((t49).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t52 * t153));
            let t157 = -t153;
            let t160 = ((t56).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t57 * t157));
            let t162 = (t156 + t160) * t64;
            let t163 = t162 * t90;
            let tvrho1 = -t45 + t91 + t3 * (-t118 + t163 + t150);
            acc_vrho_1 = tvrho1;
        }
        store_add(zk, ip, m, acc_zk);
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        ip += 8;
    }
}

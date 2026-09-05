//! LDA_X_ERF exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_x_erf.c`
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
pub fn lda_x_erf_exc_pol(
    rho: &[f64],
    zk: &mut [f64],
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
        let v_rho0 = load_strided(rho, ip, np, 2, 0);
        let v_rho1 = load_strided(rho, ip, np, 2, 1);
        let mut acc_zk = V_ZERO;
        {
            let t1 = f64x8::splat(M_CBRT3);
            let t3 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t5 = f64x8::splat(M_CBRT4);
            let t6 = t5 * t5;
            let t7 = t1 * t3 * t6;
            let t8 = f64x8::splat(M_CBRT2);
            let t9 = t8 * t8;
            let t10 = v_rho0 - v_rho1;
            let t11 = v_rho0 + v_rho1;
            let t12 = f64x8::splat(1.0) / t11;
            let t13 = t10 * t12;
            let t14 = f64x8::splat(1.0) + t13;
            let t15 = (t14).simd_le(zeta_threshold);
            let t16 = (simd::cbrt(zeta_threshold));
            let t17 = t16 * zeta_threshold;
            let t18 = (simd::cbrt(t14));
            let t20 = ((t15).select(t17, t18 * t14));
            let t21 = t9 * t20;
            let t22 = (simd::cbrt(t11));
            let t23 = (simd::cbrt(f64x8::splat(9.0)));
            let t24 = t23 * t23;
            let t25 = t3 * t3;
            let t26 = t24 * t25;
            let t27 = t26 * param_hyb_omega_0;
            let t28 = f64x8::splat(1.0) / t22;
            let t29 = t1 * t28;
            let t30 = ((t15).select(t16, t18));
            let t31 = f64x8::splat(1.0) / t30;
            let t34 = t27 * t29 * t31 / f64x8::splat(18.0);
            let t35 = (f64x8::splat(1.35)).simd_le(t34);
            let t36 = (f64x8::splat(1.35)).simd_lt(t34);
            let t37 = ((t36).select(t34, f64x8::splat(1.35)));
            let t38 = t37 * t37;
            let t41 = t38 * t38;
            let t42 = f64x8::splat(1.0) / t41;
            let t44 = t41 * t38;
            let t45 = f64x8::splat(1.0) / t44;
            let t47 = t41 * t41;
            let t48 = f64x8::splat(1.0) / t47;
            let t51 = f64x8::splat(1.0) / t47 / t38;
            let t54 = f64x8::splat(1.0) / t47 / t41;
            let t57 = f64x8::splat(1.0) / t47 / t44;
            let t59 = t47 * t47;
            let t60 = f64x8::splat(1.0) / t59;
            let t63 = ((t36).select(f64x8::splat(1.35), t34));
            let t64 = ((f64x8::splat(M_PI)).sqrt());
            let t65 = f64x8::splat(1.0) / t63;
            let t67 = (simd::erf(t65 / f64x8::splat(2.0)));
            let t69 = t63 * t63;
            let t70 = f64x8::splat(1.0) / t69;
            let t72 = (simd::exp(-t70 / f64x8::splat(4.0)));
            let t73 = t72 - f64x8::splat(1.0);
            let t76 = t72 - f64x8::splat(3.0) / f64x8::splat(2.0) - f64x8::splat(2.0) * t69 * t73;
            let t79 = f64x8::splat(2.0) * t63 * t76 + t64 * t67;
            let t83 = ((t35).select(f64x8::splat(1.0) / t38 / f64x8::splat(36.0) - t42 / f64x8::splat(960.0) + t45 / f64x8::splat(26880.0) - t48 / f64x8::splat(829440.0) + t51 / f64x8::splat(28385280.0) - t54 / f64x8::splat(1073479680.0) + t57 / f64x8::splat(44590694400.0) - t60 / f64x8::splat(2021444812800.0), f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t63 * t79));
            let t84 = t22 * t83;
            let t86 = t7 * t21 * t84;
            let t87 = f64x8::splat(1.0) - t13;
            let t88 = (t87).simd_le(zeta_threshold);
            let t89 = (simd::cbrt(t87));
            let t91 = ((t88).select(t17, t89 * t87));
            let t92 = t9 * t91;
            let t93 = ((t88).select(t16, t89));
            let t94 = f64x8::splat(1.0) / t93;
            let t97 = t27 * t29 * t94 / f64x8::splat(18.0);
            let t98 = (f64x8::splat(1.35)).simd_le(t97);
            let t99 = (f64x8::splat(1.35)).simd_lt(t97);
            let t100 = ((t99).select(t97, f64x8::splat(1.35)));
            let t101 = t100 * t100;
            let t104 = t101 * t101;
            let t105 = f64x8::splat(1.0) / t104;
            let t107 = t104 * t101;
            let t108 = f64x8::splat(1.0) / t107;
            let t110 = t104 * t104;
            let t111 = f64x8::splat(1.0) / t110;
            let t114 = f64x8::splat(1.0) / t110 / t101;
            let t117 = f64x8::splat(1.0) / t110 / t104;
            let t120 = f64x8::splat(1.0) / t110 / t107;
            let t122 = t110 * t110;
            let t123 = f64x8::splat(1.0) / t122;
            let t126 = ((t99).select(f64x8::splat(1.35), t97));
            let t127 = f64x8::splat(1.0) / t126;
            let t129 = (simd::erf(t127 / f64x8::splat(2.0)));
            let t131 = t126 * t126;
            let t132 = f64x8::splat(1.0) / t131;
            let t134 = (simd::exp(-t132 / f64x8::splat(4.0)));
            let t135 = t134 - f64x8::splat(1.0);
            let t138 = t134 - f64x8::splat(3.0) / f64x8::splat(2.0) - f64x8::splat(2.0) * t131 * t135;
            let t141 = f64x8::splat(2.0) * t126 * t138 + t64 * t129;
            let t145 = ((t98).select(f64x8::splat(1.0) / t101 / f64x8::splat(36.0) - t105 / f64x8::splat(960.0) + t108 / f64x8::splat(26880.0) - t111 / f64x8::splat(829440.0) + t114 / f64x8::splat(28385280.0) - t117 / f64x8::splat(1073479680.0) + t120 / f64x8::splat(44590694400.0) - t123 / f64x8::splat(2021444812800.0), f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t126 * t141));
            let t146 = t22 * t145;
            let t148 = t7 * t92 * t146;
            let tzk0 = -f64x8::splat(3.0) / f64x8::splat(32.0) * t86 - f64x8::splat(3.0) / f64x8::splat(32.0) * t148;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}

//! LDA_C_W20 exc pol kernel — explicit SIMD (bit-exact).
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
pub fn lda_c_w20_exc_pol(
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
            let t20 = v_rho0 + v_rho1;
            let t21 = (simd::cbrt(t20));
            let t22 = t21 * t21;
            let t23 = f64x8::splat(1.0) / t22;
            let t25 = t18 * t19 * t23;
            let t27 = (simd::exp(-t25 / f64x8::splat(40000.0)));
            let t28 = f64x8::splat(1.0) - t27;
            let t29 = f64x8::splat(M_CBRTPI);
            let t30 = t29 * t29;
            let t32 = (simd::cbrt(f64x8::splat(9.0)));
            let t33 = f64x8::splat(1.0) / t30 * t32;
            let t34 = t19 * t19;
            let t40 = t12 / f64x8::splat(2.0);
            let t41 = (-f64x8::splat(0.9) + f64x8::splat(3.0) / f64x8::splat(16.0) * t33 * t34) * t8 * t3 + t40;
            let t45 = (-f64x8::splat(2.0) * t28 * t41 + t12) * t14;
            let t46 = f64x8::splat(1.0) / t16;
            let t47 = t46 * t19;
            let t48 = t47 * t21;
            let t51 = t28 * t8;
            let t52 = ((f64x8::splat(4.0)).sqrt());
            let t53 = t13 * t16;
            let t54 = f64x8::splat(1.0) / t21;
            let t56 = t53 * t34 * t54;
            let t57 = ((t56).sqrt());
            let t59 = f64x8::splat(1.0) / t57 / t56;
            let t61 = t51 * t52 * t59;
            let t63 = t32 * t32;
            let t64 = t63 * t19;
            let t65 = t30 * t3;
            let t69 = -f64x8::splat(3.0) / f64x8::splat(40.0) * t64 * t65 * t8 + t40;
            let t73 = (-f64x8::splat(2.0) * t28 * t69 + t12) * t13;
            let t74 = f64x8::splat(1.0) / t17;
            let t75 = t74 * t34;
            let t76 = t75 * t22;
            let t79 = f64x8::splat(1.0) + t45 * t48 / f64x8::splat(3.0) - f64x8::splat(118.43525281307231) * t61 + t73 * t76 / f64x8::splat(3.0);
            let t80 = (simd::ln(t79));
            let t82 = t5 * t80 / f64x8::splat(2.0);
            let t83 = t53 * t34;
            let t84 = t54 * t27;
            let t85 = ((f64x8::splat(4.0)).sqrt().sqrt());
            let t86 = t85 * t85;
            let t87 = t86 * t85;
            let t88 = ((t56).sqrt().sqrt());
            let t92 = t27 + f64x8::splat(5.0) / f64x8::splat(8.0) * t87 * t88 * t56;
            let t93 = f64x8::splat(1.0) / t92;
            let t94 = t3 * f64x8::splat(M_PI);
            let t96 = f64x8::splat(1.0) / t29 / t94;
            let t98 = f64x8::splat(12.0) * t1;
            let t99 = f64x8::splat(7.0) / f64x8::splat(6.0) * t3 - t98 - f64x8::splat(1.0);
            let t100 = t96 * t99;
            let t101 = t14 * t46;
            let t105 = f64x8::splat(1.0) + t101 * t19 * t21 / f64x8::splat(3.0);
            let t106 = (simd::ln(t105));
            let t110 = -t64 * t100 * t106 / f64x8::splat(36.0) - f64x8::splat(0.01);
            let t111 = t93 * t110;
            let t114 = t83 * t84 * t111 / f64x8::splat(4.0);
            let t119 = (simd::exp(-f64x8::splat(4.0) * (-f64x8::splat(0.1412623711751798) + t6) * t8 * t3));
            let t120 = f64x8::splat(M_CBRT2);
            let t128 = t119 / f64x8::splat(2.0);
            let t129 = f64x8::splat(2.0) * (-f64x8::splat(0.9) + f64x8::splat(3.0) / f64x8::splat(16.0) * t33 * t34 * t120) * t8 * t3 + t128;
            let t133 = (-f64x8::splat(2.0) * t129 * t28 + t119) * t14;
            let t137 = t120 * t120;
            let t142 = -f64x8::splat(3.0) / f64x8::splat(20.0) * t64 * t65 * t137 * t8 + t128;
            let t146 = (-f64x8::splat(2.0) * t142 * t28 + t119) * t13;
            let t149 = f64x8::splat(1.0) + t133 * t48 / f64x8::splat(3.0) - f64x8::splat(236.87050562614462) * t61 + t146 * t76 / f64x8::splat(3.0);
            let t150 = (simd::ln(t149));
            let t155 = t137 * t63;
            let t157 = f64x8::splat(13.0) / f64x8::splat(12.0) * t3 - t98 + f64x8::splat(1.0) / f64x8::splat(2.0);
            let t158 = t96 * t157;
            let t160 = t155 * t158 * t106;
            let t163 = -t5 * t150 / f64x8::splat(4.0) - t53 * t84 * t93 * t160 / f64x8::splat(144.0) + t82 - t114;
            let t164 = v_rho0 - v_rho1;
            let t165 = f64x8::splat(1.0) / t20;
            let t166 = t164 * t165;
            let t167 = f64x8::splat(1.0) + t166;
            let t168 = (t167).simd_le(zeta_threshold);
            let t169 = (simd::cbrt(zeta_threshold));
            let t170 = t169 * zeta_threshold;
            let t171 = (simd::cbrt(t167));
            let t173 = ((t168).select(t170, t171 * t167));
            let t174 = f64x8::splat(1.0) - t166;
            let t175 = (t174).simd_le(zeta_threshold);
            let t176 = (simd::cbrt(t174));
            let t178 = ((t175).select(t170, t176 * t174));
            let t179 = t173 + t178 - f64x8::splat(2.0);
            let t183 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t120 - f64x8::splat(2.0));
            let t184 = t163 * t179 * t183;
            let tzk0 = -t82 + t114 + t184;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}

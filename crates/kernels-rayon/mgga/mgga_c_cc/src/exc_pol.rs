//! MGGA_C_CC exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_cc.c`
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
pub fn mgga_c_cc_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
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
        let v_sigma0 = load_strided(sigma, ip, np, 3, 0);
        let v_sigma1 = load_strided(sigma, ip, np, 3, 1);
        let v_sigma2 = load_strided(sigma, ip, np, 3, 2);
        let v_lapl0 = load_strided(lapl, ip, np, 2, 0);
        let v_lapl1 = load_strided(lapl, ip, np, 2, 1);
        let v_tau0 = load_strided(tau, ip, np, 2, 0);
        let v_tau1 = load_strided(tau, ip, np, 2, 1);
        let mut acc_zk = V_ZERO;
        {
            let t3 = v_sigma0 + f64x8::splat(2.0) * v_sigma1 + v_sigma2;
            let t4 = v_rho0 + v_rho1;
            let t5 = t4 * t4;
            let t6 = t5 * t5;
            let t7 = (simd::cbrt(t4));
            let t8 = t7 * t7;
            let t10 = f64x8::splat(1.0) / t8 / t6;
            let t11 = t3 * t10;
            let t12 = (simd::cbrt(v_rho0));
            let t13 = t12 * t12;
            let t15 = f64x8::splat(1.0) / t13 / v_rho0;
            let t16 = v_tau0 * t15;
            let t17 = v_rho0 - v_rho1;
            let t18 = f64x8::splat(1.0) / t4;
            let t19 = t17 * t18;
            let t20 = f64x8::splat(1.0) + t19;
            let t21 = t20 / f64x8::splat(2.0);
            let t22 = (simd::cbrt(t21));
            let t23 = t22 * t22;
            let t24 = t23 * t21;
            let t26 = (simd::cbrt(v_rho1));
            let t27 = t26 * t26;
            let t29 = f64x8::splat(1.0) / t27 / v_rho1;
            let t30 = v_tau1 * t29;
            let t31 = f64x8::splat(1.0) - t19;
            let t32 = t31 / f64x8::splat(2.0);
            let t33 = (simd::cbrt(t32));
            let t34 = t33 * t33;
            let t35 = t34 * t32;
            let t37 = t16 * t24 + t30 * t35;
            let t38 = f64x8::splat(1.0) / t37;
            let t39 = t17 * t17;
            let t40 = t38 * t39;
            let t43 = f64x8::splat(1.0) - t11 * t40 / f64x8::splat(8.0);
            let t44 = f64x8::splat(M_CBRT3);
            let t45 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t46 = (simd::cbrt(t45));
            let t47 = t44 * t46;
            let t48 = f64x8::splat(M_CBRT4);
            let t49 = t48 * t48;
            let t52 = t47 * t49 / t7;
            let t54 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t52;
            let t55 = ((t52).sqrt());
            let t58 = ((t52) * (t52).sqrt());
            let t60 = t44 * t44;
            let t61 = t46 * t46;
            let t62 = t60 * t61;
            let t65 = t62 * t48 / t8;
            let t67 = f64x8::splat(3.79785) * t55 + f64x8::splat(0.8969) * t52 + f64x8::splat(0.204775) * t58 + f64x8::splat(0.123235) * t65;
            let t70 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t67;
            let t71 = (simd::ln(t70));
            let t73 = f64x8::splat(0.0621814) * t54 * t71;
            let t74 = t39 * t39;
            let t75 = f64x8::splat(1.0) / t6;
            let t76 = t74 * t75;
            let t77 = (t20).simd_le(zeta_threshold);
            let t78 = (simd::cbrt(zeta_threshold));
            let t79 = t78 * zeta_threshold;
            let t80 = (simd::cbrt(t20));
            let t82 = ((t77).select(t79, t80 * t20));
            let t83 = (t31).simd_le(zeta_threshold);
            let t84 = (simd::cbrt(t31));
            let t86 = ((t83).select(t79, t84 * t31));
            let t87 = t82 + t86 - f64x8::splat(2.0);
            let t88 = f64x8::splat(M_CBRT2);
            let t91 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t88 - f64x8::splat(2.0));
            let t92 = t87 * t91;
            let t94 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t52;
            let t99 = f64x8::splat(7.05945) * t55 + f64x8::splat(1.549425) * t52 + f64x8::splat(0.420775) * t58 + f64x8::splat(0.1562925) * t65;
            let t102 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t99;
            let t103 = (simd::ln(t102));
            let t107 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t52;
            let t112 = f64x8::splat(5.1785) * t55 + f64x8::splat(0.905775) * t52 + f64x8::splat(0.1100325) * t58 + f64x8::splat(0.1241775) * t65;
            let t115 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t112;
            let t116 = (simd::ln(t115));
            let t117 = t107 * t116;
            let t119 = -f64x8::splat(0.0310907) * t94 * t103 + t73 - f64x8::splat(0.0197516734986138) * t117;
            let t120 = t92 * t119;
            let t124 = -t73 + t76 * t120 + f64x8::splat(0.0197516734986138) * t92 * t117;
            let tzk0 = t43 * t124;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}

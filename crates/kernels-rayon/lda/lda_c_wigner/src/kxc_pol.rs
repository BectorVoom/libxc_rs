//! LDA_C_WIGNER kxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_wigner.c`
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
pub fn lda_c_wigner_kxc_pol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    v3rho3: &mut [f64],
    param_a: f64,
    param_b: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_a = f64x8::splat(param_a);
    let param_b = f64x8::splat(param_b);
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
        let mut acc_v2rho2_0 = V_ZERO;
        let mut acc_v2rho2_1 = V_ZERO;
        let mut acc_v2rho2_2 = V_ZERO;
        let mut acc_v3rho3_0 = V_ZERO;
        let mut acc_v3rho3_1 = V_ZERO;
        let mut acc_v3rho3_2 = V_ZERO;
        let mut acc_v3rho3_3 = V_ZERO;
        {
            let t1 = v_rho0 - v_rho1;
            let t2 = t1 * t1;
            let t3 = v_rho0 + v_rho1;
            let t4 = t3 * t3;
            let t5 = f64x8::splat(1.0) / t4;
            let t7 = -t2 * t5 + f64x8::splat(1.0);
            let t8 = t7 * param_a;
            let t9 = f64x8::splat(M_CBRT3);
            let t10 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t11 = (simd::cbrt(t10));
            let t12 = t9 * t11;
            let t13 = f64x8::splat(M_CBRT4);
            let t14 = t13 * t13;
            let t15 = (simd::cbrt(t3));
            let t16 = f64x8::splat(1.0) / t15;
            let t20 = param_b + t12 * t14 * t16 / f64x8::splat(4.0);
            let t21 = f64x8::splat(1.0) / t20;
            let tzk0 = t8 * t21;
            acc_zk = tzk0;
            let t22 = t1 * t5;
            let t23 = t4 * t3;
            let t24 = f64x8::splat(1.0) / t23;
            let t25 = t2 * t24;
            let t27 = -f64x8::splat(2.0) * t22 + f64x8::splat(2.0) * t25;
            let t29 = param_a * t21;
            let t33 = t20 * t20;
            let t34 = f64x8::splat(1.0) / t33;
            let t36 = t11 * t14;
            let t37 = t34 * t9 * t36;
            let t39 = t16 * t7 * param_a * t37 / f64x8::splat(12.0);
            let tvrho0 = t3 * t27 * t29 + t39 + tzk0;
            acc_vrho_0 = tvrho0;
            let t41 = f64x8::splat(2.0) * t22 + f64x8::splat(2.0) * t25;
            let tvrho1 = t3 * t41 * t29 + t39 + tzk0;
            acc_vrho_1 = tvrho1;
            let t44 = t27 * param_a;
            let t45 = t44 * t21;
            let t47 = t8 * t34;
            let t51 = t12 * t14 / t15 / t3;
            let t53 = t47 * t51 / f64x8::splat(18.0);
            let t54 = f64x8::splat(2.0) * t5;
            let t56 = f64x8::splat(8.0) * t1 * t24;
            let t57 = t4 * t4;
            let t58 = f64x8::splat(1.0) / t57;
            let t60 = f64x8::splat(6.0) * t2 * t58;
            let t61 = -t54 + t56 - t60;
            let t66 = t16 * t27 * param_a * t37;
            let t68 = t15 * t15;
            let t70 = f64x8::splat(1.0) / t68 / t3;
            let t74 = f64x8::splat(1.0) / t33 / t20;
            let t75 = t9 * t9;
            let t77 = t11 * t11;
            let t79 = t74 * t75 * t77 * t13;
            let t81 = t70 * t7 * param_a * t79 / f64x8::splat(18.0);
            let tv2rho20 = f64x8::splat(2.0) * t45 + t53 + t3 * t61 * t29 + t66 / f64x8::splat(6.0) + t81;
            acc_v2rho2_0 = tv2rho20;
            let t82 = t41 * param_a;
            let t83 = t82 * t21;
            let t84 = t54 - t60;
            let t89 = t16 * t41 * param_a * t37;
            let tv2rho21 = t45 + t53 + t83 + t3 * t84 * t29 + t89 / f64x8::splat(12.0) + t66 / f64x8::splat(12.0) + t81;
            acc_v2rho2_1 = tv2rho21;
            let t93 = -t54 - t56 - t60;
            let tv2rho22 = f64x8::splat(2.0) * t83 + t53 + t3 * t93 * t29 + t89 / f64x8::splat(6.0) + t81;
            acc_v2rho2_2 = tv2rho22;
            let t97 = t61 * param_a;
            let t98 = t97 * t21;
            let t100 = t44 * t34;
            let t101 = t100 * t51;
            let t103 = t8 * t74;
            let t104 = t75 * t77;
            let t108 = t104 * t13 / t68 / t4;
            let t110 = t103 * t108 / f64x8::splat(18.0);
            let t114 = t12 * t14 / t15 / t4;
            let t116 = f64x8::splat(2.0) / f64x8::splat(27.0) * t47 * t114;
            let t117 = f64x8::splat(12.0) * t24;
            let t118 = t1 * t58;
            let t119 = f64x8::splat(36.0) * t118;
            let t121 = f64x8::splat(1.0) / t57 / t3;
            let t123 = f64x8::splat(24.0) * t2 * t121;
            let t124 = t117 - t119 + t123;
            let t129 = t16 * t61 * param_a * t37;
            let t133 = t70 * t27 * param_a * t79;
            let t136 = t33 * t33;
            let t137 = f64x8::splat(1.0) / t136;
            let t139 = param_a * t137 * t10;
            let t141 = t24 * t7 * t139 / f64x8::splat(6.0);
            let tv3rho30 = f64x8::splat(3.0) * t98 + t101 / f64x8::splat(6.0) - t110 - t116 + t3 * t124 * t29 + t129 / f64x8::splat(4.0) + t133 / f64x8::splat(6.0) + t141;
            acc_v3rho3_0 = tv3rho30;
            let t143 = t84 * param_a;
            let t145 = f64x8::splat(2.0) * t143 * t21;
            let t146 = t82 * t34;
            let t147 = t146 * t51;
            let t149 = f64x8::splat(4.0) * t24;
            let t150 = f64x8::splat(12.0) * t118;
            let t151 = -t149 - t150 + t123;
            let t157 = t16 * t84 * param_a * t37 / f64x8::splat(6.0);
            let t160 = t70 * t41 * param_a * t79;
            let tv3rho31 = t98 + t101 / f64x8::splat(9.0) - t110 - t116 + t145 + t147 / f64x8::splat(18.0) + t3 * t151 * t29 + t157 + t160 / f64x8::splat(18.0) + t129 / f64x8::splat(12.0) + t133 / f64x8::splat(9.0) + t141;
            acc_v3rho3_1 = tv3rho31;
            let t166 = t93 * param_a;
            let t167 = t166 * t21;
            let t168 = -t149 + t150 + t123;
            let t173 = t16 * t93 * param_a * t37;
            let tv3rho32 = t145 + t147 / f64x8::splat(9.0) + t101 / f64x8::splat(18.0) - t110 - t116 + t167 + t3 * t168 * t29 + t173 / f64x8::splat(12.0) + t157 + t160 / f64x8::splat(9.0) + t133 / f64x8::splat(18.0) + t141;
            acc_v3rho3_2 = tv3rho32;
            let t179 = t117 + t119 + t123;
            let tv3rho33 = f64x8::splat(3.0) * t167 + t147 / f64x8::splat(6.0) - t110 - t116 + t3 * t179 * t29 + t173 / f64x8::splat(4.0) + t160 / f64x8::splat(6.0) + t141;
            acc_v3rho3_3 = tv3rho33;
        }
        store_add(zk, ip, m, acc_zk);
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(v2rho2, ip, m, 3, 0, acc_v2rho2_0);
        store_strided(v2rho2, ip, m, 3, 1, acc_v2rho2_1);
        store_strided(v2rho2, ip, m, 3, 2, acc_v2rho2_2);
        store_strided(v3rho3, ip, m, 4, 0, acc_v3rho3_0);
        store_strided(v3rho3, ip, m, 4, 1, acc_v3rho3_1);
        store_strided(v3rho3, ip, m, 4, 2, acc_v3rho3_2);
        store_strided(v3rho3, ip, m, 4, 3, acc_v3rho3_3);
        ip += 8;
    }
}

//! GGA_C_P86 exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_p86.c`
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
pub fn gga_c_p86_exc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_aa: f64,
    param_bb: f64,
    param_ftilde: f64,
    param_malpha: f64,
    param_mbeta: f64,
    param_mgamma: f64,
    param_mdelta: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_aa = f64x8::splat(param_aa);
    let param_bb = f64x8::splat(param_bb);
    let param_ftilde = f64x8::splat(param_ftilde);
    let param_malpha = f64x8::splat(param_malpha);
    let param_mbeta = f64x8::splat(param_mbeta);
    let param_mgamma = f64x8::splat(param_mgamma);
    let param_mdelta = f64x8::splat(param_mdelta);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let mut acc_zk = V_ZERO;
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
            let t12 = (f64x8::splat(1.0)).simd_le(t11);
            let t13 = ((t10).sqrt());
            let t16 = f64x8::splat(1.0) + f64x8::splat(0.52645) * t13 + f64x8::splat(0.08335) * t10;
            let t19 = (simd::ln(t11));
            let t22 = t4 * t9 * t19;
            let t26 = ((t12).select(-f64x8::splat(0.1423) / t16, f64x8::splat(0.0311) * t19 - f64x8::splat(0.048) + f64x8::splat(0.0005) * t22 - f64x8::splat(0.0029) * t10));
            let t29 = f64x8::splat(1.0) + f64x8::splat(0.69905) * t13 + f64x8::splat(0.065275) * t10;
            let t36 = ((t12).select(-f64x8::splat(0.0843) / t29, f64x8::splat(0.01555) * t19 - f64x8::splat(0.0269) + f64x8::splat(0.000175) * t22 - f64x8::splat(0.0012) * t10));
            let t38 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t39 = (simd::cbrt(zeta_threshold));
            let t41 = ((t38).select(t39 * zeta_threshold, f64x8::splat(1.0)));
            let t43 = f64x8::splat(2.0) * t41 - f64x8::splat(2.0);
            let t45 = f64x8::splat(M_CBRT2);
            let t48 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t45 - f64x8::splat(2.0));
            let t49 = (t36 - t26) * t43 * t48;
            let t50 = v_rho * v_rho;
            let t52 = f64x8::splat(1.0) / t7 / t50;
            let t53 = v_sigma * t52;
            let t54 = param_aa + param_bb;
            let t55 = param_ftilde * t54;
            let t56 = param_malpha * t1;
            let t57 = t3 * t6;
            let t58 = t57 * t8;
            let t61 = t1 * t1;
            let t62 = param_mbeta * t61;
            let t63 = t3 * t3;
            let t64 = t63 * t5;
            let t65 = t7 * t7;
            let t66 = f64x8::splat(1.0) / t65;
            let t67 = t64 * t66;
            let t70 = param_bb + t56 * t58 / f64x8::splat(4.0) + t62 * t67 / f64x8::splat(4.0);
            let t71 = param_mgamma * t1;
            let t74 = param_mdelta * t61;
            let t77 = f64x8::splat(1.0) / v_rho;
            let t80 = f64x8::splat(1.0) + t71 * t58 / f64x8::splat(4.0) + t74 * t67 / f64x8::splat(4.0) + f64x8::splat(2387.32414637843) * param_mbeta * t77;
            let t81 = f64x8::splat(1.0) / t80;
            let t83 = t70 * t81 + param_aa;
            let t84 = f64x8::splat(1.0) / t83;
            let t85 = ((v_sigma).sqrt());
            let t86 = t84 * t85;
            let t87 = (simd::pow(v_rho, f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t89 = f64x8::splat(1.0) / t87 / v_rho;
            let t92 = (simd::exp(-t55 * t86 * t89));
            let t94 = t39 * t39;
            let t96 = ((t38).select(t94 * zeta_threshold, f64x8::splat(1.0)));
            let t97 = ((t96).sqrt());
            let t98 = f64x8::splat(1.0) / t97;
            let t99 = t92 * t83 * t98;
            let t100 = t53 * t99;
            let tzk0 = t26 + t49 + t100;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}

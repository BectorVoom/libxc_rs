//! MGGA_X_RLDA vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_rlda.c`
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
pub fn mgga_x_rlda_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_prefactor: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_prefactor = f64x8::splat(param_prefactor);
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
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        let mut acc_vsigma_0 = V_ZERO;
        let mut acc_vsigma_1 = V_ZERO;
        let mut acc_vsigma_2 = V_ZERO;
        let mut acc_vlapl_0 = V_ZERO;
        let mut acc_vlapl_1 = V_ZERO;
        let mut acc_vtau_0 = V_ZERO;
        let mut acc_vtau_1 = V_ZERO;
        {
            let t2 = (v_rho0).simd_le(dens_threshold);
            let t3 = f64x8::splat(M_CBRTPI);
            let t4 = t3 * t3;
            let t5 = v_rho0 + v_rho1;
            let t6 = f64x8::splat(1.0) / t5;
            let t9 = (f64x8::splat(2.0) * v_rho0 * t6).simd_le(zeta_threshold);
            let t10 = zeta_threshold - f64x8::splat(1.0);
            let t13 = (f64x8::splat(2.0) * v_rho1 * t6).simd_le(zeta_threshold);
            let t14 = -t10;
            let t15 = v_rho0 - v_rho1;
            let t17 = ((t9).select(t10, (t13).select(t14, t15 * t6)));
            let t18 = f64x8::splat(1.0) + t17;
            let t19 = (t18).simd_le(zeta_threshold);
            let t20 = (simd::cbrt(zeta_threshold));
            let t21 = t20 * zeta_threshold;
            let t22 = (simd::cbrt(t18));
            let t24 = ((t19).select(t21, t22 * t18));
            let t25 = t4 * t24;
            let t26 = (simd::cbrt(t5));
            let t29 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t30 = f64x8::splat(1.0) / t29;
            let t31 = param_prefactor * t30;
            let t32 = f64x8::splat(M_CBRT4);
            let t33 = (simd::cbrt(v_rho0));
            let t34 = t33 * t33;
            let t36 = f64x8::splat(1.0) / t34 / v_rho0;
            let t41 = f64x8::splat(2.0) * v_tau0 * t36 - v_lapl0 * t36 / f64x8::splat(4.0);
            let t44 = t31 * t32 / t41;
            let t47 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(15.0) / f64x8::splat(16.0) * t25 * t26 * t44));
            let t48 = (v_rho1).simd_le(dens_threshold);
            let t49 = -t15;
            let t51 = ((t13).select(t10, (t9).select(t14, t49 * t6)));
            let t52 = f64x8::splat(1.0) + t51;
            let t53 = (t52).simd_le(zeta_threshold);
            let t54 = (simd::cbrt(t52));
            let t56 = ((t53).select(t21, t54 * t52));
            let t57 = t4 * t56;
            let t59 = (simd::cbrt(v_rho1));
            let t60 = t59 * t59;
            let t62 = f64x8::splat(1.0) / t60 / v_rho1;
            let t67 = f64x8::splat(2.0) * v_tau1 * t62 - v_lapl1 * t62 / f64x8::splat(4.0);
            let t70 = t31 * t32 / t67;
            let t73 = ((t48).select(f64x8::splat(0.0), -f64x8::splat(15.0) / f64x8::splat(16.0) * t57 * t26 * t70));
            let tzk0 = t47 + t73;
            acc_zk = tzk0;
            let t74 = t5 * t5;
            let t75 = f64x8::splat(1.0) / t74;
            let t76 = t15 * t75;
            let t78 = ((t9).select(f64x8::splat(0.0), (t13).select(f64x8::splat(0.0), t6 - t76)));
            let t81 = ((t19).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t22 * t78));
            let t82 = t4 * t81;
            let t86 = t26 * t26;
            let t87 = f64x8::splat(1.0) / t86;
            let t90 = f64x8::splat(5.0) / f64x8::splat(16.0) * t25 * t87 * t44;
            let t91 = t26 * param_prefactor;
            let t92 = t25 * t91;
            let t93 = t30 * t32;
            let t94 = t41 * t41;
            let t95 = f64x8::splat(1.0) / t94;
            let t96 = v_rho0 * v_rho0;
            let t98 = f64x8::splat(1.0) / t34 / t96;
            let t103 = -f64x8::splat(10.0) / f64x8::splat(3.0) * v_tau0 * t98 + f64x8::splat(5.0) / f64x8::splat(12.0) * v_lapl0 * t98;
            let t105 = t93 * t95 * t103;
            let t109 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(15.0) / f64x8::splat(16.0) * t82 * t26 * t44 - t90 + f64x8::splat(15.0) / f64x8::splat(16.0) * t92 * t105));
            let t110 = t49 * t75;
            let t112 = ((t13).select(f64x8::splat(0.0), (t9).select(f64x8::splat(0.0), -t6 - t110)));
            let t115 = ((t53).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t54 * t112));
            let t116 = t4 * t115;
            let t122 = f64x8::splat(5.0) / f64x8::splat(16.0) * t57 * t87 * t70;
            let t124 = ((t48).select(f64x8::splat(0.0), -f64x8::splat(15.0) / f64x8::splat(16.0) * t116 * t26 * t70 - t122));
            let tvrho0 = t47 + t73 + t5 * (t109 + t124);
            acc_vrho_0 = tvrho0;
            let t128 = ((t9).select(f64x8::splat(0.0), (t13).select(f64x8::splat(0.0), -t6 - t76)));
            let t131 = ((t19).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t22 * t128));
            let t132 = t4 * t131;
            let t137 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(15.0) / f64x8::splat(16.0) * t132 * t26 * t44 - t90));
            let t139 = ((t13).select(f64x8::splat(0.0), (t9).select(f64x8::splat(0.0), t6 - t110)));
            let t142 = ((t53).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t54 * t139));
            let t143 = t4 * t142;
            let t147 = t57 * t91;
            let t148 = t67 * t67;
            let t149 = f64x8::splat(1.0) / t148;
            let t150 = v_rho1 * v_rho1;
            let t152 = f64x8::splat(1.0) / t60 / t150;
            let t157 = -f64x8::splat(10.0) / f64x8::splat(3.0) * v_tau1 * t152 + f64x8::splat(5.0) / f64x8::splat(12.0) * v_lapl1 * t152;
            let t159 = t93 * t149 * t157;
            let t163 = ((t48).select(f64x8::splat(0.0), -f64x8::splat(15.0) / f64x8::splat(16.0) * t143 * t26 * t70 - t122 + f64x8::splat(15.0) / f64x8::splat(16.0) * t147 * t159));
            let tvrho1 = t47 + t73 + t5 * (t137 + t163);
            acc_vrho_1 = tvrho1;
            let tvsigma0 = f64x8::splat(0.0);
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let tvsigma2 = f64x8::splat(0.0);
            acc_vsigma_2 = tvsigma2;
            let t167 = t93 * t95 * t36;
            let t168 = t92 * t167;
            let t170 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(15.0) / f64x8::splat(64.0) * t168));
            let tvlapl0 = t5 * t170;
            acc_vlapl_0 = tvlapl0;
            let t172 = t93 * t149 * t62;
            let t173 = t147 * t172;
            let t175 = ((t48).select(f64x8::splat(0.0), -f64x8::splat(15.0) / f64x8::splat(64.0) * t173));
            let tvlapl1 = t5 * t175;
            acc_vlapl_1 = tvlapl1;
            let t177 = ((t2).select(f64x8::splat(0.0), f64x8::splat(15.0) / f64x8::splat(8.0) * t168));
            let tvtau0 = t5 * t177;
            acc_vtau_0 = tvtau0;
            let t179 = ((t48).select(f64x8::splat(0.0), f64x8::splat(15.0) / f64x8::splat(8.0) * t173));
            let tvtau1 = t5 * t179;
            acc_vtau_1 = tvtau1;
        }
        store_add(zk, ip, m, acc_zk);
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(vsigma, ip, m, 3, 0, acc_vsigma_0);
        store_strided(vsigma, ip, m, 3, 1, acc_vsigma_1);
        store_strided(vsigma, ip, m, 3, 2, acc_vsigma_2);
        store_strided(vlapl, ip, m, 2, 0, acc_vlapl_0);
        store_strided(vlapl, ip, m, 2, 1, acc_vlapl_1);
        store_strided(vtau, ip, m, 2, 0, acc_vtau_0);
        store_strided(vtau, ip, m, 2, 1, acc_vtau_1);
        ip += 8;
    }
}

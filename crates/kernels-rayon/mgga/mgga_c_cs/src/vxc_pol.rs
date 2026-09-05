//! MGGA_C_CS vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_cs.c`
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
pub fn mgga_c_cs_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
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
            let t2 = v_rho0 - v_rho1;
            let t3 = t2 * t2;
            let t4 = v_rho0 + v_rho1;
            let t5 = t4 * t4;
            let t6 = f64x8::splat(1.0) / t5;
            let t8 = -t3 * t6 + f64x8::splat(1.0);
            let t9 = (simd::cbrt(t4));
            let t10 = f64x8::splat(1.0) / t9;
            let t12 = f64x8::splat(1.0) + f64x8::splat(0.349) * t10;
            let t13 = f64x8::splat(1.0) / t12;
            let t14 = t8 * t13;
            let t16 = (simd::exp(-f64x8::splat(0.2533) * t10));
            let t17 = f64x8::splat(1.0) / t4;
            let t18 = t2 * t17;
            let t19 = f64x8::splat(1.0) + t18;
            let t20 = (t19).simd_le(zeta_threshold);
            let t21 = zeta_threshold * zeta_threshold;
            let t22 = (simd::cbrt(zeta_threshold));
            let t23 = t22 * t22;
            let t24 = t23 * t21;
            let t25 = t19 * t19;
            let t26 = (simd::cbrt(t19));
            let t27 = t26 * t26;
            let t29 = ((t20).select(t24, t27 * t25));
            let t30 = f64x8::splat(M_CBRT2);
            let t31 = t29 * t30;
            let t32 = (simd::cbrt(v_rho0));
            let t33 = t32 * t32;
            let t35 = f64x8::splat(1.0) / t33 / v_rho0;
            let t37 = v_lapl0 * t35;
            let t39 = v_tau0 * t35 - t37 / f64x8::splat(8.0);
            let t41 = f64x8::splat(1.0) - t18;
            let t42 = (t41).simd_le(zeta_threshold);
            let t43 = t41 * t41;
            let t44 = (simd::cbrt(t41));
            let t45 = t44 * t44;
            let t47 = ((t42).select(t24, t45 * t43));
            let t48 = t47 * t30;
            let t49 = (simd::cbrt(v_rho1));
            let t50 = t49 * t49;
            let t52 = f64x8::splat(1.0) / t50 / v_rho1;
            let t54 = v_lapl1 * t52;
            let t56 = v_tau1 * t52 - t54 / f64x8::splat(8.0);
            let t59 = v_sigma0 + f64x8::splat(2.0) * v_sigma1 + v_sigma2;
            let t60 = t9 * t9;
            let t62 = f64x8::splat(1.0) / t60 / t5;
            let t64 = t19 / f64x8::splat(2.0);
            let t65 = (simd::cbrt(t64));
            let t66 = t65 * t65;
            let t67 = t66 * t64;
            let t69 = t41 / f64x8::splat(2.0);
            let t70 = (simd::cbrt(t69));
            let t71 = t70 * t70;
            let t72 = t71 * t69;
            let t75 = t31 * t39 / f64x8::splat(8.0) + t37 * t67 / f64x8::splat(8.0) + t48 * t56 / f64x8::splat(8.0) + t54 * t72 / f64x8::splat(8.0) - t59 * t62 / f64x8::splat(8.0);
            let t78 = f64x8::splat(1.0) + f64x8::splat(0.264) * t16 * t75;
            let tzk0 = -f64x8::splat(0.04918) * t14 * t78;
            acc_zk = tzk0;
            let t81 = t2 * t6;
            let t82 = t5 * t4;
            let t83 = f64x8::splat(1.0) / t82;
            let t84 = t3 * t83;
            let t86 = -f64x8::splat(2.0) * t81 + f64x8::splat(2.0) * t84;
            let t87 = t4 * t86;
            let t88 = t13 * t78;
            let t91 = t10 * t8;
            let t92 = t12 * t12;
            let t93 = f64x8::splat(1.0) / t92;
            let t94 = t93 * t78;
            let t96 = f64x8::splat(0.005721273333333333) * t91 * t94;
            let t97 = t4 * t8;
            let t99 = f64x8::splat(1.0) / t9 / t4;
            let t100 = t99 * t16;
            let t102 = f64x8::splat(0.0222904) * t100 * t75;
            let t103 = t27 * t19;
            let t104 = t17 - t81;
            let t107 = ((t20).select(f64x8::splat(0.0), f64x8::splat(8.0) / f64x8::splat(3.0) * t103 * t104));
            let t108 = t107 * t30;
            let t111 = v_rho0 * v_rho0;
            let t113 = f64x8::splat(1.0) / t33 / t111;
            let t116 = v_lapl0 * t113;
            let t118 = -f64x8::splat(5.0) / f64x8::splat(3.0) * v_tau0 * t113 + f64x8::splat(5.0) / f64x8::splat(24.0) * t116;
            let t121 = t45 * t41;
            let t122 = -t104;
            let t125 = ((t42).select(f64x8::splat(0.0), f64x8::splat(8.0) / f64x8::splat(3.0) * t121 * t122));
            let t126 = t125 * t30;
            let t130 = f64x8::splat(1.0) / t60 / t82;
            let t132 = t59 * t130 / f64x8::splat(3.0);
            let t135 = t104 / f64x8::splat(2.0);
            let t136 = t66 * t135;
            let t139 = -t135;
            let t140 = t71 * t139;
            let t143 = t108 * t39 / f64x8::splat(8.0) + t31 * t118 / f64x8::splat(8.0) + t126 * t56 / f64x8::splat(8.0) + t132 - f64x8::splat(5.0) / f64x8::splat(24.0) * t116 * t67 + f64x8::splat(5.0) / f64x8::splat(24.0) * t37 * t136 + f64x8::splat(5.0) / f64x8::splat(24.0) * t54 * t140;
            let t146 = t102 + f64x8::splat(0.264) * t16 * t143;
            let t147 = t13 * t146;
            let tvrho0 = tzk0 - f64x8::splat(0.04918) * t87 * t88 - t96 - f64x8::splat(0.04918) * t97 * t147;
            acc_vrho_0 = tvrho0;
            let t151 = f64x8::splat(2.0) * t81 + f64x8::splat(2.0) * t84;
            let t152 = t4 * t151;
            let t155 = -t17 - t81;
            let t158 = ((t20).select(f64x8::splat(0.0), f64x8::splat(8.0) / f64x8::splat(3.0) * t103 * t155));
            let t159 = t158 * t30;
            let t162 = -t155;
            let t165 = ((t42).select(f64x8::splat(0.0), f64x8::splat(8.0) / f64x8::splat(3.0) * t121 * t162));
            let t166 = t165 * t30;
            let t169 = v_rho1 * v_rho1;
            let t171 = f64x8::splat(1.0) / t50 / t169;
            let t174 = v_lapl1 * t171;
            let t176 = -f64x8::splat(5.0) / f64x8::splat(3.0) * v_tau1 * t171 + f64x8::splat(5.0) / f64x8::splat(24.0) * t174;
            let t179 = t155 / f64x8::splat(2.0);
            let t180 = t66 * t179;
            let t185 = -t179;
            let t186 = t71 * t185;
            let t189 = t159 * t39 / f64x8::splat(8.0) + t166 * t56 / f64x8::splat(8.0) + t48 * t176 / f64x8::splat(8.0) + t132 + f64x8::splat(5.0) / f64x8::splat(24.0) * t37 * t180 - f64x8::splat(5.0) / f64x8::splat(24.0) * t174 * t72 + f64x8::splat(5.0) / f64x8::splat(24.0) * t54 * t186;
            let t192 = t102 + f64x8::splat(0.264) * t16 * t189;
            let t193 = t13 * t192;
            let tvrho1 = tzk0 - f64x8::splat(0.04918) * t152 * t88 - t96 - f64x8::splat(0.04918) * t97 * t193;
            acc_vrho_1 = tvrho1;
            let t197 = f64x8::splat(1.0) / t60 / t4;
            let t198 = t197 * t8;
            let t199 = t13 * t16;
            let t200 = t198 * t199;
            let tvsigma0 = f64x8::splat(0.00162294) * t200;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.00324588) * t200;
            acc_vsigma_1 = tvsigma1;
            let tvsigma2 = tvsigma0;
            acc_vsigma_2 = tvsigma2;
            let t201 = t31 * t35;
            let t205 = -t201 / f64x8::splat(64.0) + t35 * t67 / f64x8::splat(8.0);
            let t206 = t199 * t205;
            let tvlapl0 = -f64x8::splat(0.01298352) * t97 * t206;
            acc_vlapl_0 = tvlapl0;
            let t209 = t48 * t52;
            let t213 = -t209 / f64x8::splat(64.0) + t52 * t72 / f64x8::splat(8.0);
            let t214 = t199 * t213;
            let tvlapl1 = -f64x8::splat(0.01298352) * t97 * t214;
            acc_vlapl_1 = tvlapl1;
            let t217 = t97 * t13;
            let t218 = t16 * t29;
            let t219 = t30 * t35;
            let t220 = t218 * t219;
            let tvtau0 = -f64x8::splat(0.00162294) * t217 * t220;
            acc_vtau_0 = tvtau0;
            let t223 = t16 * t47;
            let t224 = t30 * t52;
            let t225 = t223 * t224;
            let tvtau1 = -f64x8::splat(0.00162294) * t217 * t225;
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

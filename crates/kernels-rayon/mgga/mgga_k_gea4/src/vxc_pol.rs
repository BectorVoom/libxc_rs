//! MGGA_K_GEA4 vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_k_gea4.c`
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
pub fn mgga_k_gea4_vxc_pol(
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
            let t2 = (v_rho0).simd_le(dens_threshold);
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = t3 * t3;
            let t5 = f64x8::splat(M_CBRTPI);
            let t7 = t4 * t5 * f64x8::splat(M_PI);
            let t8 = v_rho0 + v_rho1;
            let t9 = f64x8::splat(1.0) / t8;
            let t12 = (f64x8::splat(2.0) * v_rho0 * t9).simd_le(zeta_threshold);
            let t13 = zeta_threshold - f64x8::splat(1.0);
            let t16 = (f64x8::splat(2.0) * v_rho1 * t9).simd_le(zeta_threshold);
            let t17 = -t13;
            let t18 = v_rho0 - v_rho1;
            let t20 = ((t12).select(t13, (t16).select(t17, t18 * t9)));
            let t21 = f64x8::splat(1.0) + t20;
            let t22 = (t21).simd_le(zeta_threshold);
            let t23 = (simd::cbrt(zeta_threshold));
            let t24 = t23 * t23;
            let t25 = t24 * zeta_threshold;
            let t26 = (simd::cbrt(t21));
            let t27 = t26 * t26;
            let t29 = ((t22).select(t25, t27 * t21));
            let t30 = (simd::cbrt(t8));
            let t31 = t30 * t30;
            let t32 = t29 * t31;
            let t33 = f64x8::splat(M_CBRT6);
            let t34 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t35 = (simd::cbrt(t34));
            let t36 = t35 * t35;
            let t38 = t33 / t36;
            let t39 = v_rho0 * v_rho0;
            let t40 = (simd::cbrt(v_rho0));
            let t41 = t40 * t40;
            let t43 = f64x8::splat(1.0) / t41 / t39;
            let t48 = f64x8::splat(1.0) / t41 / v_rho0;
            let t52 = t33 * t33;
            let t54 = f64x8::splat(1.0) / t35 / t34;
            let t55 = t52 * t54;
            let t56 = v_lapl0 * v_lapl0;
            let t57 = t39 * v_rho0;
            let t59 = f64x8::splat(1.0) / t40 / t57;
            let t63 = t39 * t39;
            let t65 = f64x8::splat(1.0) / t40 / t63;
            let t66 = v_sigma0 * t65;
            let t70 = v_sigma0 * v_sigma0;
            let t71 = t63 * v_rho0;
            let t73 = f64x8::splat(1.0) / t40 / t71;
            let t77 = f64x8::splat(1.0) + f64x8::splat(5.0) / f64x8::splat(648.0) * t38 * v_sigma0 * t43 + f64x8::splat(5.0) / f64x8::splat(54.0) * t38 * v_lapl0 * t48 + t55 * t56 * t59 / f64x8::splat(5832.0) - t55 * t66 * v_lapl0 / f64x8::splat(5184.0) + t55 * t70 * t73 / f64x8::splat(17496.0);
            let t81 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t32 * t77));
            let t82 = (v_rho1).simd_le(dens_threshold);
            let t83 = -t18;
            let t85 = ((t16).select(t13, (t12).select(t17, t83 * t9)));
            let t86 = f64x8::splat(1.0) + t85;
            let t87 = (t86).simd_le(zeta_threshold);
            let t88 = (simd::cbrt(t86));
            let t89 = t88 * t88;
            let t91 = ((t87).select(t25, t89 * t86));
            let t92 = t91 * t31;
            let t93 = v_rho1 * v_rho1;
            let t94 = (simd::cbrt(v_rho1));
            let t95 = t94 * t94;
            let t97 = f64x8::splat(1.0) / t95 / t93;
            let t102 = f64x8::splat(1.0) / t95 / v_rho1;
            let t106 = v_lapl1 * v_lapl1;
            let t107 = t93 * v_rho1;
            let t109 = f64x8::splat(1.0) / t94 / t107;
            let t113 = t93 * t93;
            let t115 = f64x8::splat(1.0) / t94 / t113;
            let t116 = v_sigma2 * t115;
            let t120 = v_sigma2 * v_sigma2;
            let t121 = t113 * v_rho1;
            let t123 = f64x8::splat(1.0) / t94 / t121;
            let t127 = f64x8::splat(1.0) + f64x8::splat(5.0) / f64x8::splat(648.0) * t38 * v_sigma2 * t97 + f64x8::splat(5.0) / f64x8::splat(54.0) * t38 * v_lapl1 * t102 + t55 * t106 * t109 / f64x8::splat(5832.0) - t55 * t116 * v_lapl1 / f64x8::splat(5184.0) + t55 * t120 * t123 / f64x8::splat(17496.0);
            let t131 = ((t82).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t92 * t127));
            let tzk0 = t81 + t131;
            acc_zk = tzk0;
            let t132 = t8 * t8;
            let t133 = f64x8::splat(1.0) / t132;
            let t134 = t18 * t133;
            let t136 = ((t12).select(f64x8::splat(0.0), (t16).select(f64x8::splat(0.0), t9 - t134)));
            let t139 = ((t22).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t27 * t136));
            let t140 = t139 * t31;
            let t144 = f64x8::splat(1.0) / t30;
            let t145 = t29 * t144;
            let t148 = t7 * t145 * t77 / f64x8::splat(10.0);
            let t150 = f64x8::splat(1.0) / t41 / t57;
            let t160 = v_sigma0 * t73;
            let t164 = t63 * t39;
            let t166 = f64x8::splat(1.0) / t40 / t164;
            let t170 = -f64x8::splat(5.0) / f64x8::splat(243.0) * t38 * v_sigma0 * t150 - f64x8::splat(25.0) / f64x8::splat(162.0) * t38 * v_lapl0 * t43 - f64x8::splat(5.0) / f64x8::splat(8748.0) * t55 * t56 * t65 + f64x8::splat(13.0) / f64x8::splat(15552.0) * t55 * t160 * v_lapl0 - f64x8::splat(2.0) / f64x8::splat(6561.0) * t55 * t70 * t166;
            let t175 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t140 * t77 + t148 + f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t32 * t170));
            let t176 = t83 * t133;
            let t178 = ((t16).select(f64x8::splat(0.0), (t12).select(f64x8::splat(0.0), -t9 - t176)));
            let t181 = ((t87).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t89 * t178));
            let t182 = t181 * t31;
            let t186 = t91 * t144;
            let t189 = t7 * t186 * t127 / f64x8::splat(10.0);
            let t191 = ((t82).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t182 * t127 + t189));
            let tvrho0 = t81 + t131 + t8 * (t175 + t191);
            acc_vrho_0 = tvrho0;
            let t195 = ((t12).select(f64x8::splat(0.0), (t16).select(f64x8::splat(0.0), -t9 - t134)));
            let t198 = ((t22).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t27 * t195));
            let t199 = t198 * t31;
            let t204 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t199 * t77 + t148));
            let t206 = ((t16).select(f64x8::splat(0.0), (t12).select(f64x8::splat(0.0), t9 - t176)));
            let t209 = ((t87).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t89 * t206));
            let t210 = t209 * t31;
            let t215 = f64x8::splat(1.0) / t95 / t107;
            let t225 = v_sigma2 * t123;
            let t229 = t113 * t93;
            let t231 = f64x8::splat(1.0) / t94 / t229;
            let t235 = -f64x8::splat(5.0) / f64x8::splat(243.0) * t38 * v_sigma2 * t215 - f64x8::splat(25.0) / f64x8::splat(162.0) * t38 * v_lapl1 * t97 - f64x8::splat(5.0) / f64x8::splat(8748.0) * t55 * t106 * t115 + f64x8::splat(13.0) / f64x8::splat(15552.0) * t55 * t225 * v_lapl1 - f64x8::splat(2.0) / f64x8::splat(6561.0) * t55 * t120 * t231;
            let t240 = ((t82).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t210 * t127 + t189 + f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t92 * t235));
            let tvrho1 = t81 + t131 + t8 * (t204 + t240);
            acc_vrho_1 = tvrho1;
            let t243 = t38 * t43;
            let t246 = t55 * t65 * v_lapl0;
            let t248 = t55 * t160;
            let t250 = f64x8::splat(5.0) / f64x8::splat(648.0) * t243 - t246 / f64x8::splat(5184.0) + t248 / f64x8::splat(8748.0);
            let t254 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t32 * t250));
            let tvsigma0 = t8 * t254;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t255 = t38 * t97;
            let t258 = t55 * t115 * v_lapl1;
            let t260 = t55 * t225;
            let t262 = f64x8::splat(5.0) / f64x8::splat(648.0) * t255 - t258 / f64x8::splat(5184.0) + t260 / f64x8::splat(8748.0);
            let t266 = ((t82).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t92 * t262));
            let tvsigma2 = t8 * t266;
            acc_vsigma_2 = tvsigma2;
            let t274 = f64x8::splat(5.0) / f64x8::splat(54.0) * t38 * t48 + t55 * v_lapl0 * t59 / f64x8::splat(2916.0) - t55 * t66 / f64x8::splat(5184.0);
            let t278 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t32 * t274));
            let tvlapl0 = t8 * t278;
            acc_vlapl_0 = tvlapl0;
            let t286 = f64x8::splat(5.0) / f64x8::splat(54.0) * t38 * t102 + t55 * v_lapl1 * t109 / f64x8::splat(2916.0) - t55 * t116 / f64x8::splat(5184.0);
            let t290 = ((t82).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t7 * t92 * t286));
            let tvlapl1 = t8 * t290;
            acc_vlapl_1 = tvlapl1;
            let tvtau0 = f64x8::splat(0.0);
            acc_vtau_0 = tvtau0;
            let tvtau1 = f64x8::splat(0.0);
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

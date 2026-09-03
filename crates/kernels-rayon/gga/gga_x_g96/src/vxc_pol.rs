//! GGA_X_G96 vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_g96.c`
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

/// Store 8 elements with a given stride and offset.
#[inline(always)]
fn store_strided(s: &mut [f64], ip: usize, m: usize, stride: usize, offset: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let base = ip * stride + offset;
        s[base] = a[0];
        s[base + stride] = a[1];
        s[base + 2 * stride] = a[2];
        s[base + 3 * stride] = a[3];
        s[base + 4 * stride] = a[4];
        s[base + 5 * stride] = a[5];
        s[base + 6 * stride] = a[6];
        s[base + 7 * stride] = a[7];
    } else {
        for k in 0..m {
            s[(ip + k) * stride + offset] = a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_g96_vxc_pol(
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
        let v_rho0 = load_strided(rho, ip, np, 2, 0);
        let v_rho1 = load_strided(rho, ip, np, 2, 1);
        let v_sigma0 = load_strided(sigma, ip, np, 3, 0);
        let v_sigma1 = load_strided(sigma, ip, np, 3, 1);
        let v_sigma2 = load_strided(sigma, ip, np, 3, 2);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        let mut acc_vsigma_0 = V_ZERO;
        let mut acc_vsigma_1 = V_ZERO;
        let mut acc_vsigma_2 = V_ZERO;
        {
            let t1 = (v_rho0).simd_le(dens_threshold);
            let t2 = f64x8::splat(M_CBRT3);
            let t3 = f64x8::splat(M_CBRTPI);
            let t4 = f64x8::splat(1.0) / t3;
            let t5 = t2 * t4;
            let t6 = v_rho0 + v_rho1;
            let t7 = f64x8::splat(1.0) / t6;
            let t10 = (f64x8::splat(2.0) * v_rho0 * t7).simd_le(zeta_threshold);
            let t11 = zeta_threshold - f64x8::splat(1.0);
            let t14 = (f64x8::splat(2.0) * v_rho1 * t7).simd_le(zeta_threshold);
            let t15 = -t11;
            let t16 = v_rho0 - v_rho1;
            let t18 = ((t10).select(t11, (t14).select(t15, t16 * t7)));
            let t19 = f64x8::splat(1.0) + t18;
            let t20 = (t19).simd_le(zeta_threshold);
            let t21 = (simd::cbrt(zeta_threshold));
            let t22 = t21 * zeta_threshold;
            let t23 = (simd::cbrt(t19));
            let t25 = ((t20).select(t22, t23 * t19));
            let t26 = (simd::cbrt(t6));
            let t28 = t2 * t2;
            let t30 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t31 = f64x8::splat(1.0) / t30;
            let t32 = t28 * t31;
            let t33 = f64x8::splat(M_CBRT4);
            let t34 = ((v_sigma0).sqrt());
            let t35 = (simd::cbrt(v_rho0));
            let t37 = f64x8::splat(1.0) / t35 / v_rho0;
            let t38 = t34 * t37;
            let t39 = ((t38).sqrt());
            let t40 = t39 * t38;
            let t44 = f64x8::splat(1.0) + f64x8::splat(2.0) / f64x8::splat(1233.0) * t32 * t33 * t40;
            let t48 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t25 * t26 * t44));
            let t49 = (v_rho1).simd_le(dens_threshold);
            let t50 = -t16;
            let t52 = ((t14).select(t11, (t10).select(t15, t50 * t7)));
            let t53 = f64x8::splat(1.0) + t52;
            let t54 = (t53).simd_le(zeta_threshold);
            let t55 = (simd::cbrt(t53));
            let t57 = ((t54).select(t22, t55 * t53));
            let t59 = ((v_sigma2).sqrt());
            let t60 = (simd::cbrt(v_rho1));
            let t62 = f64x8::splat(1.0) / t60 / v_rho1;
            let t63 = t59 * t62;
            let t64 = ((t63).sqrt());
            let t65 = t64 * t63;
            let t69 = f64x8::splat(1.0) + f64x8::splat(2.0) / f64x8::splat(1233.0) * t32 * t33 * t65;
            let t73 = ((t49).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t57 * t26 * t69));
            let tzk0 = t48 + t73;
            acc_zk = tzk0;
            let t74 = t6 * t6;
            let t75 = f64x8::splat(1.0) / t74;
            let t76 = t16 * t75;
            let t78 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), t7 - t76)));
            let t81 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t78));
            let t86 = t26 * t26;
            let t87 = f64x8::splat(1.0) / t86;
            let t91 = t5 * t25 * t87 * t44 / f64x8::splat(8.0);
            let t92 = t4 * t25;
            let t93 = t26 * t31;
            let t94 = t92 * t93;
            let t95 = t33 * t39;
            let t96 = v_rho0 * v_rho0;
            let t98 = f64x8::splat(1.0) / t35 / t96;
            let t100 = t95 * t34 * t98;
            let t104 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t81 * t26 * t44 - t91 + t94 * t100 / f64x8::splat(274.0)));
            let t105 = t50 * t75;
            let t107 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), -t7 - t105)));
            let t110 = ((t54).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t55 * t107));
            let t118 = t5 * t57 * t87 * t69 / f64x8::splat(8.0);
            let t120 = ((t49).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t110 * t26 * t69 - t118));
            let tvrho0 = t48 + t73 + t6 * (t104 + t120);
            acc_vrho_0 = tvrho0;
            let t124 = ((t10).select(f64x8::splat(0.0), (t14).select(f64x8::splat(0.0), -t7 - t76)));
            let t127 = ((t20).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t23 * t124));
            let t133 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t127 * t26 * t44 - t91));
            let t135 = ((t14).select(f64x8::splat(0.0), (t10).select(f64x8::splat(0.0), t7 - t105)));
            let t138 = ((t54).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t55 * t135));
            let t143 = t4 * t57;
            let t144 = t143 * t93;
            let t145 = t33 * t64;
            let t146 = v_rho1 * v_rho1;
            let t148 = f64x8::splat(1.0) / t60 / t146;
            let t150 = t145 * t59 * t148;
            let t154 = ((t49).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t5 * t138 * t26 * t69 - t118 + t144 * t150 / f64x8::splat(274.0)));
            let tvrho1 = t48 + t73 + t6 * (t133 + t154);
            acc_vrho_1 = tvrho1;
            let t157 = f64x8::splat(1.0) / t34;
            let t159 = t95 * t157 * t37;
            let t162 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(2192.0) * t94 * t159));
            let tvsigma0 = t6 * t162;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t163 = f64x8::splat(1.0) / t59;
            let t165 = t145 * t163 * t62;
            let t168 = ((t49).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(2192.0) * t144 * t165));
            let tvsigma2 = t6 * t168;
            acc_vsigma_2 = tvsigma2;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(vsigma, ip, m, 3, 0, acc_vsigma_0);
        store_strided(vsigma, ip, m, 3, 1, acc_vsigma_1);
        store_strided(vsigma, ip, m, 3, 2, acc_vsigma_2);
        ip += 8;
    }
}

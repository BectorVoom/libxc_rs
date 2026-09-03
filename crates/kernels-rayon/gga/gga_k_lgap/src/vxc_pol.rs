//! GGA_K_LGAP vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_lgap.c`
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
pub fn gga_k_lgap_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_mu_0: f64,
    param_mu_1: f64,
    param_mu_2: f64,
    param_kappa: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_mu_0 = f64x8::splat(param_mu_0);
    let param_mu_1 = f64x8::splat(param_mu_1);
    let param_mu_2 = f64x8::splat(param_mu_2);
    let param_kappa = f64x8::splat(param_kappa);
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
            let t3 = t2 * t2;
            let t4 = f64x8::splat(M_CBRTPI);
            let t6 = t3 * t4 * f64x8::splat(M_PI);
            let t7 = v_rho0 + v_rho1;
            let t8 = f64x8::splat(1.0) / t7;
            let t11 = (f64x8::splat(2.0) * v_rho0 * t8).simd_le(zeta_threshold);
            let t12 = zeta_threshold - f64x8::splat(1.0);
            let t15 = (f64x8::splat(2.0) * v_rho1 * t8).simd_le(zeta_threshold);
            let t16 = -t12;
            let t17 = v_rho0 - v_rho1;
            let t19 = ((t11).select(t12, (t15).select(t16, t17 * t8)));
            let t20 = f64x8::splat(1.0) + t19;
            let t21 = (t20).simd_le(zeta_threshold);
            let t22 = (simd::cbrt(zeta_threshold));
            let t23 = t22 * t22;
            let t24 = t23 * zeta_threshold;
            let t25 = (simd::cbrt(t20));
            let t26 = t25 * t25;
            let t28 = ((t21).select(t24, t26 * t20));
            let t29 = (simd::cbrt(t7));
            let t30 = t29 * t29;
            let t31 = t28 * t30;
            let t33 = f64x8::splat(M_CBRT6);
            let t34 = t33 * t33;
            let t35 = param_mu_0 * t34;
            let t36 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t37 = (simd::cbrt(t36));
            let t38 = f64x8::splat(1.0) / t37;
            let t39 = ((v_sigma0).sqrt());
            let t40 = t38 * t39;
            let t41 = (simd::cbrt(v_rho0));
            let t43 = f64x8::splat(1.0) / t41 / v_rho0;
            let t48 = param_mu_1 * t33;
            let t49 = t37 * t37;
            let t50 = f64x8::splat(1.0) / t49;
            let t51 = t50 * v_sigma0;
            let t52 = v_rho0 * v_rho0;
            let t53 = t41 * t41;
            let t55 = f64x8::splat(1.0) / t53 / t52;
            let t61 = param_mu_2 / t36;
            let t62 = t39 * v_sigma0;
            let t63 = t52 * t52;
            let t64 = f64x8::splat(1.0) / t63;
            let t69 = (simd::exp(-t35 * t40 * t43 / f64x8::splat(12.0) - t48 * t51 * t55 / f64x8::splat(24.0) - t61 * t62 * t64 / f64x8::splat(48.0)));
            let t72 = f64x8::splat(1.0) + param_kappa * (f64x8::splat(1.0) - t69);
            let t76 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t31 * t72));
            let t77 = (v_rho1).simd_le(dens_threshold);
            let t78 = -t17;
            let t80 = ((t15).select(t12, (t11).select(t16, t78 * t8)));
            let t81 = f64x8::splat(1.0) + t80;
            let t82 = (t81).simd_le(zeta_threshold);
            let t83 = (simd::cbrt(t81));
            let t84 = t83 * t83;
            let t86 = ((t82).select(t24, t84 * t81));
            let t87 = t86 * t30;
            let t88 = ((v_sigma2).sqrt());
            let t89 = t38 * t88;
            let t90 = (simd::cbrt(v_rho1));
            let t92 = f64x8::splat(1.0) / t90 / v_rho1;
            let t96 = t50 * v_sigma2;
            let t97 = v_rho1 * v_rho1;
            let t98 = t90 * t90;
            let t100 = f64x8::splat(1.0) / t98 / t97;
            let t104 = t88 * v_sigma2;
            let t105 = t97 * t97;
            let t106 = f64x8::splat(1.0) / t105;
            let t111 = (simd::exp(-t35 * t89 * t92 / f64x8::splat(12.0) - t48 * t96 * t100 / f64x8::splat(24.0) - t61 * t104 * t106 / f64x8::splat(48.0)));
            let t114 = f64x8::splat(1.0) + param_kappa * (f64x8::splat(1.0) - t111);
            let t118 = ((t77).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t87 * t114));
            let tzk0 = t76 + t118;
            acc_zk = tzk0;
            let t119 = t7 * t7;
            let t120 = f64x8::splat(1.0) / t119;
            let t121 = t17 * t120;
            let t123 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), t8 - t121)));
            let t126 = ((t21).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t123));
            let t127 = t126 * t30;
            let t131 = f64x8::splat(1.0) / t29;
            let t132 = t28 * t131;
            let t135 = t6 * t132 * t72 / f64x8::splat(10.0);
            let t136 = t6 * t28;
            let t137 = t30 * param_kappa;
            let t139 = f64x8::splat(1.0) / t41 / t52;
            let t143 = t52 * v_rho0;
            let t145 = f64x8::splat(1.0) / t53 / t143;
            let t149 = t63 * v_rho0;
            let t150 = f64x8::splat(1.0) / t149;
            let t154 = t35 * t40 * t139 / f64x8::splat(9.0) + t48 * t51 * t145 / f64x8::splat(9.0) + t61 * t62 * t150 / f64x8::splat(12.0);
            let t155 = t154 * t69;
            let t156 = t137 * t155;
            let t160 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t127 * t72 + t135 - f64x8::splat(3.0) / f64x8::splat(20.0) * t136 * t156));
            let t161 = t78 * t120;
            let t163 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), -t8 - t161)));
            let t166 = ((t82).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t84 * t163));
            let t167 = t166 * t30;
            let t171 = t86 * t131;
            let t174 = t6 * t171 * t114 / f64x8::splat(10.0);
            let t176 = ((t77).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t167 * t114 + t174));
            let tvrho0 = t76 + t118 + t7 * (t160 + t176);
            acc_vrho_0 = tvrho0;
            let t180 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), -t8 - t121)));
            let t183 = ((t21).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t26 * t180));
            let t184 = t183 * t30;
            let t189 = ((t1).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t184 * t72 + t135));
            let t191 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), t8 - t161)));
            let t194 = ((t82).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t84 * t191));
            let t195 = t194 * t30;
            let t199 = t6 * t86;
            let t201 = f64x8::splat(1.0) / t90 / t97;
            let t205 = t97 * v_rho1;
            let t207 = f64x8::splat(1.0) / t98 / t205;
            let t211 = t105 * v_rho1;
            let t212 = f64x8::splat(1.0) / t211;
            let t216 = t35 * t89 * t201 / f64x8::splat(9.0) + t48 * t96 * t207 / f64x8::splat(9.0) + t61 * t104 * t212 / f64x8::splat(12.0);
            let t217 = t216 * t111;
            let t218 = t137 * t217;
            let t222 = ((t77).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t6 * t195 * t114 + t174 - f64x8::splat(3.0) / f64x8::splat(20.0) * t199 * t218));
            let tvrho1 = t76 + t118 + t7 * (t189 + t222);
            acc_vrho_1 = tvrho1;
            let t225 = f64x8::splat(1.0) / t39;
            let t226 = t38 * t225;
            let t236 = -t35 * t226 * t43 / f64x8::splat(24.0) - t48 * t50 * t55 / f64x8::splat(24.0) - t61 * t39 * t64 / f64x8::splat(32.0);
            let t237 = t236 * t69;
            let t238 = t137 * t237;
            let t241 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(20.0) * t136 * t238));
            let tvsigma0 = t7 * t241;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t242 = f64x8::splat(1.0) / t88;
            let t243 = t38 * t242;
            let t253 = -t35 * t243 * t92 / f64x8::splat(24.0) - t48 * t50 * t100 / f64x8::splat(24.0) - t61 * t88 * t106 / f64x8::splat(32.0);
            let t254 = t253 * t111;
            let t255 = t137 * t254;
            let t258 = ((t77).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(20.0) * t199 * t255));
            let tvsigma2 = t7 * t258;
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

//! GGA_X_ITYH exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_ityh.c`
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
pub fn gga_x_ityh_exc_pol(
    rho: &[f64],
    sigma: &[f64],
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
        let v_sigma0 = load_strided(sigma, ip, np, 3, 0);
        let v_sigma1 = load_strided(sigma, ip, np, 3, 1);
        let v_sigma2 = load_strided(sigma, ip, np, 3, 2);
        let mut acc_zk = V_ZERO;
        {
            let t1 = (v_rho0).simd_le(dens_threshold);
            let t2 = f64x8::splat(M_CBRT3);
            let t3 = f64x8::splat(M_CBRTPI);
            let t5 = t2 / t3;
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
            let t26 = t5 * t25;
            let t27 = (simd::cbrt(t6));
            let t28 = t2 * t2;
            let t29 = f64x8::splat(M_PI) * t28;
            let t30 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t31 = (simd::cbrt(t30));
            let t32 = f64x8::splat(1.0) / t31;
            let t33 = f64x8::splat(M_CBRT4);
            let t34 = t32 * t33;
            let t35 = t28 * t32;
            let t36 = t35 * t33;
            let t37 = v_rho0 * v_rho0;
            let t38 = (simd::cbrt(v_rho0));
            let t39 = t38 * t38;
            let t41 = f64x8::splat(1.0) / t39 / t37;
            let t42 = v_sigma0 * t41;
            let t43 = ((v_sigma0).sqrt());
            let t45 = f64x8::splat(1.0) / t38 / v_rho0;
            let t46 = t43 * t45;
            let t47 = (simd::ln(t46 + ((t46 * t46 + f64x8::splat(1.0)).sqrt())));
            let t50 = f64x8::splat(1.0) + f64x8::splat(0.0252) * t46 * t47;
            let t51 = f64x8::splat(1.0) / t50;
            let t55 = f64x8::splat(1.0) + f64x8::splat(0.0009333333333333333) * t36 * t42 * t51;
            let t58 = t29 * t34 / t55;
            let t59 = ((t58).sqrt());
            let t61 = param_hyb_omega_0 / t59;
            let t62 = f64x8::splat(M_CBRT2);
            let t63 = t19 * t6;
            let t64 = (simd::cbrt(t63));
            let t65 = f64x8::splat(1.0) / t64;
            let t66 = t62 * t65;
            let t68 = t61 * t66 / f64x8::splat(2.0);
            let t69 = (f64x8::splat(1.35)).simd_le(t68);
            let t70 = (f64x8::splat(1.35)).simd_lt(t68);
            let t71 = ((t70).select(t68, f64x8::splat(1.35)));
            let t72 = t71 * t71;
            let t75 = t72 * t72;
            let t76 = f64x8::splat(1.0) / t75;
            let t78 = t75 * t72;
            let t79 = f64x8::splat(1.0) / t78;
            let t81 = t75 * t75;
            let t82 = f64x8::splat(1.0) / t81;
            let t85 = f64x8::splat(1.0) / t81 / t72;
            let t88 = f64x8::splat(1.0) / t81 / t75;
            let t91 = f64x8::splat(1.0) / t81 / t78;
            let t93 = t81 * t81;
            let t94 = f64x8::splat(1.0) / t93;
            let t97 = ((t70).select(f64x8::splat(1.35), t68));
            let t98 = ((f64x8::splat(M_PI)).sqrt());
            let t99 = f64x8::splat(1.0) / t97;
            let t101 = (simd::erf(t99 / f64x8::splat(2.0)));
            let t103 = t97 * t97;
            let t104 = f64x8::splat(1.0) / t103;
            let t106 = (simd::exp(-t104 / f64x8::splat(4.0)));
            let t107 = t106 - f64x8::splat(1.0);
            let t110 = t106 - f64x8::splat(3.0) / f64x8::splat(2.0) - f64x8::splat(2.0) * t103 * t107;
            let t113 = t98 * t101 + f64x8::splat(2.0) * t97 * t110;
            let t117 = ((t69).select(f64x8::splat(1.0) / t72 / f64x8::splat(36.0) - t76 / f64x8::splat(960.0) + t79 / f64x8::splat(26880.0) - t82 / f64x8::splat(829440.0) + t85 / f64x8::splat(28385280.0) - t88 / f64x8::splat(1073479680.0) + t91 / f64x8::splat(44590694400.0) - t94 / f64x8::splat(2021444812800.0), f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t97 * t113));
            let t118 = t27 * t117;
            let t119 = t118 * t55;
            let t122 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t26 * t119));
            let t123 = (v_rho1).simd_le(dens_threshold);
            let t124 = -t16;
            let t126 = ((t14).select(t11, (t10).select(t15, t124 * t7)));
            let t127 = f64x8::splat(1.0) + t126;
            let t128 = (t127).simd_le(zeta_threshold);
            let t129 = (simd::cbrt(t127));
            let t131 = ((t128).select(t22, t129 * t127));
            let t132 = t5 * t131;
            let t133 = v_rho1 * v_rho1;
            let t134 = (simd::cbrt(v_rho1));
            let t135 = t134 * t134;
            let t137 = f64x8::splat(1.0) / t135 / t133;
            let t138 = v_sigma2 * t137;
            let t139 = ((v_sigma2).sqrt());
            let t141 = f64x8::splat(1.0) / t134 / v_rho1;
            let t142 = t139 * t141;
            let t143 = (simd::ln(t142 + ((t142 * t142 + f64x8::splat(1.0)).sqrt())));
            let t146 = f64x8::splat(1.0) + f64x8::splat(0.0252) * t142 * t143;
            let t147 = f64x8::splat(1.0) / t146;
            let t151 = f64x8::splat(1.0) + f64x8::splat(0.0009333333333333333) * t36 * t138 * t147;
            let t154 = t29 * t34 / t151;
            let t155 = ((t154).sqrt());
            let t157 = param_hyb_omega_0 / t155;
            let t158 = t127 * t6;
            let t159 = (simd::cbrt(t158));
            let t160 = f64x8::splat(1.0) / t159;
            let t161 = t62 * t160;
            let t163 = t157 * t161 / f64x8::splat(2.0);
            let t164 = (f64x8::splat(1.35)).simd_le(t163);
            let t165 = (f64x8::splat(1.35)).simd_lt(t163);
            let t166 = ((t165).select(t163, f64x8::splat(1.35)));
            let t167 = t166 * t166;
            let t170 = t167 * t167;
            let t171 = f64x8::splat(1.0) / t170;
            let t173 = t170 * t167;
            let t174 = f64x8::splat(1.0) / t173;
            let t176 = t170 * t170;
            let t177 = f64x8::splat(1.0) / t176;
            let t180 = f64x8::splat(1.0) / t176 / t167;
            let t183 = f64x8::splat(1.0) / t176 / t170;
            let t186 = f64x8::splat(1.0) / t176 / t173;
            let t188 = t176 * t176;
            let t189 = f64x8::splat(1.0) / t188;
            let t192 = ((t165).select(f64x8::splat(1.35), t163));
            let t193 = f64x8::splat(1.0) / t192;
            let t195 = (simd::erf(t193 / f64x8::splat(2.0)));
            let t197 = t192 * t192;
            let t198 = f64x8::splat(1.0) / t197;
            let t200 = (simd::exp(-t198 / f64x8::splat(4.0)));
            let t201 = t200 - f64x8::splat(1.0);
            let t204 = t200 - f64x8::splat(3.0) / f64x8::splat(2.0) - f64x8::splat(2.0) * t197 * t201;
            let t207 = f64x8::splat(2.0) * t192 * t204 + t98 * t195;
            let t211 = ((t164).select(f64x8::splat(1.0) / t167 / f64x8::splat(36.0) - t171 / f64x8::splat(960.0) + t174 / f64x8::splat(26880.0) - t177 / f64x8::splat(829440.0) + t180 / f64x8::splat(28385280.0) - t183 / f64x8::splat(1073479680.0) + t186 / f64x8::splat(44590694400.0) - t189 / f64x8::splat(2021444812800.0), f64x8::splat(1.0) - f64x8::splat(8.0) / f64x8::splat(3.0) * t192 * t207));
            let t212 = t27 * t211;
            let t213 = t212 * t151;
            let t216 = ((t123).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t132 * t213));
            let tzk0 = t122 + t216;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}

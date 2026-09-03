//! MGGA_X_REVTM exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_revtm.c`
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
pub fn mgga_x_revtm_exc_pol(
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
            let t2 = (v_rho0).simd_le(dens_threshold);
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = f64x8::splat(M_CBRTPI);
            let t6 = t3 / t4;
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
            let t23 = t22 * zeta_threshold;
            let t24 = (simd::cbrt(t20));
            let t26 = ((t21).select(t23, t24 * t20));
            let t27 = (simd::cbrt(t7));
            let t28 = t26 * t27;
            let t29 = f64x8::splat(1.0) / v_rho0;
            let t30 = v_sigma0 * t29;
            let t31 = f64x8::splat(1.0) / v_tau0;
            let t33 = t30 * t31 / f64x8::splat(8.0);
            let t34 = (t33).simd_lt(f64x8::splat(1.0));
            let t35 = ((t34).select(t33, f64x8::splat(1.0)));
            let t36 = t35 * t35;
            let t37 = t36 * t35;
            let t39 = t36 + f64x8::splat(3.0) * t37;
            let t40 = f64x8::splat(1.0) + t37;
            let t41 = t40 * t40;
            let t42 = f64x8::splat(1.0) / t41;
            let t43 = t39 * t42;
            let t44 = f64x8::splat(M_CBRT6);
            let t45 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t46 = (simd::cbrt(t45));
            let t47 = t46 * t46;
            let t48 = f64x8::splat(1.0) / t47;
            let t49 = t44 * t48;
            let t50 = v_rho0 * v_rho0;
            let t51 = (simd::cbrt(v_rho0));
            let t52 = t51 * t51;
            let t54 = f64x8::splat(1.0) / t52 / t50;
            let t55 = v_sigma0 * t54;
            let t56 = t49 * t55;
            let t58 = t44 * t44;
            let t60 = f64x8::splat(1.0) / t46 / t45;
            let t61 = t58 * t60;
            let t62 = v_sigma0 * v_sigma0;
            let t63 = t50 * t50;
            let t64 = t63 * v_rho0;
            let t66 = f64x8::splat(1.0) / t51 / t64;
            let t70 = f64x8::splat(1.0) + f64x8::splat(0.1504548888888889) * t56 + f64x8::splat(0.002689949046226295) * t61 * t62 * t66;
            let t71 = (simd::pow(t70, f64x8::splat(1.0) / f64x8::splat(5.0)));
            let t75 = f64x8::splat(1.0) / t52 / v_rho0;
            let t76 = v_tau0 * t75;
            let t79 = f64x8::splat(0.256337604) * t58 * t47;
            let t85 = f64x8::splat(1.0) + f64x8::splat(0.06394332777777778) * t56 - f64x8::splat(5.0) / f64x8::splat(9.0) * (f64x8::splat(0.14554132) * t76 + t79 + f64x8::splat(0.011867481666666667) * t55) * t44 * t48;
            let t86 = t71 * t71;
            let t87 = f64x8::splat(1.0) / t86;
            let t90 = f64x8::splat(1.0) / t71 + f64x8::splat(7.0) / f64x8::splat(9.0) * t85 * t87;
            let t92 = f64x8::splat(1.0) - t43;
            let t95 = (f64x8::splat(10.0) / f64x8::splat(81.0) + f64x8::splat(25.0) / f64x8::splat(8748.0) * t56) * t44;
            let t96 = t48 * v_sigma0;
            let t101 = t76 - t55 / f64x8::splat(8.0);
            let t102 = t101 * t44;
            let t105 = f64x8::splat(5.0) / f64x8::splat(9.0) * t102 * t48 - f64x8::splat(1.0);
            let t106 = t48 * t105;
            let t109 = f64x8::splat(1.0) + f64x8::splat(0.2222222222222222) * t102 * t106;
            let t110 = ((t109).sqrt());
            let t111 = f64x8::splat(1.0) / t110;
            let t115 = f64x8::splat(9.0) / f64x8::splat(20.0) * t105 * t111 + t56 / f64x8::splat(36.0);
            let t116 = t115 * t115;
            let t118 = t115 * t35;
            let t119 = f64x8::splat(1.0) - t35;
            let t122 = f64x8::splat(1.0) + f64x8::splat(5.0) / f64x8::splat(12.0) * t95 * t96 * t54 + f64x8::splat(292.0) / f64x8::splat(405.0) * t116 - f64x8::splat(146.0) / f64x8::splat(135.0) * t118 * t119;
            let t123 = (simd::pow(t122, f64x8::splat(1.0) / f64x8::splat(10.0)));
            let t125 = t92 * t123 + t43 * t90;
            let t129 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t28 * t125));
            let t130 = (v_rho1).simd_le(dens_threshold);
            let t131 = -t17;
            let t133 = ((t15).select(t12, (t11).select(t16, t131 * t8)));
            let t134 = f64x8::splat(1.0) + t133;
            let t135 = (t134).simd_le(zeta_threshold);
            let t136 = (simd::cbrt(t134));
            let t138 = ((t135).select(t23, t136 * t134));
            let t139 = t138 * t27;
            let t140 = f64x8::splat(1.0) / v_rho1;
            let t141 = v_sigma2 * t140;
            let t142 = f64x8::splat(1.0) / v_tau1;
            let t144 = t141 * t142 / f64x8::splat(8.0);
            let t145 = (t144).simd_lt(f64x8::splat(1.0));
            let t146 = ((t145).select(t144, f64x8::splat(1.0)));
            let t147 = t146 * t146;
            let t148 = t147 * t146;
            let t150 = t147 + f64x8::splat(3.0) * t148;
            let t151 = f64x8::splat(1.0) + t148;
            let t152 = t151 * t151;
            let t153 = f64x8::splat(1.0) / t152;
            let t154 = t150 * t153;
            let t155 = v_rho1 * v_rho1;
            let t156 = (simd::cbrt(v_rho1));
            let t157 = t156 * t156;
            let t159 = f64x8::splat(1.0) / t157 / t155;
            let t160 = v_sigma2 * t159;
            let t161 = t49 * t160;
            let t163 = v_sigma2 * v_sigma2;
            let t164 = t155 * t155;
            let t165 = t164 * v_rho1;
            let t167 = f64x8::splat(1.0) / t156 / t165;
            let t171 = f64x8::splat(1.0) + f64x8::splat(0.1504548888888889) * t161 + f64x8::splat(0.002689949046226295) * t61 * t163 * t167;
            let t172 = (simd::pow(t171, f64x8::splat(1.0) / f64x8::splat(5.0)));
            let t176 = f64x8::splat(1.0) / t157 / v_rho1;
            let t177 = v_tau1 * t176;
            let t184 = f64x8::splat(1.0) + f64x8::splat(0.06394332777777778) * t161 - f64x8::splat(5.0) / f64x8::splat(9.0) * (f64x8::splat(0.14554132) * t177 + t79 + f64x8::splat(0.011867481666666667) * t160) * t44 * t48;
            let t185 = t172 * t172;
            let t186 = f64x8::splat(1.0) / t185;
            let t189 = f64x8::splat(1.0) / t172 + f64x8::splat(7.0) / f64x8::splat(9.0) * t184 * t186;
            let t191 = f64x8::splat(1.0) - t154;
            let t194 = (f64x8::splat(10.0) / f64x8::splat(81.0) + f64x8::splat(25.0) / f64x8::splat(8748.0) * t161) * t44;
            let t195 = t48 * v_sigma2;
            let t200 = t177 - t160 / f64x8::splat(8.0);
            let t201 = t200 * t44;
            let t204 = f64x8::splat(5.0) / f64x8::splat(9.0) * t201 * t48 - f64x8::splat(1.0);
            let t205 = t48 * t204;
            let t208 = f64x8::splat(1.0) + f64x8::splat(0.2222222222222222) * t201 * t205;
            let t209 = ((t208).sqrt());
            let t210 = f64x8::splat(1.0) / t209;
            let t214 = f64x8::splat(9.0) / f64x8::splat(20.0) * t204 * t210 + t161 / f64x8::splat(36.0);
            let t215 = t214 * t214;
            let t217 = t214 * t146;
            let t218 = f64x8::splat(1.0) - t146;
            let t221 = f64x8::splat(1.0) + f64x8::splat(5.0) / f64x8::splat(12.0) * t194 * t195 * t159 + f64x8::splat(292.0) / f64x8::splat(405.0) * t215 - f64x8::splat(146.0) / f64x8::splat(135.0) * t217 * t218;
            let t222 = (simd::pow(t221, f64x8::splat(1.0) / f64x8::splat(10.0)));
            let t224 = t154 * t189 + t191 * t222;
            let t228 = ((t130).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t139 * t224));
            let tzk0 = t129 + t228;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}

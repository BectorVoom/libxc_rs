//! GGA_C_OP_PW91 exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_op_pw91.c`
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
pub fn gga_c_op_pw91_exc_pol(
    rho: &[f64],
    sigma: &[f64],
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
        let mut acc_zk = V_ZERO;
        {
            let t1 = v_rho0 - v_rho1;
            let t2 = v_rho0 + v_rho1;
            let t3 = f64x8::splat(1.0) / t2;
            let t4 = t1 * t3;
            let t5 = ((t4).abs());
            let t11 = ((f64x8::splat(1.0) - t5).simd_le(zeta_threshold)) | (((v_rho0).simd_le(dens_threshold)) & ((v_rho1).simd_le(dens_threshold)));
            let t13 = (f64x8::splat(1.0) + t4).simd_le(zeta_threshold);
            let t14 = zeta_threshold - f64x8::splat(1.0);
            let t16 = (f64x8::splat(1.0) - t4).simd_le(zeta_threshold);
            let t17 = -t14;
            let t18 = ((t13).select(t14, (t16).select(t17, t4)));
            let t19 = t18 * t18;
            let t20 = f64x8::splat(1.0) - t19;
            let t21 = t20 * t2;
            let t24 = (f64x8::splat(2.0) * v_rho0 * t3).simd_le(zeta_threshold);
            let t27 = (f64x8::splat(2.0) * v_rho1 * t3).simd_le(zeta_threshold);
            let t28 = ((t24).select(t14, (t27).select(t17, t4)));
            let t29 = f64x8::splat(1.0) + t28;
            let t32 = (t29 * t2 / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t33 = f64x8::splat(M_CBRT3);
            let t34 = t33 * t33;
            let t36 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t38 = t34 / t36;
            let t39 = f64x8::splat(M_CBRT4);
            let t40 = t38 * t39;
            let t41 = f64x8::splat(M_CBRT2);
            let t42 = (t29).simd_le(zeta_threshold);
            let t43 = f64x8::splat(1.0) - t28;
            let t44 = (t43).simd_le(zeta_threshold);
            let t45 = ((t42).select(t14, (t44).select(t17, t28)));
            let t46 = f64x8::splat(1.0) + t45;
            let t47 = t46 * t2;
            let t48 = (simd::cbrt(t47));
            let t49 = f64x8::splat(1.0) / t48;
            let t50 = t41 * t49;
            let t51 = f64x8::splat(M_CBRT6);
            let t52 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t53 = (simd::cbrt(t52));
            let t54 = t53 * t53;
            let t55 = f64x8::splat(1.0) / t54;
            let t56 = t51 * t55;
            let t57 = v_rho0 * v_rho0;
            let t58 = (simd::cbrt(v_rho0));
            let t59 = t58 * t58;
            let t61 = f64x8::splat(1.0) / t59 / t57;
            let t63 = t56 * v_sigma0 * t61;
            let t65 = (simd::exp(-f64x8::splat(25.0) / f64x8::splat(6.0) * t63));
            let t68 = (f64x8::splat(0.2743) - f64x8::splat(0.1508) * t65) * t51;
            let t69 = t55 * v_sigma0;
            let t73 = t51 * t51;
            let t75 = f64x8::splat(1.0) / t53 / t52;
            let t76 = t73 * t75;
            let t77 = v_sigma0 * v_sigma0;
            let t78 = t57 * t57;
            let t79 = t78 * v_rho0;
            let t81 = f64x8::splat(1.0) / t58 / t79;
            let t84 = f64x8::splat(6.944444444444445e-06) * t76 * t77 * t81;
            let t85 = t68 * t69 * t61 / f64x8::splat(24.0) - t84;
            let t87 = t73 / t53;
            let t88 = ((v_sigma0).sqrt());
            let t90 = f64x8::splat(1.0) / t58 / v_rho0;
            let t91 = t88 * t90;
            let t94 = (simd::ln(f64x8::splat(0.6496333333333333) * t87 * t91 + ((((f64x8::splat(0.6496333333333333) * t87 * t91) * (f64x8::splat(0.6496333333333333) * t87 * t91)) + f64x8::splat(1.0)).sqrt())));
            let t98 = f64x8::splat(1.0) + f64x8::splat(0.016370833333333334) * t87 * t91 * t94 + t84;
            let t99 = f64x8::splat(1.0) / t98;
            let t101 = t85 * t99 + f64x8::splat(1.0);
            let t102 = f64x8::splat(1.0) / t101;
            let t106 = ((t32).select(f64x8::splat(0.0), t40 * t50 * t102 / f64x8::splat(9.0)));
            let t110 = (t43 * t2 / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t111 = ((t44).select(t14, (t42).select(t17, -t28)));
            let t112 = f64x8::splat(1.0) + t111;
            let t113 = t112 * t2;
            let t114 = (simd::cbrt(t113));
            let t115 = f64x8::splat(1.0) / t114;
            let t116 = t41 * t115;
            let t117 = v_rho1 * v_rho1;
            let t118 = (simd::cbrt(v_rho1));
            let t119 = t118 * t118;
            let t121 = f64x8::splat(1.0) / t119 / t117;
            let t123 = t56 * v_sigma2 * t121;
            let t125 = (simd::exp(-f64x8::splat(25.0) / f64x8::splat(6.0) * t123));
            let t128 = (f64x8::splat(0.2743) - f64x8::splat(0.1508) * t125) * t51;
            let t129 = t55 * v_sigma2;
            let t133 = v_sigma2 * v_sigma2;
            let t134 = t117 * t117;
            let t135 = t134 * v_rho1;
            let t137 = f64x8::splat(1.0) / t118 / t135;
            let t140 = f64x8::splat(6.944444444444445e-06) * t76 * t133 * t137;
            let t141 = t128 * t129 * t121 / f64x8::splat(24.0) - t140;
            let t142 = ((v_sigma2).sqrt());
            let t144 = f64x8::splat(1.0) / t118 / v_rho1;
            let t145 = t142 * t144;
            let t148 = (simd::ln(f64x8::splat(0.6496333333333333) * t87 * t145 + ((((f64x8::splat(0.6496333333333333) * t87 * t145) * (f64x8::splat(0.6496333333333333) * t87 * t145)) + f64x8::splat(1.0)).sqrt())));
            let t152 = f64x8::splat(1.0) + f64x8::splat(0.016370833333333334) * t87 * t145 * t148 + t140;
            let t153 = f64x8::splat(1.0) / t152;
            let t155 = t141 * t153 + f64x8::splat(1.0);
            let t156 = f64x8::splat(1.0) / t155;
            let t160 = ((t110).select(f64x8::splat(0.0), t40 * t116 * t156 / f64x8::splat(9.0)));
            let t161 = t106 + t160;
            let t162 = (t161).simd_eq(f64x8::splat(0.0));
            let t163 = ((t162).select(f64x8::splat(f64::EPSILON), t161));
            let t166 = f64x8::splat(3.60663084) / t163 + f64x8::splat(0.5764);
            let t167 = t163 * t163;
            let t168 = t167 * t167;
            let t169 = f64x8::splat(1.0) / t168;
            let t171 = t167 * t163;
            let t172 = f64x8::splat(1.0) / t171;
            let t174 = f64x8::splat(1.0) / t167;
            let t176 = f64x8::splat(31.58152667175181) * t169 + f64x8::splat(15.032732091624375) * t172 + f64x8::splat(1.788764629788) * t174;
            let t177 = f64x8::splat(1.0) / t176;
            let t178 = t166 * t177;
            let tzk0 = ((t11).select(f64x8::splat(0.0), -f64x8::splat(0.25) * t21 * t178));
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}

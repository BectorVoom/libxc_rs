//! GGA_X_LG93 exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_lg93.c`
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
pub fn gga_x_lg93_exc_pol(
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
            let t28 = f64x8::splat(M_CBRT6);
            let t29 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t30 = (simd::cbrt(t29));
            let t31 = t30 * t30;
            let t32 = f64x8::splat(1.0) / t31;
            let t33 = t28 * t32;
            let t34 = v_rho0 * v_rho0;
            let t35 = (simd::cbrt(v_rho0));
            let t36 = t35 * t35;
            let t38 = f64x8::splat(1.0) / t36 / t34;
            let t40 = t33 * v_sigma0 * t38;
            let t42 = t28 * t28;
            let t44 = f64x8::splat(1.0) / t30 / t29;
            let t45 = t42 * t44;
            let t46 = v_sigma0 * v_sigma0;
            let t47 = t34 * t34;
            let t48 = t47 * v_rho0;
            let t50 = f64x8::splat(1.0) / t35 / t48;
            let t54 = t46 * v_sigma0;
            let t55 = t47 * t47;
            let t56 = f64x8::splat(1.0) / t55;
            let t59 = t29 * t29;
            let t62 = t28 / t31 / t59;
            let t63 = t46 * t46;
            let t64 = t55 * t34;
            let t66 = f64x8::splat(1.0) / t36 / t64;
            let t73 = t42 / t30 / t59 / t29;
            let t74 = t63 * v_sigma0;
            let t75 = t55 * t48;
            let t77 = f64x8::splat(1.0) / t35 / t75;
            let t81 = t63 * t46;
            let t82 = t55 * t55;
            let t83 = f64x8::splat(1.0) / t82;
            let t86 = f64x8::splat(1.0) + f64x8::splat(0.2058807993646726) * t40 + f64x8::splat(0.05171875) * t45 * t46 * t50 + f64x8::splat(9.988390807433105e-05) * t54 * t56 + f64x8::splat(0.00021916594328703703) * t62 * t63 * t66 + f64x8::splat(0.0011831024546682099) * t73 * t74 * t77 + f64x8::splat(1.1106816177675317e-09) * t81 * t83;
            let t87 = (simd::pow(t86, f64x8::splat(0.024974)));
            let t88 = t27 * t87;
            let t90 = f64x8::splat(1.0) + f64x8::splat(4.166666666666667e-10) * t40;
            let t91 = f64x8::splat(1.0) / t90;
            let t92 = t88 * t91;
            let t95 = ((t1).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t26 * t92));
            let t96 = (v_rho1).simd_le(dens_threshold);
            let t97 = -t16;
            let t99 = ((t14).select(t11, (t10).select(t15, t97 * t7)));
            let t100 = f64x8::splat(1.0) + t99;
            let t101 = (t100).simd_le(zeta_threshold);
            let t102 = (simd::cbrt(t100));
            let t104 = ((t101).select(t22, t102 * t100));
            let t105 = t5 * t104;
            let t106 = v_rho1 * v_rho1;
            let t107 = (simd::cbrt(v_rho1));
            let t108 = t107 * t107;
            let t110 = f64x8::splat(1.0) / t108 / t106;
            let t112 = t33 * v_sigma2 * t110;
            let t114 = v_sigma2 * v_sigma2;
            let t115 = t106 * t106;
            let t116 = t115 * v_rho1;
            let t118 = f64x8::splat(1.0) / t107 / t116;
            let t122 = t114 * v_sigma2;
            let t123 = t115 * t115;
            let t124 = f64x8::splat(1.0) / t123;
            let t127 = t114 * t114;
            let t128 = t123 * t106;
            let t130 = f64x8::splat(1.0) / t108 / t128;
            let t134 = t127 * v_sigma2;
            let t135 = t123 * t116;
            let t137 = f64x8::splat(1.0) / t107 / t135;
            let t141 = t127 * t114;
            let t142 = t123 * t123;
            let t143 = f64x8::splat(1.0) / t142;
            let t146 = f64x8::splat(1.0) + f64x8::splat(0.2058807993646726) * t112 + f64x8::splat(0.05171875) * t45 * t114 * t118 + f64x8::splat(9.988390807433105e-05) * t122 * t124 + f64x8::splat(0.00021916594328703703) * t62 * t127 * t130 + f64x8::splat(0.0011831024546682099) * t73 * t134 * t137 + f64x8::splat(1.1106816177675317e-09) * t141 * t143;
            let t147 = (simd::pow(t146, f64x8::splat(0.024974)));
            let t148 = t27 * t147;
            let t150 = f64x8::splat(1.0) + f64x8::splat(4.166666666666667e-10) * t112;
            let t151 = f64x8::splat(1.0) / t150;
            let t152 = t148 * t151;
            let t155 = ((t96).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t105 * t152));
            let tzk0 = t95 + t155;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}

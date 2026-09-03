//! GGA_C_OP_XALPHA vxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_op_xalpha.c`
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
pub fn gga_c_op_xalpha_vxc_pol(
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
            let t40 = f64x8::splat(M_CBRT2);
            let t41 = t39 * t40;
            let t42 = (t29).simd_le(zeta_threshold);
            let t43 = f64x8::splat(1.0) - t28;
            let t44 = (t43).simd_le(zeta_threshold);
            let t45 = ((t42).select(t14, (t44).select(t17, t28)));
            let t46 = f64x8::splat(1.0) + t45;
            let t47 = t46 * t2;
            let t48 = (simd::cbrt(t47));
            let t53 = ((t32).select(f64x8::splat(0.0), t38 * t41 / t48 / f64x8::splat(9.0)));
            let t57 = (t43 * t2 / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t58 = ((t44).select(t14, (t42).select(t17, -t28)));
            let t59 = f64x8::splat(1.0) + t58;
            let t60 = t59 * t2;
            let t61 = (simd::cbrt(t60));
            let t66 = ((t57).select(f64x8::splat(0.0), t38 * t41 / t61 / f64x8::splat(9.0)));
            let t67 = t53 + t66;
            let t68 = (t67).simd_eq(f64x8::splat(0.0));
            let t69 = ((t68).select(f64x8::splat(f64::EPSILON), t67));
            let t72 = f64x8::splat(3.90299956) / t69 + f64x8::splat(0.5764);
            let t73 = t69 * t69;
            let t74 = t73 * t73;
            let t75 = f64x8::splat(1.0) / t74;
            let t77 = t73 * t69;
            let t78 = f64x8::splat(1.0) / t77;
            let t80 = f64x8::splat(1.0) / t73;
            let t82 = f64x8::splat(43.31320905673766) * t75 + f64x8::splat(19.051463748196298) * t78 + f64x8::splat(2.094820520028) * t80;
            let t83 = f64x8::splat(1.0) / t82;
            let t84 = t72 * t83;
            let tzk0 = ((t11).select(f64x8::splat(0.0), -f64x8::splat(0.25) * t21 * t84));
            acc_zk = tzk0;
            let t87 = t2 * t2;
            let t88 = f64x8::splat(1.0) / t87;
            let t89 = t1 * t88;
            let t90 = t3 - t89;
            let t91 = ((t13).select(f64x8::splat(0.0), (t16).select(f64x8::splat(0.0), t90)));
            let t92 = t18 * t91;
            let t94 = t2 * t72 * t83;
            let t97 = t20 * t72;
            let t99 = f64x8::splat(0.25) * t97 * t83;
            let t100 = t38 * t39;
            let t103 = t40 / t48 / t47;
            let t104 = ((t24).select(f64x8::splat(0.0), (t27).select(f64x8::splat(0.0), t90)));
            let t105 = ((t42).select(f64x8::splat(0.0), (t44).select(f64x8::splat(0.0), t104)));
            let t107 = t105 * t2 + t45 + f64x8::splat(1.0);
            let t111 = ((t32).select(f64x8::splat(0.0), -t100 * t103 * t107 / f64x8::splat(27.0)));
            let t114 = t40 / t61 / t60;
            let t115 = ((t44).select(f64x8::splat(0.0), (t42).select(f64x8::splat(0.0), -t104)));
            let t117 = t115 * t2 + t58 + f64x8::splat(1.0);
            let t121 = ((t57).select(f64x8::splat(0.0), -t100 * t114 * t117 / f64x8::splat(27.0)));
            let t123 = ((t68).select(f64x8::splat(0.0), t111 + t121));
            let t124 = t80 * t123;
            let t125 = t124 * t83;
            let t128 = t82 * t82;
            let t129 = f64x8::splat(1.0) / t128;
            let t130 = t72 * t129;
            let t132 = f64x8::splat(1.0) / t74 / t69;
            let t133 = t132 * t123;
            let t135 = t75 * t123;
            let t139 = -f64x8::splat(173.25283622695065) * t133 - f64x8::splat(57.15439124458889) * t135 - f64x8::splat(4.189641040056) * t78 * t123;
            let t140 = t130 * t139;
            let t144 = ((t11).select(f64x8::splat(0.0), f64x8::splat(0.5) * t92 * t94 - t99 + f64x8::splat(0.97574989) * t21 * t125 + f64x8::splat(0.25) * t21 * t140));
            let tvrho0 = t2 * t144 + tzk0;
            acc_vrho_0 = tvrho0;
            let t146 = -t3 - t89;
            let t147 = ((t13).select(f64x8::splat(0.0), (t16).select(f64x8::splat(0.0), t146)));
            let t148 = t18 * t147;
            let t151 = ((t24).select(f64x8::splat(0.0), (t27).select(f64x8::splat(0.0), t146)));
            let t152 = ((t42).select(f64x8::splat(0.0), (t44).select(f64x8::splat(0.0), t151)));
            let t154 = t152 * t2 + t45 + f64x8::splat(1.0);
            let t158 = ((t32).select(f64x8::splat(0.0), -t100 * t103 * t154 / f64x8::splat(27.0)));
            let t159 = ((t44).select(f64x8::splat(0.0), (t42).select(f64x8::splat(0.0), -t151)));
            let t161 = t159 * t2 + t58 + f64x8::splat(1.0);
            let t165 = ((t57).select(f64x8::splat(0.0), -t100 * t114 * t161 / f64x8::splat(27.0)));
            let t167 = ((t68).select(f64x8::splat(0.0), t158 + t165));
            let t168 = t80 * t167;
            let t169 = t168 * t83;
            let t172 = t132 * t167;
            let t174 = t75 * t167;
            let t176 = t78 * t167;
            let t178 = -f64x8::splat(173.25283622695065) * t172 - f64x8::splat(57.15439124458889) * t174 - f64x8::splat(4.189641040056) * t176;
            let t179 = t130 * t178;
            let t183 = ((t11).select(f64x8::splat(0.0), f64x8::splat(0.5) * t148 * t94 - t99 + f64x8::splat(0.97574989) * t21 * t169 + f64x8::splat(0.25) * t21 * t179));
            let tvrho1 = t2 * t183 + tzk0;
            acc_vrho_1 = tvrho1;
            let tvsigma0 = f64x8::splat(0.0);
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let tvsigma2 = f64x8::splat(0.0);
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

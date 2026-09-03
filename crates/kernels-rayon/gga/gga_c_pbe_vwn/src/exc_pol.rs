//! GGA_C_PBE_VWN exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_pbe_vwn.c`
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
pub fn gga_c_pbe_vwn_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    param_gamma: f64,
    param_BB: f64,
    param_beta: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_gamma = f64x8::splat(param_gamma);
    let param_BB = f64x8::splat(param_BB);
    let param_beta = f64x8::splat(param_beta);
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
            let t1 = f64x8::splat(M_CBRT3);
            let t2 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t3 = (simd::cbrt(t2));
            let t4 = t1 * t3;
            let t5 = f64x8::splat(M_CBRT4);
            let t6 = t5 * t5;
            let t7 = v_rho0 + v_rho1;
            let t8 = (simd::cbrt(t7));
            let t9 = f64x8::splat(1.0) / t8;
            let t10 = t6 * t9;
            let t11 = t4 * t10;
            let t12 = t11 / f64x8::splat(4.0);
            let t13 = ((t11).sqrt());
            let t15 = t12 + f64x8::splat(1.86372) * t13 + f64x8::splat(12.9352);
            let t16 = f64x8::splat(1.0) / t15;
            let t20 = (simd::ln(t4 * t10 * t16 / f64x8::splat(4.0)));
            let t21 = f64x8::splat(0.0310907) * t20;
            let t22 = t13 + f64x8::splat(3.72744);
            let t25 = (simd::atan(f64x8::splat(6.15199081975908) / t22));
            let t26 = f64x8::splat(0.038783294878113016) * t25;
            let t27 = t13 / f64x8::splat(2.0);
            let t28 = t27 + f64x8::splat(0.10498);
            let t29 = t28 * t28;
            let t31 = (simd::ln(t29 * t16));
            let t32 = f64x8::splat(0.0009690227711544374) * t31;
            let t33 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t34 = f64x8::splat(1.0) / t33;
            let t36 = t12 + f64x8::splat(0.565535) * t13 + f64x8::splat(13.0045);
            let t37 = f64x8::splat(1.0) / t36;
            let t41 = (simd::ln(t4 * t10 * t37 / f64x8::splat(4.0)));
            let t42 = t13 + f64x8::splat(1.13107);
            let t45 = (simd::atan(f64x8::splat(7.123108917818118) / t42));
            let t47 = t27 + f64x8::splat(0.0047584);
            let t48 = t47 * t47;
            let t50 = (simd::ln(t48 * t37));
            let t53 = t34 * (t41 + f64x8::splat(0.31770800474394145) * t45 + f64x8::splat(0.00041403379428206277) * t50);
            let t54 = v_rho0 - v_rho1;
            let t55 = f64x8::splat(1.0) / t7;
            let t56 = t54 * t55;
            let t57 = f64x8::splat(1.0) + t56;
            let t58 = (t57).simd_le(zeta_threshold);
            let t59 = (simd::cbrt(zeta_threshold));
            let t60 = t59 * zeta_threshold;
            let t61 = (simd::cbrt(t57));
            let t62 = t61 * t57;
            let t63 = ((t58).select(t60, t62));
            let t64 = f64x8::splat(1.0) - t56;
            let t65 = (t64).simd_le(zeta_threshold);
            let t66 = (simd::cbrt(t64));
            let t67 = t66 * t64;
            let t68 = ((t65).select(t60, t67));
            let t69 = t63 + t68 - f64x8::splat(2.0);
            let t70 = t53 * t69;
            let t71 = f64x8::splat(M_CBRT2);
            let t72 = t71 - f64x8::splat(1.0);
            let t74 = f64x8::splat(1.0) / t72 / f64x8::splat(2.0);
            let t75 = t54 * t54;
            let t76 = t75 * t75;
            let t77 = t7 * t7;
            let t78 = t77 * t77;
            let t79 = f64x8::splat(1.0) / t78;
            let t83 = f64x8::splat(9.0) * t72;
            let t84 = t74 * (-t76 * t79 + f64x8::splat(1.0)) * t83;
            let t86 = t70 * t84 / f64x8::splat(24.0);
            let t88 = t12 + f64x8::splat(3.53021) * t13 + f64x8::splat(18.0578);
            let t89 = f64x8::splat(1.0) / t88;
            let t93 = (simd::ln(t4 * t10 * t89 / f64x8::splat(4.0)));
            let t95 = t13 + f64x8::splat(7.06042);
            let t98 = (simd::atan(f64x8::splat(4.730926909560113) / t95));
            let t100 = t27 + f64x8::splat(0.325);
            let t101 = t100 * t100;
            let t103 = (simd::ln(t101 * t89));
            let t105 = f64x8::splat(0.01554535) * t93 + f64x8::splat(0.05249139316978094) * t98 + f64x8::splat(0.0022478670955426118) * t103 - t21 - t26 - t32;
            let t106 = t105 * t69;
            let t107 = t74 * t76;
            let t108 = t107 * t79;
            let t109 = t106 * t108;
            let t110 = t59 * t59;
            let t111 = t61 * t61;
            let t112 = ((t58).select(t110, t111));
            let t113 = t66 * t66;
            let t114 = ((t65).select(t110, t113));
            let t116 = t112 / f64x8::splat(2.0) + t114 / f64x8::splat(2.0);
            let t117 = t116 * t116;
            let t118 = t117 * t116;
            let t119 = param_gamma * t118;
            let t121 = v_sigma0 + f64x8::splat(2.0) * v_sigma1 + v_sigma2;
            let t123 = f64x8::splat(1.0) / t8 / t77;
            let t124 = t121 * t123;
            let t126 = f64x8::splat(1.0) / t117;
            let t127 = t1 * t1;
            let t129 = f64x8::splat(1.0) / t3;
            let t130 = t129 * t5;
            let t131 = t126 * t127 * t130;
            let t134 = param_BB * param_beta;
            let t135 = f64x8::splat(1.0) / param_gamma;
            let t137 = (t21 + t26 + t32 - t86 + t109) * t135;
            let t138 = f64x8::splat(1.0) / t118;
            let t140 = (simd::exp(-t137 * t138));
            let t141 = t140 - f64x8::splat(1.0);
            let t142 = f64x8::splat(1.0) / t141;
            let t143 = t135 * t142;
            let t144 = t121 * t121;
            let t146 = t134 * t143 * t144;
            let t147 = t8 * t8;
            let t149 = f64x8::splat(1.0) / t147 / t78;
            let t150 = t71 * t71;
            let t151 = t149 * t150;
            let t152 = t117 * t117;
            let t153 = f64x8::splat(1.0) / t152;
            let t154 = t151 * t153;
            let t155 = t3 * t3;
            let t156 = f64x8::splat(1.0) / t155;
            let t157 = t1 * t156;
            let t158 = t157 * t6;
            let t159 = t154 * t158;
            let t162 = t124 * t71 * t131 / f64x8::splat(96.0) + t146 * t159 / f64x8::splat(3072.0);
            let t163 = param_beta * t162;
            let t164 = param_beta * t135;
            let t167 = t164 * t142 * t162 + f64x8::splat(1.0);
            let t168 = f64x8::splat(1.0) / t167;
            let t169 = t135 * t168;
            let t171 = t163 * t169 + f64x8::splat(1.0);
            let t172 = (simd::ln(t171));
            let t173 = t119 * t172;
            let tzk0 = t21 + t26 + t32 - t86 + t109 + t173;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}

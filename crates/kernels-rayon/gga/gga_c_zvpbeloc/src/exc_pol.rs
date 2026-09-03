//! GGA_C_ZVPBELOC exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_zvpbeloc.c`
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
pub fn gga_c_zvpbeloc_exc_pol(
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
            let t1 = (simd::pow(f64x8::splat(4.0), f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t2 = t1 * t1;
            let t3 = t2 * t2;
            let t5 = (simd::pow(f64x8::splat(3.0), f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t7 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t8 = f64x8::splat(1.0) / t7;
            let t9 = (simd::pow(t8, f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t10 = t3 * t1 * t5 * t9;
            let t11 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t12 = (simd::cbrt(t11));
            let t13 = v_rho0 + v_rho1;
            let t14 = (simd::cbrt(t13));
            let t15 = f64x8::splat(1.0) / t14;
            let t16 = t12 * t15;
            let t17 = v_rho0 - v_rho1;
            let t18 = t17 * t17;
            let t19 = t13 * t13;
            let t20 = f64x8::splat(1.0) / t19;
            let t21 = t18 * t20;
            let t22 = (f64x8::splat(1e-20)).simd_lt(t21);
            let t23 = ((t22).select(t21, f64x8::splat(1e-20)));
            let t27 = (simd::exp(-f64x8::splat(1.0) * t10 * t16 * t23));
            let t28 = f64x8::splat(M_CBRT3);
            let t29 = t28 * t12;
            let t30 = f64x8::splat(M_CBRT4);
            let t31 = t30 * t30;
            let t33 = t29 * t31 * t15;
            let t35 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t33;
            let t36 = ((t33).sqrt());
            let t39 = ((t33) * (t33).sqrt());
            let t41 = t28 * t28;
            let t42 = t12 * t12;
            let t43 = t41 * t42;
            let t44 = t14 * t14;
            let t47 = t43 * t30 / t44;
            let t49 = f64x8::splat(3.79785) * t36 + f64x8::splat(0.8969) * t33 + f64x8::splat(0.204775) * t39 + f64x8::splat(0.123235) * t47;
            let t52 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t49;
            let t53 = (simd::ln(t52));
            let t55 = f64x8::splat(0.0621814) * t35 * t53;
            let t56 = t18 * t18;
            let t57 = t19 * t19;
            let t58 = f64x8::splat(1.0) / t57;
            let t59 = t56 * t58;
            let t60 = f64x8::splat(1.0) / t13;
            let t61 = t17 * t60;
            let t62 = f64x8::splat(1.0) + t61;
            let t63 = (t62).simd_le(zeta_threshold);
            let t64 = (simd::cbrt(zeta_threshold));
            let t65 = t64 * zeta_threshold;
            let t66 = (simd::cbrt(t62));
            let t67 = t66 * t62;
            let t68 = ((t63).select(t65, t67));
            let t69 = f64x8::splat(1.0) - t61;
            let t70 = (t69).simd_le(zeta_threshold);
            let t71 = (simd::cbrt(t69));
            let t72 = t71 * t69;
            let t73 = ((t70).select(t65, t72));
            let t74 = t68 + t73 - f64x8::splat(2.0);
            let t75 = f64x8::splat(M_CBRT2);
            let t78 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t75 - f64x8::splat(2.0));
            let t79 = t74 * t78;
            let t81 = f64x8::splat(1.0) + f64x8::splat(0.05137) * t33;
            let t86 = f64x8::splat(7.05945) * t36 + f64x8::splat(1.549425) * t33 + f64x8::splat(0.420775) * t39 + f64x8::splat(0.1562925) * t47;
            let t89 = f64x8::splat(1.0) + f64x8::splat(32.16395899738507) / t86;
            let t90 = (simd::ln(t89));
            let t94 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t33;
            let t99 = f64x8::splat(5.1785) * t36 + f64x8::splat(0.905775) * t33 + f64x8::splat(0.1100325) * t39 + f64x8::splat(0.1241775) * t47;
            let t102 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t99;
            let t103 = (simd::ln(t102));
            let t104 = t94 * t103;
            let t106 = -f64x8::splat(0.0310907) * t81 * t90 + t55 - f64x8::splat(0.0197516734986138) * t104;
            let t107 = t79 * t106;
            let t108 = t59 * t107;
            let t110 = f64x8::splat(0.0197516734986138) * t79 * t104;
            let t111 = (simd::ln(f64x8::splat(2.0)));
            let t112 = f64x8::splat(1.0) - t111;
            let t113 = t112 * t8;
            let t114 = t64 * t64;
            let t115 = t66 * t66;
            let t116 = ((t63).select(t114, t115));
            let t117 = t71 * t71;
            let t118 = ((t70).select(t114, t117));
            let t120 = t116 / f64x8::splat(2.0) + t118 / f64x8::splat(2.0);
            let t121 = t120 * t120;
            let t122 = t121 * t120;
            let t124 = v_sigma0 + f64x8::splat(2.0) * v_sigma1 + v_sigma2;
            let t126 = f64x8::splat(1.0) / t14 / t19;
            let t127 = t124 * t126;
            let t128 = f64x8::splat(1.0) / t121;
            let t129 = t75 * t128;
            let t131 = f64x8::splat(1.0) / t12;
            let t132 = t41 * t131;
            let t134 = (simd::exp(-t47 / f64x8::splat(4.0)));
            let t135 = f64x8::splat(1.0) - t134;
            let t136 = t30 * t135;
            let t137 = t132 * t136;
            let t140 = f64x8::splat(0.0375) + f64x8::splat(0.0008333333333333334) * t127 * t129 * t137;
            let t142 = t128 * t41;
            let t143 = t131 * t30;
            let t144 = t142 * t143;
            let t147 = f64x8::splat(1.0) / t112;
            let t148 = t140 * t147;
            let t150 = (-t55 + t108 + t110) * t147;
            let t151 = f64x8::splat(1.0) / t122;
            let t152 = t7 * t151;
            let t154 = (simd::exp(-t150 * t152));
            let t155 = t154 - f64x8::splat(1.0);
            let t156 = f64x8::splat(1.0) / t155;
            let t157 = t7 * t156;
            let t158 = t124 * t124;
            let t159 = t157 * t158;
            let t160 = t148 * t159;
            let t162 = f64x8::splat(1.0) / t44 / t57;
            let t163 = t75 * t75;
            let t164 = t162 * t163;
            let t165 = t121 * t121;
            let t166 = f64x8::splat(1.0) / t165;
            let t168 = f64x8::splat(1.0) / t42;
            let t169 = t28 * t168;
            let t170 = t169 * t31;
            let t171 = t164 * t166 * t170;
            let t174 = t127 * t75 * t144 / f64x8::splat(96.0) + t160 * t171 / f64x8::splat(3072.0);
            let t175 = t140 * t174;
            let t176 = t147 * t7;
            let t177 = t157 * t174;
            let t179 = t148 * t177 + f64x8::splat(1.0);
            let t180 = f64x8::splat(1.0) / t179;
            let t181 = t176 * t180;
            let t183 = t175 * t181 + f64x8::splat(1.0);
            let t184 = (simd::ln(t183));
            let t187 = t113 * t122 * t184 + t108 + t110 - t55;
            let tzk0 = t27 * t187;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}

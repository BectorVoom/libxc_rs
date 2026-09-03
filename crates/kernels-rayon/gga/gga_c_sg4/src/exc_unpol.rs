//! GGA_C_SG4 exc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_sg4.c`
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

#[allow(unused_variables, non_snake_case)]
pub fn gga_c_sg4_exc_unpol(
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
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let mut acc_zk = V_ZERO;
        {
            let t1 = f64x8::splat(M_CBRT3);
            let t2 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t3 = (simd::cbrt(t2));
            let t4 = t1 * t3;
            let t5 = f64x8::splat(M_CBRT4);
            let t6 = t5 * t5;
            let t7 = (simd::cbrt(v_rho));
            let t10 = t4 * t6 / t7;
            let t12 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t10;
            let t13 = ((t10).sqrt());
            let t16 = ((t10) * (t10).sqrt());
            let t18 = t1 * t1;
            let t19 = t3 * t3;
            let t20 = t18 * t19;
            let t21 = t7 * t7;
            let t24 = t20 * t5 / t21;
            let t26 = f64x8::splat(3.79785) * t13 + f64x8::splat(0.8969) * t10 + f64x8::splat(0.204775) * t16 + f64x8::splat(0.123235) * t24;
            let t29 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t26;
            let t30 = (simd::ln(t29));
            let t32 = f64x8::splat(0.0621814) * t12 * t30;
            let t33 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t34 = (simd::cbrt(zeta_threshold));
            let t36 = ((t33).select(t34 * zeta_threshold, f64x8::splat(1.0)));
            let t39 = f64x8::splat(M_CBRT2);
            let t43 = (f64x8::splat(2.0) * t36 - f64x8::splat(2.0)) / (f64x8::splat(2.0) * t39 - f64x8::splat(2.0));
            let t45 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t10;
            let t50 = f64x8::splat(5.1785) * t13 + f64x8::splat(0.905775) * t10 + f64x8::splat(0.1100325) * t16 + f64x8::splat(0.1241775) * t24;
            let t53 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t50;
            let t54 = (simd::ln(t53));
            let t57 = f64x8::splat(0.0197516734986138) * t43 * t45 * t54;
            let t58 = t34 * t34;
            let t59 = ((t33).select(t58, f64x8::splat(1.0)));
            let t60 = ((v_sigma).sqrt());
            let t61 = t60 * v_sigma;
            let t62 = v_rho * v_rho;
            let t63 = t62 * t62;
            let t64 = f64x8::splat(1.0) / t63;
            let t66 = t59 * t59;
            let t67 = t66 * t59;
            let t68 = f64x8::splat(1.0) / t67;
            let t70 = f64x8::splat(1.0) / t13 / t10;
            let t71 = t68 * t70;
            let t74 = (simd::pow(t59, f64x8::splat(0.05) * t61 * t64 * t71));
            let t75 = (simd::ln(f64x8::splat(2.0)));
            let t76 = f64x8::splat(1.0) - t75;
            let t77 = t74 * t76;
            let t78 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t79 = f64x8::splat(1.0) / t78;
            let t80 = t79 * t67;
            let t82 = f64x8::splat(1.0) / t7 / v_rho;
            let t84 = t39 * t39;
            let t86 = f64x8::splat(1.0) / t59;
            let t87 = f64x8::splat(1.0) / t13;
            let t88 = t86 * t87;
            let t90 = (simd::exp(-t24 / f64x8::splat(4.0)));
            let t91 = f64x8::splat(1.0) - t90;
            let t92 = t88 * t91;
            let t95 = f64x8::splat(0.07963845034287749) + f64x8::splat(0.0175) * t60 * t82 * t84 * t92;
            let t97 = f64x8::splat(1.0) / t7 / t62;
            let t100 = f64x8::splat(1.0) / t66;
            let t102 = f64x8::splat(1.0) / t3;
            let t104 = t100 * t18 * t102 * t5;
            let t107 = f64x8::splat(1.0) / t76;
            let t108 = t95 * t107;
            let t113 = (simd::exp(-(-t32 + t57) * t107 * t78 * t68));
            let t114 = t113 - f64x8::splat(1.0);
            let t115 = f64x8::splat(1.0) / t114;
            let t116 = t78 * t115;
            let t117 = v_sigma * v_sigma;
            let t118 = t116 * t117;
            let t119 = t108 * t118;
            let t121 = f64x8::splat(1.0) / t21 / t63;
            let t122 = t121 * t84;
            let t123 = t66 * t66;
            let t124 = f64x8::splat(1.0) / t123;
            let t126 = f64x8::splat(1.0) / t19;
            let t127 = t1 * t126;
            let t128 = t127 * t6;
            let t129 = t122 * t124 * t128;
            let t132 = v_sigma * t97 * t39 * t104 / f64x8::splat(96.0) + t119 * t129 / f64x8::splat(3072.0);
            let t133 = t95 * t132;
            let t134 = t107 * t78;
            let t135 = t116 * t132;
            let t137 = t108 * t135 + f64x8::splat(1.0);
            let t138 = f64x8::splat(1.0) / t137;
            let t139 = t134 * t138;
            let t141 = t133 * t139 + f64x8::splat(1.0);
            let t142 = (simd::ln(t141));
            let t144 = t77 * t80 * t142;
            let tzk0 = -t32 + t57 + t144;
            acc_zk = tzk0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}

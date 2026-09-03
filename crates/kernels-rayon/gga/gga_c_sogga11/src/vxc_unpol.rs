//! GGA_C_SOGGA11 vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_sogga11.c`
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
pub fn gga_c_sogga11_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    param_sogga11_a_1: f64,
    param_sogga11_a_2: f64,
    param_sogga11_a_3: f64,
    param_sogga11_a_4: f64,
    param_sogga11_a_5: f64,
    param_sogga11_b_1: f64,
    param_sogga11_b_2: f64,
    param_sogga11_b_3: f64,
    param_sogga11_b_4: f64,
    param_sogga11_b_5: f64,
    param_sogga11_a_0: f64,
    param_sogga11_b_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_sogga11_a_1 = f64x8::splat(param_sogga11_a_1);
    let param_sogga11_a_2 = f64x8::splat(param_sogga11_a_2);
    let param_sogga11_a_3 = f64x8::splat(param_sogga11_a_3);
    let param_sogga11_a_4 = f64x8::splat(param_sogga11_a_4);
    let param_sogga11_a_5 = f64x8::splat(param_sogga11_a_5);
    let param_sogga11_b_1 = f64x8::splat(param_sogga11_b_1);
    let param_sogga11_b_2 = f64x8::splat(param_sogga11_b_2);
    let param_sogga11_b_3 = f64x8::splat(param_sogga11_b_3);
    let param_sogga11_b_4 = f64x8::splat(param_sogga11_b_4);
    let param_sogga11_b_5 = f64x8::splat(param_sogga11_b_5);
    let param_sogga11_a_0 = f64x8::splat(param_sogga11_a_0);
    let param_sogga11_b_0 = f64x8::splat(param_sogga11_b_0);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
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
            let t33 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t34 = (simd::cbrt(zeta_threshold));
            let t36 = ((t33).select(t34 * zeta_threshold, f64x8::splat(1.0)));
            let t39 = f64x8::splat(M_CBRT2);
            let t43 = (f64x8::splat(2.0) * t36 - f64x8::splat(2.0)) / (f64x8::splat(2.0) * t39 - f64x8::splat(2.0));
            let t45 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t10;
            let t50 = f64x8::splat(5.1785) * t13 + f64x8::splat(0.905775) * t10 + f64x8::splat(0.1100325) * t16 + f64x8::splat(0.1241775) * t24;
            let t53 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t50;
            let t54 = (simd::ln(t53));
            let t58 = -f64x8::splat(0.0621814) * t12 * t30 + f64x8::splat(0.0197516734986138) * t43 * t45 * t54;
            let t60 = param_sogga11_a_1;
            let t61 = t34 * t34;
            let t62 = ((t33).select(t61, f64x8::splat(1.0)));
            let t63 = t39 * t62;
            let t64 = v_rho * v_rho;
            let t66 = f64x8::splat(1.0) / t7 / t64;
            let t67 = v_sigma * t66;
            let t68 = t63 * t67;
            let t69 = f64x8::splat(1.0) / t3;
            let t70 = t18 * t69;
            let t71 = f64x8::splat(1.0) / t58;
            let t72 = t5 * t71;
            let t73 = t70 * t72;
            let t75 = f64x8::splat(0.0006950658458333333) * t68 * t73;
            let t76 = f64x8::splat(1.0) - t75;
            let t78 = f64x8::splat(1.0) - f64x8::splat(1.0) / t76;
            let t80 = param_sogga11_a_2;
            let t81 = t78 * t78;
            let t83 = param_sogga11_a_3;
            let t84 = t81 * t78;
            let t86 = param_sogga11_a_4;
            let t87 = t81 * t81;
            let t89 = param_sogga11_a_5;
            let t93 = param_sogga11_b_1;
            let t94 = (simd::exp(t75));
            let t95 = f64x8::splat(1.0) - t94;
            let t97 = param_sogga11_b_2;
            let t98 = t95 * t95;
            let t100 = param_sogga11_b_3;
            let t101 = t98 * t95;
            let t103 = param_sogga11_b_4;
            let t104 = t98 * t98;
            let t106 = param_sogga11_b_5;
            let t109 = t106 * t104 * t95 + t89 * t87 * t78 + t100 * t101 + t103 * t104 + t60 * t78 + t80 * t81 + t83 * t84 + t86 * t87 + t93 * t95 + t97 * t98 + param_sogga11_a_0 + param_sogga11_b_0;
            let tzk0 = t58 * t109;
            acc_zk = tzk0;
            let t111 = f64x8::splat(1.0) / t7 / v_rho;
            let t112 = t6 * t111;
            let t116 = t26 * t26;
            let t117 = f64x8::splat(1.0) / t116;
            let t118 = t12 * t117;
            let t120 = f64x8::splat(1.0) / t13 * t1;
            let t121 = t3 * t6;
            let t122 = t121 * t111;
            let t123 = t120 * t122;
            let t125 = t4 * t112;
            let t127 = ((t10).sqrt());
            let t128 = t127 * t1;
            let t129 = t128 * t122;
            let t134 = t20 * t5 / t21 / v_rho;
            let t136 = -f64x8::splat(0.632975) * t123 - f64x8::splat(0.29896666666666666) * t125 - f64x8::splat(0.1023875) * t129 - f64x8::splat(0.08215666666666667) * t134;
            let t137 = f64x8::splat(1.0) / t29;
            let t138 = t136 * t137;
            let t141 = t43 * t1;
            let t146 = t43 * t45;
            let t147 = t50 * t50;
            let t148 = f64x8::splat(1.0) / t147;
            let t153 = -f64x8::splat(0.8630833333333333) * t123 - f64x8::splat(0.301925) * t125 - f64x8::splat(0.05501625) * t129 - f64x8::splat(0.082785) * t134;
            let t155 = f64x8::splat(1.0) / t53;
            let t156 = t148 * t153 * t155;
            let t159 = f64x8::splat(0.0011073470983333333) * t4 * t112 * t30 + f64x8::splat(1.0) * t118 * t138 - f64x8::splat(0.00018311447306006544) * t141 * t121 * t111 * t54 - f64x8::splat(0.5848223622634646) * t146 * t156;
            let t160 = v_rho * t159;
            let t162 = v_rho * t58;
            let t163 = t76 * t76;
            let t164 = f64x8::splat(1.0) / t163;
            let t165 = t60 * t164;
            let t166 = t64 * v_rho;
            let t168 = f64x8::splat(1.0) / t7 / t166;
            let t169 = v_sigma * t168;
            let t170 = t63 * t169;
            let t173 = t58 * t58;
            let t174 = f64x8::splat(1.0) / t173;
            let t175 = t5 * t174;
            let t176 = t175 * t159;
            let t177 = t70 * t176;
            let t180 = f64x8::splat(0.0016218203069444444) * t170 * t73 + f64x8::splat(0.0006950658458333333) * t68 * t177;
            let t182 = t80 * t78;
            let t183 = t164 * t180;
            let t186 = t83 * t81;
            let t189 = t86 * t84;
            let t192 = t89 * t87;
            let t195 = -t180;
            let t196 = t93 * t195;
            let t198 = t97 * t95;
            let t199 = t195 * t94;
            let t202 = t100 * t98;
            let t205 = t103 * t101;
            let t208 = t106 * t104;
            let t211 = t165 * t180 + f64x8::splat(2.0) * t182 * t183 + f64x8::splat(3.0) * t186 * t183 + f64x8::splat(4.0) * t189 * t183 + f64x8::splat(5.0) * t192 * t183 - t196 * t94 - f64x8::splat(2.0) * t198 * t199 - f64x8::splat(3.0) * t202 * t199 - f64x8::splat(4.0) * t205 * t199 - f64x8::splat(5.0) * t208 * t199;
            let tvrho0 = t160 * t109 + t162 * t211 + tzk0;
            acc_vrho = tvrho0;
            let t213 = t165 * t63;
            let t214 = t66 * t18;
            let t215 = t69 * t5;
            let t216 = t215 * t71;
            let t217 = t214 * t216;
            let t221 = t164 * t39 * t62;
            let t222 = t182 * t221;
            let t225 = t186 * t221;
            let t228 = t189 * t221;
            let t231 = t192 * t221;
            let t234 = t93 * t39;
            let t235 = t62 * t66;
            let t238 = t70 * t72 * t94;
            let t241 = t63 * t66;
            let t242 = t198 * t241;
            let t245 = t202 * t241;
            let t248 = t205 * t241;
            let t251 = t208 * t241;
            let t254 = -f64x8::splat(0.0006950658458333333) * t213 * t217 - f64x8::splat(0.0013901316916666666) * t222 * t217 - f64x8::splat(0.0020851975375) * t225 * t217 - f64x8::splat(0.0027802633833333332) * t228 * t217 - f64x8::splat(0.0034753292291666666) * t231 * t217 - f64x8::splat(0.0006950658458333333) * t234 * t235 * t238 - f64x8::splat(0.0013901316916666666) * t242 * t238 - f64x8::splat(0.0020851975375) * t245 * t238 - f64x8::splat(0.0027802633833333332) * t248 * t238 - f64x8::splat(0.0034753292291666666) * t251 * t238;
            let tvsigma0 = t162 * t254;
            acc_vsigma = tvsigma0;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}

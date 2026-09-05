//! MGGA_C_CCALDA vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_c_ccalda.c`
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

/// Accumulate 8 consecutive grid points into an output array.
///
/// `+=`, not `=`. The scalar kernel writes `out[ip] += v`; a plain store is a
/// different operation in two ways. It keeps the sign of a negative zero where
/// `0.0 + -0.0` gives `+0.0` -- a bit difference the fingerprint gate reports
/// as a rejection even though no value changed (`gga_x_pbepow fxc` was
/// rejected on exactly this, 273 of 200,000 `v2sigma2` elements) -- and it
/// would discard whatever a caller had already put in the buffer.
#[inline(always)]
fn store_add(s: &mut [f64], ip: usize, m: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let mut b = [0.0f64; 8];
        b.copy_from_slice(&s[ip..ip + 8]);
        let r: [f64; 8] = (f64x8::new(b) + acc).into();
        s[ip..ip + 8].copy_from_slice(&r);
    } else {
        for k in 0..m {
            s[ip + k] += a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn mgga_c_ccalda_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
    param_c: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_c = f64x8::splat(param_c);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let v_lapl = load(lapl, ip, np);
        let v_tau = load(tau, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
        let mut acc_vlapl = V_ZERO;
        let mut acc_vtau = V_ZERO;
        {
            let t2 = f64x8::splat(1.0) + param_c;
            let t3 = (simd::cbrt(v_rho));
            let t4 = t3 * t3;
            let t6 = f64x8::splat(1.0) / t4 / v_rho;
            let t8 = v_rho * v_rho;
            let t10 = f64x8::splat(1.0) / t4 / t8;
            let t13 = v_tau * t6 - v_sigma * t10 / f64x8::splat(8.0);
            let t14 = t2 * t13;
            let t15 = f64x8::splat(M_CBRT6);
            let t16 = t14 * t15;
            let t17 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t18 = (simd::cbrt(t17));
            let t19 = t18 * t18;
            let t20 = f64x8::splat(1.0) / t19;
            let t21 = f64x8::splat(M_CBRT2);
            let t22 = t21 * t21;
            let t23 = t20 * t22;
            let t26 = t15 * t20 * t22;
            let t29 = f64x8::splat(1.0) + f64x8::splat(5.0) / f64x8::splat(9.0) * param_c * t13 * t26;
            let t30 = f64x8::splat(1.0) / t29;
            let t31 = f64x8::splat(M_CBRT3);
            let t32 = f64x8::splat(1.0) / f64x8::splat(M_PI);
            let t33 = (simd::cbrt(t32));
            let t34 = t31 * t33;
            let t35 = f64x8::splat(M_CBRT4);
            let t36 = t35 * t35;
            let t39 = t34 * t36 / t3;
            let t41 = f64x8::splat(1.0) + f64x8::splat(0.053425) * t39;
            let t42 = ((t39).sqrt());
            let t45 = ((t39) * (t39).sqrt());
            let t47 = t31 * t31;
            let t48 = t33 * t33;
            let t49 = t47 * t48;
            let t52 = t49 * t35 / t4;
            let t54 = f64x8::splat(3.79785) * t42 + f64x8::splat(0.8969) * t39 + f64x8::splat(0.204775) * t45 + f64x8::splat(0.123235) * t52;
            let t57 = f64x8::splat(1.0) + f64x8::splat(16.081979498692537) / t54;
            let t58 = (simd::ln(t57));
            let t62 = (simd::cbrt(zeta_threshold));
            let t64 = (((f64x8::splat(1.0)).simd_le(zeta_threshold)).select(t62 * zeta_threshold, f64x8::splat(1.0)));
            let t70 = (f64x8::splat(2.0) * t64 - f64x8::splat(2.0)) / (f64x8::splat(2.0) * t21 - f64x8::splat(2.0));
            let t72 = f64x8::splat(1.0) + f64x8::splat(0.0278125) * t39;
            let t77 = f64x8::splat(5.1785) * t42 + f64x8::splat(0.905775) * t39 + f64x8::splat(0.1100325) * t45 + f64x8::splat(0.1241775) * t52;
            let t80 = f64x8::splat(1.0) + f64x8::splat(29.608749977793437) / t77;
            let t81 = (simd::ln(t80));
            let t85 = -f64x8::splat(0.0621814) * t41 * t58 + f64x8::splat(0.0197516734986138) * t70 * t72 * t81;
            let t87 = t23 * t30 * t85;
            let t89 = f64x8::splat(5.0) / f64x8::splat(9.0) * t16 * t87;
            let t90 = t23 * t30;
            let t93 = f64x8::splat(1.0) - f64x8::splat(5.0) / f64x8::splat(9.0) * t16 * t90;
            let t94 = t93 * t85;
            let tzk0 = t89 + t94;
            acc_zk = tzk0;
            let t97 = t8 * v_rho;
            let t99 = f64x8::splat(1.0) / t4 / t97;
            let t102 = -f64x8::splat(5.0) / f64x8::splat(3.0) * v_tau * t10 + v_sigma * t99 / f64x8::splat(3.0);
            let t103 = t2 * t102;
            let t104 = t103 * t15;
            let t105 = t104 * t87;
            let t107 = t15 * t15;
            let t109 = f64x8::splat(1.0) / t18 / t17;
            let t110 = t107 * t109;
            let t111 = t14 * t110;
            let t112 = t29 * t29;
            let t113 = f64x8::splat(1.0) / t112;
            let t114 = t21 * t113;
            let t115 = t85 * param_c;
            let t117 = t114 * t115 * t102;
            let t118 = t111 * t117;
            let t121 = f64x8::splat(1.0) / t3 / v_rho;
            let t122 = t36 * t121;
            let t126 = t54 * t54;
            let t127 = f64x8::splat(1.0) / t126;
            let t128 = t41 * t127;
            let t130 = f64x8::splat(1.0) / t42 * t31;
            let t131 = t33 * t36;
            let t132 = t131 * t121;
            let t133 = t130 * t132;
            let t135 = t34 * t122;
            let t137 = ((t39).sqrt());
            let t138 = t137 * t31;
            let t139 = t138 * t132;
            let t142 = t49 * t35 * t6;
            let t144 = -f64x8::splat(0.632975) * t133 - f64x8::splat(0.29896666666666666) * t135 - f64x8::splat(0.1023875) * t139 - f64x8::splat(0.08215666666666667) * t142;
            let t145 = f64x8::splat(1.0) / t57;
            let t146 = t144 * t145;
            let t149 = t70 * t31;
            let t154 = t70 * t72;
            let t155 = t77 * t77;
            let t156 = f64x8::splat(1.0) / t155;
            let t161 = -f64x8::splat(0.8630833333333333) * t133 - f64x8::splat(0.301925) * t135 - f64x8::splat(0.05501625) * t139 - f64x8::splat(0.082785) * t142;
            let t163 = f64x8::splat(1.0) / t80;
            let t164 = t156 * t161 * t163;
            let t167 = f64x8::splat(0.0011073470983333333) * t34 * t122 * t58 + f64x8::splat(1.0) * t128 * t146 - f64x8::splat(0.00018311447306006544) * t149 * t131 * t121 * t81 - f64x8::splat(0.5848223622634646) * t154 * t164;
            let t169 = t23 * t30 * t167;
            let t170 = t16 * t169;
            let t175 = t114 * param_c * t102;
            let t178 = -f64x8::splat(5.0) / f64x8::splat(9.0) * t104 * t90 + f64x8::splat(50.0) / f64x8::splat(81.0) * t111 * t175;
            let t179 = t178 * t85;
            let t180 = t93 * t167;
            let tvrho0 = t89 + t94 + v_rho * (f64x8::splat(5.0) / f64x8::splat(9.0) * t105 - f64x8::splat(50.0) / f64x8::splat(81.0) * t118 + f64x8::splat(5.0) / f64x8::splat(9.0) * t170 + t179 + t180);
            acc_vrho = tvrho0;
            let t183 = t2 * t10;
            let t184 = t183 * t15;
            let t185 = t184 * t87;
            let t186 = f64x8::splat(5.0) / f64x8::splat(72.0) * t185;
            let t189 = t111 * t114 * t115 * t10;
            let t190 = f64x8::splat(25.0) / f64x8::splat(324.0) * t189;
            let t191 = t184 * t90;
            let t195 = t111 * t114 * param_c * t10;
            let t197 = f64x8::splat(5.0) / f64x8::splat(72.0) * t191 - f64x8::splat(25.0) / f64x8::splat(324.0) * t195;
            let t198 = t197 * t85;
            let tvsigma0 = v_rho * (-t186 + t190 + t198);
            acc_vsigma = tvsigma0;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl = tvlapl0;
            let t200 = t2 * t6;
            let t201 = t200 * t15;
            let t203 = f64x8::splat(5.0) / f64x8::splat(9.0) * t201 * t87;
            let t207 = f64x8::splat(50.0) / f64x8::splat(81.0) * t111 * t114 * t115 * t6;
            let t214 = -f64x8::splat(5.0) / f64x8::splat(9.0) * t201 * t90 + f64x8::splat(50.0) / f64x8::splat(81.0) * t111 * t114 * param_c * t6;
            let t215 = t214 * t85;
            let tvtau0 = v_rho * (t203 - t207 + t215);
            acc_vtau = tvtau0;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        store_add(vlapl, ip, m, acc_vlapl);
        store_add(vtau, ip, m, acc_vtau);
        ip += 8;
    }
}

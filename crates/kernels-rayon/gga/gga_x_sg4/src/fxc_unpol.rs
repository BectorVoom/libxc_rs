//! GGA_X_SG4 fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_sg4.c`
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
pub fn gga_x_sg4_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
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
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
        let mut acc_v2rho2 = V_ZERO;
        let mut acc_v2rhosigma = V_ZERO;
        let mut acc_v2sigma2 = V_ZERO;
        {
            let t2 = (v_rho / f64x8::splat(2.0)).simd_le(dens_threshold);
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = f64x8::splat(M_CBRTPI);
            let t6 = t3 / t4;
            let t7 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t8 = zeta_threshold - f64x8::splat(1.0);
            let t10 = ((t7).select(t8, (t7).select(-t8, f64x8::splat(0.0))));
            let t11 = f64x8::splat(1.0) + t10;
            let t13 = (simd::cbrt(zeta_threshold));
            let t15 = (simd::cbrt(t11));
            let t17 = (((t11).simd_le(zeta_threshold)).select(t13 * zeta_threshold, t15 * t11));
            let t18 = (simd::cbrt(v_rho));
            let t19 = t17 * t18;
            let t20 = f64x8::splat(M_CBRT6);
            let t21 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t22 = (simd::cbrt(t21));
            let t23 = t22 * t22;
            let t24 = f64x8::splat(1.0) / t23;
            let t25 = t20 * t24;
            let t26 = f64x8::splat(M_CBRT2);
            let t27 = t26 * t26;
            let t28 = v_sigma * t27;
            let t29 = v_rho * v_rho;
            let t30 = t18 * t18;
            let t32 = f64x8::splat(1.0) / t30 / t29;
            let t34 = t25 * t28 * t32;
            let t36 = f64x8::splat(1.0) - f64x8::splat(0.0031233982573039467) * t34;
            let t37 = t20 * t20;
            let t38 = t21 * t21;
            let t39 = t38 * t21;
            let t41 = f64x8::splat(1.0) / t22 / t39;
            let t42 = t37 * t41;
            let t43 = v_sigma * v_sigma;
            let t44 = t43 * t43;
            let t45 = t44 * v_sigma;
            let t47 = t29 * t29;
            let t48 = t47 * v_rho;
            let t49 = t47 * t47;
            let t50 = t49 * t48;
            let t52 = f64x8::splat(1.0) / t18 / t50;
            let t56 = f64x8::splat(1.0) - f64x8::splat(1.426849132767203e-11) * t42 * t45 * t26 * t52;
            let t57 = f64x8::splat(1.0) / t56;
            let t61 = f64x8::splat(1.0) + f64x8::splat(0.03727064220183486) * t34;
            let t64 = f64x8::splat(1.804) - f64x8::splat(0.5602871794871794) * t36 * t57 - f64x8::splat(0.2437128205128205) / t61;
            let t68 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t64));
            let tzk0 = f64x8::splat(2.0) * t68;
            acc_zk = tzk0;
            let t70 = t17 / t30;
            let t74 = t25 * v_sigma;
            let t75 = t29 * v_rho;
            let t77 = f64x8::splat(1.0) / t30 / t75;
            let t79 = t27 * t77 * t57;
            let t82 = t56 * t56;
            let t83 = f64x8::splat(1.0) / t82;
            let t85 = t36 * t83 * t37;
            let t86 = t41 * t45;
            let t87 = t47 * t29;
            let t88 = t49 * t87;
            let t91 = t26 / t18 / t88;
            let t95 = t61 * t61;
            let t97 = f64x8::splat(1.0) / t95 * t20;
            let t98 = t97 * t24;
            let t102 = -f64x8::splat(0.004666666666666667) * t74 * t79 + f64x8::splat(1.0659270348691523e-10) * t85 * t86 * t91 - f64x8::splat(0.02422222222222222) * t98 * t28 * t77;
            let t107 = ((t2).select(f64x8::splat(0.0), -t6 * t70 * t64 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t102));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t107 + f64x8::splat(2.0) * t68;
            acc_vrho = tvrho0;
            let t114 = t41 * t44;
            let t115 = t26 * t52;
            let t119 = t24 * t27;
            let t123 = f64x8::splat(0.00175) * t25 * t27 * t32 * t57 - f64x8::splat(3.997226380759321e-11) * t85 * t114 * t115 + f64x8::splat(0.009083333333333334) * t97 * t119 * t32;
            let t127 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t123));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t127;
            acc_vsigma = tvsigma0;
            let t132 = t17 / t30 / v_rho;
            let t140 = f64x8::splat(1.0) / t30 / t47;
            let t142 = t27 * t140 * t57;
            let t145 = t44 * t43;
            let t146 = t49 * t49;
            let t148 = f64x8::splat(1.0) / t146 / t29;
            let t153 = f64x8::splat(1.0) / t82 / t56;
            let t155 = t36 * t153 * t20;
            let t156 = t38 * t38;
            let t159 = f64x8::splat(1.0) / t23 / t156 / t38;
            let t160 = t44 * t44;
            let t161 = t160 * t43;
            let t162 = t159 * t161;
            let t163 = t49 * t47;
            let t167 = t27 / t30 / t146 / t163;
            let t171 = t47 * t75;
            let t172 = t49 * t171;
            let t175 = t26 / t18 / t172;
            let t181 = f64x8::splat(1.0) / t95 / t61 * t37;
            let t183 = f64x8::splat(1.0) / t22 / t21;
            let t184 = t181 * t183;
            let t185 = t43 * t26;
            let t187 = f64x8::splat(1.0) / t18 / t171;
            let t194 = f64x8::splat(0.01711111111111111) * t74 * t142 + f64x8::splat(2.245617754729564e-15) * t145 * t148 * t83 - f64x8::splat(2.4334673044738656e-19) * t155 * t162 * t167 - f64x8::splat(1.5278287499791183e-09) * t85 * t86 * t175 - f64x8::splat(0.00962962962962963) * t184 * t185 * t187 + f64x8::splat(0.08881481481481482) * t98 * t28 * t140;
            let t199 = ((t2).select(f64x8::splat(0.0), t6 * t132 * t64 / f64x8::splat(12.0) - t6 * t70 * t102 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t194));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t199 + f64x8::splat(4.0) * t107;
            acc_v2rho2 = tv2rho20;
            let t207 = t146 * v_rho;
            let t209 = f64x8::splat(1.0) / t207 * t83;
            let t212 = t160 * v_sigma;
            let t213 = t159 * t212;
            let t214 = t49 * t75;
            let t218 = t27 / t30 / t146 / t214;
            let t226 = f64x8::splat(1.0) / t18 / t87;
            let t234 = -f64x8::splat(0.004666666666666667) * t25 * t79 - f64x8::splat(8.421066580235865e-16) * t209 * t45 + f64x8::splat(9.125502391776996e-20) * t155 * t213 * t218 + f64x8::splat(5.329635174345761e-10) * t85 * t114 * t91 + f64x8::splat(0.003611111111111111) * t184 * t26 * t226 * v_sigma - f64x8::splat(0.02422222222222222) * t97 * t119 * t77;
            let t239 = ((t2).select(f64x8::splat(0.0), -t6 * t70 * t123 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t234));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t239 + f64x8::splat(2.0) * t127;
            acc_v2rhosigma = tv2rhosigma0;
            let t243 = f64x8::splat(1.0) / t146 * t83;
            let t246 = t159 * t160;
            let t247 = t49 * t29;
            let t251 = t27 / t30 / t146 / t247;
            let t255 = t43 * v_sigma;
            let t256 = t41 * t255;
            let t260 = t183 * t26;
            let t266 = f64x8::splat(3.157899967588449e-16) * t243 * t44 - f64x8::splat(3.4220633969163733e-20) * t155 * t246 * t251 - f64x8::splat(1.5988905523037283e-10) * t85 * t256 * t115 - f64x8::splat(0.0013541666666666667) * t181 * t260 / t18 / t48;
            let t270 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t266));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t270;
            acc_v2sigma2 = tv2sigma20;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        store_add(v2rho2, ip, m, acc_v2rho2);
        store_add(v2rhosigma, ip, m, acc_v2rhosigma);
        store_add(v2sigma2, ip, m, acc_v2sigma2);
        ip += 8;
    }
}

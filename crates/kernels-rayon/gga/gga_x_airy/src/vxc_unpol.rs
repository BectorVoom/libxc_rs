//! GGA_X_AIRY vxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_airy.c`
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
pub fn gga_x_airy_vxc_unpol(
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
        let v_rho = load(rho, ip, np);
        let v_sigma = load(sigma, ip, np);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_vsigma = V_ZERO;
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
            let t21 = t20 * t20;
            let t22 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t23 = (simd::cbrt(t22));
            let t24 = f64x8::splat(1.0) / t23;
            let t25 = t21 * t24;
            let t26 = ((v_sigma).sqrt());
            let t27 = f64x8::splat(M_CBRT2);
            let t28 = t26 * t27;
            let t30 = f64x8::splat(1.0) / t18 / v_rho;
            let t32 = t25 * t28 * t30;
            let t33 = (simd::pow(t32, f64x8::splat(2.626712)));
            let t35 = f64x8::splat(1.0) + f64x8::splat(0.00013471619689594795) * t33;
            let t36 = (simd::pow(t35, -f64x8::splat(0.657946)));
            let t39 = (simd::pow(t32, f64x8::splat(3.217063)));
            let t41 = (simd::pow(t32, f64x8::splat(3.223476)));
            let t43 = f64x8::splat(1.0) - f64x8::splat(0.04521241301076986) * t39 + f64x8::splat(0.04540222195662038) * t41;
            let t44 = (simd::pow(t32, f64x8::splat(3.473804)));
            let t46 = f64x8::splat(1.0) + f64x8::splat(0.0004770218022490335) * t44;
            let t47 = f64x8::splat(1.0) / t46;
            let t49 = f64x8::splat(6.014601922021111e-05) * t33 * t36 + t43 * t47;
            let t53 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t49));
            let tzk0 = f64x8::splat(2.0) * t53;
            acc_zk = tzk0;
            let t54 = t18 * t18;
            let t56 = t17 / t54;
            let t60 = (simd::pow(t32, f64x8::splat(1.626712)));
            let t62 = t60 * t36 * t21;
            let t63 = t24 * t26;
            let t64 = v_rho * v_rho;
            let t66 = f64x8::splat(1.0) / t18 / t64;
            let t67 = t27 * t66;
            let t68 = t63 * t67;
            let t71 = (simd::pow(t32, f64x8::splat(4.253424)));
            let t72 = (simd::pow(t35, -f64x8::splat(1.657946)));
            let t74 = t71 * t72 * t21;
            let t77 = (simd::pow(t32, f64x8::splat(2.217063)));
            let t79 = t77 * t21 * t24;
            let t80 = t28 * t66;
            let t83 = (simd::pow(t32, f64x8::splat(2.223476)));
            let t85 = t83 * t21 * t24;
            let t88 = f64x8::splat(0.19393490805022173) * t79 * t80 - f64x8::splat(0.19513729709845176) * t85 * t80;
            let t90 = t46 * t46;
            let t91 = f64x8::splat(1.0) / t90;
            let t92 = t43 * t91;
            let t93 = (simd::pow(t32, f64x8::splat(2.473804)));
            let t94 = t93 * t21;
            let t95 = t92 * t94;
            let t98 = -f64x8::splat(0.00021064836058394556) * t62 * t68 + f64x8::splat(1.8671024483029836e-08) * t74 * t68 + t88 * t47 + f64x8::splat(0.0022094403263198687) * t95 * t68;
            let t103 = ((t2).select(f64x8::splat(0.0), -t6 * t56 * t49 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t98));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t103 + f64x8::splat(2.0) * t53;
            acc_vrho = tvrho0;
            let t106 = f64x8::splat(1.0) / t26;
            let t107 = t24 * t106;
            let t108 = t27 * t30;
            let t109 = t107 * t108;
            let t114 = t106 * t27;
            let t115 = t114 * t30;
            let t120 = -f64x8::splat(0.07272559051883315) * t79 * t115 + f64x8::splat(0.07317648641191941) * t85 * t115;
            let t124 = f64x8::splat(7.899313521897959e-05) * t62 * t109 - f64x8::splat(7.001634181136188e-09) * t74 * t109 + t120 * t47 - f64x8::splat(0.0008285401223699508) * t95 * t109;
            let t128 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t124));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t128;
            acc_vsigma = tvsigma0;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        ip += 8;
    }
}

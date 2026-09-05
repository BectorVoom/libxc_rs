//! GGA_X_PBEA kxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_pbea.c`
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
pub fn gga_x_pbea_kxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    v3rho3: &mut [f64],
    v3rho2sigma: &mut [f64],
    v3rhosigma2: &mut [f64],
    v3sigma3: &mut [f64],
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
        let mut acc_v3rho3 = V_ZERO;
        let mut acc_v3rho2sigma = V_ZERO;
        let mut acc_v3rhosigma2 = V_ZERO;
        let mut acc_v3sigma3 = V_ZERO;
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
            let t20 = f64x8::splat(M_CBRT2);
            let t21 = t20 * t20;
            let t23 = v_rho * v_rho;
            let t24 = t18 * t18;
            let t26 = f64x8::splat(1.0) / t24 / t23;
            let t29 = f64x8::splat(1.0) + f64x8::splat(0.008639940809536326) * v_sigma * t21 * t26;
            let t30 = (simd::pow(t29, -f64x8::splat(0.52)));
            let t32 = f64x8::splat(1.804) - f64x8::splat(0.804) * t30;
            let t36 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t17 * t18 * t32));
            let tzk0 = f64x8::splat(2.0) * t36;
            acc_zk = tzk0;
            let t42 = t3 * t17;
            let t43 = t23 * v_rho;
            let t45 = f64x8::splat(1.0) / t18 / t43;
            let t47 = (simd::pow(t29, -f64x8::splat(1.52)));
            let t49 = t47 * v_sigma * t21;
            let t53 = ((t2).select(f64x8::splat(0.0), -t6 * t17 / t24 * t32 / f64x8::splat(8.0) + f64x8::splat(0.00246634334405953) * t42 * t45 * t49));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t53 + f64x8::splat(2.0) * t36;
            acc_vrho = tvrho0;
            let t62 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(0.0009248787540223239) * t42 / t18 / t23 * t47 * t21));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t62;
            acc_vsigma = tvsigma0;
            let t71 = t23 * t23;
            let t73 = f64x8::splat(1.0) / t18 / t71;
            let t77 = t71 * t43;
            let t78 = f64x8::splat(1.0) / t77;
            let t79 = t42 * t78;
            let t80 = (simd::pow(t29, -f64x8::splat(2.52)));
            let t81 = v_sigma * v_sigma;
            let t83 = t80 * t81 * t20;
            let t87 = ((t2).select(f64x8::splat(0.0), t6 * t17 / t24 / v_rho * t32 / f64x8::splat(12.0) - f64x8::splat(0.007399030032178591) * t42 * t73 * t49 + f64x8::splat(0.00017274545052360375) * t79 * t83));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t87 + f64x8::splat(4.0) * t53;
            acc_v2rho2 = tv2rho20;
            let t94 = t71 * t23;
            let t95 = f64x8::splat(1.0) / t94;
            let t98 = t80 * t20 * v_sigma;
            let t102 = ((t2).select(f64x8::splat(0.0), f64x8::splat(0.002158050426052089) * t42 * t45 * t47 * t21 - f64x8::splat(6.47795439463514e-05) * t42 * t95 * t98));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t102 + f64x8::splat(2.0) * t62;
            acc_v2rhosigma = tv2rhosigma0;
            let t105 = t71 * v_rho;
            let t111 = ((t2).select(f64x8::splat(0.0), f64x8::splat(2.429232897988178e-05) * t42 / t105 * t80 * t20));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t111;
            acc_v2sigma2 = tv2sigma20;
            let t119 = f64x8::splat(1.0) / t18 / t105;
            let t123 = t71 * t71;
            let t125 = t42 / t123;
            let t128 = t123 * t23;
            let t131 = (simd::pow(t29, -f64x8::splat(3.52)));
            let t132 = f64x8::splat(1.0) / t24 / t128 * t131;
            let t133 = t81 * v_sigma;
            let t138 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(5.0) / f64x8::splat(36.0) * t6 * t17 * t26 * t32 + f64x8::splat(0.031514387174094) * t42 * t119 * t49 - f64x8::splat(0.0017274545052360377) * t125 * t83 + f64x8::splat(2.0059340685089964e-05) * t42 * t132 * t133));
            let tv3rho30 = f64x8::splat(2.0) * v_rho * t138 + f64x8::splat(6.0) * t87;
            acc_v3rho3 = tv3rho30;
            let t148 = t123 * v_rho;
            let t151 = f64x8::splat(1.0) / t24 / t148 * t131;
            let t156 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(0.00719350142017363) * t42 * t73 * t47 * t21 + f64x8::splat(0.0005398295328862617) * t79 * t98 - f64x8::splat(7.522252756908737e-06) * t42 * t151 * t81));
            let tv3rho2sigma0 = f64x8::splat(2.0) * v_rho * t156 + f64x8::splat(4.0) * t102;
            acc_v3rho2sigma = tv3rho2sigma0;
            let t165 = f64x8::splat(1.0) / t24 / t123 * t131;
            let t170 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(0.0001214616448994089) * t42 * t95 * t80 * t20 + f64x8::splat(2.820844783840776e-06) * t42 * t165 * v_sigma));
            let tv3rhosigma20 = f64x8::splat(2.0) * v_rho * t170 + f64x8::splat(2.0) * t111;
            acc_v3rhosigma2 = tv3rhosigma20;
            let t178 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(1.0578167939402912e-06) * t42 / t24 / t77 * t131));
            let tv3sigma30 = f64x8::splat(2.0) * v_rho * t178;
            acc_v3sigma3 = tv3sigma30;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(vsigma, ip, m, acc_vsigma);
        store_add(v2rho2, ip, m, acc_v2rho2);
        store_add(v2rhosigma, ip, m, acc_v2rhosigma);
        store_add(v2sigma2, ip, m, acc_v2sigma2);
        store_add(v3rho3, ip, m, acc_v3rho3);
        store_add(v3rho2sigma, ip, m, acc_v3rho2sigma);
        store_add(v3rhosigma2, ip, m, acc_v3rhosigma2);
        store_add(v3sigma3, ip, m, acc_v3sigma3);
        ip += 8;
    }
}

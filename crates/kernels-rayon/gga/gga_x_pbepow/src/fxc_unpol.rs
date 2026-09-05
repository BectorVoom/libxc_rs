//! GGA_X_PBEPOW fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_pbepow.c`
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
pub fn gga_x_pbepow_fxc_unpol(
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
            let t26 = t25 * v_sigma;
            let t27 = f64x8::splat(M_CBRT2);
            let t28 = t27 * t27;
            let t29 = v_rho * v_rho;
            let t30 = t18 * t18;
            let t32 = f64x8::splat(1.0) / t30 / t29;
            let t33 = t28 * t32;
            let t34 = v_sigma * t28;
            let t38 = f64x8::splat(0.9146457198521546) * t25 * t34 * t32 + f64x8::splat(0.804);
            let t39 = f64x8::splat(1.0) / t38;
            let t40 = t33 * t39;
            let t41 = t26 * t40;
            let t42 = (simd::pow(t41, f64x8::splat(100.0)));
            let t44 = f64x8::splat(0.0001334414156799501) * t42 - f64x8::splat(1.0);
            let t45 = t33 * t44;
            let t48 = f64x8::splat(1.0) - f64x8::splat(0.009146457198521547) * t26 * t45;
            let t52 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t48));
            let tzk0 = f64x8::splat(2.0) * t52;
            acc_zk = tzk0;
            let t54 = t17 / t30;
            let t58 = t29 * v_rho;
            let t60 = f64x8::splat(1.0) / t30 / t58;
            let t61 = t28 * t60;
            let t62 = t61 * t44;
            let t65 = (simd::pow(t41, f64x8::splat(99.0)));
            let t66 = t61 * t39;
            let t69 = t20 * t20;
            let t72 = t69 / t22 / t21;
            let t73 = v_sigma * v_sigma;
            let t74 = t72 * t73;
            let t75 = t29 * t29;
            let t76 = t75 * t29;
            let t78 = f64x8::splat(1.0) / t18 / t76;
            let t80 = t38 * t38;
            let t81 = f64x8::splat(1.0) / t80;
            let t82 = t27 * t78 * t81;
            let t85 = -f64x8::splat(8.0) / f64x8::splat(3.0) * t26 * t66 + f64x8::splat(4.8781105058781575) * t74 * t82;
            let t86 = t65 * t85;
            let t90 = f64x8::splat(0.024390552529390788) * t26 * t62 - f64x8::splat(0.00012205161970267855) * t26 * t33 * t86;
            let t95 = ((t2).select(f64x8::splat(0.0), -t6 * t54 * t48 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t90));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t95 + f64x8::splat(2.0) * t52;
            acc_vrho = tvrho0;
            let t102 = t75 * v_rho;
            let t106 = t27 / t18 / t102 * t81;
            let t109 = t25 * t40 - f64x8::splat(1.8292914397043092) * t72 * v_sigma * t106;
            let t110 = t65 * t109;
            let t114 = -f64x8::splat(0.009146457198521547) * t25 * t45 - f64x8::splat(0.00012205161970267855) * t26 * t33 * t110;
            let t118 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t114));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t118;
            acc_vsigma = tvsigma0;
            let t123 = t17 / t30 / v_rho;
            let t131 = f64x8::splat(1.0) / t30 / t75;
            let t132 = t28 * t131;
            let t133 = t132 * t44;
            let t139 = (simd::pow(t41, f64x8::splat(98.0)));
            let t140 = t85 * t85;
            let t141 = t139 * t140;
            let t145 = t132 * t39;
            let t148 = t75 * t58;
            let t150 = f64x8::splat(1.0) / t18 / t148;
            let t152 = t27 * t150 * t81;
            let t155 = t73 * v_sigma;
            let t156 = t75 * t75;
            let t157 = t156 * t29;
            let t158 = f64x8::splat(1.0) / t157;
            let t161 = f64x8::splat(1.0) / t80 / t38;
            let t164 = f64x8::splat(88.0) / f64x8::splat(9.0) * t26 * t145 - f64x8::splat(43.90299455290342) * t74 * t152 + f64x8::splat(2.931467096752081) * t155 * t158 * t161;
            let t165 = t65 * t164;
            let t169 = -f64x8::splat(0.08943202594109956) * t26 * t133 + f64x8::splat(0.0006509419717476189) * t26 * t61 * t86 - f64x8::splat(0.012083110350565177) * t26 * t33 * t141 - f64x8::splat(0.00012205161970267855) * t26 * t33 * t165;
            let t174 = ((t2).select(f64x8::splat(0.0), t6 * t123 * t48 / f64x8::splat(12.0) - t6 * t54 * t90 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t169));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t174 + f64x8::splat(4.0) * t95;
            acc_v2rho2 = tv2rho20;
            let t182 = t25 * t28;
            let t183 = t32 * t65;
            let t190 = t25 * t34;
            let t191 = t32 * t139;
            let t192 = t109 * t85;
            let t193 = t191 * t192;
            let t198 = t72 * t27;
            let t203 = t156 * v_rho;
            let t204 = f64x8::splat(1.0) / t203;
            let t208 = -f64x8::splat(8.0) / f64x8::splat(3.0) * t25 * t66 + f64x8::splat(14.634331517634473) * t198 * t78 * t81 * v_sigma - f64x8::splat(1.0993001612820303) * t73 * t204 * t161;
            let t209 = t65 * t208;
            let t213 = f64x8::splat(0.024390552529390788) * t25 * t62 - f64x8::splat(0.00012205161970267855) * t182 * t183 * t85 + f64x8::splat(0.00032547098587380947) * t26 * t61 * t110 - f64x8::splat(0.012083110350565177) * t190 * t193 - f64x8::splat(0.00012205161970267855) * t26 * t33 * t209;
            let t218 = ((t2).select(f64x8::splat(0.0), -t6 * t54 * t114 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t213));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t218 + f64x8::splat(2.0) * t118;
            acc_v2rhosigma = tv2rhosigma0;
            let t224 = t109 * t109;
            let t225 = t139 * t224;
            let t231 = f64x8::splat(1.0) / t156;
            let t235 = -f64x8::splat(3.6585828794086184) * t72 * t106 + f64x8::splat(0.4122375604807614) * v_sigma * t231 * t161;
            let t236 = t65 * t235;
            let t240 = -f64x8::splat(0.0002441032394053571) * t182 * t183 * t109 - f64x8::splat(0.012083110350565177) * t26 * t33 * t225 - f64x8::splat(0.00012205161970267855) * t26 * t33 * t236;
            let t244 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t240));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t244;
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

//! GGA_C_CHACHIYO fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_c_chachiyo.c`
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
pub fn gga_c_chachiyo_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    param_af: f64,
    param_ap: f64,
    param_bf: f64,
    param_bp: f64,
    param_cf: f64,
    param_cp: f64,
    param_h: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_af = f64x8::splat(param_af);
    let param_ap = f64x8::splat(param_ap);
    let param_bf = f64x8::splat(param_bf);
    let param_bp = f64x8::splat(param_bp);
    let param_cf = f64x8::splat(param_cf);
    let param_cp = f64x8::splat(param_cp);
    let param_h = f64x8::splat(param_h);
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
            let t1 = f64x8::splat(M_CBRT3);
            let t2 = t1 * t1;
            let t3 = param_bp * t2;
            let t5 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t7 = f64x8::splat(M_CBRT4);
            let t8 = f64x8::splat(1.0) / t5 * t7;
            let t9 = (simd::cbrt(v_rho));
            let t10 = t8 * t9;
            let t13 = param_cp * t1;
            let t14 = t5 * t5;
            let t16 = t7 * t7;
            let t17 = f64x8::splat(1.0) / t14 * t16;
            let t18 = t9 * t9;
            let t19 = t17 * t18;
            let t22 = f64x8::splat(1.0) + t3 * t10 / f64x8::splat(3.0) + t13 * t19 / f64x8::splat(3.0);
            let t23 = (simd::ln(t22));
            let t24 = param_ap * t23;
            let t25 = param_bf * t2;
            let t28 = param_cf * t1;
            let t31 = f64x8::splat(1.0) + t25 * t10 / f64x8::splat(3.0) + t28 * t19 / f64x8::splat(3.0);
            let t32 = (simd::ln(t31));
            let t36 = (simd::cbrt(zeta_threshold));
            let t37 = t36 * t36;
            let t38 = (((f64x8::splat(1.0)).simd_le(zeta_threshold)).select(t37, f64x8::splat(1.0)));
            let t39 = t38 * t38;
            let t42 = -f64x8::splat(2.0) * t39 * t38 + f64x8::splat(2.0);
            let t44 = t24 + (param_af * t32 - t24) * t42;
            let t45 = f64x8::splat(M_CBRTPI);
            let t46 = t2 * t45;
            let t47 = v_rho * v_rho;
            let t49 = f64x8::splat(1.0) / t9 / t47;
            let t53 = f64x8::splat(1.0) + t46 * t49 * v_sigma / f64x8::splat(48.0);
            let t54 = f64x8::splat(1.0) / t44;
            let t55 = param_h * t54;
            let t56 = (simd::pow(t53, t55));
            let tzk0 = t44 * t56;
            acc_zk = tzk0;
            let t58 = t8 / t18;
            let t62 = t17 / t9;
            let t65 = t3 * t58 / f64x8::splat(9.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t13 * t62;
            let t67 = f64x8::splat(1.0) / t22;
            let t68 = param_ap * t65 * t67;
            let t73 = t25 * t58 / f64x8::splat(9.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t28 * t62;
            let t75 = f64x8::splat(1.0) / t31;
            let t79 = t68 + (param_af * t73 * t75 - t68) * t42;
            let t80 = v_rho * t79;
            let t82 = v_rho * t44;
            let t83 = t44 * t44;
            let t84 = f64x8::splat(1.0) / t83;
            let t85 = param_h * t84;
            let t86 = (simd::ln(t53));
            let t87 = t79 * t86;
            let t89 = t55 * t2;
            let t90 = t47 * v_rho;
            let t92 = f64x8::splat(1.0) / t9 / t90;
            let t93 = t45 * t92;
            let t94 = f64x8::splat(1.0) / t53;
            let t95 = v_sigma * t94;
            let t96 = t93 * t95;
            let t99 = -t85 * t87 - f64x8::splat(7.0) / f64x8::splat(144.0) * t89 * t96;
            let t100 = t56 * t99;
            let tvrho0 = t82 * t100 + t80 * t56 + tzk0;
            acc_vrho = tvrho0;
            let t103 = f64x8::splat(1.0) / t9 / v_rho;
            let t104 = t103 * t56;
            let t106 = t46 * t94;
            let tvsigma0 = t104 * param_h * t106 / f64x8::splat(48.0);
            acc_vsigma = tvsigma0;
            let t108 = t79 * t56;
            let t110 = t44 * t56;
            let t115 = t8 / t18 / v_rho;
            let t117 = t17 * t103;
            let t120 = -f64x8::splat(2.0) / f64x8::splat(27.0) * t3 * t115 - f64x8::splat(2.0) / f64x8::splat(27.0) * t13 * t117;
            let t121 = param_ap * t120;
            let t122 = t121 * t67;
            let t123 = t65 * t65;
            let t125 = t22 * t22;
            let t126 = f64x8::splat(1.0) / t125;
            let t127 = param_ap * t123 * t126;
            let t131 = -f64x8::splat(2.0) / f64x8::splat(27.0) * t25 * t115 - f64x8::splat(2.0) / f64x8::splat(27.0) * t28 * t117;
            let t132 = param_af * t131;
            let t134 = t73 * t73;
            let t136 = t31 * t31;
            let t137 = f64x8::splat(1.0) / t136;
            let t141 = t122 - t127 + (-param_af * t134 * t137 + t132 * t75 - t122 + t127) * t42;
            let t142 = v_rho * t141;
            let t146 = t99 * t99;
            let t147 = t56 * t146;
            let t150 = f64x8::splat(1.0) / t83 / t44;
            let t151 = param_h * t150;
            let t152 = t79 * t79;
            let t153 = t152 * t86;
            let t158 = t79 * t2;
            let t159 = t85 * t158;
            let t162 = t47 * t47;
            let t164 = f64x8::splat(1.0) / t9 / t162;
            let t166 = t45 * t164 * t95;
            let t169 = t55 * t1;
            let t170 = t45 * t45;
            let t171 = t162 * t47;
            let t173 = f64x8::splat(1.0) / t18 / t171;
            let t175 = v_sigma * v_sigma;
            let t176 = t53 * t53;
            let t177 = f64x8::splat(1.0) / t176;
            let t178 = t175 * t177;
            let t179 = t170 * t173 * t178;
            let t182 = f64x8::splat(2.0) * t151 * t153 - t85 * t141 * t86 + f64x8::splat(7.0) / f64x8::splat(72.0) * t159 * t96 + f64x8::splat(35.0) / f64x8::splat(216.0) * t89 * t166 - f64x8::splat(49.0) / f64x8::splat(6912.0) * t169 * t179;
            let t183 = t56 * t182;
            let tv2rho20 = f64x8::splat(2.0) * t80 * t100 + f64x8::splat(2.0) * t110 * t99 + t142 * t56 + t82 * t147 + t82 * t183 + f64x8::splat(2.0) * t108;
            acc_v2rho2 = tv2rho20;
            let t185 = t49 * t56;
            let t192 = param_h * t2 * t45 * t94;
            let t197 = f64x8::splat(1.0) / t18 / t162 * t56;
            let t198 = t197 * param_h;
            let t199 = t1 * t170;
            let t201 = t199 * t177 * v_sigma;
            let tv2rhosigma0 = -t185 * param_h * t106 / f64x8::splat(36.0) + t104 * t99 * t192 / f64x8::splat(48.0) + f64x8::splat(7.0) / f64x8::splat(2304.0) * t198 * t201;
            acc_v2rhosigma = tv2rhosigma0;
            let t205 = f64x8::splat(1.0) / t18 / t90;
            let t206 = t205 * t56;
            let t207 = param_h * param_h;
            let t210 = t170 * t177;
            let t211 = t54 * t1 * t210;
            let t214 = t199 * t177;
            let tv2sigma20 = t206 * t207 * t211 / f64x8::splat(768.0) - t206 * param_h * t214 / f64x8::splat(768.0);
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

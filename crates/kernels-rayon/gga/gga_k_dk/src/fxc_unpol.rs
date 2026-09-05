//! GGA_K_DK fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_k_dk.c`
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
pub fn gga_k_dk_fxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    v2rho2: &mut [f64],
    v2rhosigma: &mut [f64],
    v2sigma2: &mut [f64],
    param_aa_1: f64,
    param_aa_2: f64,
    param_aa_3: f64,
    param_aa_4: f64,
    param_aa_0: f64,
    param_bb_1: f64,
    param_bb_2: f64,
    param_bb_3: f64,
    param_bb_4: f64,
    param_bb_0: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_aa_1 = f64x8::splat(param_aa_1);
    let param_aa_2 = f64x8::splat(param_aa_2);
    let param_aa_3 = f64x8::splat(param_aa_3);
    let param_aa_4 = f64x8::splat(param_aa_4);
    let param_aa_0 = f64x8::splat(param_aa_0);
    let param_bb_1 = f64x8::splat(param_bb_1);
    let param_bb_2 = f64x8::splat(param_bb_2);
    let param_bb_3 = f64x8::splat(param_bb_3);
    let param_bb_4 = f64x8::splat(param_bb_4);
    let param_bb_0 = f64x8::splat(param_bb_0);
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
            let t4 = t3 * t3;
            let t5 = f64x8::splat(M_CBRTPI);
            let t7 = t4 * t5 * f64x8::splat(M_PI);
            let t8 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t9 = zeta_threshold - f64x8::splat(1.0);
            let t11 = ((t8).select(t9, (t8).select(-t9, f64x8::splat(0.0))));
            let t12 = f64x8::splat(1.0) + t11;
            let t14 = (simd::cbrt(zeta_threshold));
            let t15 = t14 * t14;
            let t17 = (simd::cbrt(t12));
            let t18 = t17 * t17;
            let t20 = (((t12).simd_le(zeta_threshold)).select(t15 * zeta_threshold, t18 * t12));
            let t21 = t7 * t20;
            let t22 = (simd::cbrt(v_rho));
            let t23 = t22 * t22;
            let t25 = param_aa_1;
            let t26 = t25 * v_sigma;
            let t27 = f64x8::splat(M_CBRT2);
            let t28 = t27 * t27;
            let t29 = v_rho * v_rho;
            let t31 = f64x8::splat(1.0) / t23 / t29;
            let t32 = t28 * t31;
            let t34 = param_aa_2;
            let t35 = v_sigma * v_sigma;
            let t36 = t34 * t35;
            let t37 = t29 * t29;
            let t38 = t37 * v_rho;
            let t40 = f64x8::splat(1.0) / t22 / t38;
            let t41 = t27 * t40;
            let t44 = param_aa_3;
            let t45 = t35 * v_sigma;
            let t46 = t44 * t45;
            let t47 = t37 * t37;
            let t48 = f64x8::splat(1.0) / t47;
            let t51 = param_aa_4;
            let t52 = t35 * t35;
            let t53 = t51 * t52;
            let t54 = t47 * t29;
            let t57 = t28 / t23 / t54;
            let t60 = t26 * t32 + f64x8::splat(2.0) * t36 * t41 + f64x8::splat(4.0) * t46 * t48 + f64x8::splat(4.0) * t53 * t57 + param_aa_0;
            let t61 = t23 * t60;
            let t63 = param_bb_1;
            let t64 = t63 * v_sigma;
            let t66 = param_bb_2;
            let t67 = t66 * t35;
            let t70 = param_bb_3;
            let t71 = t70 * t45;
            let t74 = param_bb_4;
            let t75 = t74 * t52;
            let t78 = t64 * t32 + f64x8::splat(2.0) * t67 * t41 + f64x8::splat(4.0) * t71 * t48 + f64x8::splat(4.0) * t75 * t57 + param_bb_0;
            let t79 = f64x8::splat(1.0) / t78;
            let t83 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t21 * t61 * t79));
            let tzk0 = f64x8::splat(2.0) * t83;
            acc_zk = tzk0;
            let t84 = f64x8::splat(1.0) / t22;
            let t85 = t84 * t60;
            let t89 = t29 * v_rho;
            let t91 = f64x8::splat(1.0) / t23 / t89;
            let t92 = t28 * t91;
            let t95 = t37 * t29;
            let t97 = f64x8::splat(1.0) / t22 / t95;
            let t98 = t27 * t97;
            let t101 = t47 * v_rho;
            let t102 = f64x8::splat(1.0) / t101;
            let t105 = t47 * t89;
            let t108 = t28 / t23 / t105;
            let t111 = -f64x8::splat(8.0) / f64x8::splat(3.0) * t26 * t92 - f64x8::splat(32.0) / f64x8::splat(3.0) * t36 * t98 - f64x8::splat(32.0) * t46 * t102 - f64x8::splat(128.0) / f64x8::splat(3.0) * t53 * t108;
            let t112 = t23 * t111;
            let t116 = t78 * t78;
            let t117 = f64x8::splat(1.0) / t116;
            let t126 = -f64x8::splat(8.0) / f64x8::splat(3.0) * t64 * t92 - f64x8::splat(32.0) / f64x8::splat(3.0) * t67 * t98 - f64x8::splat(32.0) * t71 * t102 - f64x8::splat(128.0) / f64x8::splat(3.0) * t75 * t108;
            let t127 = t117 * t126;
            let t132 = ((t2).select(f64x8::splat(0.0), t21 * t85 * t79 / f64x8::splat(10.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t21 * t112 * t79 - f64x8::splat(3.0) / f64x8::splat(20.0) * t21 * t61 * t127));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t132 + f64x8::splat(2.0) * t83;
            acc_vrho = tvrho0;
            let t135 = t25 * t28;
            let t137 = t34 * v_sigma;
            let t140 = t44 * t35;
            let t143 = t51 * t45;
            let t146 = t135 * t31 + f64x8::splat(4.0) * t137 * t41 + f64x8::splat(12.0) * t140 * t48 + f64x8::splat(16.0) * t143 * t57;
            let t147 = t23 * t146;
            let t150 = t63 * t28;
            let t152 = t66 * v_sigma;
            let t155 = t70 * t35;
            let t158 = t74 * t45;
            let t161 = t150 * t31 + f64x8::splat(4.0) * t152 * t41 + f64x8::splat(12.0) * t155 * t48 + f64x8::splat(16.0) * t158 * t57;
            let t162 = t117 * t161;
            let t167 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t21 * t147 * t79 - f64x8::splat(3.0) / f64x8::splat(20.0) * t21 * t61 * t162));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t167;
            acc_vsigma = tvsigma0;
            let t171 = f64x8::splat(1.0) / t22 / v_rho;
            let t172 = t171 * t60;
            let t176 = t84 * t111;
            let t184 = f64x8::splat(1.0) / t23 / t37;
            let t185 = t28 * t184;
            let t190 = f64x8::splat(1.0) / t22 / t37 / t89;
            let t191 = t27 * t190;
            let t194 = f64x8::splat(1.0) / t54;
            let t197 = t47 * t37;
            let t200 = t28 / t23 / t197;
            let t203 = f64x8::splat(88.0) / f64x8::splat(9.0) * t26 * t185 + f64x8::splat(608.0) / f64x8::splat(9.0) * t36 * t191 + f64x8::splat(288.0) * t46 * t194 + f64x8::splat(4480.0) / f64x8::splat(9.0) * t53 * t200;
            let t204 = t23 * t203;
            let t212 = f64x8::splat(1.0) / t116 / t78;
            let t213 = t126 * t126;
            let t214 = t212 * t213;
            let t226 = f64x8::splat(88.0) / f64x8::splat(9.0) * t64 * t185 + f64x8::splat(608.0) / f64x8::splat(9.0) * t67 * t191 + f64x8::splat(288.0) * t71 * t194 + f64x8::splat(4480.0) / f64x8::splat(9.0) * t75 * t200;
            let t227 = t117 * t226;
            let t232 = ((t2).select(f64x8::splat(0.0), -t21 * t172 * t79 / f64x8::splat(30.0) + t21 * t176 * t79 / f64x8::splat(5.0) - t21 * t85 * t127 / f64x8::splat(5.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t21 * t204 * t79 - f64x8::splat(3.0) / f64x8::splat(10.0) * t21 * t112 * t127 + f64x8::splat(3.0) / f64x8::splat(10.0) * t21 * t61 * t214 - f64x8::splat(3.0) / f64x8::splat(20.0) * t21 * t61 * t227));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t232 + f64x8::splat(4.0) * t132;
            acc_v2rho2 = tv2rho20;
            let t235 = t84 * t146;
            let t247 = -f64x8::splat(8.0) / f64x8::splat(3.0) * t135 * t91 - f64x8::splat(64.0) / f64x8::splat(3.0) * t137 * t98 - f64x8::splat(96.0) * t140 * t102 - f64x8::splat(512.0) / f64x8::splat(3.0) * t143 * t108;
            let t248 = t23 * t247;
            let t262 = t7 * t20 * t23;
            let t263 = t60 * t212;
            let t264 = t161 * t126;
            let t265 = t263 * t264;
            let t276 = -f64x8::splat(8.0) / f64x8::splat(3.0) * t150 * t91 - f64x8::splat(64.0) / f64x8::splat(3.0) * t152 * t98 - f64x8::splat(96.0) * t155 * t102 - f64x8::splat(512.0) / f64x8::splat(3.0) * t158 * t108;
            let t277 = t117 * t276;
            let t282 = ((t2).select(f64x8::splat(0.0), t21 * t235 * t79 / f64x8::splat(10.0) + f64x8::splat(3.0) / f64x8::splat(20.0) * t21 * t248 * t79 - f64x8::splat(3.0) / f64x8::splat(20.0) * t21 * t147 * t127 - t21 * t85 * t162 / f64x8::splat(10.0) - f64x8::splat(3.0) / f64x8::splat(20.0) * t21 * t112 * t162 + f64x8::splat(3.0) / f64x8::splat(10.0) * t262 * t265 - f64x8::splat(3.0) / f64x8::splat(20.0) * t21 * t61 * t277));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t282 + f64x8::splat(2.0) * t167;
            acc_v2rhosigma = tv2rhosigma0;
            let t285 = t34 * t27;
            let t288 = t44 * v_sigma;
            let t291 = t51 * t35;
            let t294 = f64x8::splat(4.0) * t285 * t40 + f64x8::splat(24.0) * t288 * t48 + f64x8::splat(48.0) * t291 * t57;
            let t295 = t23 * t294;
            let t302 = t161 * t161;
            let t303 = t212 * t302;
            let t307 = t66 * t27;
            let t310 = t70 * v_sigma;
            let t313 = t74 * t35;
            let t316 = f64x8::splat(4.0) * t307 * t40 + f64x8::splat(24.0) * t310 * t48 + f64x8::splat(48.0) * t313 * t57;
            let t317 = t117 * t316;
            let t322 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.0) / f64x8::splat(20.0) * t21 * t295 * t79 - f64x8::splat(3.0) / f64x8::splat(10.0) * t21 * t147 * t162 + f64x8::splat(3.0) / f64x8::splat(10.0) * t21 * t61 * t303 - f64x8::splat(3.0) / f64x8::splat(20.0) * t21 * t61 * t317));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t322;
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

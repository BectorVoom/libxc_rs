//! LDA_C_2D_AMGB lxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_2d_amgb.c`
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
pub fn lda_c_2d_amgb_lxc_unpol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    v3rho3: &mut [f64],
    v4rho4: &mut [f64],
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
        let mut acc_zk = V_ZERO;
        let mut acc_vrho = V_ZERO;
        let mut acc_v2rho2 = V_ZERO;
        let mut acc_v3rho3 = V_ZERO;
        let mut acc_v4rho4 = V_ZERO;
        {
            let t1 = ((v_rho).sqrt());
            let t2 = f64x8::splat(1.0) / t1;
            let t4 = f64x8::splat(1.0) / v_rho;
            let t7 = f64x8::splat(1.0) / t1 / v_rho;
            let t9 = f64x8::splat(0.04869723403850762) * t2 + f64x8::splat(0.018219548589342285) * t4 + f64x8::splat(0.000603947002028882) * t7;
            let t11 = ((f64x8::splat(M_PI)).sqrt());
            let t12 = f64x8::splat(1.0) / t11;
            let t13 = t12 * t2;
            let t14 = ((t13) * (t13).sqrt());
            let t18 = f64x8::splat(0.5654308006315614) * t2 - f64x8::splat(0.02069) * t14 + f64x8::splat(0.10821581200590331) * t4 + f64x8::splat(0.00313738702352666) * t7;
            let t20 = f64x8::splat(1.0) + f64x8::splat(1.0) / t18;
            let t21 = (simd::ln(t20));
            let t22 = t9 * t21;
            let t24 = (simd::exp(-f64x8::splat(0.7552241765370266) * t2));
            let t26 = f64x8::splat(M_SQRT2);
            let t27 = (t24 - f64x8::splat(1.0)) * t26;
            let t30 = ((zeta_threshold).sqrt());
            let t32 = (((f64x8::splat(1.0)).simd_le(zeta_threshold)).select(t30 * zeta_threshold, f64x8::splat(1.0)));
            let t33 = t32 - f64x8::splat(1.0);
            let t36 = f64x8::splat(4.0) / f64x8::splat(3.0) * t27 * t12 * t1 * t33;
            let tzk0 = -f64x8::splat(0.1925) + t22 - t36;
            acc_zk = tzk0;
            let t38 = v_rho * v_rho;
            let t39 = f64x8::splat(1.0) / t38;
            let t42 = f64x8::splat(1.0) / t1 / t38;
            let t44 = -f64x8::splat(0.02434861701925381) * t7 - f64x8::splat(0.018219548589342285) * t39 - f64x8::splat(0.000905920503043323) * t42;
            let t45 = t44 * t21;
            let t46 = t18 * t18;
            let t47 = f64x8::splat(1.0) / t46;
            let t48 = t9 * t47;
            let t50 = ((t13).sqrt());
            let t51 = t50 * t12;
            let t56 = -f64x8::splat(0.2827154003157807) * t7 + f64x8::splat(0.0155175) * t51 * t7 - f64x8::splat(0.10821581200590331) * t39 - f64x8::splat(0.00470608053528999) * t42;
            let t57 = f64x8::splat(1.0) / t20;
            let t58 = t56 * t57;
            let t59 = t48 * t58;
            let t61 = t26 * t33;
            let t62 = t4 * t24 * t61;
            let t65 = t27 * t13 * t33;
            let tvrho0 = -f64x8::splat(0.1925) + t22 - t36 + v_rho * (t45 - t59 - f64x8::splat(0.2840597424304148) * t62 - f64x8::splat(2.0) / f64x8::splat(3.0) * t65);
            acc_vrho = tvrho0;
            let t74 = t38 * v_rho;
            let t75 = f64x8::splat(1.0) / t74;
            let t78 = f64x8::splat(1.0) / t1 / t74;
            let t80 = f64x8::splat(0.036522925528880715) * t42 + f64x8::splat(0.03643909717868457) * t75 + f64x8::splat(0.0022648012576083074) * t78;
            let t81 = t80 * t21;
            let t82 = t44 * t47;
            let t83 = t82 * t58;
            let t85 = t46 * t18;
            let t86 = f64x8::splat(1.0) / t85;
            let t87 = t9 * t86;
            let t88 = t56 * t56;
            let t89 = t88 * t57;
            let t90 = t87 * t89;
            let t93 = f64x8::splat(1.0)/((t13).sqrt());
            let t95 = t93 / f64x8::splat(M_PI);
            let t102 = f64x8::splat(0.424073100473671) * t42 - f64x8::splat(0.003879375) * t95 * t75 - f64x8::splat(0.02327625) * t51 * t42 + f64x8::splat(0.21643162401180663) * t75 + f64x8::splat(0.011765201338224974) * t78;
            let t103 = t102 * t57;
            let t104 = t48 * t103;
            let t105 = t46 * t46;
            let t106 = f64x8::splat(1.0) / t105;
            let t107 = t9 * t106;
            let t108 = t20 * t20;
            let t109 = f64x8::splat(1.0) / t108;
            let t110 = t88 * t109;
            let t111 = t107 * t110;
            let t113 = t39 * t24 * t61;
            let t116 = t42 * t24 * t61;
            let t120 = t27 * t12 * t7 * t33;
            let tv2rho20 = f64x8::splat(2.0) * t45 - f64x8::splat(2.0) * t59 - f64x8::splat(0.5681194848608296) * t62 - f64x8::splat(4.0) / f64x8::splat(3.0) * t65 + v_rho * (t81 - f64x8::splat(2.0) * t83 + f64x8::splat(2.0) * t90 - t104 - t111 + f64x8::splat(0.1420298712152074) * t113 - f64x8::splat(0.10726439253216494) * t116 + t120 / f64x8::splat(3.0));
            acc_v2rho2 = tv2rho20;
            let t132 = t38 * t38;
            let t133 = f64x8::splat(1.0) / t132;
            let t136 = f64x8::splat(1.0) / t1 / t132;
            let t138 = -f64x8::splat(0.09130731382220178) * t78 - f64x8::splat(0.1093172915360537) * t133 - f64x8::splat(0.007926804401629076) * t136;
            let t139 = t138 * t21;
            let t140 = t80 * t47;
            let t141 = t140 * t58;
            let t143 = t44 * t86;
            let t144 = t143 * t89;
            let t146 = t82 * t103;
            let t148 = t44 * t106;
            let t149 = t148 * t110;
            let t151 = t88 * t56;
            let t152 = t151 * t57;
            let t153 = t107 * t152;
            let t155 = t58 * t102;
            let t156 = t87 * t155;
            let t159 = f64x8::splat(1.0) / t105 / t18;
            let t160 = t9 * t159;
            let t161 = t151 * t109;
            let t162 = t160 * t161;
            let t165 = f64x8::splat(1.0)/((t13) * (t13).sqrt());
            let t168 = t165 / t11 / f64x8::splat(M_PI);
            let t177 = -f64x8::splat(1.0601827511841775) * t78 - f64x8::splat(0.00096984375) * t168 * t136 + f64x8::splat(0.0174571875) * t95 * t133 + f64x8::splat(0.058190625) * t51 * t78 - f64x8::splat(0.6492948720354199) * t133 - f64x8::splat(0.04117820468378741) * t136;
            let t178 = t177 * t57;
            let t179 = t48 * t178;
            let t181 = t102 * t109 * t56;
            let t182 = t107 * t181;
            let t185 = f64x8::splat(1.0) / t105 / t46;
            let t186 = t9 * t185;
            let t188 = f64x8::splat(1.0) / t108 / t20;
            let t189 = t151 * t188;
            let t190 = t186 * t189;
            let t193 = t75 * t24 * t61;
            let t196 = t78 * t24 * t61;
            let t199 = t133 * t24 * t61;
            let t203 = t27 * t12 * t42 * t33;
            let t205 = t139 - f64x8::splat(3.0) * t141 + f64x8::splat(6.0) * t144 - f64x8::splat(3.0) * t146 - f64x8::splat(3.0) * t149 - f64x8::splat(6.0) * t153 + f64x8::splat(6.0) * t156 + f64x8::splat(6.0) * t162 - t179 - f64x8::splat(3.0) * t182 - f64x8::splat(2.0) * t190 - f64x8::splat(0.2130448068228111) * t193 + f64x8::splat(0.32179317759649484) * t196 - f64x8::splat(0.04050433126092432) * t199 - t203 / f64x8::splat(2.0);
            let tv3rho30 = f64x8::splat(3.0) * t81 - f64x8::splat(6.0) * t83 + f64x8::splat(6.0) * t90 - f64x8::splat(3.0) * t104 - f64x8::splat(3.0) * t111 + f64x8::splat(0.4260896136456222) * t113 - f64x8::splat(0.32179317759649484) * t116 + t120 + v_rho * t205;
            acc_v3rho3 = tv3rho30;
            let t253 = t88 * t88;
            let t257 = t102 * t102;
            let t264 = f64x8::splat(24.0) * t143 * t155 - f64x8::splat(12.0) * t148 * t181 - f64x8::splat(36.0) * t107 * t89 * t102 + f64x8::splat(8.0) * t87 * t58 * t177 + f64x8::splat(36.0) * t160 * t110 * t102 - f64x8::splat(4.0) * t107 * t177 * t109 * t56 - f64x8::splat(12.0) * t186 * t102 * t188 * t88 + f64x8::splat(5.0) / f64x8::splat(4.0) * t27 * t12 * t78 * t33 + f64x8::splat(0.5326120170570278) * t199 + f64x8::splat(12.0) * t80 * t86 * t89 - f64x8::splat(24.0) * t148 * t152 + f64x8::splat(24.0) * t160 * t253 * t57 + f64x8::splat(6.0) * t87 * t257 * t57 - f64x8::splat(3.0) * t107 * t257 * t109;
            let t291 = (simd::pow(t13, -f64x8::splat(2.5)));
            let t292 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t299 = t132 * v_rho;
            let t301 = f64x8::splat(1.0) / t1 / t299;
            let t304 = f64x8::splat(1.0) / t299;
            let t314 = t105 * t105;
            let t317 = t108 * t108;
            let t336 = -f64x8::splat(4.0) * t138 * t47 * t58 - f64x8::splat(6.0) * t140 * t103 - f64x8::splat(6.0) * t80 * t106 * t110 + f64x8::splat(24.0) * t44 * t159 * t161 - f64x8::splat(4.0) * t82 * t178 - f64x8::splat(8.0) * t44 * t185 * t189 - f64x8::splat(36.0) * t186 * t253 * t109 + f64x8::splat(24.0) * t9 / t105 / t85 * t253 * t188 - t48 * (f64x8::splat(3.7106396291446213) * t136 - f64x8::splat(0.0007273828125) * t291 / t292 / t132 / t38 + f64x8::splat(0.00872859375) * t168 * t301 - f64x8::splat(0.08437640625) * t95 * t304 - f64x8::splat(0.2036671875) * t51 * t136 + f64x8::splat(2.5971794881416796) * t304 + f64x8::splat(0.18530192107704335) * t301) * t57 - f64x8::splat(6.0) * t9 / t314 * t253 / t317 - f64x8::splat(1.2067244159868555) * t136 * t24 * t61 + f64x8::splat(0.2835303188264703) * t304 * t24 * t61 - f64x8::splat(0.015294925111357258) * t301 * t24 * t61 + (f64x8::splat(0.3195755983777062) * t136 + f64x8::splat(0.4372691661442148) * t304 + f64x8::splat(0.03567061980733084) * t301) * t21;
            let tv4rho40 = f64x8::splat(24.0) * t144 - f64x8::splat(24.0) * t153 + f64x8::splat(24.0) * t156 - f64x8::splat(12.0) * t182 - f64x8::splat(0.8521792272912444) * t193 - f64x8::splat(12.0) * t141 - f64x8::splat(12.0) * t146 - f64x8::splat(12.0) * t149 + f64x8::splat(24.0) * t162 - f64x8::splat(4.0) * t179 - f64x8::splat(8.0) * t190 + f64x8::splat(1.2871727103859794) * t196 - f64x8::splat(0.16201732504369729) * t199 + f64x8::splat(4.0) * t139 - f64x8::splat(2.0) * t203 + v_rho * (t264 + t336);
            acc_v4rho4 = tv4rho40;
        }
        store_add(zk, ip, m, acc_zk);
        store_add(vrho, ip, m, acc_vrho);
        store_add(v2rho2, ip, m, acc_v2rho2);
        store_add(v3rho3, ip, m, acc_v3rho3);
        store_add(v4rho4, ip, m, acc_v4rho4);
        ip += 8;
    }
}

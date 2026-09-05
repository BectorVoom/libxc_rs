//! LDA_K_ZLP kxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_k_zlp.c`
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

/// Load 8 elements with a given stride and offset.
#[inline(always)]
fn load_strided(s: &[f64], ip: usize, np: usize, stride: usize, offset: usize) -> f64x8 {
    let mut b = [0.0f64; 8];
    if ip + 8 <= np {
        let base = ip * stride + offset;
        b[0] = s[base];
        b[1] = s[base + stride];
        b[2] = s[base + 2 * stride];
        b[3] = s[base + 3 * stride];
        b[4] = s[base + 4 * stride];
        b[5] = s[base + 5 * stride];
        b[6] = s[base + 6 * stride];
        b[7] = s[base + 7 * stride];
    } else {
        for k in 0..8 {
            let p = (ip + k).min(np - 1);
            b[k] = s[p * stride + offset];
        }
    }
    f64x8::new(b)
}

/// Store 8 elements with a given stride and offset.
#[inline(always)]
fn store_strided(s: &mut [f64], ip: usize, m: usize, stride: usize, offset: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let base = ip * stride + offset;
        s[base] = a[0];
        s[base + stride] = a[1];
        s[base + 2 * stride] = a[2];
        s[base + 3 * stride] = a[3];
        s[base + 4 * stride] = a[4];
        s[base + 5 * stride] = a[5];
        s[base + 6 * stride] = a[6];
        s[base + 7 * stride] = a[7];
    } else {
        for k in 0..m {
            s[(ip + k) * stride + offset] = a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn lda_k_zlp_kxc_pol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    v3rho3: &mut [f64],
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho0 = load_strided(rho, ip, np, 2, 0);
        let v_rho1 = load_strided(rho, ip, np, 2, 1);
        let mut acc_zk = V_ZERO;
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        let mut acc_v2rho2_0 = V_ZERO;
        let mut acc_v2rho2_1 = V_ZERO;
        let mut acc_v2rho2_2 = V_ZERO;
        let mut acc_v3rho3_0 = V_ZERO;
        let mut acc_v3rho3_1 = V_ZERO;
        let mut acc_v3rho3_2 = V_ZERO;
        let mut acc_v3rho3_3 = V_ZERO;
        {
            let t1 = f64x8::splat(M_CBRT3);
            let t2 = t1 * t1;
            let t4 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t5 = f64x8::splat(1.0) / t4;
            let t7 = f64x8::splat(M_CBRT4);
            let t8 = t2 * t5 * t7;
            let t9 = v_rho0 - v_rho1;
            let t10 = v_rho0 + v_rho1;
            let t11 = f64x8::splat(1.0) / t10;
            let t12 = t9 * t11;
            let t13 = f64x8::splat(1.0) + t12;
            let t14 = (t13).simd_le(zeta_threshold);
            let t15 = (simd::cbrt(zeta_threshold));
            let t16 = t15 * t15;
            let t17 = t16 * zeta_threshold;
            let t18 = (simd::cbrt(t13));
            let t19 = t18 * t18;
            let t21 = ((t14).select(t17, t19 * t13));
            let t22 = f64x8::splat(1.0) - t12;
            let t23 = (t22).simd_le(zeta_threshold);
            let t24 = (simd::cbrt(t22));
            let t25 = t24 * t24;
            let t27 = ((t23).select(t17, t25 * t22));
            let t29 = t21 / f64x8::splat(2.0) + t27 / f64x8::splat(2.0);
            let t30 = (simd::cbrt(t10));
            let t31 = t30 * t30;
            let t32 = t29 * t31;
            let t33 = f64x8::splat(1.0) / t30;
            let t35 = f64x8::splat(1.0) + f64x8::splat(510.2040816326531) * t33;
            let t36 = (simd::ln(t35));
            let t39 = f64x8::splat(1.0) - f64x8::splat(0.00196) * t30 * t36;
            let t41 = t8 * t32 * t39;
            let tzk0 = f64x8::splat(1.0790666666666666) * t41;
            acc_zk = tzk0;
            let t42 = f64x8::splat(1.7984444444444445) * t41;
            let t43 = t31 * t10;
            let t45 = t43 * t2 * t5;
            let t46 = t10 * t10;
            let t47 = f64x8::splat(1.0) / t46;
            let t48 = t9 * t47;
            let t49 = t11 - t48;
            let t52 = ((t14).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t19 * t49));
            let t53 = -t49;
            let t56 = ((t23).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t25 * t53));
            let t58 = t52 / f64x8::splat(2.0) + t56 / f64x8::splat(2.0);
            let t59 = t7 * t58;
            let t63 = t7 * t29;
            let t67 = f64x8::splat(1.0) / t35;
            let t70 = -f64x8::splat(0.0006533333333333333) / t31 * t36 + f64x8::splat(0.3333333333333333) * t11 * t67;
            let t73 = f64x8::splat(1.0790666666666666) * t45 * t63 * t70;
            let tvrho0 = t42 + f64x8::splat(1.0790666666666666) * t45 * t59 * t39 + t73;
            acc_vrho_0 = tvrho0;
            let t74 = -t11 - t48;
            let t77 = ((t14).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t19 * t74));
            let t78 = -t74;
            let t81 = ((t23).select(f64x8::splat(0.0), f64x8::splat(5.0) / f64x8::splat(3.0) * t25 * t78));
            let t84 = t7 * (t77 / f64x8::splat(2.0) + t81 / f64x8::splat(2.0));
            let t85 = t84 * t39;
            let tvrho1 = t42 + f64x8::splat(1.0790666666666666) * t45 * t85 + t73;
            acc_vrho_1 = tvrho1;
            let t88 = t58 * t31;
            let t90 = t8 * t88 * t39;
            let t92 = t29 * t33;
            let t95 = f64x8::splat(1.198962962962963) * t8 * t92 * t39;
            let t98 = f64x8::splat(3.596888888888889) * t8 * t32 * t70;
            let t99 = f64x8::splat(1.0) / t18;
            let t100 = t49 * t49;
            let t103 = t46 * t10;
            let t104 = f64x8::splat(1.0) / t103;
            let t105 = t9 * t104;
            let t107 = -f64x8::splat(2.0) * t47 + f64x8::splat(2.0) * t105;
            let t111 = ((t14).select(f64x8::splat(0.0), f64x8::splat(10.0) / f64x8::splat(9.0) * t99 * t100 + f64x8::splat(5.0) / f64x8::splat(3.0) * t19 * t107));
            let t112 = f64x8::splat(1.0) / t24;
            let t113 = t53 * t53;
            let t116 = -t107;
            let t120 = ((t23).select(f64x8::splat(0.0), f64x8::splat(10.0) / f64x8::splat(9.0) * t112 * t113 + f64x8::splat(5.0) / f64x8::splat(3.0) * t25 * t116));
            let t122 = t111 / f64x8::splat(2.0) + t120 / f64x8::splat(2.0);
            let t123 = t7 * t122;
            let t128 = t45 * t59 * t70;
            let t136 = f64x8::splat(1.0) / t30 / t46;
            let t137 = t35 * t35;
            let t138 = f64x8::splat(1.0) / t137;
            let t141 = f64x8::splat(0.00043555555555555557) / t43 * t36 - f64x8::splat(0.2222222222222222) * t47 * t67 + f64x8::splat(56.68934240362812) * t136 * t138;
            let t144 = f64x8::splat(1.0790666666666666) * t45 * t63 * t141;
            let tv2rho20 = f64x8::splat(3.596888888888889) * t90 + t95 + t98 + f64x8::splat(1.0790666666666666) * t45 * t123 * t39 + f64x8::splat(2.1581333333333332) * t128 + t144;
            acc_v2rho2_0 = tv2rho20;
            let t147 = t31 * t2 * t5;
            let t148 = t147 * t85;
            let t150 = t99 * t74;
            let t153 = t19 * t9;
            let t157 = ((t14).select(f64x8::splat(0.0), f64x8::splat(10.0) / f64x8::splat(9.0) * t150 * t49 + f64x8::splat(10.0) / f64x8::splat(3.0) * t153 * t104));
            let t158 = t112 * t78;
            let t161 = t25 * t9;
            let t165 = ((t23).select(f64x8::splat(0.0), f64x8::splat(10.0) / f64x8::splat(9.0) * t158 * t53 - f64x8::splat(10.0) / f64x8::splat(3.0) * t161 * t104));
            let t168 = t7 * (t157 / f64x8::splat(2.0) + t165 / f64x8::splat(2.0));
            let t169 = t168 * t39;
            let t172 = t84 * t70;
            let t173 = t45 * t172;
            let tv2rho21 = f64x8::splat(1.7984444444444445) * t90 + t95 + t98 + f64x8::splat(1.7984444444444445) * t148 + f64x8::splat(1.0790666666666666) * t45 * t169 + f64x8::splat(1.0790666666666666) * t173 + f64x8::splat(1.0790666666666666) * t128 + t144;
            acc_v2rho2_1 = tv2rho21;
            let t177 = t74 * t74;
            let t181 = f64x8::splat(2.0) * t47 + f64x8::splat(2.0) * t105;
            let t185 = ((t14).select(f64x8::splat(0.0), f64x8::splat(10.0) / f64x8::splat(9.0) * t99 * t177 + f64x8::splat(5.0) / f64x8::splat(3.0) * t19 * t181));
            let t186 = t78 * t78;
            let t189 = -t181;
            let t193 = ((t23).select(f64x8::splat(0.0), f64x8::splat(10.0) / f64x8::splat(9.0) * t112 * t186 + f64x8::splat(5.0) / f64x8::splat(3.0) * t25 * t189));
            let t196 = t7 * (t185 / f64x8::splat(2.0) + t193 / f64x8::splat(2.0));
            let t197 = t196 * t39;
            let tv2rho22 = f64x8::splat(3.596888888888889) * t148 + t95 + t98 + f64x8::splat(1.0790666666666666) * t45 * t197 + f64x8::splat(2.1581333333333332) * t173 + t144;
            acc_v2rho2_2 = tv2rho22;
            let t201 = t122 * t31;
            let t203 = t8 * t201 * t39;
            let t205 = t58 * t33;
            let t207 = t8 * t205 * t39;
            let t210 = t8 * t88 * t70;
            let t213 = f64x8::splat(1.0) / t30 / t10;
            let t214 = t29 * t213;
            let t217 = f64x8::splat(0.3996543209876543) * t8 * t214 * t39;
            let t220 = f64x8::splat(3.596888888888889) * t8 * t92 * t70;
            let t223 = f64x8::splat(5.395333333333333) * t8 * t32 * t141;
            let t225 = f64x8::splat(1.0) / t18 / t13;
            let t226 = t100 * t49;
            let t229 = t99 * t49;
            let t232 = t46 * t46;
            let t233 = f64x8::splat(1.0) / t232;
            let t234 = t9 * t233;
            let t236 = f64x8::splat(6.0) * t104 - f64x8::splat(6.0) * t234;
            let t240 = ((t14).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t225 * t226 + f64x8::splat(10.0) / f64x8::splat(3.0) * t229 * t107 + f64x8::splat(5.0) / f64x8::splat(3.0) * t19 * t236));
            let t242 = f64x8::splat(1.0) / t24 / t22;
            let t243 = t113 * t53;
            let t246 = t112 * t53;
            let t249 = -t236;
            let t253 = ((t23).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t242 * t243 + f64x8::splat(10.0) / f64x8::splat(3.0) * t246 * t116 + f64x8::splat(5.0) / f64x8::splat(3.0) * t25 * t249));
            let t255 = t240 / f64x8::splat(2.0) + t253 / f64x8::splat(2.0);
            let t256 = t7 * t255;
            let t261 = t45 * t123 * t70;
            let t264 = t45 * t59 * t141;
            let t277 = f64x8::splat(1.0) / t31 / t103;
            let t279 = f64x8::splat(1.0) / t137 / t35;
            let t282 = -f64x8::splat(0.000725925925925926) / t31 / t46 * t36 + f64x8::splat(0.37037037037037035) * t104 * t67 - f64x8::splat(170.06802721088437) / t30 / t103 * t138 + f64x8::splat(19282.089252934733) * t277 * t279;
            let t285 = f64x8::splat(1.0790666666666666) * t45 * t63 * t282;
            let tv3rho30 = f64x8::splat(5.395333333333333) * t203 + f64x8::splat(3.596888888888889) * t207 + f64x8::splat(10.790666666666667) * t210 - t217 + t220 + t223 + f64x8::splat(1.0790666666666666) * t45 * t256 * t39 + f64x8::splat(3.2372) * t261 + f64x8::splat(3.2372) * t264 + t285;
            acc_v3rho3_0 = tv3rho30;
            let t290 = t33 * t2 * t5;
            let t291 = t290 * t85;
            let t294 = f64x8::splat(3.596888888888889) * t147 * t169;
            let t295 = t147 * t172;
            let t297 = t225 * t74;
            let t300 = t99 * t9;
            let t311 = ((t14).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t297 * t100 + f64x8::splat(40.0) / f64x8::splat(9.0) * t300 * t104 * t49 + f64x8::splat(10.0) / f64x8::splat(9.0) * t150 * t107 + f64x8::splat(10.0) / f64x8::splat(3.0) * t19 * t104 - f64x8::splat(10.0) * t153 * t233));
            let t312 = t242 * t78;
            let t315 = t112 * t9;
            let t326 = ((t23).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t312 * t113 - f64x8::splat(40.0) / f64x8::splat(9.0) * t315 * t104 * t53 + f64x8::splat(10.0) / f64x8::splat(9.0) * t158 * t116 - f64x8::splat(10.0) / f64x8::splat(3.0) * t25 * t104 + f64x8::splat(10.0) * t161 * t233));
            let t329 = t7 * (t311 / f64x8::splat(2.0) + t326 / f64x8::splat(2.0));
            let t330 = t329 * t39;
            let t333 = t168 * t70;
            let t335 = f64x8::splat(2.1581333333333332) * t45 * t333;
            let t336 = t84 * t141;
            let t337 = t45 * t336;
            let tv3rho31 = f64x8::splat(1.7984444444444445) * t203 + f64x8::splat(2.397925925925926) * t207 + f64x8::splat(7.193777777777778) * t210 - t217 + t220 + t223 + f64x8::splat(1.198962962962963) * t291 + t294 + f64x8::splat(3.596888888888889) * t295 + f64x8::splat(1.0790666666666666) * t45 * t330 + t335 + f64x8::splat(1.0790666666666666) * t337 + f64x8::splat(1.0790666666666666) * t261 + f64x8::splat(2.1581333333333332) * t264 + t285;
            acc_v3rho3_1 = tv3rho31;
            let t345 = t147 * t197;
            let t347 = t225 * t177;
            let t352 = t99 * t181;
            let t357 = -f64x8::splat(2.0) * t104 - f64x8::splat(6.0) * t234;
            let t361 = ((t14).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t347 * t49 + f64x8::splat(40.0) / f64x8::splat(9.0) * t150 * t105 + f64x8::splat(10.0) / f64x8::splat(9.0) * t352 * t49 + f64x8::splat(5.0) / f64x8::splat(3.0) * t19 * t357));
            let t362 = t242 * t186;
            let t367 = t112 * t189;
            let t370 = -t357;
            let t374 = ((t23).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t362 * t53 - f64x8::splat(40.0) / f64x8::splat(9.0) * t158 * t105 + f64x8::splat(10.0) / f64x8::splat(9.0) * t367 * t53 + f64x8::splat(5.0) / f64x8::splat(3.0) * t25 * t370));
            let t377 = t7 * (t361 / f64x8::splat(2.0) + t374 / f64x8::splat(2.0));
            let t378 = t377 * t39;
            let t381 = t196 * t70;
            let t382 = t45 * t381;
            let tv3rho32 = f64x8::splat(2.397925925925926) * t291 + t294 + f64x8::splat(7.193777777777778) * t295 + f64x8::splat(1.198962962962963) * t207 - t217 + t220 + f64x8::splat(3.596888888888889) * t210 + t223 + f64x8::splat(1.7984444444444445) * t345 + f64x8::splat(1.0790666666666666) * t45 * t378 + f64x8::splat(1.0790666666666666) * t382 + t335 + f64x8::splat(2.1581333333333332) * t337 + f64x8::splat(1.0790666666666666) * t264 + t285;
            acc_v3rho3_2 = tv3rho32;
            let t389 = t177 * t74;
            let t395 = -f64x8::splat(6.0) * t104 - f64x8::splat(6.0) * t234;
            let t399 = ((t14).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t225 * t389 + f64x8::splat(10.0) / f64x8::splat(3.0) * t150 * t181 + f64x8::splat(5.0) / f64x8::splat(3.0) * t19 * t395));
            let t400 = t186 * t78;
            let t405 = -t395;
            let t409 = ((t23).select(f64x8::splat(0.0), -f64x8::splat(10.0) / f64x8::splat(27.0) * t242 * t400 + f64x8::splat(10.0) / f64x8::splat(3.0) * t158 * t189 + f64x8::splat(5.0) / f64x8::splat(3.0) * t25 * t405));
            let t412 = t7 * (t399 / f64x8::splat(2.0) + t409 / f64x8::splat(2.0));
            let t413 = t412 * t39;
            let tv3rho33 = f64x8::splat(3.596888888888889) * t291 + f64x8::splat(5.395333333333333) * t345 + f64x8::splat(10.790666666666667) * t295 - t217 + t220 + t223 + f64x8::splat(1.0790666666666666) * t45 * t413 + f64x8::splat(3.2372) * t382 + f64x8::splat(3.2372) * t337 + t285;
            acc_v3rho3_3 = tv3rho33;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(v2rho2, ip, m, 3, 0, acc_v2rho2_0);
        store_strided(v2rho2, ip, m, 3, 1, acc_v2rho2_1);
        store_strided(v2rho2, ip, m, 3, 2, acc_v2rho2_2);
        store_strided(v3rho3, ip, m, 4, 0, acc_v3rho3_0);
        store_strided(v3rho3, ip, m, 4, 1, acc_v3rho3_1);
        store_strided(v3rho3, ip, m, 4, 2, acc_v3rho3_2);
        store_strided(v3rho3, ip, m, 4, 3, acc_v3rho3_3);
        ip += 8;
    }
}

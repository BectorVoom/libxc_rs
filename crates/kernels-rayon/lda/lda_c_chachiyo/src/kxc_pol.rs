//! LDA_C_CHACHIYO kxc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/lda_exc/lda_c_chachiyo.c`
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

/// Accumulate 8 elements with a given stride and offset.
///
/// `+=`, not `=`: the scalar kernel this was translated from writes
/// `out[ip * stride + offset] += v`, and a plain store is not the same
/// operation. It differs on the sign of zero -- `0.0 + -0.0` is `+0.0`
/// while a store of `-0.0` keeps the sign -- which is a bit difference
/// the fingerprint gate sees, and it would silently drop a caller's
/// existing contribution if one were ever there.
///
/// The read is not free on this path: a polarized `kxc`/`lxc` kernel
/// writes many strided outputs per point, and `lda_c_pw_erf kxc pol`
/// measured 84 -> 114 ns/pt (1.36x). It is charged anyway, because the
/// scalar kernel this is compared against does the same read. Gathering
/// into a vector, adding once and scattering back was tried and is no
/// faster (117 ns/pt), so the cost is the load itself, not scheduling.
#[inline(always)]
fn store_strided(s: &mut [f64], ip: usize, m: usize, stride: usize, offset: usize, acc: f64x8) {
    let a: [f64; 8] = acc.into();
    if m == 8 {
        let base = ip * stride + offset;
        s[base] += a[0];
        s[base + stride] += a[1];
        s[base + 2 * stride] += a[2];
        s[base + 3 * stride] += a[3];
        s[base + 4 * stride] += a[4];
        s[base + 5 * stride] += a[5];
        s[base + 6 * stride] += a[6];
        s[base + 7 * stride] += a[7];
    } else {
        for k in 0..m {
            s[(ip + k) * stride + offset] += a[k];
        }
    }
}

#[allow(unused_variables, non_snake_case)]
pub fn lda_c_chachiyo_kxc_pol(
    rho: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    v2rho2: &mut [f64],
    v3rho3: &mut [f64],
    param_af: f64,
    param_ap: f64,
    param_bf: f64,
    param_bp: f64,
    param_cf: f64,
    param_cp: f64,
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
            let t3 = param_bp * t2;
            let t5 = (simd::cbrt(f64x8::splat(1.0) / f64x8::splat(M_PI)));
            let t7 = f64x8::splat(M_CBRT4);
            let t8 = f64x8::splat(1.0) / t5 * t7;
            let t9 = v_rho0 + v_rho1;
            let t10 = (simd::cbrt(t9));
            let t11 = t8 * t10;
            let t14 = param_cp * t1;
            let t15 = t5 * t5;
            let t17 = t7 * t7;
            let t18 = f64x8::splat(1.0) / t15 * t17;
            let t19 = t10 * t10;
            let t20 = t18 * t19;
            let t23 = f64x8::splat(1.0) + t3 * t11 / f64x8::splat(3.0) + t14 * t20 / f64x8::splat(3.0);
            let t24 = (simd::ln(t23));
            let t25 = param_ap * t24;
            let t26 = param_bf * t2;
            let t29 = param_cf * t1;
            let t32 = f64x8::splat(1.0) + t26 * t11 / f64x8::splat(3.0) + t29 * t20 / f64x8::splat(3.0);
            let t33 = (simd::ln(t32));
            let t35 = param_af * t33 - t25;
            let t36 = v_rho0 - v_rho1;
            let t37 = f64x8::splat(1.0) / t9;
            let t38 = t36 * t37;
            let t39 = f64x8::splat(1.0) + t38;
            let t40 = (t39).simd_le(zeta_threshold);
            let t41 = (simd::cbrt(zeta_threshold));
            let t42 = t41 * zeta_threshold;
            let t43 = (simd::cbrt(t39));
            let t45 = ((t40).select(t42, t43 * t39));
            let t46 = f64x8::splat(1.0) - t38;
            let t47 = (t46).simd_le(zeta_threshold);
            let t48 = (simd::cbrt(t46));
            let t50 = ((t47).select(t42, t48 * t46));
            let t51 = t45 + t50 - f64x8::splat(2.0);
            let t53 = f64x8::splat(M_CBRT2);
            let t56 = f64x8::splat(1.0) / (f64x8::splat(2.0) * t53 - f64x8::splat(2.0));
            let t57 = t35 * t51 * t56;
            let tzk0 = t25 + t57;
            acc_zk = tzk0;
            let t59 = t8 / t19;
            let t63 = t18 / t10;
            let t66 = t3 * t59 / f64x8::splat(9.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t14 * t63;
            let t68 = f64x8::splat(1.0) / t23;
            let t69 = param_ap * t66 * t68;
            let t74 = t26 * t59 / f64x8::splat(9.0) + f64x8::splat(2.0) / f64x8::splat(9.0) * t29 * t63;
            let t76 = f64x8::splat(1.0) / t32;
            let t78 = param_af * t74 * t76 - t69;
            let t80 = t78 * t51 * t56;
            let t81 = t9 * t9;
            let t82 = f64x8::splat(1.0) / t81;
            let t83 = t36 * t82;
            let t84 = t37 - t83;
            let t87 = ((t40).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t43 * t84));
            let t88 = -t84;
            let t91 = ((t47).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t48 * t88));
            let t92 = t87 + t91;
            let t94 = t35 * t92 * t56;
            let tvrho0 = t25 + t57 + t9 * (t69 + t80 + t94);
            acc_vrho_0 = tvrho0;
            let t97 = -t37 - t83;
            let t100 = ((t40).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t43 * t97));
            let t101 = -t97;
            let t104 = ((t47).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t48 * t101));
            let t105 = t100 + t104;
            let t107 = t35 * t105 * t56;
            let tvrho1 = t25 + t57 + t9 * (t69 + t80 + t107);
            acc_vrho_1 = tvrho1;
            let t110 = f64x8::splat(2.0) * t69;
            let t111 = f64x8::splat(2.0) * t80;
            let t115 = t8 / t19 / t9;
            let t119 = t18 / t10 / t9;
            let t122 = -f64x8::splat(2.0) / f64x8::splat(27.0) * t3 * t115 - f64x8::splat(2.0) / f64x8::splat(27.0) * t14 * t119;
            let t123 = param_ap * t122;
            let t124 = t123 * t68;
            let t125 = t66 * t66;
            let t127 = t23 * t23;
            let t128 = f64x8::splat(1.0) / t127;
            let t129 = param_ap * t125 * t128;
            let t133 = -f64x8::splat(2.0) / f64x8::splat(27.0) * t26 * t115 - f64x8::splat(2.0) / f64x8::splat(27.0) * t29 * t119;
            let t134 = param_af * t133;
            let t136 = t74 * t74;
            let t138 = t32 * t32;
            let t139 = f64x8::splat(1.0) / t138;
            let t141 = -param_af * t136 * t139 + t134 * t76 - t124 + t129;
            let t143 = t141 * t51 * t56;
            let t145 = t78 * t92 * t56;
            let t146 = f64x8::splat(2.0) * t145;
            let t147 = t43 * t43;
            let t148 = f64x8::splat(1.0) / t147;
            let t149 = t84 * t84;
            let t152 = t81 * t9;
            let t153 = f64x8::splat(1.0) / t152;
            let t154 = t36 * t153;
            let t156 = -f64x8::splat(2.0) * t82 + f64x8::splat(2.0) * t154;
            let t160 = ((t40).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t148 * t149 + f64x8::splat(4.0) / f64x8::splat(3.0) * t43 * t156));
            let t161 = t48 * t48;
            let t162 = f64x8::splat(1.0) / t161;
            let t163 = t88 * t88;
            let t166 = -t156;
            let t170 = ((t47).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t162 * t163 + f64x8::splat(4.0) / f64x8::splat(3.0) * t48 * t166));
            let t171 = t160 + t170;
            let t173 = t35 * t171 * t56;
            let tv2rho20 = t110 + t111 + f64x8::splat(2.0) * t94 + t9 * (t124 - t129 + t143 + t146 + t173);
            acc_v2rho2_0 = tv2rho20;
            let t177 = t78 * t105 * t56;
            let t178 = t148 * t97;
            let t181 = t43 * t36;
            let t185 = ((t40).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t178 * t84 + f64x8::splat(8.0) / f64x8::splat(3.0) * t181 * t153));
            let t186 = t162 * t101;
            let t189 = t48 * t36;
            let t193 = ((t47).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t186 * t88 - f64x8::splat(8.0) / f64x8::splat(3.0) * t189 * t153));
            let t194 = t185 + t193;
            let t196 = t35 * t194 * t56;
            let tv2rho21 = t110 + t111 + t94 + t107 + t9 * (t124 - t129 + t143 + t145 + t177 + t196);
            acc_v2rho2_1 = tv2rho21;
            let t200 = f64x8::splat(2.0) * t177;
            let t201 = t97 * t97;
            let t205 = f64x8::splat(2.0) * t82 + f64x8::splat(2.0) * t154;
            let t209 = ((t40).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t148 * t201 + f64x8::splat(4.0) / f64x8::splat(3.0) * t43 * t205));
            let t210 = t101 * t101;
            let t213 = -t205;
            let t217 = ((t47).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(9.0) * t162 * t210 + f64x8::splat(4.0) / f64x8::splat(3.0) * t48 * t213));
            let t218 = t209 + t217;
            let t220 = t35 * t218 * t56;
            let tv2rho22 = t110 + t111 + f64x8::splat(2.0) * t107 + t9 * (t124 - t129 + t143 + t200 + t220);
            acc_v2rho2_2 = tv2rho22;
            let t223 = f64x8::splat(3.0) * t124;
            let t224 = f64x8::splat(3.0) * t129;
            let t225 = f64x8::splat(3.0) * t143;
            let t230 = t8 / t19 / t81;
            let t235 = t18 / t10 / t81;
            let t239 = param_ap * (f64x8::splat(10.0) / f64x8::splat(81.0) * t3 * t230 + f64x8::splat(8.0) / f64x8::splat(81.0) * t14 * t235);
            let t240 = t239 * t68;
            let t241 = t128 * t66;
            let t242 = t123 * t241;
            let t243 = f64x8::splat(3.0) * t242;
            let t247 = f64x8::splat(1.0) / t127 / t23;
            let t248 = param_ap * t125 * t66 * t247;
            let t249 = f64x8::splat(2.0) * t248;
            let t255 = param_af * (f64x8::splat(10.0) / f64x8::splat(81.0) * t26 * t230 + f64x8::splat(8.0) / f64x8::splat(81.0) * t29 * t235);
            let t257 = t139 * t74;
            let t263 = f64x8::splat(1.0) / t138 / t32;
            let t266 = f64x8::splat(2.0) * param_af * t136 * t74 * t263 - f64x8::splat(3.0) * t134 * t257 + t255 * t76 - t240 + t243 - t249;
            let t268 = t266 * t51 * t56;
            let t270 = t141 * t92 * t56;
            let t271 = f64x8::splat(3.0) * t270;
            let t273 = t78 * t171 * t56;
            let t276 = f64x8::splat(1.0) / t147 / t39;
            let t277 = t149 * t84;
            let t280 = t148 * t84;
            let t283 = t81 * t81;
            let t284 = f64x8::splat(1.0) / t283;
            let t285 = t36 * t284;
            let t287 = f64x8::splat(6.0) * t153 - f64x8::splat(6.0) * t285;
            let t291 = ((t40).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t276 * t277 + f64x8::splat(4.0) / f64x8::splat(3.0) * t280 * t156 + f64x8::splat(4.0) / f64x8::splat(3.0) * t43 * t287));
            let t293 = f64x8::splat(1.0) / t161 / t46;
            let t294 = t163 * t88;
            let t297 = t162 * t88;
            let t300 = -t287;
            let t304 = ((t47).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t293 * t294 + f64x8::splat(4.0) / f64x8::splat(3.0) * t297 * t166 + f64x8::splat(4.0) / f64x8::splat(3.0) * t48 * t300));
            let t305 = t291 + t304;
            let t307 = t35 * t305 * t56;
            let tv3rho30 = t223 - t224 + t225 + f64x8::splat(6.0) * t145 + f64x8::splat(3.0) * t173 + t9 * (t240 - t243 + t249 + t268 + t271 + f64x8::splat(3.0) * t273 + t307);
            acc_v3rho3_0 = tv3rho30;
            let t311 = f64x8::splat(2.0) * t196;
            let t314 = t141 * t105 * t56;
            let t316 = t78 * t194 * t56;
            let t317 = f64x8::splat(2.0) * t316;
            let t318 = t276 * t97;
            let t321 = t148 * t36;
            let t332 = ((t40).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t318 * t149 + f64x8::splat(16.0) / f64x8::splat(9.0) * t321 * t153 * t84 + f64x8::splat(4.0) / f64x8::splat(9.0) * t178 * t156 + f64x8::splat(8.0) / f64x8::splat(3.0) * t43 * t153 - f64x8::splat(8.0) * t181 * t284));
            let t333 = t293 * t101;
            let t336 = t162 * t36;
            let t347 = ((t47).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t333 * t163 - f64x8::splat(16.0) / f64x8::splat(9.0) * t336 * t153 * t88 + f64x8::splat(4.0) / f64x8::splat(9.0) * t186 * t166 - f64x8::splat(8.0) / f64x8::splat(3.0) * t48 * t153 + f64x8::splat(8.0) * t189 * t284));
            let t348 = t332 + t347;
            let t350 = t35 * t348 * t56;
            let tv3rho31 = t223 - t224 + t225 + f64x8::splat(4.0) * t145 + t173 + t200 + t311 + t9 * (t240 - t243 + t249 + t268 + f64x8::splat(2.0) * t270 + t273 + t314 + t317 + t350);
            acc_v3rho3_1 = tv3rho31;
            let t356 = t78 * t218 * t56;
            let t357 = t276 * t201;
            let t362 = t148 * t205;
            let t367 = -f64x8::splat(2.0) * t153 - f64x8::splat(6.0) * t285;
            let t371 = ((t40).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t357 * t84 + f64x8::splat(16.0) / f64x8::splat(9.0) * t178 * t154 + f64x8::splat(4.0) / f64x8::splat(9.0) * t362 * t84 + f64x8::splat(4.0) / f64x8::splat(3.0) * t43 * t367));
            let t372 = t293 * t210;
            let t377 = t162 * t213;
            let t380 = -t367;
            let t384 = ((t47).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t372 * t88 - f64x8::splat(16.0) / f64x8::splat(9.0) * t186 * t154 + f64x8::splat(4.0) / f64x8::splat(9.0) * t377 * t88 + f64x8::splat(4.0) / f64x8::splat(3.0) * t48 * t380));
            let t385 = t371 + t384;
            let t387 = t35 * t385 * t56;
            let tv3rho32 = t223 - t224 + t225 + t146 + f64x8::splat(4.0) * t177 + t311 + t220 + t9 * (t240 - t243 + t249 + t268 + t270 + f64x8::splat(2.0) * t314 + t317 + t356 + t387);
            acc_v3rho3_2 = tv3rho32;
            let t392 = f64x8::splat(3.0) * t314;
            let t394 = t201 * t97;
            let t400 = -f64x8::splat(6.0) * t153 - f64x8::splat(6.0) * t285;
            let t404 = ((t40).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t276 * t394 + f64x8::splat(4.0) / f64x8::splat(3.0) * t178 * t205 + f64x8::splat(4.0) / f64x8::splat(3.0) * t43 * t400));
            let t405 = t210 * t101;
            let t410 = -t400;
            let t414 = ((t47).select(f64x8::splat(0.0), -f64x8::splat(8.0) / f64x8::splat(27.0) * t293 * t405 + f64x8::splat(4.0) / f64x8::splat(3.0) * t186 * t213 + f64x8::splat(4.0) / f64x8::splat(3.0) * t48 * t410));
            let t415 = t404 + t414;
            let t417 = t35 * t415 * t56;
            let tv3rho33 = t223 - t224 + t225 + f64x8::splat(6.0) * t177 + f64x8::splat(3.0) * t220 + t9 * (t240 - t243 + t249 + t268 + t392 + f64x8::splat(3.0) * t356 + t417);
            acc_v3rho3_3 = tv3rho33;
        }
        store_add(zk, ip, m, acc_zk);
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

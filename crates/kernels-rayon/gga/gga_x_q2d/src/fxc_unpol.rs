//! GGA_X_Q2D fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_q2d.c`
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
pub fn gga_x_q2d_fxc_unpol(
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
            let t7 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t8 = zeta_threshold - f64x8::splat(1.0);
            let t10 = ((t7).select(t8, (t7).select(-t8, f64x8::splat(0.0))));
            let t11 = f64x8::splat(1.0) + t10;
            let t13 = (simd::cbrt(zeta_threshold));
            let t15 = (simd::cbrt(t11));
            let t17 = (((t11).simd_le(zeta_threshold)).select(t13 * zeta_threshold, t15 * t11));
            let t18 = t3 / t4 * t17;
            let t19 = (simd::cbrt(v_rho));
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
            let t30 = t19 * t19;
            let t32 = f64x8::splat(1.0) / t30 / t29;
            let t34 = t25 * t28 * t32;
            let t36 = f64x8::splat(0.804) + f64x8::splat(5.0) / f64x8::splat(972.0) * t34;
            let t39 = f64x8::splat(1.804) - f64x8::splat(0.646416) / t36;
            let t40 = t20 * t20;
            let t42 = f64x8::splat(1.0) / t22 / t21;
            let t43 = t40 * t42;
            let t44 = v_sigma * v_sigma;
            let t45 = t44 * t26;
            let t46 = t29 * t29;
            let t47 = t46 * v_rho;
            let t49 = f64x8::splat(1.0) / t19 / t47;
            let t53 = f64x8::splat(100.0) - t43 * t45 * t49 / f64x8::splat(288.0);
            let t55 = f64x8::splat(1.0) / t22;
            let t56 = t40 * t55;
            let t57 = ((v_sigma).sqrt());
            let t60 = f64x8::splat(1.0) / t19 / v_rho;
            let t62 = t56 * t57 * t26 * t60;
            let t63 = (simd::pow(t62, f64x8::splat(3.5)));
            let t65 = f64x8::splat(1.0) + t34 / f64x8::splat(24.0);
            let t68 = t39 * t53 + f64x8::splat(8.715382969798257e-05) * t63 * t65;
            let t70 = t21 * t21;
            let t71 = f64x8::splat(1.0) / t70;
            let t72 = t44 * v_sigma;
            let t74 = t46 * t46;
            let t75 = f64x8::splat(1.0) / t74;
            let t78 = f64x8::splat(100.0) + t71 * t72 * t75 / f64x8::splat(576.0);
            let t79 = f64x8::splat(1.0) / t78;
            let t83 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t19 * t68 * t79));
            let tzk0 = f64x8::splat(2.0) * t83;
            acc_zk = tzk0;
            let t84 = f64x8::splat(1.0) / t30;
            let t89 = t36 * t36;
            let t90 = f64x8::splat(1.0) / t89;
            let t92 = t90 * t20 * t24;
            let t93 = t29 * v_rho;
            let t95 = f64x8::splat(1.0) / t30 / t93;
            let t100 = t39 * t40;
            let t101 = t100 * t42;
            let t102 = t46 * t29;
            let t104 = f64x8::splat(1.0) / t19 / t102;
            let t108 = (simd::pow(t62, f64x8::splat(2.5)));
            let t110 = t108 * t65 * t40;
            let t111 = t55 * t57;
            let t114 = t26 / t19 / t29;
            let t118 = t63 * t20;
            let t119 = t118 * t24;
            let t123 = -f64x8::splat(0.00886716049382716) * t92 * t28 * t95 * t53 + t101 * t45 * t104 / f64x8::splat(54.0) - f64x8::splat(0.00040671787192391866) * t110 * t111 * t114 - f64x8::splat(9.683758855331397e-06) * t119 * t28 * t95;
            let t131 = t3 / t4 / t70 * t17;
            let t133 = f64x8::splat(1.0) / t30 / t74;
            let t134 = t133 * t68;
            let t135 = t78 * t78;
            let t136 = f64x8::splat(1.0) / t135;
            let t137 = t136 * t72;
            let t142 = ((t2).select(f64x8::splat(0.0), -t18 * t84 * t68 * t79 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t19 * t123 * t79 - t131 * t134 * t137 / f64x8::splat(192.0)));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t142 + f64x8::splat(2.0) * t83;
            acc_vrho = tvrho0;
            let t145 = t27 * t32;
            let t149 = v_sigma * t26;
            let t153 = f64x8::splat(1.0) / t57;
            let t154 = t55 * t153;
            let t155 = t26 * t60;
            let t159 = t24 * t27;
            let t163 = f64x8::splat(0.0033251851851851854) * t92 * t145 * t53 - t101 * t149 * t49 / f64x8::splat(144.0) + f64x8::splat(0.0001525192019714695) * t110 * t154 * t155 + f64x8::splat(3.6314095707492738e-06) * t118 * t159 * t32;
            let t168 = t46 * t93;
            let t170 = f64x8::splat(1.0) / t30 / t168;
            let t171 = t170 * t68;
            let t172 = t136 * t44;
            let t177 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t19 * t163 * t79 + t131 * t171 * t172 / f64x8::splat(512.0)));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t177;
            acc_vsigma = tvsigma0;
            let t181 = f64x8::splat(1.0) / t30 / v_rho;
            let t190 = t74 * v_rho;
            let t192 = f64x8::splat(1.0) / t30 / t190;
            let t193 = t192 * t68;
            let t198 = f64x8::splat(1.0) / t89 / t36;
            let t200 = t198 * t40 * t42;
            let t202 = f64x8::splat(1.0) / t19 / t168;
            let t208 = f64x8::splat(1.0) / t30 / t46;
            let t213 = t90 * t71;
            let t214 = t74 * t29;
            let t215 = f64x8::splat(1.0) / t214;
            let t219 = t90 * t72;
            let t225 = ((t62) * (t62).sqrt());
            let t227 = t225 * t65 * t20;
            let t228 = t24 * v_sigma;
            let t229 = t27 * t208;
            let t233 = f64x8::splat(1.0) / t21;
            let t234 = t108 * t233;
            let t235 = t57 * v_sigma;
            let t236 = f64x8::splat(1.0) / t102;
            let t242 = t26 / t19 / t93;
            let t249 = -f64x8::splat(0.00048653829870107875) * t200 * t45 * t202 * t53 + f64x8::splat(0.03251292181069959) * t92 * t28 * t208 * t53 - f64x8::splat(0.0019704801097393688) * t213 * t72 * t215 - f64x8::splat(2.0228913839792802e-05) * t219 * t215 - f64x8::splat(19.0) / f64x8::splat(162.0) * t101 * t45 * t202 + f64x8::splat(0.008134357438478373) * t227 * t228 * t229 + f64x8::splat(0.0010845809917971164) * t234 * t235 * t236 + f64x8::splat(0.0009490083678224769) * t110 * t111 * t242 + f64x8::splat(3.550711580288179e-05) * t119 * t28 * t208;
            let t254 = t133 * t123;
            let t258 = t70 * t70;
            let t262 = t3 / t4 / t258 * t17;
            let t263 = t74 * t74;
            let t264 = t263 * v_rho;
            let t266 = f64x8::splat(1.0) / t30 / t264;
            let t267 = t266 * t68;
            let t269 = f64x8::splat(1.0) / t135 / t78;
            let t270 = t44 * t44;
            let t271 = t270 * t44;
            let t272 = t269 * t271;
            let t277 = ((t2).select(f64x8::splat(0.0), t18 * t181 * t68 * t79 / f64x8::splat(12.0) - t18 * t84 * t123 * t79 / f64x8::splat(4.0) + f64x8::splat(25.0) / f64x8::splat(576.0) * t131 * t193 * t137 - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t19 * t249 * t79 - t131 * t254 * t137 / f64x8::splat(96.0) - t262 * t267 * t272 / f64x8::splat(6912.0)));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t277 + f64x8::splat(4.0) * t142;
            acc_v2rho2 = tv2rho20;
            let t284 = t26 * t104;
            let t285 = t53 * v_sigma;
            let t289 = t27 * t95;
            let t293 = f64x8::splat(1.0) / t190;
            let t297 = t90 * t44;
            let t303 = t159 * t95;
            let t306 = f64x8::splat(1.0) / t47;
            let t315 = f64x8::splat(0.00018245186201290453) * t200 * t284 * t285 - f64x8::splat(0.00886716049382716) * t92 * t289 * t53 + f64x8::splat(0.0007389300411522634) * t213 * t293 * t44 + f64x8::splat(7.585842689922302e-06) * t297 * t293 + t101 * t149 * t104 / f64x8::splat(27.0) - f64x8::splat(0.00305038403942939) * t227 * t303 - f64x8::splat(0.00040671787192391866) * t234 * t57 * t306 - f64x8::splat(0.00020335893596195933) * t110 * t154 * t114 - f64x8::splat(9.683758855331397e-06) * t118 * t303;
            let t320 = t133 * t163;
            let t327 = t170 * t123;
            let t332 = f64x8::splat(1.0) / t30 / t263;
            let t333 = t332 * t68;
            let t334 = t270 * v_sigma;
            let t335 = t269 * t334;
            let t340 = ((t2).select(f64x8::splat(0.0), -t18 * t84 * t163 * t79 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t19 * t315 * t79 - t131 * t320 * t137 / f64x8::splat(192.0) - f64x8::splat(23.0) / f64x8::splat(1536.0) * t131 * t134 * t172 + t131 * t327 * t172 / f64x8::splat(512.0) + t262 * t333 * t335 / f64x8::splat(18432.0)));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t340 + f64x8::splat(2.0) * t177;
            acc_v2rhosigma = tv2rhosigma0;
            let t343 = t26 * t49;
            let t350 = t90 * t75;
            let t353 = t42 * t26;
            let t357 = f64x8::splat(1.0) / v_sigma;
            let t358 = t24 * t357;
            let t362 = f64x8::splat(1.0) / t46;
            let t366 = f64x8::splat(1.0) / t235;
            let t367 = t55 * t366;
            let t371 = -f64x8::splat(6.84194482548392e-05) * t200 * t343 * t53 - f64x8::splat(0.00027709876543209876) * t213 * t75 * v_sigma - f64x8::splat(2.844691008720863e-06) * t350 * v_sigma - t100 * t353 * t49 / f64x8::splat(144.0) + f64x8::splat(0.0011438940147860213) * t227 * t358 * t145 + f64x8::splat(0.0001525192019714695) * t234 * t362 * t153 - f64x8::splat(7.625960098573475e-05) * t110 * t367 * t155;
            let t376 = t170 * t163;
            let t380 = t74 * t168;
            let t382 = f64x8::splat(1.0) / t30 / t380;
            let t383 = t382 * t68;
            let t384 = t269 * t270;
            let t388 = t136 * v_sigma;
            let t393 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t18 * t19 * t371 * t79 + t131 * t376 * t172 / f64x8::splat(256.0) - t262 * t383 * t384 / f64x8::splat(49152.0) + t131 * t171 * t388 / f64x8::splat(256.0)));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t393;
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

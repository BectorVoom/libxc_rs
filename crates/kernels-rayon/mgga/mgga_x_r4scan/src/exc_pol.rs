//! MGGA_X_R4SCAN exc pol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_r4scan.c`
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
pub fn mgga_x_r4scan_exc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    param_c1: f64,
    param_c2: f64,
    param_d: f64,
    param_da4: f64,
    param_dp2: f64,
    param_dp4: f64,
    param_eta: f64,
    param_k1: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let np = zk.len();
    let param_c1 = f64x8::splat(param_c1);
    let param_c2 = f64x8::splat(param_c2);
    let param_d = f64x8::splat(param_d);
    let param_da4 = f64x8::splat(param_da4);
    let param_dp2 = f64x8::splat(param_dp2);
    let param_dp4 = f64x8::splat(param_dp4);
    let param_eta = f64x8::splat(param_eta);
    let param_k1 = f64x8::splat(param_k1);
    let dens_threshold = f64x8::splat(dens_threshold);
    let zeta_threshold = f64x8::splat(zeta_threshold);
    let mut ip = 0usize;
    while ip < np {
        let m = (np - ip).min(8);
        let v_rho0 = load_strided(rho, ip, np, 2, 0);
        let v_rho1 = load_strided(rho, ip, np, 2, 1);
        let v_sigma0 = load_strided(sigma, ip, np, 3, 0);
        let v_sigma1 = load_strided(sigma, ip, np, 3, 1);
        let v_sigma2 = load_strided(sigma, ip, np, 3, 2);
        let v_lapl0 = load_strided(lapl, ip, np, 2, 0);
        let v_lapl1 = load_strided(lapl, ip, np, 2, 1);
        let v_tau0 = load_strided(tau, ip, np, 2, 0);
        let v_tau1 = load_strided(tau, ip, np, 2, 1);
        let mut acc_zk = V_ZERO;
        {
            let t2 = (v_rho0).simd_le(dens_threshold);
            let t3 = f64x8::splat(M_CBRT3);
            let t4 = f64x8::splat(M_CBRTPI);
            let t6 = t3 / t4;
            let t7 = v_rho0 + v_rho1;
            let t8 = f64x8::splat(1.0) / t7;
            let t11 = (f64x8::splat(2.0) * v_rho0 * t8).simd_le(zeta_threshold);
            let t12 = zeta_threshold - f64x8::splat(1.0);
            let t15 = (f64x8::splat(2.0) * v_rho1 * t8).simd_le(zeta_threshold);
            let t16 = -t12;
            let t17 = v_rho0 - v_rho1;
            let t19 = ((t11).select(t12, (t15).select(t16, t17 * t8)));
            let t20 = f64x8::splat(1.0) + t19;
            let t21 = (t20).simd_le(zeta_threshold);
            let t22 = (simd::cbrt(zeta_threshold));
            let t23 = t22 * zeta_threshold;
            let t24 = (simd::cbrt(t20));
            let t26 = ((t21).select(t23, t24 * t20));
            let t27 = t6 * t26;
            let t28 = (simd::cbrt(t7));
            let t30 = f64x8::splat(20.0) / f64x8::splat(27.0) + f64x8::splat(5.0) / f64x8::splat(3.0) * param_eta;
            let t31 = f64x8::splat(M_CBRT6);
            let t32 = t31 * t31;
            let t33 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t34 = (simd::cbrt(t33));
            let t35 = t34 * t33;
            let t36 = f64x8::splat(1.0) / t35;
            let t37 = t32 * t36;
            let t38 = v_sigma0 * v_sigma0;
            let t39 = v_rho0 * v_rho0;
            let t40 = t39 * t39;
            let t41 = t40 * v_rho0;
            let t42 = (simd::cbrt(v_rho0));
            let t44 = f64x8::splat(1.0) / t42 / t41;
            let t45 = t38 * t44;
            let t46 = param_dp2 * param_dp2;
            let t47 = t46 * t46;
            let t48 = f64x8::splat(1.0) / t47;
            let t52 = (simd::exp(-t37 * t45 * t48 / f64x8::splat(576.0)));
            let t56 = (-f64x8::splat(0.162742215233874) * t30 * t52 + f64x8::splat(10.0) / f64x8::splat(81.0)) * t31;
            let t57 = t34 * t34;
            let t58 = f64x8::splat(1.0) / t57;
            let t59 = t58 * v_sigma0;
            let t60 = t42 * t42;
            let t61 = t60 * t39;
            let t62 = f64x8::splat(1.0) / t61;
            let t63 = t59 * t62;
            let t66 = param_k1 + t56 * t63 / f64x8::splat(24.0);
            let t70 = param_k1 * (f64x8::splat(1.0) - param_k1 / t66);
            let t71 = t60 * v_rho0;
            let t72 = f64x8::splat(1.0) / t71;
            let t74 = v_sigma0 * t62;
            let t76 = v_tau0 * t72 - t74 / f64x8::splat(8.0);
            let t78 = f64x8::splat(3.0) / f64x8::splat(10.0) * t32 * t57;
            let t79 = param_eta * v_sigma0;
            let t82 = t78 + t79 * t62 / f64x8::splat(8.0);
            let t83 = f64x8::splat(1.0) / t82;
            let t84 = t76 * t83;
            let t85 = (t84).simd_le(f64x8::splat(0.0));
            let t86 = (f64x8::splat(0.0)).simd_lt(t84);
            let t87 = ((t86).select(f64x8::splat(0.0), t84));
            let t88 = param_c1 * t87;
            let t89 = f64x8::splat(1.0) - t87;
            let t90 = f64x8::splat(1.0) / t89;
            let t92 = (simd::exp(-t88 * t90));
            let t93 = (t84).simd_le(f64x8::splat(2.5));
            let t94 = (f64x8::splat(2.5)).simd_lt(t84);
            let t95 = ((t94).select(f64x8::splat(2.5), t84));
            let t97 = t95 * t95;
            let t99 = t97 * t95;
            let t101 = t97 * t97;
            let t103 = t101 * t95;
            let t105 = t101 * t97;
            let t110 = ((t94).select(t84, f64x8::splat(2.5)));
            let t111 = f64x8::splat(1.0) - t110;
            let t114 = (simd::exp(param_c2 / t111));
            let t116 = ((t85).select(t92, (t93).select(f64x8::splat(1.0) - f64x8::splat(0.667) * t95 - f64x8::splat(0.4445555) * t97 - f64x8::splat(0.663086601049) * t99 + f64x8::splat(1.45129704449) * t101 - f64x8::splat(0.887998041597) * t103 + f64x8::splat(0.234528941479) * t105 - f64x8::splat(0.023185843322) * t101 * t99, -param_d * t114)));
            let t117 = f64x8::splat(0.174) - t70;
            let t120 = t30 * t31;
            let t123 = f64x8::splat(1.0) - t84;
            let t124 = t123 * t123;
            let t128 = (f64x8::splat(0.040570770199022686) - f64x8::splat(0.3023546802608101) * param_eta) * t31;
            let t129 = t128 * t58;
            let t135 = ((f64x8::splat(3.0) / f64x8::splat(4.0) * param_eta + f64x8::splat(2.0) / f64x8::splat(3.0)) * (f64x8::splat(3.0) / f64x8::splat(4.0) * param_eta + f64x8::splat(2.0) / f64x8::splat(3.0)));
            let t140 = ((f64x8::splat(0.0029070010613279013) - f64x8::splat(0.27123702538979) * param_eta) * (f64x8::splat(0.0029070010613279013) - f64x8::splat(0.27123702538979) * param_eta));
            let t144 = (f64x8::splat(146.0) / f64x8::splat(2025.0) * t135 - f64x8::splat(73.0) / f64x8::splat(540.0) * param_eta - f64x8::splat(146.0) / f64x8::splat(1215.0) + t140 / param_k1) * t32;
            let t145 = t36 * t38;
            let t149 = -f64x8::splat(0.162742215233874) + f64x8::splat(0.162742215233874) * t84 + f64x8::splat(0.00678092563474475) * t120 * t63 - f64x8::splat(0.059353125082804) * t124 + t129 * t74 * t123 / f64x8::splat(24.0) + t144 * t145 * t44 / f64x8::splat(576.0);
            let t150 = t76 * t76;
            let t151 = t149 * t150;
            let t152 = t82 * t82;
            let t153 = f64x8::splat(1.0) / t152;
            let t154 = t150 * t150;
            let t155 = t152 * t152;
            let t156 = f64x8::splat(1.0) / t155;
            let t158 = t154 * t156 + f64x8::splat(1.0);
            let t159 = f64x8::splat(1.0) / t158;
            let t160 = t153 * t159;
            let t161 = param_da4 * param_da4;
            let t162 = f64x8::splat(1.0) / t161;
            let t164 = param_dp4 * param_dp4;
            let t165 = t164 * t164;
            let t166 = f64x8::splat(1.0) / t165;
            let t171 = (simd::exp(-t124 * t162 - t37 * t45 * t166 / f64x8::splat(576.0)));
            let t172 = t160 * t171;
            let t175 = t116 * t117 + f64x8::splat(2.0) * t151 * t172 + t70 + f64x8::splat(1.0);
            let t176 = t28 * t175;
            let t177 = ((f64x8::splat(3.0)).sqrt());
            let t178 = f64x8::splat(1.0) / t34;
            let t179 = t32 * t178;
            let t180 = ((v_sigma0).sqrt());
            let t181 = t42 * v_rho0;
            let t182 = f64x8::splat(1.0) / t181;
            let t184 = t179 * t180 * t182;
            let t185 = ((t184).sqrt());
            let t189 = (simd::exp(-f64x8::splat(9.8958) * t177 / t185));
            let t190 = f64x8::splat(1.0) - t189;
            let t191 = t176 * t190;
            let t194 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t27 * t191));
            let t195 = (v_rho1).simd_le(dens_threshold);
            let t196 = -t17;
            let t198 = ((t15).select(t12, (t11).select(t16, t196 * t8)));
            let t199 = f64x8::splat(1.0) + t198;
            let t200 = (t199).simd_le(zeta_threshold);
            let t201 = (simd::cbrt(t199));
            let t203 = ((t200).select(t23, t201 * t199));
            let t204 = t6 * t203;
            let t205 = v_sigma2 * v_sigma2;
            let t206 = v_rho1 * v_rho1;
            let t207 = t206 * t206;
            let t208 = t207 * v_rho1;
            let t209 = (simd::cbrt(v_rho1));
            let t211 = f64x8::splat(1.0) / t209 / t208;
            let t212 = t205 * t211;
            let t216 = (simd::exp(-t37 * t212 * t48 / f64x8::splat(576.0)));
            let t220 = (-f64x8::splat(0.162742215233874) * t30 * t216 + f64x8::splat(10.0) / f64x8::splat(81.0)) * t31;
            let t221 = t58 * v_sigma2;
            let t222 = t209 * t209;
            let t223 = t222 * t206;
            let t224 = f64x8::splat(1.0) / t223;
            let t225 = t221 * t224;
            let t228 = param_k1 + t220 * t225 / f64x8::splat(24.0);
            let t232 = param_k1 * (f64x8::splat(1.0) - param_k1 / t228);
            let t233 = t222 * v_rho1;
            let t234 = f64x8::splat(1.0) / t233;
            let t236 = v_sigma2 * t224;
            let t238 = v_tau1 * t234 - t236 / f64x8::splat(8.0);
            let t239 = param_eta * v_sigma2;
            let t242 = t78 + t239 * t224 / f64x8::splat(8.0);
            let t243 = f64x8::splat(1.0) / t242;
            let t244 = t238 * t243;
            let t245 = (t244).simd_le(f64x8::splat(0.0));
            let t246 = (f64x8::splat(0.0)).simd_lt(t244);
            let t247 = ((t246).select(f64x8::splat(0.0), t244));
            let t248 = param_c1 * t247;
            let t249 = f64x8::splat(1.0) - t247;
            let t250 = f64x8::splat(1.0) / t249;
            let t252 = (simd::exp(-t248 * t250));
            let t253 = (t244).simd_le(f64x8::splat(2.5));
            let t254 = (f64x8::splat(2.5)).simd_lt(t244);
            let t255 = ((t254).select(f64x8::splat(2.5), t244));
            let t257 = t255 * t255;
            let t259 = t257 * t255;
            let t261 = t257 * t257;
            let t263 = t261 * t255;
            let t265 = t261 * t257;
            let t270 = ((t254).select(t244, f64x8::splat(2.5)));
            let t271 = f64x8::splat(1.0) - t270;
            let t274 = (simd::exp(param_c2 / t271));
            let t276 = ((t245).select(t252, (t253).select(f64x8::splat(1.0) - f64x8::splat(0.667) * t255 - f64x8::splat(0.4445555) * t257 - f64x8::splat(0.663086601049) * t259 + f64x8::splat(1.45129704449) * t261 - f64x8::splat(0.887998041597) * t263 + f64x8::splat(0.234528941479) * t265 - f64x8::splat(0.023185843322) * t261 * t259, -param_d * t274)));
            let t277 = f64x8::splat(0.174) - t232;
            let t282 = f64x8::splat(1.0) - t244;
            let t283 = t282 * t282;
            let t288 = t36 * t205;
            let t292 = -f64x8::splat(0.162742215233874) + f64x8::splat(0.162742215233874) * t244 + f64x8::splat(0.00678092563474475) * t120 * t225 - f64x8::splat(0.059353125082804) * t283 + t129 * t236 * t282 / f64x8::splat(24.0) + t144 * t288 * t211 / f64x8::splat(576.0);
            let t293 = t238 * t238;
            let t294 = t292 * t293;
            let t295 = t242 * t242;
            let t296 = f64x8::splat(1.0) / t295;
            let t297 = t293 * t293;
            let t298 = t295 * t295;
            let t299 = f64x8::splat(1.0) / t298;
            let t301 = t297 * t299 + f64x8::splat(1.0);
            let t302 = f64x8::splat(1.0) / t301;
            let t303 = t296 * t302;
            let t309 = (simd::exp(-t283 * t162 - t37 * t212 * t166 / f64x8::splat(576.0)));
            let t310 = t303 * t309;
            let t313 = t276 * t277 + f64x8::splat(2.0) * t294 * t310 + t232 + f64x8::splat(1.0);
            let t314 = t28 * t313;
            let t315 = ((v_sigma2).sqrt());
            let t316 = t209 * v_rho1;
            let t317 = f64x8::splat(1.0) / t316;
            let t319 = t179 * t315 * t317;
            let t320 = ((t319).sqrt());
            let t324 = (simd::exp(-f64x8::splat(9.8958) * t177 / t320));
            let t325 = f64x8::splat(1.0) - t324;
            let t326 = t314 * t325;
            let t329 = ((t195).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t204 * t326));
            let tzk0 = t194 + t329;
            acc_zk = tzk0;
        }
        store_add(zk, ip, m, acc_zk);
        ip += 8;
    }
}

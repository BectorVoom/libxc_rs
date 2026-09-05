//! MGGA_X_R4SCAN vxc pol kernel — explicit SIMD (bit-exact).
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
pub fn mgga_x_r4scan_vxc_pol(
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
    vlapl: &mut [f64],
    vtau: &mut [f64],
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
        let mut acc_vrho_0 = V_ZERO;
        let mut acc_vrho_1 = V_ZERO;
        let mut acc_vsigma_0 = V_ZERO;
        let mut acc_vsigma_1 = V_ZERO;
        let mut acc_vsigma_2 = V_ZERO;
        let mut acc_vlapl_0 = V_ZERO;
        let mut acc_vlapl_1 = V_ZERO;
        let mut acc_vtau_0 = V_ZERO;
        let mut acc_vtau_1 = V_ZERO;
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
            let t330 = t7 * t7;
            let t331 = f64x8::splat(1.0) / t330;
            let t332 = t17 * t331;
            let t334 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), t8 - t332)));
            let t337 = ((t21).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t24 * t334));
            let t338 = t6 * t337;
            let t341 = t28 * t28;
            let t342 = f64x8::splat(1.0) / t341;
            let t343 = t342 * t175;
            let t344 = t343 * t190;
            let t346 = t27 * t344 / f64x8::splat(8.0);
            let t347 = param_k1 * param_k1;
            let t348 = t66 * t66;
            let t349 = f64x8::splat(1.0) / t348;
            let t350 = t347 * t349;
            let t351 = t38 * v_sigma0;
            let t352 = t30 * t351;
            let t353 = t40 * t40;
            let t354 = t353 * v_rho0;
            let t355 = f64x8::splat(1.0) / t354;
            let t357 = t355 * t48 * t52;
            let t360 = t39 * v_rho0;
            let t362 = f64x8::splat(1.0) / t60 / t360;
            let t363 = t59 * t362;
            let t366 = -f64x8::splat(3.867381235367984e-06) * t352 * t357 - t56 * t363 / f64x8::splat(9.0);
            let t370 = v_sigma0 * t362;
            let t372 = -f64x8::splat(5.0) / f64x8::splat(3.0) * v_tau0 * t62 + t370 / f64x8::splat(3.0);
            let t373 = t372 * t83;
            let t374 = t76 * t153;
            let t375 = t79 * t362;
            let t376 = t374 * t375;
            let t378 = t373 + t376 / f64x8::splat(3.0);
            let t379 = ((t86).select(f64x8::splat(0.0), t378));
            let t382 = t89 * t89;
            let t383 = f64x8::splat(1.0) / t382;
            let t384 = t383 * t379;
            let t386 = -param_c1 * t379 * t90 - t88 * t384;
            let t387 = t386 * t92;
            let t388 = ((t94).select(f64x8::splat(0.0), t378));
            let t390 = t95 * t388;
            let t392 = t97 * t388;
            let t394 = t99 * t388;
            let t396 = t101 * t388;
            let t398 = t103 * t388;
            let t403 = param_d * param_c2;
            let t404 = t111 * t111;
            let t405 = f64x8::splat(1.0) / t404;
            let t406 = ((t94).select(t378, f64x8::splat(0.0)));
            let t410 = ((t85).select(t387, (t93).select(-f64x8::splat(0.667) * t388 - f64x8::splat(0.889111) * t390 - f64x8::splat(1.989259803147) * t392 + f64x8::splat(5.80518817796) * t394 - f64x8::splat(4.439990207985) * t396 + f64x8::splat(1.407173648874) * t398 - f64x8::splat(0.162300903254) * t105 * t388, -t403 * t405 * t406 * t114)));
            let t412 = t116 * t347;
            let t413 = t349 * t366;
            let t419 = -t378;
            let t428 = t40 * t39;
            let t430 = f64x8::splat(1.0) / t42 / t428;
            let t434 = f64x8::splat(0.162742215233874) * t373 + f64x8::splat(0.054247405077958) * t376 - f64x8::splat(0.018082468359319332) * t120 * t363 - f64x8::splat(0.118706250165608) * t123 * t419 - t129 * t370 * t123 / f64x8::splat(9.0) + t129 * t74 * t419 / f64x8::splat(24.0) - t144 * t145 * t430 / f64x8::splat(108.0);
            let t435 = t434 * t150;
            let t438 = t149 * t76;
            let t439 = t438 * t153;
            let t440 = t159 * t171;
            let t441 = t440 * t372;
            let t444 = t152 * t82;
            let t445 = f64x8::splat(1.0) / t444;
            let t446 = t445 * t159;
            let t447 = t151 * t446;
            let t448 = t171 * param_eta;
            let t449 = t448 * t370;
            let t452 = t151 * t153;
            let t453 = t158 * t158;
            let t454 = f64x8::splat(1.0) / t453;
            let t455 = t454 * t171;
            let t456 = t150 * t76;
            let t457 = t456 * t156;
            let t461 = f64x8::splat(1.0) / t155 / t82;
            let t462 = t154 * t461;
            let t465 = f64x8::splat(4.0) * t457 * t372 + f64x8::splat(4.0) / f64x8::splat(3.0) * t462 * t375;
            let t466 = t455 * t465;
            let t469 = t123 * t162;
            let t476 = -f64x8::splat(2.0) * t469 * t419 + t37 * t38 * t430 * t166 / f64x8::splat(108.0);
            let t477 = t159 * t476;
            let t478 = t477 * t171;
            let t481 = t350 * t366 + t410 * t117 - t412 * t413 + f64x8::splat(2.0) * t435 * t172 + f64x8::splat(4.0) * t439 * t441 + f64x8::splat(4.0) / f64x8::splat(3.0) * t447 * t449 - f64x8::splat(2.0) * t452 * t466 + f64x8::splat(2.0) * t452 * t478;
            let t482 = t28 * t481;
            let t483 = t482 * t190;
            let t486 = (simd::pow(f64x8::splat(3.0), f64x8::splat(1.0) / f64x8::splat(6.0)));
            let t487 = t486 * t486;
            let t488 = t487 * t487;
            let t489 = t488 * t486;
            let t490 = t489 * t26;
            let t492 = f64x8::splat(1.0) / t185 / t184;
            let t493 = t176 * t492;
            let t494 = t490 * t493;
            let t496 = f64x8::splat(1.0) / t42 / t39;
            let t499 = t179 * t180 * t496 * t189;
            let t503 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t338 * t191 - t346 - f64x8::splat(3.0) / f64x8::splat(8.0) * t27 * t483 - f64x8::splat(1.6891736332904388) * t494 * t499));
            let t504 = t196 * t331;
            let t506 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), -t8 - t504)));
            let t509 = ((t200).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t201 * t506));
            let t510 = t6 * t509;
            let t513 = t342 * t313;
            let t514 = t513 * t325;
            let t516 = t204 * t514 / f64x8::splat(8.0);
            let t518 = ((t195).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t510 * t326 - t516));
            let tvrho0 = t194 + t329 + t7 * (t503 + t518);
            acc_vrho_0 = tvrho0;
            let t522 = ((t11).select(f64x8::splat(0.0), (t15).select(f64x8::splat(0.0), -t8 - t332)));
            let t525 = ((t21).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t24 * t522));
            let t526 = t6 * t525;
            let t530 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t526 * t191 - t346));
            let t532 = ((t15).select(f64x8::splat(0.0), (t11).select(f64x8::splat(0.0), t8 - t504)));
            let t535 = ((t200).select(f64x8::splat(0.0), f64x8::splat(4.0) / f64x8::splat(3.0) * t201 * t532));
            let t536 = t6 * t535;
            let t539 = t228 * t228;
            let t540 = f64x8::splat(1.0) / t539;
            let t541 = t347 * t540;
            let t542 = t205 * v_sigma2;
            let t543 = t30 * t542;
            let t544 = t207 * t207;
            let t545 = t544 * v_rho1;
            let t546 = f64x8::splat(1.0) / t545;
            let t548 = t546 * t48 * t216;
            let t551 = t206 * v_rho1;
            let t553 = f64x8::splat(1.0) / t222 / t551;
            let t554 = t221 * t553;
            let t557 = -f64x8::splat(3.867381235367984e-06) * t543 * t548 - t220 * t554 / f64x8::splat(9.0);
            let t561 = v_sigma2 * t553;
            let t563 = -f64x8::splat(5.0) / f64x8::splat(3.0) * v_tau1 * t224 + t561 / f64x8::splat(3.0);
            let t564 = t563 * t243;
            let t565 = t238 * t296;
            let t566 = t239 * t553;
            let t567 = t565 * t566;
            let t569 = t564 + t567 / f64x8::splat(3.0);
            let t570 = ((t246).select(f64x8::splat(0.0), t569));
            let t573 = t249 * t249;
            let t574 = f64x8::splat(1.0) / t573;
            let t575 = t574 * t570;
            let t577 = -param_c1 * t570 * t250 - t248 * t575;
            let t578 = t577 * t252;
            let t579 = ((t254).select(f64x8::splat(0.0), t569));
            let t581 = t255 * t579;
            let t583 = t257 * t579;
            let t585 = t259 * t579;
            let t587 = t261 * t579;
            let t589 = t263 * t579;
            let t594 = t271 * t271;
            let t595 = f64x8::splat(1.0) / t594;
            let t596 = ((t254).select(t569, f64x8::splat(0.0)));
            let t600 = ((t245).select(t578, (t253).select(-f64x8::splat(0.667) * t579 - f64x8::splat(0.889111) * t581 - f64x8::splat(1.989259803147) * t583 + f64x8::splat(5.80518817796) * t585 - f64x8::splat(4.439990207985) * t587 + f64x8::splat(1.407173648874) * t589 - f64x8::splat(0.162300903254) * t265 * t579, -t403 * t595 * t596 * t274)));
            let t602 = t276 * t347;
            let t603 = t540 * t557;
            let t609 = -t569;
            let t618 = t207 * t206;
            let t620 = f64x8::splat(1.0) / t209 / t618;
            let t624 = f64x8::splat(0.162742215233874) * t564 + f64x8::splat(0.054247405077958) * t567 - f64x8::splat(0.018082468359319332) * t120 * t554 - f64x8::splat(0.118706250165608) * t282 * t609 - t129 * t561 * t282 / f64x8::splat(9.0) + t129 * t236 * t609 / f64x8::splat(24.0) - t144 * t288 * t620 / f64x8::splat(108.0);
            let t625 = t624 * t293;
            let t628 = t292 * t238;
            let t629 = t628 * t296;
            let t630 = t302 * t309;
            let t631 = t630 * t563;
            let t634 = t295 * t242;
            let t635 = f64x8::splat(1.0) / t634;
            let t636 = t635 * t302;
            let t637 = t294 * t636;
            let t638 = t309 * param_eta;
            let t639 = t638 * t561;
            let t642 = t294 * t296;
            let t643 = t301 * t301;
            let t644 = f64x8::splat(1.0) / t643;
            let t645 = t644 * t309;
            let t646 = t293 * t238;
            let t647 = t646 * t299;
            let t651 = f64x8::splat(1.0) / t298 / t242;
            let t652 = t297 * t651;
            let t655 = f64x8::splat(4.0) * t647 * t563 + f64x8::splat(4.0) / f64x8::splat(3.0) * t652 * t566;
            let t656 = t645 * t655;
            let t659 = t282 * t162;
            let t666 = -f64x8::splat(2.0) * t659 * t609 + t37 * t205 * t620 * t166 / f64x8::splat(108.0);
            let t667 = t302 * t666;
            let t668 = t667 * t309;
            let t671 = t541 * t557 + t600 * t277 - t602 * t603 + f64x8::splat(2.0) * t625 * t310 + f64x8::splat(4.0) * t629 * t631 + f64x8::splat(4.0) / f64x8::splat(3.0) * t637 * t639 - f64x8::splat(2.0) * t642 * t656 + f64x8::splat(2.0) * t642 * t668;
            let t672 = t28 * t671;
            let t673 = t672 * t325;
            let t676 = t489 * t203;
            let t678 = f64x8::splat(1.0) / t320 / t319;
            let t679 = t314 * t678;
            let t680 = t676 * t679;
            let t682 = f64x8::splat(1.0) / t209 / t206;
            let t685 = t179 * t315 * t682 * t324;
            let t689 = ((t195).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t536 * t326 - t516 - f64x8::splat(3.0) / f64x8::splat(8.0) * t204 * t673 - f64x8::splat(1.6891736332904388) * t680 * t685));
            let tvrho1 = t194 + t329 + t7 * (t530 + t689);
            acc_vrho_1 = tvrho1;
            let t692 = t30 * t38;
            let t693 = f64x8::splat(1.0) / t353;
            let t695 = t693 * t48 * t52;
            let t698 = t58 * t62;
            let t701 = f64x8::splat(1.450267963262994e-06) * t692 * t695 + t56 * t698 / f64x8::splat(24.0);
            let t703 = t62 * t83;
            let t704 = param_eta * t62;
            let t705 = t374 * t704;
            let t707 = -t703 / f64x8::splat(8.0) - t705 / f64x8::splat(8.0);
            let t708 = ((t86).select(f64x8::splat(0.0), t707));
            let t709 = param_c1 * t708;
            let t711 = t383 * t708;
            let t713 = -t709 * t90 - t88 * t711;
            let t714 = t713 * t92;
            let t715 = ((t94).select(f64x8::splat(0.0), t707));
            let t717 = t95 * t715;
            let t719 = t97 * t715;
            let t721 = t99 * t715;
            let t723 = t101 * t715;
            let t725 = t103 * t715;
            let t730 = ((t94).select(t707, f64x8::splat(0.0)));
            let t734 = ((t85).select(t714, (t93).select(-f64x8::splat(0.667) * t715 - f64x8::splat(0.889111) * t717 - f64x8::splat(1.989259803147) * t719 + f64x8::splat(5.80518817796) * t721 - f64x8::splat(4.439990207985) * t723 + f64x8::splat(1.407173648874) * t725 - f64x8::splat(0.162300903254) * t105 * t715, -t403 * t405 * t730 * t114)));
            let t736 = t349 * t701;
            let t742 = -t707;
            let t751 = t36 * v_sigma0;
            let t755 = -f64x8::splat(0.02034277690423425) * t703 - f64x8::splat(0.02034277690423425) * t705 + f64x8::splat(0.00678092563474475) * t120 * t698 - f64x8::splat(0.118706250165608) * t123 * t742 + t128 * t698 * t123 / f64x8::splat(24.0) + t129 * t74 * t742 / f64x8::splat(24.0) + t144 * t751 * t44 / f64x8::splat(288.0);
            let t756 = t755 * t150;
            let t759 = t440 * t62;
            let t760 = t439 * t759;
            let t762 = t151 * t445;
            let t763 = t440 * t704;
            let t769 = -t457 * t62 / f64x8::splat(2.0) - t462 * t704 / f64x8::splat(2.0);
            let t770 = t455 * t769;
            let t775 = v_sigma0 * t44;
            let t779 = -f64x8::splat(2.0) * t469 * t742 - t37 * t775 * t166 / f64x8::splat(288.0);
            let t780 = t159 * t779;
            let t781 = t780 * t171;
            let t784 = t350 * t701 + t734 * t117 - t412 * t736 + f64x8::splat(2.0) * t756 * t172 - t760 / f64x8::splat(2.0) - t762 * t763 / f64x8::splat(2.0) - f64x8::splat(2.0) * t452 * t770 + f64x8::splat(2.0) * t452 * t781;
            let t785 = t28 * t784;
            let t786 = t785 * t190;
            let t789 = f64x8::splat(1.0) / t180;
            let t792 = t179 * t789 * t182 * t189;
            let t796 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t27 * t786 + f64x8::splat(0.6334401124839145) * t494 * t792));
            let tvsigma0 = t7 * t796;
            acc_vsigma_0 = tvsigma0;
            let tvsigma1 = f64x8::splat(0.0);
            acc_vsigma_1 = tvsigma1;
            let t797 = t30 * t205;
            let t798 = f64x8::splat(1.0) / t544;
            let t800 = t798 * t48 * t216;
            let t803 = t58 * t224;
            let t806 = f64x8::splat(1.450267963262994e-06) * t797 * t800 + t220 * t803 / f64x8::splat(24.0);
            let t808 = t224 * t243;
            let t809 = param_eta * t224;
            let t810 = t565 * t809;
            let t812 = -t808 / f64x8::splat(8.0) - t810 / f64x8::splat(8.0);
            let t813 = ((t246).select(f64x8::splat(0.0), t812));
            let t814 = param_c1 * t813;
            let t816 = t574 * t813;
            let t818 = -t248 * t816 - t814 * t250;
            let t819 = t818 * t252;
            let t820 = ((t254).select(f64x8::splat(0.0), t812));
            let t822 = t255 * t820;
            let t824 = t257 * t820;
            let t826 = t259 * t820;
            let t828 = t261 * t820;
            let t830 = t263 * t820;
            let t835 = ((t254).select(t812, f64x8::splat(0.0)));
            let t839 = ((t245).select(t819, (t253).select(-f64x8::splat(0.667) * t820 - f64x8::splat(0.889111) * t822 - f64x8::splat(1.989259803147) * t824 + f64x8::splat(5.80518817796) * t826 - f64x8::splat(4.439990207985) * t828 + f64x8::splat(1.407173648874) * t830 - f64x8::splat(0.162300903254) * t265 * t820, -t403 * t595 * t835 * t274)));
            let t841 = t540 * t806;
            let t847 = -t812;
            let t856 = t36 * v_sigma2;
            let t860 = -f64x8::splat(0.02034277690423425) * t808 - f64x8::splat(0.02034277690423425) * t810 + f64x8::splat(0.00678092563474475) * t120 * t803 - f64x8::splat(0.118706250165608) * t282 * t847 + t128 * t803 * t282 / f64x8::splat(24.0) + t129 * t236 * t847 / f64x8::splat(24.0) + t144 * t856 * t211 / f64x8::splat(288.0);
            let t861 = t860 * t293;
            let t864 = t630 * t224;
            let t865 = t629 * t864;
            let t867 = t294 * t635;
            let t868 = t630 * t809;
            let t874 = -t647 * t224 / f64x8::splat(2.0) - t652 * t809 / f64x8::splat(2.0);
            let t875 = t645 * t874;
            let t880 = v_sigma2 * t211;
            let t884 = -f64x8::splat(2.0) * t659 * t847 - t37 * t880 * t166 / f64x8::splat(288.0);
            let t885 = t302 * t884;
            let t886 = t885 * t309;
            let t889 = t541 * t806 + t839 * t277 - t602 * t841 + f64x8::splat(2.0) * t861 * t310 - t865 / f64x8::splat(2.0) - t867 * t868 / f64x8::splat(2.0) - f64x8::splat(2.0) * t642 * t875 + f64x8::splat(2.0) * t642 * t886;
            let t890 = t28 * t889;
            let t891 = t890 * t325;
            let t894 = f64x8::splat(1.0) / t315;
            let t897 = t179 * t894 * t317 * t324;
            let t901 = ((t195).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t204 * t891 + f64x8::splat(0.6334401124839145) * t680 * t897));
            let tvsigma2 = t7 * t901;
            acc_vsigma_2 = tvsigma2;
            let tvlapl0 = f64x8::splat(0.0);
            acc_vlapl_0 = tvlapl0;
            let tvlapl1 = f64x8::splat(0.0);
            acc_vlapl_1 = tvlapl1;
            let t902 = t72 * t83;
            let t903 = ((t86).select(f64x8::splat(0.0), t902));
            let t904 = param_c1 * t903;
            let t906 = t383 * t903;
            let t908 = -t88 * t906 - t904 * t90;
            let t909 = t908 * t92;
            let t910 = ((t94).select(f64x8::splat(0.0), t902));
            let t912 = t95 * t910;
            let t914 = t97 * t910;
            let t916 = t99 * t910;
            let t918 = t101 * t910;
            let t920 = t103 * t910;
            let t925 = ((t94).select(t902, f64x8::splat(0.0)));
            let t929 = ((t85).select(t909, (t93).select(-f64x8::splat(0.667) * t910 - f64x8::splat(0.889111) * t912 - f64x8::splat(1.989259803147) * t914 + f64x8::splat(5.80518817796) * t916 - f64x8::splat(4.439990207985) * t918 + f64x8::splat(1.407173648874) * t920 - f64x8::splat(0.162300903254) * t105 * t910, -t403 * t405 * t925 * t114)));
            let t936 = f64x8::splat(1.0) / t42 / t40;
            let t941 = f64x8::splat(0.162742215233874) * t902 + f64x8::splat(0.118706250165608) * t123 * t72 * t83 - t129 * v_sigma0 * t936 * t83 / f64x8::splat(24.0);
            let t942 = t941 * t150;
            let t945 = t440 * t72;
            let t948 = t154 * t76;
            let t949 = t149 * t948;
            let t950 = t155 * t152;
            let t951 = f64x8::splat(1.0) / t950;
            let t952 = t949 * t951;
            let t953 = t455 * t72;
            let t956 = t72 * t171;
            let t957 = t469 * t956;
            let t960 = t929 * t117 + f64x8::splat(2.0) * t942 * t172 + f64x8::splat(4.0) * t439 * t945 + f64x8::splat(4.0) * t447 * t957 - f64x8::splat(8.0) * t952 * t953;
            let t961 = t28 * t960;
            let t962 = t961 * t190;
            let t965 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t27 * t962));
            let tvtau0 = t7 * t965;
            acc_vtau_0 = tvtau0;
            let t966 = t234 * t243;
            let t967 = ((t246).select(f64x8::splat(0.0), t966));
            let t968 = param_c1 * t967;
            let t970 = t574 * t967;
            let t972 = -t248 * t970 - t968 * t250;
            let t973 = t972 * t252;
            let t974 = ((t254).select(f64x8::splat(0.0), t966));
            let t976 = t255 * t974;
            let t978 = t257 * t974;
            let t980 = t259 * t974;
            let t982 = t261 * t974;
            let t984 = t263 * t974;
            let t989 = ((t254).select(t966, f64x8::splat(0.0)));
            let t993 = ((t245).select(t973, (t253).select(-f64x8::splat(0.667) * t974 - f64x8::splat(0.889111) * t976 - f64x8::splat(1.989259803147) * t978 + f64x8::splat(5.80518817796) * t980 - f64x8::splat(4.439990207985) * t982 + f64x8::splat(1.407173648874) * t984 - f64x8::splat(0.162300903254) * t265 * t974, -t403 * t595 * t989 * t274)));
            let t1000 = f64x8::splat(1.0) / t209 / t207;
            let t1005 = f64x8::splat(0.162742215233874) * t966 + f64x8::splat(0.118706250165608) * t282 * t234 * t243 - t129 * v_sigma2 * t1000 * t243 / f64x8::splat(24.0);
            let t1006 = t1005 * t293;
            let t1009 = t630 * t234;
            let t1012 = t297 * t238;
            let t1013 = t292 * t1012;
            let t1014 = t298 * t295;
            let t1015 = f64x8::splat(1.0) / t1014;
            let t1016 = t1013 * t1015;
            let t1017 = t645 * t234;
            let t1020 = t234 * t309;
            let t1021 = t659 * t1020;
            let t1024 = f64x8::splat(2.0) * t1006 * t310 + f64x8::splat(4.0) * t629 * t1009 - f64x8::splat(8.0) * t1016 * t1017 + f64x8::splat(4.0) * t637 * t1021 + t993 * t277;
            let t1025 = t28 * t1024;
            let t1026 = t1025 * t325;
            let t1029 = ((t195).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t204 * t1026));
            let tvtau1 = t7 * t1029;
            acc_vtau_1 = tvtau1;
        }
        store_add(zk, ip, m, acc_zk);
        store_strided(vrho, ip, m, 2, 0, acc_vrho_0);
        store_strided(vrho, ip, m, 2, 1, acc_vrho_1);
        store_strided(vsigma, ip, m, 3, 0, acc_vsigma_0);
        store_strided(vsigma, ip, m, 3, 1, acc_vsigma_1);
        store_strided(vsigma, ip, m, 3, 2, acc_vsigma_2);
        store_strided(vlapl, ip, m, 2, 0, acc_vlapl_0);
        store_strided(vlapl, ip, m, 2, 1, acc_vlapl_1);
        store_strided(vtau, ip, m, 2, 0, acc_vtau_0);
        store_strided(vtau, ip, m, 2, 1, acc_vtau_1);
        ip += 8;
    }
}

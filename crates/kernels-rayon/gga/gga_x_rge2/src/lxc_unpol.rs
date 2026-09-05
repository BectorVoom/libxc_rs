//! GGA_X_RGE2 lxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_rge2.c`
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

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_rge2_lxc_unpol(
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
    v4rho4: &mut [f64],
    v4rho3sigma: &mut [f64],
    v4rho2sigma2: &mut [f64],
    v4rhosigma3: &mut [f64],
    v4sigma4: &mut [f64],
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
        let mut acc_v4rho4 = V_ZERO;
        let mut acc_v4rho3sigma = V_ZERO;
        let mut acc_v4rho2sigma2 = V_ZERO;
        let mut acc_v4rhosigma3 = V_ZERO;
        let mut acc_v4sigma4 = V_ZERO;
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
            let t20 = f64x8::splat(M_CBRT6);
            let t21 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t22 = (simd::cbrt(t21));
            let t23 = t22 * t22;
            let t25 = t20 / t23;
            let t26 = f64x8::splat(M_CBRT2);
            let t27 = t26 * t26;
            let t28 = v_sigma * t27;
            let t29 = v_rho * v_rho;
            let t30 = t18 * t18;
            let t32 = f64x8::splat(1.0) / t30 / t29;
            let t36 = t20 * t20;
            let t38 = f64x8::splat(1.0) / t22 / t21;
            let t39 = t36 * t38;
            let t40 = v_sigma * v_sigma;
            let t41 = t40 * t26;
            let t42 = t29 * t29;
            let t43 = t42 * v_rho;
            let t45 = f64x8::splat(1.0) / t18 / t43;
            let t49 = f64x8::splat(0.804) + f64x8::splat(5.0) / f64x8::splat(972.0) * t25 * t28 * t32 + f64x8::splat(6.582356890714508e-05) * t39 * t41 * t45;
            let t52 = f64x8::splat(1.804) - f64x8::splat(0.646416) / t49;
            let t56 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t17 * t18 * t52));
            let tzk0 = f64x8::splat(2.0) * t56;
            acc_zk = tzk0;
            let t57 = f64x8::splat(1.0) / t30;
            let t62 = t3 * t17;
            let t63 = t49 * t49;
            let t64 = f64x8::splat(1.0) / t63;
            let t65 = t18 * t64;
            let t66 = t29 * v_rho;
            let t68 = f64x8::splat(1.0) / t30 / t66;
            let t72 = t42 * t29;
            let t74 = f64x8::splat(1.0) / t18 / t72;
            let t78 = -f64x8::splat(10.0) / f64x8::splat(729.0) * t25 * t28 * t68 - f64x8::splat(0.00035105903417144045) * t39 * t41 * t74;
            let t83 = ((t2).select(f64x8::splat(0.0), -t6 * t17 * t57 * t52 / f64x8::splat(8.0) - f64x8::splat(0.1655109536374632) * t62 * t65 * t78));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t83 + f64x8::splat(2.0) * t56;
            acc_vrho = tvrho0;
            let t89 = v_sigma * t26;
            let t93 = f64x8::splat(5.0) / f64x8::splat(972.0) * t25 * t27 * t32 + f64x8::splat(0.00013164713781429015) * t39 * t89 * t45;
            let t97 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(0.1655109536374632) * t62 * t65 * t93));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t97;
            acc_vsigma = tvsigma0;
            let t101 = f64x8::splat(1.0) / t30 / v_rho;
            let t106 = t57 * t64;
            let t111 = f64x8::splat(1.0) / t63 / t49;
            let t112 = t18 * t111;
            let t113 = t78 * t78;
            let t118 = f64x8::splat(1.0) / t30 / t42;
            let t122 = t42 * t66;
            let t124 = f64x8::splat(1.0) / t18 / t122;
            let t128 = f64x8::splat(110.0) / f64x8::splat(2187.0) * t25 * t28 * t118 + f64x8::splat(0.0022233738830857892) * t39 * t41 * t124;
            let t133 = ((t2).select(f64x8::splat(0.0), t6 * t17 * t101 * t52 / f64x8::splat(12.0) - f64x8::splat(0.1103406357583088) * t62 * t106 * t78 + f64x8::splat(0.3310219072749264) * t62 * t112 * t113 - f64x8::splat(0.1655109536374632) * t62 * t65 * t128));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t133 + f64x8::splat(4.0) * t83;
            acc_v2rho2 = tv2rho20;
            let t139 = t62 * t18;
            let t140 = t111 * t93;
            let t141 = t140 * t78;
            let t150 = -f64x8::splat(10.0) / f64x8::splat(729.0) * t25 * t27 * t68 - f64x8::splat(0.0007021180683428809) * t39 * t89 * t74;
            let t155 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(0.0551703178791544) * t62 * t106 * t93 + f64x8::splat(0.3310219072749264) * t139 * t141 - f64x8::splat(0.1655109536374632) * t62 * t65 * t150));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t155 + f64x8::splat(2.0) * t97;
            acc_v2rhosigma = tv2rhosigma0;
            let t158 = t93 * t93;
            let t162 = f64x8::splat(1.0) / t43;
            let t165 = t38 * t26;
            let t166 = t64 * t36 * t165;
            let t170 = ((t2).select(f64x8::splat(0.0), f64x8::splat(0.3310219072749264) * t62 * t112 * t158 - f64x8::splat(2.1789043323285708e-05) * t62 * t162 * t166));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t170;
            acc_v2sigma2 = tv2sigma20;
            let t177 = t101 * t64;
            let t181 = t57 * t111;
            let t188 = t63 * t63;
            let t189 = f64x8::splat(1.0) / t188;
            let t190 = t18 * t189;
            let t191 = t113 * t78;
            let t195 = t111 * t78;
            let t196 = t195 * t128;
            let t200 = f64x8::splat(1.0) / t30 / t43;
            let t204 = t42 * t42;
            let t206 = f64x8::splat(1.0) / t18 / t204;
            let t210 = -f64x8::splat(1540.0) / f64x8::splat(6561.0) * t25 * t28 * t200 - f64x8::splat(0.016304741809295788) * t39 * t41 * t206;
            let t215 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(5.0) / f64x8::splat(36.0) * t6 * t17 * t32 * t52 + f64x8::splat(0.1103406357583088) * t62 * t177 * t78 + f64x8::splat(0.3310219072749264) * t62 * t181 * t113 - f64x8::splat(0.1655109536374632) * t62 * t106 * t128 - f64x8::splat(0.9930657218247793) * t62 * t190 * t191 + f64x8::splat(0.9930657218247793) * t139 * t196 - f64x8::splat(0.1655109536374632) * t62 * t65 * t210));
            let tv3rho30 = f64x8::splat(2.0) * v_rho * t215 + f64x8::splat(6.0) * t133;
            acc_v3rho3 = tv3rho30;
            let t222 = t62 * t57;
            let t228 = t189 * t93;
            let t229 = t228 * t113;
            let t232 = t111 * t150;
            let t233 = t232 * t78;
            let t236 = t140 * t128;
            let t245 = f64x8::splat(110.0) / f64x8::splat(2187.0) * t25 * t27 * t118 + f64x8::splat(0.0044467477661715785) * t39 * t89 * t124;
            let t250 = ((t2).select(f64x8::splat(0.0), f64x8::splat(0.03678021191943627) * t62 * t177 * t93 + f64x8::splat(0.2206812715166176) * t222 * t141 - f64x8::splat(0.1103406357583088) * t62 * t106 * t150 - f64x8::splat(0.9930657218247793) * t139 * t229 + f64x8::splat(0.6620438145498528) * t139 * t233 + f64x8::splat(0.3310219072749264) * t139 * t236 - f64x8::splat(0.1655109536374632) * t62 * t65 * t245));
            let tv3rho2sigma0 = f64x8::splat(2.0) * v_rho * t250 + f64x8::splat(4.0) * t155;
            acc_v3rho2sigma = tv3rho2sigma0;
            let t256 = t189 * t158;
            let t257 = t256 * t78;
            let t260 = t140 * t150;
            let t263 = f64x8::splat(1.0) / t72;
            let t268 = t62 * t162 * t111;
            let t270 = t39 * t26 * t78;
            let t274 = ((t2).select(f64x8::splat(0.0), f64x8::splat(0.1103406357583088) * t62 * t181 * t158 - f64x8::splat(0.9930657218247793) * t139 * t257 + f64x8::splat(0.6620438145498528) * t139 * t260 + f64x8::splat(0.00010894521661642854) * t62 * t263 * t166 + f64x8::splat(4.3578086646571417e-05) * t268 * t270));
            let tv3rhosigma20 = f64x8::splat(2.0) * v_rho * t274 + f64x8::splat(2.0) * t170;
            acc_v3rhosigma2 = tv3rhosigma20;
            let t277 = t158 * t93;
            let t281 = t93 * t36;
            let t282 = t281 * t165;
            let t286 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(0.9930657218247793) * t62 * t190 * t277 + f64x8::splat(0.00013073425993971426) * t268 * t282));
            let tv3sigma30 = f64x8::splat(2.0) * v_rho * t286;
            acc_v3sigma3 = tv3sigma30;
            let t293 = t32 * t64;
            let t297 = t101 * t111;
            let t304 = t57 * t189;
            let t314 = f64x8::splat(1.0) / t188 / t49;
            let t315 = t18 * t314;
            let t316 = t113 * t113;
            let t324 = t128 * t128;
            let t346 = f64x8::splat(10.0) / f64x8::splat(27.0) * t6 * t17 * t68 * t52 - f64x8::splat(0.24520141279624177) * t62 * t293 * t78 - f64x8::splat(0.4413625430332352) * t62 * t297 * t113 + f64x8::splat(0.2206812715166176) * t62 * t177 * t128 - f64x8::splat(1.3240876290997057) * t62 * t304 * t191 + f64x8::splat(1.3240876290997057) * t222 * t196 - f64x8::splat(0.2206812715166176) * t62 * t106 * t210 + f64x8::splat(3.972262887299117) * t62 * t315 * t316 - f64x8::splat(5.958394330948676) * t139 * t189 * t113 * t128 + f64x8::splat(0.9930657218247793) * t62 * t112 * t324 + f64x8::splat(1.3240876290997057) * t139 * t195 * t210 - f64x8::splat(0.1655109536374632) * t62 * t65 * (f64x8::splat(26180.0) / f64x8::splat(19683.0) * t25 * t28 / t30 / t72 + f64x8::splat(0.13587284841079825) * t39 * t41 / t18 / t204 / v_rho);
            let t347 = ((t2).select(f64x8::splat(0.0), t346));
            let tv4rho40 = f64x8::splat(2.0) * v_rho * t347 + f64x8::splat(8.0) * t215;
            acc_v4rho4 = tv4rho40;
            let t401 = f64x8::splat(0.9930657218247793) * t139 * t111 * t245 * t78 + f64x8::splat(0.9930657218247793) * t139 * t232 * t128 + f64x8::splat(0.3310219072749264) * t139 * t140 * t210 - f64x8::splat(0.1655109536374632) * t62 * t65 * (-f64x8::splat(1540.0) / f64x8::splat(6561.0) * t25 * t27 * t200 - f64x8::splat(0.032609483618591575) * t39 * t89 * t206) + f64x8::splat(0.1103406357583088) * t62 * t177 * t150 - f64x8::splat(0.1655109536374632) * t62 * t106 * t245 - f64x8::splat(0.061300353199060444) * t62 * t293 * t93 - f64x8::splat(0.2206812715166176) * t62 * t101 * t141 + f64x8::splat(0.6620438145498528) * t222 * t233 + f64x8::splat(0.3310219072749264) * t222 * t236 - f64x8::splat(2.979197165474338) * t139 * t189 * t150 * t113 - f64x8::splat(0.9930657218247793) * t222 * t229 + f64x8::splat(3.972262887299117) * t139 * t314 * t93 * t191 - f64x8::splat(2.979197165474338) * t139 * t228 * t78 * t128;
            let t402 = ((t2).select(f64x8::splat(0.0), t401));
            let tv4rho3sigma0 = f64x8::splat(2.0) * v_rho * t402 + f64x8::splat(6.0) * t250;
            acc_v4rho3sigma = tv4rho3sigma0;
            let t424 = t150 * t150;
            let t436 = t62 * t263 * t111;
            let t440 = t62 * t162 * t189;
            let t449 = -f64x8::splat(0.07356042383887254) * t62 * t297 * t158 - f64x8::splat(0.6620438145498528) * t222 * t257 + f64x8::splat(0.4413625430332352) * t222 * t260 + f64x8::splat(3.972262887299117) * t139 * t314 * t158 * t113 - f64x8::splat(3.972262887299117) * t139 * t228 * t78 * t150 - f64x8::splat(0.9930657218247793) * t139 * t256 * t128 + f64x8::splat(0.6620438145498528) * t62 * t112 * t424 + f64x8::splat(0.6620438145498528) * t139 * t140 * t245 - f64x8::splat(0.0006536712996985713) * t62 / t122 * t166 - f64x8::splat(0.00043578086646571415) * t436 * t270 - f64x8::splat(0.00013073425993971426) * t440 * t39 * t26 * t113 + f64x8::splat(4.3578086646571417e-05) * t268 * t39 * t26 * t128;
            let t450 = ((t2).select(f64x8::splat(0.0), t449));
            let tv4rho2sigma20 = f64x8::splat(2.0) * v_rho * t450 + f64x8::splat(4.0) * t274;
            acc_v4rho2sigma2 = tv4rho2sigma20;
            let t474 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(0.3310219072749264) * t62 * t304 * t277 + f64x8::splat(3.972262887299117) * t139 * t314 * t277 * t78 - f64x8::splat(2.979197165474338) * t139 * t256 * t150 - f64x8::splat(0.0006536712996985713) * t436 * t282 - f64x8::splat(0.00039220277981914277) * t440 * t281 * t165 * t78 + f64x8::splat(0.00013073425993971426) * t268 * t150 * t36 * t165));
            let tv4rhosigma30 = f64x8::splat(2.0) * v_rho * t474 + f64x8::splat(2.0) * t286;
            acc_v4rhosigma3 = tv4rhosigma30;
            let t477 = t158 * t158;
            let t490 = t21 * t21;
            let t498 = ((t2).select(f64x8::splat(0.0), f64x8::splat(3.972262887299117) * t62 * t315 * t477 - f64x8::splat(0.0007844055596382855) * t440 * t158 * t36 * t165 + f64x8::splat(1.0326474681199678e-07) * t62 / t18 / t204 / t29 * t111 * t20 / t23 / t490 * t27));
            let tv4sigma40 = f64x8::splat(2.0) * v_rho * t498;
            acc_v4sigma4 = tv4sigma40;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rho2.into(); v2rho2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rhosigma.into(); v2rhosigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2sigma2.into(); v2sigma2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v3rho3.into(); v3rho3[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v3rho2sigma.into(); v3rho2sigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v3rhosigma2.into(); v3rhosigma2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v3sigma3.into(); v3sigma3[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v4rho4.into(); v4rho4[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v4rho3sigma.into(); v4rho3sigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v4rho2sigma2.into(); v4rho2sigma2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v4rhosigma3.into(); v4rhosigma3[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v4sigma4.into(); v4sigma4[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}

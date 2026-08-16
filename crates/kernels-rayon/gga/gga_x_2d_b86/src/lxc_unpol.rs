//! GGA_X_2D_B86 lxc unpol kernel (rayon backend).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_2d_b86.c`.
//! Preserves exact maple2c variable names and FP operation order.
//! Mechanically converted from the CubeCL form by tools/translate_rayon/xform.py.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};

#[allow(unused_variables, non_snake_case)]
pub fn gga_x_2d_b86_lxc_unpol(
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
    for ip in 0..zk.len() {
        let t2 = rho[ip] / 2.0 <= dens_threshold;
        let t3 = f64::sqrt(M_PI);
        let t5 = 1.0 <= zeta_threshold;
        let t6 = zeta_threshold - 1.0;
        let t8 = piecewise5(t5, t6, t5, -t6, 0.0);
        let t9 = 1.0 + t8;
        let t11 = f64::sqrt(zeta_threshold);
        let t13 = f64::sqrt(t9);
        let t15 = piecewise3(t9 <= zeta_threshold, t11 * zeta_threshold, t13 * t9);
        let t17 = M_SQRT2;
        let t18 = 1.0 / t3 * t15 * t17;
        let t19 = f64::sqrt(rho[ip]);
        let t20 = rho[ip] * rho[ip];
        let t21 = t20 * rho[ip];
        let t23 = sigma[ip] / t21;
        let t25 = 1.0 + 0.421e-2 * t23;
        let t28 = 1.0 + 0.238e-3 * t23;
        let t29 = 1.0 / t28;
        let t33 = piecewise3(t2, 0.0, -2.0 / 3.0 * t18 * t19 * t25 * t29);
        let tzk0 = 2.0 * t33;
        zk[ip] += tzk0;
        let t39 = t15 * t17;
        let t41 = 1.0 / t19 / t21;
        let t47 = t28 * t28;
        let t48 = 1.0 / t47;
        let t50 = t25 * t48 * sigma[ip];
        let t54 = piecewise3(t2, 0.0, -t18 / t19 * t25 * t29 / 3.0 + 0.47504762934721079361e-2 * t39 * t41 * sigma[ip] * t29 - 0.26855424176873199259e-3 * t39 * t41 * t50);
        let tvrho0 = 2.0 * rho[ip] * t54 + 2.0 * t33;
        vrho[ip] += tvrho0;
        let t58 = 1.0 / t19 / t20;
        let t62 = t58 * t25;
        let t67 = piecewise3(t2, 0.0, -0.15834920978240359787e-2 * t39 * t58 * t29 + 0.8951808058957733086e-4 * t39 * t62 * t48);
        let tvsigma0 = 2.0 * rho[ip] * t67;
        vsigma[ip] += tvsigma0;
        let t76 = t20 * t20;
        let t78 = 1.0 / t19 / t76;
        let t86 = t76 * t21;
        let t88 = 1.0 / t19 / t86;
        let t89 = sigma[ip] * sigma[ip];
        let t94 = t39 * t88;
        let t96 = 1.0 / t47 / t28;
        let t97 = t25 * t96;
        let t98 = t97 * t89;
        let t102 = piecewise3(t2, 0.0, t18 / t19 / rho[ip] * t25 * t29 / 6.0 - 0.14251428880416323808e-1 * t39 * t78 * sigma[ip] * t29 + 0.80566272530619597777e-3 * t39 * t78 * t50 + 0.67836801470781701328e-5 * t39 * t88 * t89 * t48 - 0.38349545724574928542e-6 * t94 * t98);
        let tv2rho20 = 2.0 * rho[ip] * t102 + 4.0 * t54;
        v2rho2[ip] += tv2rho20;
        let t108 = t76 * t20;
        let t110 = 1.0 / t19 / t108;
        let t111 = t110 * t48;
        let t115 = t41 * t25;
        let t119 = t39 * t110;
        let t120 = t97 * sigma[ip];
        let t124 = piecewise3(t2, 0.0, 0.39587302445600899468e-2 * t39 * t41 * t29 - 0.22612267156927233776e-5 * t39 * t111 * sigma[ip] - 0.22379520147394332715e-3 * t39 * t115 * t48 + 0.12783181908191642847e-6 * t119 * t120);
        let tv2rhosigma0 = 2.0 * rho[ip] * t124 + 2.0 * t67;
        v2rhosigma[ip] += tv2rhosigma0;
        let t127 = t76 * rho[ip];
        let t129 = 1.0 / t19 / t127;
        let t133 = t129 * t25;
        let t138 = piecewise3(t2, 0.0, 0.75374223856424112585e-6 * t39 * t129 * t48 - 0.42610606360638809489e-7 * t39 * t133 * t96);
        let tv2sigma20 = 2.0 * rho[ip] * t138;
        v2sigma2[ip] += tv2sigma20;
        let t151 = t76 * t76;
        let t153 = 1.0 / t19 / t151;
        let t158 = t39 * t153;
        let t163 = 1.0 / t19 / t151 / t21;
        let t164 = t89 * sigma[ip];
        let t169 = t39 * t163;
        let t170 = t47 * t47;
        let t171 = 1.0 / t170;
        let t172 = t25 * t171;
        let t173 = t172 * t164;
        let t177 = piecewise3(t2, 0.0, -t18 * t62 * t29 / 4.0 + 0.62943810888505430152e-1 * t39 * t129 * sigma[ip] * t29 - 0.35583437034356989019e-2 * t39 * t129 * t50 - 0.71228641544320786394e-4 * t39 * t153 * t89 * t48 + 0.40267023010803674969e-5 * t158 * t98 + 0.14530642875041440424e-7 * t39 * t163 * t164 * t96 - 0.82144726942039496937e-9 * t169 * t173);
        let tv3rho30 = 2.0 * rho[ip] * t177 + 6.0 * t102;
        v3rho3[ip] += tv3rho30;
        let t184 = t88 * t48;
        let t190 = 1.0 / t19 / t151 / t20;
        let t191 = t190 * t96;
        let t201 = t39 * t190;
        let t202 = t172 * t89;
        let t206 = piecewise3(t2, 0.0, -0.13855555855960314814e-1 * t39 * t78 * t29 + 0.20351040441234510398e-4 * t39 * t184 * sigma[ip] - 0.48435476250138134748e-8 * t39 * t191 * t89 + 0.78328320515880164502e-3 * t39 * t78 * t25 * t48 - 0.11504863717372478562e-5 * t94 * t120 + 0.27381575647346498978e-9 * t201 * t202);
        let tv3rho2sigma0 = 2.0 * rho[ip] * t206 + 4.0 * t124;
        v3rho2sigma[ip] += tv3rho2sigma0;
        let t213 = 1.0 / t19 / t151 / rho[ip];
        let t214 = t213 * t96;
        let t222 = t39 * t213;
        let t223 = t172 * sigma[ip];
        let t227 = piecewise3(t2, 0.0, -0.41455823121033261922e-5 * t39 * t111 + 0.16145158750046044916e-8 * t39 * t214 * sigma[ip] + 0.23435833498351345219e-6 * t39 * t110 * t25 * t96 - 0.91271918824488329925e-10 * t222 * t223);
        let tv3rhosigma20 = 2.0 * rho[ip] * t227 + 2.0 * t138;
        v3rhosigma2[ip] += tv3rhosigma20;
        let t238 = piecewise3(t2, 0.0, -0.53817195833486816385e-9 * t39 * t153 * t96 + 0.30423972941496109975e-10 * t39 * t153 * t25 * t171);
        let tv3sigma30 = 2.0 * rho[ip] * t238;
        v3sigma3[ip] += tv3sigma30;
        let t258 = 1.0 / t19 / t151 / t76;
        let t263 = t39 * t258;
        let t268 = 1.0 / t19 / t151 / t86;
        let t269 = t89 * t89;
        let t276 = 1.0 / t170 / t28;
        let t277 = t25 * t276;
        let t282 = piecewise3(t2, 0.0, 5.0 / 8.0 * t18 * t115 * t29 - 0.34440953127672782536e0 * t39 * t110 * sigma[ip] * t29 + 0.19470182528233069463e-1 * t119 * t50 + 0.69532721507551243861e-3 * t39 * t213 * t89 * t48 - 0.39308284367689301756e-4 * t222 * t98 - 0.31967414325091168934e-6 * t39 * t258 * t164 * t96 + 0.18071839927248689326e-7 * t263 * t173 + 0.41499516051118353851e-10 * t39 * t268 * t269 * t171 - 0.23460534014646480325e-11 * t39 * t268 * t277 * t269);
        let tv4rho40 = 2.0 * rho[ip] * t282 + 8.0 * t177;
        v4rho4[ip] += tv4rho40;
        let t299 = 1.0 / t19 / t151 / t108;
        let t316 = piecewise3(t2, 0.0, 0.62350001351821416663e-1 * t39 * t129 * t29 - 0.17241853707157015753e-3 * t39 * t153 * t48 * sigma[ip] + 0.94449178687769362757e-7 * t39 * t163 * t96 * t89 - 0.13833172017039451284e-10 * t39 * t299 * t171 * t164 - 0.35247744232146074026e-2 * t39 * t133 * t48 + 0.97471762049961276706e-5 * t158 * t120 - 0.53394072512325673007e-8 * t169 * t202 + 0.78201780048821601081e-12 * t39 * t299 * t277 * t164);
        let tv4rho3sigma0 = 2.0 * rho[ip] * t316 + 6.0 * t206;
        v4rho3sigma[ip] += tv4rho3sigma0;
        let t327 = 1.0 / t19 / t151 / t127;
        let t343 = piecewise3(t2, 0.0, 0.26946285028671620249e-4 * t39 * t184 - 0.24217738125069067374e-7 * t39 * t191 * sigma[ip] + 0.4611057339013150428e-11 * t39 * t327 * t171 * t89 - 0.15233291773928374392e-5 * t39 * t88 * t25 * t96 + 0.13690787823673249489e-8 * t201 * t223 - 0.26067260016273867027e-12 * t39 * t327 * t277 * t89);
        let tv4rho2sigma20 = 2.0 * rho[ip] * t343 + 4.0 * t227;
        v4rho2sigma2[ip] += tv4rho2sigma20;
        let t360 = piecewise3(t2, 0.0, 0.45744616458463793927e-8 * t39 * t214 - 0.1537019113004383476e-11 * t39 * t258 * t171 * sigma[ip] - 0.25860377000271693479e-9 * t39 * t213 * t25 * t171 + 0.86890866720912890089e-13 * t263 * t277 * sigma[ip]);
        let tv4rhosigma30 = 2.0 * rho[ip] * t360 + 2.0 * t238;
        v4rhosigma3[ip] += tv4rhosigma30;
        let t371 = piecewise3(t2, 0.0, 0.51233970433479449198e-12 * t39 * t163 * t171 - 0.28963622240304296696e-13 * t39 * t163 * t25 * t276);
        let tv4sigma40 = 2.0 * rho[ip] * t371;
        v4sigma4[ip] += tv4sigma40;
    }
}

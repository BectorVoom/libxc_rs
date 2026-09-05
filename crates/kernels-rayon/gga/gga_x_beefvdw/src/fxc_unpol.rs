//! GGA_X_BEEFVDW fxc unpol kernel — explicit SIMD (bit-exact).
//!
//! Auto-translated from `libxc-master/src/maple2c/gga_exc/gga_x_beefvdw.c`
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
pub fn gga_x_beefvdw_fxc_unpol(
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
            let t6 = t3 / t4;
            let t7 = (f64x8::splat(1.0)).simd_le(zeta_threshold);
            let t8 = zeta_threshold - f64x8::splat(1.0);
            let t10 = ((t7).select(t8, (t7).select(-t8, f64x8::splat(0.0))));
            let t11 = t10 + f64x8::splat(1.0);
            let t13 = (simd::cbrt(zeta_threshold));
            let t15 = (simd::cbrt(t11));
            let t17 = (((t11).simd_le(zeta_threshold)).select(t13 * zeta_threshold, t15 * t11));
            let t18 = (simd::cbrt(v_rho));
            let t19 = t17 * t18;
            let t20 = f64x8::splat(M_CBRT6);
            let t21 = f64x8::splat(M_PI) * f64x8::splat(M_PI);
            let t22 = (simd::cbrt(t21));
            let t23 = t22 * t22;
            let t24 = f64x8::splat(1.0) / t23;
            let t25 = t20 * t24;
            let t26 = t25 * v_sigma;
            let t27 = f64x8::splat(M_CBRT2);
            let t28 = t27 * t27;
            let t29 = v_rho * v_rho;
            let t30 = t18 * t18;
            let t32 = f64x8::splat(1.0) / t30 / t29;
            let t38 = f64x8::splat(4.0) + t25 * v_sigma * t28 * t32 / f64x8::splat(24.0);
            let t39 = f64x8::splat(1.0) / t38;
            let t40 = t28 * t32 * t39;
            let t41 = t26 * t40;
            let t43 = t41 / f64x8::splat(12.0) - f64x8::splat(1.0);
            let t44 = t43 * t43;
            let t45 = t44 * t44;
            let t46 = t45 * t45;
            let t47 = t46 * t45;
            let t48 = t46 * t46;
            let t49 = t48 * t47;
            let t51 = t45 * t43;
            let t52 = t46 * t51;
            let t55 = t44 * t43;
            let t56 = t46 * t55;
            let t57 = t48 * t56;
            let t59 = t46 * t44;
            let t60 = t48 * t59;
            let t62 = t46 * t43;
            let t63 = t48 * t62;
            let t65 = t45 * t55;
            let t66 = t48 * t65;
            let t68 = t48 * t46;
            let t70 = t45 * t44;
            let t71 = t48 * t70;
            let t78 = t48 * t44;
            let t81 = -f64x8::splat(5427.777462637186) * t49 + f64x8::splat(4135.586188014654) * t48 * t52 - f64x8::splat(29150.193011493262) * t57 + f64x8::splat(40074.93585443239) * t60 + f64x8::splat(90365.6111085228) * t63 - f64x8::splat(161142.1539984628) * t66 - f64x8::splat(132044.6618218215) * t68 + f64x8::splat(255894.79526235335) * t71 - f64x8::splat(0.6945973517763898) * t45 + f64x8::splat(0.527556201155898) * t55 - f64x8::splat(0.38916037779196816) * t44 + f64x8::splat(86.00573049927964) * t65 + f64x8::splat(30.54203495931585) * t70 + f64x8::splat(279670.48856303055) * t78 + f64x8::splat(0.037534251004296526) * t41;
            let t88 = t46 * t70;
            let t91 = t48 * t45;
            let t93 = t48 * t51;
            let t95 = t48 * t55;
            let t97 = t48 * t43;
            let t99 = t46 * t65;
            let t102 = f64x8::splat(1.1313514630621233) - f64x8::splat(7.2975787893717134) * t51 + f64x8::splat(3783.53964072524) * t59 - f64x8::splat(617.547861045286) * t62 - f64x8::splat(442.33229018433804) * t46 - f64x8::splat(20148.24517562505) * t47 + f64x8::splat(2274.8997850816486) * t56 + f64x8::splat(70504.54186903402) * t88 - f64x8::splat(2810.240180568463) * t52 - f64x8::splat(323524.0313604933) * t91 + f64x8::splat(180782.00670879145) * t93 - f64x8::splat(129814.81812794984) * t95 + f64x8::splat(56174.00797937267) * t97 - f64x8::splat(10276.426607863825) * t99 - f64x8::splat(168370.8413901412) * t48;
            let t103 = t81 + t102;
            let t107 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t103));
            let tzk0 = f64x8::splat(2.0) * t107;
            acc_zk = tzk0;
            let t109 = t17 / t30;
            let t113 = t29 * v_rho;
            let t115 = f64x8::splat(1.0) / t30 / t113;
            let t117 = t28 * t115 * t39;
            let t118 = t26 * t117;
            let t120 = t20 * t20;
            let t122 = f64x8::splat(1.0) / t22 / t21;
            let t123 = t120 * t122;
            let t124 = v_sigma * v_sigma;
            let t125 = t123 * t124;
            let t126 = t29 * t29;
            let t127 = t126 * t29;
            let t129 = f64x8::splat(1.0) / t18 / t127;
            let t131 = t38 * t38;
            let t132 = f64x8::splat(1.0) / t131;
            let t133 = t27 * t129 * t132;
            let t134 = t125 * t133;
            let t136 = -f64x8::splat(2.0) / f64x8::splat(9.0) * t118 + t134 / f64x8::splat(54.0);
            let t167 = -f64x8::splat(6470480.6272098655) * t95 * t136 + f64x8::splat(3796422.1408846206) * t91 * t136 - f64x8::splat(2466481.544431047) * t78 * t136 + f64x8::splat(954958.1356493353) * t48 * t136 + f64x8::splat(5034068.79413455) * t97 * t136 - f64x8::splat(154146.39911795736) * t88 * t136 - f64x8::splat(2693933.462242259) * t99 * t136 - f64x8::splat(36533.12234739002) * t47 * t136 + f64x8::splat(987063.5861664761) * t52 * t136 + f64x8::splat(25023.897635898134) * t59 * t136 - f64x8::splat(241778.94210750057) * t56 * t136 - f64x8::splat(3538.6583214747043) * t65 * t136 - f64x8::splat(5557.930749407574) * t46 * t136 + f64x8::splat(37835.3964072524) * t62 * t136 - f64x8::splat(36.48789394685857) * t45 * t136;
            let t196 = f64x8::splat(183.2522097558951) * t51 * t136 + f64x8::splat(602.0401134949575) * t70 * t136 - f64x8::splat(0.7783207555839363) * t43 * t136 + f64x8::splat(1.582668603467694) * t44 * t136 - f64x8::splat(2.7783894071055593) * t55 * t136 - f64x8::splat(151977.7689538412) * t57 * t136 + f64x8::splat(119931.99945242496) * t49 * t136 - f64x8::splat(787055.2113103181) * t60 * t136 + f64x8::splat(1041948.3322152421) * t63 * t136 + f64x8::splat(2259140.27771307) * t68 * t136 - f64x8::splat(3706269.5419646446) * t71 * t136 - f64x8::splat(3169071.8837237163) * t66 * t136 + f64x8::splat(5629685.495771773) * t93 * t136 - f64x8::splat(0.10009133601145741) * t118 + f64x8::splat(0.00834094466762145) * t134;
            let t197 = t167 + t196;
            let t202 = ((t2).select(f64x8::splat(0.0), -t6 * t109 * t103 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t197));
            let tvrho0 = f64x8::splat(2.0) * v_rho * t202 + f64x8::splat(2.0) * t107;
            acc_vrho = tvrho0;
            let t205 = t25 * t40;
            let t209 = t126 * v_rho;
            let t213 = t27 / t18 / t209 * t132;
            let t214 = t123 * v_sigma * t213;
            let t216 = t205 / f64x8::splat(12.0) - t214 / f64x8::splat(144.0);
            let t217 = t47 * t216;
            let t219 = t52 * t216;
            let t221 = t59 * t216;
            let t223 = t56 * t216;
            let t225 = t65 * t216;
            let t227 = t46 * t216;
            let t229 = t62 * t216;
            let t231 = t45 * t216;
            let t233 = t51 * t216;
            let t235 = t70 * t216;
            let t237 = t43 * t216;
            let t239 = t44 * t216;
            let t241 = t55 * t216;
            let t243 = t57 * t216;
            let t245 = f64x8::splat(0.037534251004296526) * t205 - f64x8::splat(36533.12234739002) * t217 + f64x8::splat(987063.5861664761) * t219 + f64x8::splat(25023.897635898134) * t221 - f64x8::splat(241778.94210750057) * t223 - f64x8::splat(3538.6583214747043) * t225 - f64x8::splat(5557.930749407574) * t227 + f64x8::splat(37835.3964072524) * t229 - f64x8::splat(36.48789394685857) * t231 + f64x8::splat(183.2522097558951) * t233 + f64x8::splat(602.0401134949575) * t235 - f64x8::splat(0.7783207555839363) * t237 + f64x8::splat(1.582668603467694) * t239 - f64x8::splat(2.7783894071055593) * t241 - f64x8::splat(151977.7689538412) * t243;
            let t248 = t60 * t216;
            let t250 = t63 * t216;
            let t252 = t68 * t216;
            let t254 = t71 * t216;
            let t256 = t66 * t216;
            let t258 = t93 * t216;
            let t260 = t95 * t216;
            let t262 = t91 * t216;
            let t264 = t78 * t216;
            let t266 = t48 * t216;
            let t268 = t97 * t216;
            let t270 = t88 * t216;
            let t272 = t99 * t216;
            let t275 = f64x8::splat(119931.99945242496) * t49 * t216 - f64x8::splat(787055.2113103181) * t248 + f64x8::splat(1041948.3322152421) * t250 + f64x8::splat(2259140.27771307) * t252 - f64x8::splat(3706269.5419646446) * t254 - f64x8::splat(3169071.8837237163) * t256 + f64x8::splat(5629685.495771773) * t258 - f64x8::splat(6470480.6272098655) * t260 + f64x8::splat(3796422.1408846206) * t262 - f64x8::splat(2466481.544431047) * t264 + f64x8::splat(954958.1356493353) * t266 + f64x8::splat(5034068.79413455) * t268 - f64x8::splat(154146.39911795736) * t270 - f64x8::splat(2693933.462242259) * t272 - f64x8::splat(0.003127854250358044) * t214;
            let t276 = t245 + t275;
            let t280 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t276));
            let tvsigma0 = f64x8::splat(2.0) * v_rho * t280;
            acc_vsigma = tvsigma0;
            let t285 = t17 / t30 / v_rho;
            let t295 = t28 / t30 / t126 * t39;
            let t296 = t26 * t295;
            let t298 = t126 * t113;
            let t300 = f64x8::splat(1.0) / t18 / t298;
            let t302 = t27 * t300 * t132;
            let t303 = t125 * t302;
            let t305 = t136 * t136;
            let t309 = t21 * t21;
            let t310 = f64x8::splat(1.0) / t309;
            let t312 = t310 * t124 * v_sigma;
            let t313 = t126 * t126;
            let t314 = t313 * t29;
            let t315 = f64x8::splat(1.0) / t314;
            let t317 = f64x8::splat(1.0) / t131 / t38;
            let t319 = t312 * t315 * t317;
            let t321 = f64x8::splat(22.0) / f64x8::splat(27.0) * t296 - t303 / f64x8::splat(6.0) + f64x8::splat(4.0) / f64x8::splat(81.0) * t319;
            let t324 = t60 * t321;
            let t326 = t63 * t321;
            let t328 = t68 * t321;
            let t330 = t71 * t321;
            let t332 = t66 * t321;
            let t334 = t93 * t321;
            let t336 = t59 * t321;
            let t338 = t56 * t321;
            let t340 = t65 * t321;
            let t342 = t46 * t321;
            let t344 = f64x8::splat(0.36700156537534384) * t296 - f64x8::splat(0.07506850200859305) * t303 - f64x8::splat(0.7783207555839363) * t305 + f64x8::splat(119931.99945242496) * t49 * t321 - f64x8::splat(787055.2113103181) * t324 + f64x8::splat(1041948.3322152421) * t326 + f64x8::splat(2259140.27771307) * t328 - f64x8::splat(3706269.5419646446) * t330 - f64x8::splat(3169071.8837237163) * t332 + f64x8::splat(5629685.495771773) * t334 + f64x8::splat(25023.897635898134) * t336 - f64x8::splat(241778.94210750057) * t338 - f64x8::splat(3538.6583214747043) * t340 - f64x8::splat(5557.930749407574) * t342;
            let t345 = t62 * t321;
            let t347 = t45 * t321;
            let t349 = t51 * t321;
            let t351 = t70 * t321;
            let t353 = t43 * t321;
            let t355 = t44 * t321;
            let t357 = t55 * t321;
            let t359 = t57 * t321;
            let t361 = t95 * t321;
            let t363 = t91 * t321;
            let t365 = t78 * t321;
            let t367 = t48 * t321;
            let t369 = t97 * t321;
            let t371 = t88 * t321;
            let t373 = t99 * t321;
            let t375 = f64x8::splat(37835.3964072524) * t345 - f64x8::splat(36.48789394685857) * t347 + f64x8::splat(183.2522097558951) * t349 + f64x8::splat(602.0401134949575) * t351 - f64x8::splat(0.7783207555839363) * t353 + f64x8::splat(1.582668603467694) * t355 - f64x8::splat(2.7783894071055593) * t357 - f64x8::splat(151977.7689538412) * t359 - f64x8::splat(6470480.6272098655) * t361 + f64x8::splat(3796422.1408846206) * t363 - f64x8::splat(2466481.544431047) * t365 + f64x8::splat(954958.1356493353) * t367 + f64x8::splat(5034068.79413455) * t369 - f64x8::splat(154146.39911795736) * t371 - f64x8::splat(2693933.462242259) * t373;
            let t377 = t47 * t321;
            let t379 = t52 * t321;
            let t381 = t97 * t305;
            let t407 = -f64x8::splat(36533.12234739002) * t377 + f64x8::splat(987063.5861664761) * t379 - f64x8::splat(44396667.799758844) * t381 + f64x8::splat(15279330.170389365) * t99 * t305 + f64x8::splat(85579169.50028734) * t48 * t305 - f64x8::splat(2158049.587651403) * t52 * t305 - f64x8::splat(40409001.93363389) * t88 * t305 - f64x8::splat(438397.4681686802) * t56 * t305 + f64x8::splat(12831826.62016419) * t47 * t305 + f64x8::splat(250238.97635898134) * t62 * t305 - f64x8::splat(2659568.363182506) * t59 * t305 - f64x8::splat(24770.60825032293) * t70 * t305 - f64x8::splat(44463.44599526059) * t65 * t305 + f64x8::splat(340518.56766527164) * t46 * t305 - f64x8::splat(145.95157578743428) * t55 * t305;
            let t408 = t45 * t305;
            let t412 = t43 * t305;
            let t416 = t60 * t305;
            let t420 = t63 * t305;
            let t422 = t68 * t305;
            let t424 = t66 * t305;
            let t426 = t93 * t305;
            let t428 = t71 * t305;
            let t430 = t91 * t305;
            let t432 = t78 * t305;
            let t434 = t95 * t305;
            let t437 = f64x8::splat(916.2610487794755) * t408 + f64x8::splat(3612.240680969745) * t51 * t305 + f64x8::splat(3.165337206935388) * t412 - f64x8::splat(8.335168221316678) * t44 * t305 - f64x8::splat(4103399.761753713) * t416 + f64x8::splat(3358095.984667899) * t57 * t305 - f64x8::splat(20463435.49406827) * t420 + f64x8::splat(26048708.305381052) * t422 + f64x8::splat(54219366.66511369) * t424 - f64x8::splat(81537929.92322218) * t426 - f64x8::splat(72888653.32564548) * t428 + f64x8::splat(118223395.41120724) * t430 - f64x8::splat(122939131.91698745) * t432 + f64x8::splat(75928442.81769241) * t434 + f64x8::splat(0.0222425191136572) * t319;
            let t439 = t344 + t375 + t407 + t437;
            let t444 = ((t2).select(f64x8::splat(0.0), t6 * t285 * t103 / f64x8::splat(12.0) - t6 * t109 * t197 / f64x8::splat(4.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t439));
            let tv2rho20 = f64x8::splat(2.0) * v_rho * t444 + f64x8::splat(4.0) * t202;
            acc_v2rho2 = tv2rho20;
            let t478 = f64x8::splat(26048708.305381052) * t252 * t136 + f64x8::splat(54219366.66511369) * t256 * t136 - f64x8::splat(81537929.92322218) * t258 * t136 - f64x8::splat(72888653.32564548) * t254 * t136 + f64x8::splat(118223395.41120724) * t262 * t136 - f64x8::splat(122939131.91698745) * t264 * t136 + f64x8::splat(75928442.81769241) * t260 * t136 - f64x8::splat(44396667.799758844) * t268 * t136 + f64x8::splat(15279330.170389365) * t272 * t136 + f64x8::splat(85579169.50028734) * t266 * t136 - f64x8::splat(2158049.587651403) * t219 * t136 - f64x8::splat(40409001.93363389) * t270 * t136 - f64x8::splat(438397.4681686802) * t223 * t136 + f64x8::splat(12831826.62016419) * t217 * t136;
            let t505 = t25 * t117;
            let t507 = t123 * t27;
            let t510 = t507 * t129 * t132 * v_sigma;
            let t512 = f64x8::splat(250238.97635898134) * t229 * t136 - f64x8::splat(2659568.363182506) * t221 * t136 - f64x8::splat(24770.60825032293) * t235 * t136 - f64x8::splat(44463.44599526059) * t225 * t136 + f64x8::splat(340518.56766527164) * t227 * t136 - f64x8::splat(145.95157578743428) * t241 * t136 + f64x8::splat(916.2610487794755) * t231 * t136 + f64x8::splat(3612.240680969745) * t233 * t136 + f64x8::splat(3.165337206935388) * t237 * t136 - f64x8::splat(8.335168221316678) * t239 * t136 - f64x8::splat(4103399.761753713) * t248 * t136 + f64x8::splat(3358095.984667899) * t243 * t136 - f64x8::splat(20463435.49406827) * t250 * t136 - f64x8::splat(0.10009133601145741) * t505 + f64x8::splat(0.025022834002864352) * t510;
            let t516 = t310 * t124;
            let t517 = t313 * v_rho;
            let t518 = f64x8::splat(1.0) / t517;
            let t520 = t516 * t518 * t317;
            let t522 = -f64x8::splat(2.0) / f64x8::splat(9.0) * t505 + t510 / f64x8::splat(18.0) - t520 / f64x8::splat(54.0);
            let t523 = t95 * t522;
            let t525 = t91 * t522;
            let t527 = t78 * t522;
            let t529 = t48 * t522;
            let t531 = t97 * t522;
            let t533 = t88 * t522;
            let t535 = t99 * t522;
            let t537 = t70 * t522;
            let t539 = t136 * t216;
            let t541 = t43 * t522;
            let t543 = t44 * t522;
            let t545 = t55 * t522;
            let t547 = t57 * t522;
            let t551 = t60 * t522;
            let t553 = -f64x8::splat(6470480.6272098655) * t523 + f64x8::splat(3796422.1408846206) * t525 - f64x8::splat(2466481.544431047) * t527 + f64x8::splat(954958.1356493353) * t529 + f64x8::splat(5034068.79413455) * t531 - f64x8::splat(154146.39911795736) * t533 - f64x8::splat(2693933.462242259) * t535 + f64x8::splat(602.0401134949575) * t537 - f64x8::splat(0.7783207555839363) * t539 - f64x8::splat(0.7783207555839363) * t541 + f64x8::splat(1.582668603467694) * t543 - f64x8::splat(2.7783894071055593) * t545 - f64x8::splat(151977.7689538412) * t547 + f64x8::splat(119931.99945242496) * t49 * t522 - f64x8::splat(787055.2113103181) * t551;
            let t554 = t63 * t522;
            let t556 = t68 * t522;
            let t558 = t71 * t522;
            let t560 = t66 * t522;
            let t562 = t93 * t522;
            let t564 = t47 * t522;
            let t566 = t52 * t522;
            let t568 = t59 * t522;
            let t570 = t56 * t522;
            let t572 = t65 * t522;
            let t574 = t46 * t522;
            let t576 = t62 * t522;
            let t578 = t45 * t522;
            let t580 = t51 * t522;
            let t583 = f64x8::splat(1041948.3322152421) * t554 + f64x8::splat(2259140.27771307) * t556 - f64x8::splat(3706269.5419646446) * t558 - f64x8::splat(3169071.8837237163) * t560 + f64x8::splat(5629685.495771773) * t562 - f64x8::splat(36533.12234739002) * t564 + f64x8::splat(987063.5861664761) * t566 + f64x8::splat(25023.897635898134) * t568 - f64x8::splat(241778.94210750057) * t570 - f64x8::splat(3538.6583214747043) * t572 - f64x8::splat(5557.930749407574) * t574 + f64x8::splat(37835.3964072524) * t576 - f64x8::splat(36.48789394685857) * t578 + f64x8::splat(183.2522097558951) * t580 - f64x8::splat(0.00834094466762145) * t520;
            let t585 = t478 + t512 + t553 + t583;
            let t590 = ((t2).select(f64x8::splat(0.0), -t6 * t109 * t276 / f64x8::splat(8.0) - f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t585));
            let tv2rhosigma0 = f64x8::splat(2.0) * v_rho * t590 + f64x8::splat(2.0) * t280;
            acc_v2rhosigma = tv2rhosigma0;
            let t593 = t216 * t216;
            let t594 = t91 * t593;
            let t596 = t78 * t593;
            let t598 = t95 * t593;
            let t600 = t97 * t593;
            let t602 = t99 * t593;
            let t604 = t48 * t593;
            let t606 = t52 * t593;
            let t608 = t88 * t593;
            let t610 = t56 * t593;
            let t612 = t123 * t213;
            let t614 = t310 * v_sigma;
            let t615 = f64x8::splat(1.0) / t313;
            let t617 = t614 * t615 * t317;
            let t619 = -t612 / f64x8::splat(72.0) + t617 / f64x8::splat(144.0);
            let t620 = t71 * t619;
            let t622 = t66 * t619;
            let t624 = t93 * t619;
            let t626 = t95 * t619;
            let t628 = t91 * t619;
            let t630 = f64x8::splat(118223395.41120724) * t594 - f64x8::splat(122939131.91698745) * t596 + f64x8::splat(75928442.81769241) * t598 - f64x8::splat(44396667.799758844) * t600 + f64x8::splat(15279330.170389365) * t602 + f64x8::splat(85579169.50028734) * t604 - f64x8::splat(2158049.587651403) * t606 - f64x8::splat(40409001.93363389) * t608 - f64x8::splat(438397.4681686802) * t610 - f64x8::splat(3706269.5419646446) * t620 - f64x8::splat(3169071.8837237163) * t622 + f64x8::splat(5629685.495771773) * t624 - f64x8::splat(6470480.6272098655) * t626 + f64x8::splat(3796422.1408846206) * t628;
            let t631 = t78 * t619;
            let t633 = t48 * t619;
            let t635 = t97 * t619;
            let t637 = t88 * t619;
            let t639 = t99 * t619;
            let t641 = t62 * t619;
            let t643 = t45 * t619;
            let t645 = t51 * t619;
            let t647 = t70 * t619;
            let t649 = t43 * t619;
            let t651 = t44 * t619;
            let t653 = t55 * t619;
            let t655 = t57 * t619;
            let t659 = t60 * t619;
            let t661 = -f64x8::splat(2466481.544431047) * t631 + f64x8::splat(954958.1356493353) * t633 + f64x8::splat(5034068.79413455) * t635 - f64x8::splat(154146.39911795736) * t637 - f64x8::splat(2693933.462242259) * t639 + f64x8::splat(37835.3964072524) * t641 - f64x8::splat(36.48789394685857) * t643 + f64x8::splat(183.2522097558951) * t645 + f64x8::splat(602.0401134949575) * t647 - f64x8::splat(0.7783207555839363) * t649 + f64x8::splat(1.582668603467694) * t651 - f64x8::splat(2.7783894071055593) * t653 - f64x8::splat(151977.7689538412) * t655 + f64x8::splat(119931.99945242496) * t49 * t619 - f64x8::splat(787055.2113103181) * t659;
            let t663 = t63 * t619;
            let t665 = t68 * t619;
            let t667 = t47 * t619;
            let t669 = t52 * t619;
            let t671 = t59 * t619;
            let t673 = t56 * t619;
            let t675 = t65 * t619;
            let t677 = t46 * t619;
            let t679 = t47 * t593;
            let t681 = t62 * t593;
            let t683 = t59 * t593;
            let t685 = t70 * t593;
            let t687 = t65 * t593;
            let t689 = t46 * t593;
            let t691 = f64x8::splat(1041948.3322152421) * t663 + f64x8::splat(2259140.27771307) * t665 - f64x8::splat(36533.12234739002) * t667 + f64x8::splat(987063.5861664761) * t669 + f64x8::splat(25023.897635898134) * t671 - f64x8::splat(241778.94210750057) * t673 - f64x8::splat(3538.6583214747043) * t675 - f64x8::splat(5557.930749407574) * t677 + f64x8::splat(12831826.62016419) * t679 + f64x8::splat(250238.97635898134) * t681 - f64x8::splat(2659568.363182506) * t683 - f64x8::splat(24770.60825032293) * t685 - f64x8::splat(44463.44599526059) * t687 + f64x8::splat(340518.56766527164) * t689;
            let t692 = t55 * t593;
            let t694 = t45 * t593;
            let t696 = t51 * t593;
            let t698 = t43 * t593;
            let t700 = t44 * t593;
            let t702 = t60 * t593;
            let t706 = t63 * t593;
            let t708 = t68 * t593;
            let t710 = t66 * t593;
            let t712 = t93 * t593;
            let t714 = t71 * t593;
            let t719 = -f64x8::splat(145.95157578743428) * t692 + f64x8::splat(916.2610487794755) * t694 + f64x8::splat(3612.240680969745) * t696 + f64x8::splat(3.165337206935388) * t698 - f64x8::splat(8.335168221316678) * t700 - f64x8::splat(4103399.761753713) * t702 + f64x8::splat(3358095.984667899) * t57 * t593 - f64x8::splat(20463435.49406827) * t706 + f64x8::splat(26048708.305381052) * t708 + f64x8::splat(54219366.66511369) * t710 - f64x8::splat(81537929.92322218) * t712 - f64x8::splat(72888653.32564548) * t714 - f64x8::splat(0.7783207555839363) * t593 + f64x8::splat(0.003127854250358044) * t617 - f64x8::splat(0.006255708500716088) * t612;
            let t721 = t630 + t661 + t691 + t719;
            let t725 = ((t2).select(f64x8::splat(0.0), -f64x8::splat(3.0) / f64x8::splat(8.0) * t6 * t19 * t721));
            let tv2sigma20 = f64x8::splat(2.0) * v_rho * t725;
            acc_v2sigma2 = tv2sigma20;
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rho2.into(); v2rho2[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2rhosigma.into(); v2rhosigma[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_v2sigma2.into(); v2sigma2[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}

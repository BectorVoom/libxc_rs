//! GGA_X_BEEFVDW vxc unpol kernel — explicit SIMD (bit-exact).
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
pub fn gga_x_beefvdw_vxc_unpol(
    rho: &[f64],
    sigma: &[f64],
    zk: &mut [f64],
    vrho: &mut [f64],
    vsigma: &mut [f64],
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
        }
        { let a: [f64; 8] = acc_zk.into(); zk[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vrho.into(); vrho[ip..ip + m].copy_from_slice(&a[..m]); }
        { let a: [f64; 8] = acc_vsigma.into(); vsigma[ip..ip + m].copy_from_slice(&a[..m]); }
        ip += 8;
    }
}

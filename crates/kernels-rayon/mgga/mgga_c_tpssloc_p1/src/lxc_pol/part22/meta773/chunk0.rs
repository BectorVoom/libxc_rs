//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2644/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2644(t28: f64, t265: f64, t504: f64, t68418: f64, t71222: f64, t71252: f64, t72059: f64, t72074: f64, t72077: f64, t72078: f64, t72099: f64, t73931: f64, t1081: f64, t1260: f64, t1409: f64, t1534: f64, t1649: f64, t16558: f64, t17133: f64, t1768: f64, t18196: f64, t19276: f64, t20217: f64, t20390: f64, t21076: f64, t22414: f64, t3966: f64, t4324: f64, t506: f64, t5099: f64, t52: f64, t5398: f64, t5966: f64, t607: f64, t6279: f64, t67060: f64, t68427: f64, t71090: f64, t873: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> f64 {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t73935 = piecewise3(t505, t71222 + t71252 + t72059 + t72074 + t72077 + t72078 + t72099 + t73931, t68418);
    let t73953 = piecewise3(t401, t68418 * t28 / 2.0_f64 + t21076 * t1081 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t17133 * t1649 - t68427 + 3.0_f64 / 2.0_f64 * t4324 * t5966 + 3.0_f64 / 2.0_f64 * t1534 * t18196 + t873 * t20390 / 2.0_f64 + t265 * t71090 / 2.0_f64, t73935 * t52 / 2.0_f64 - t22414 * t607 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t19276 * t1409 - 3.0_f64 / 2.0_f64 * t6279 * t3966 - 3.0_f64 / 2.0_f64 * t5099 * t5398 - 3.0_f64 / 2.0_f64 * t1768 * t16558 - t1260 * t20217 / 2.0_f64 - t506 * t67060 / 2.0_f64);
    t73953
}

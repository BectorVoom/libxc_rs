//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2500/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2500(t25: f64, t265: f64, t394: f64, t68418: f64, t68765: f64, t68897: f64, t68931: f64, t68999: f64, t69031: f64, t69462: f64, t69464: f64, t71055: f64, t1074: f64, t1408: f64, t1409: f64, t1534: f64, t1642: f64, t16557: f64, t16558: f64, t17133: f64, t18176: f64, t20216: f64, t20217: f64, t21076: f64, t21703: f64, t396: f64, t3966: f64, t40: f64, t4324: f64, t4705: f64, t5397: f64, t5398: f64, t5955: f64, t606: f64, t607: f64, t67059: f64, t67060: f64, t68427: f64, t873: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> f64 {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t71059 = piecewise3(t395, t68765 + t68897 + t68931 + t68999 + t69031 + t69462 + t69464 + t71055, t68418);
    let t71077 = piecewise3(t115, t68418 * t25 / 2.0_f64 + t21076 * t606 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t17133 * t1408 + t68427 + 3.0_f64 / 2.0_f64 * t4324 * t5397 + 3.0_f64 / 2.0_f64 * t1534 * t16557 + t873 * t20216 / 2.0_f64 + t265 * t67059 / 2.0_f64, t71059 * t40 / 2.0_f64 + t21703 * t607 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t18176 * t1409 + 3.0_f64 / 2.0_f64 * t5955 * t3966 + 3.0_f64 / 2.0_f64 * t4705 * t5398 + 3.0_f64 / 2.0_f64 * t1642 * t16558 + t1074 * t20217 / 2.0_f64 + t396 * t67060 / 2.0_f64);
    t71077
}

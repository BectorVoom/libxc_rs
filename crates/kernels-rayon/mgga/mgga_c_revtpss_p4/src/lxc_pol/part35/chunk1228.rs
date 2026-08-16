//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1228/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1228(t30: f64, t265: f64, t393: f64, t115819: f64, t115462: f64, t115763: f64, t1469: f64, t2078: f64, t22671: f64, t30463: f64, t45: f64, t5825: f64, t8040: f64, t102888: f64, t103586: f64, t110177: f64, t113123: f64, t114101: f64, t114104: f64, t114121: f64, t114165: f64, t114171: f64, t114184: f64, t114192: f64, t114196: f64, t1711: f64, t1940: f64, t2071: f64, t2082: f64, t2403: f64, t26425: f64, t26590: f64, t28291: f64, t28472: f64, t29946: f64, t29953: f64, t29964: f64, t30420: f64, t6416: f64, t7432: f64, t7869: f64, t8020: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t115820 = piecewise3(t394, 0.0_f64, t115819);
    let t115830 = piecewise3(t120, t115462 + t115763, t115820 * t45 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t30463 * t1469 + 3.0_f64 / 2.0_f64 * t8040 * t5825 + t2078 * t22671 / 2.0_f64);
    let t115870 = 9.0_f64 * t26425 * t114101 - 9.0_f64 * t26425 * t114104 - 9.0_f64 * t102888 * t29946 - 9.0_f64 * t28291 * t114165 + 3.0_f64 * t28472 * t114196 + 3.0_f64 / 2.0_f64 * t1940 * t8020 * t6416 - 3.0_f64 / 2.0_f64 * t1940 * t7432 * t114184 + 3.0_f64 / 2.0_f64 * t2403 * t2071 * t114171 + 3.0_f64 * t113123 * t2082 + 3.0_f64 * t1940 * t26590 * t114121 + 9.0_f64 / 2.0_f64 * t2403 * t2071 * t114192 + 9.0_f64 / 2.0_f64 * t2403 * t8020 * t29953 - 3.0_f64 / 2.0_f64 * t1940 * t110177 * t7869 + 3.0_f64 / 2.0_f64 * t1940 * t30420 * t1711 + 3.0_f64 * t1940 * t103586 * t29964;
    (t115830, t115870)
}

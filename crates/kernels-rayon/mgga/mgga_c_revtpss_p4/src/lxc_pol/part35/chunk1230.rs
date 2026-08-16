//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1230/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1230(t33: f64, t265: f64, t502: f64, t115819: f64, t115870: f64, t115913: f64, t1469: f64, t2085: f64, t22671: f64, t30503: f64, t57: f64, t5825: f64, t8059: f64, t114378: f64, t114452: f64, t114800: f64, t114812: f64, t114820: f64, t114905: f64, t115358: f64, t115830: f64, t118: f64, t1502: f64, t18245: f64, t1843: f64, t2014: f64, t2056: f64, t22483: f64, t25082: f64, t26405: f64, t28167: f64, t29508: f64, t30138: f64, t30315: f64, t30511: f64, t30558: f64, t30578: f64, t30584: f64, t30589: f64, t4248: f64, t508: f64, t651: f64, t7732: f64, t7898: f64, t7978: f64, t7984: f64, t7988: f64, t8108: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> f64 {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t115915 = piecewise3(t503, 0.0_f64, t115819);
    let t115925 = piecewise3(t400, t115870 + t115913, t115915 * t57 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t30503 * t1469 - 3.0_f64 / 2.0_f64 * t8059 * t5825 - t2085 * t22671 / 2.0_f64);
    let t115962 = -6.0_f64 * t18245 * t7984 - 6.0_f64 * t114378 * t2056 - 12.0_f64 * t30138 * t7978 - 9.0_f64 * t25082 * t26405 * t114800 - t118 * (t115830 + t115925) - 3.0_f64 * t1502 * t30511 - 18.0_f64 * t28167 * t26405 * t114452 - 9.0_f64 * t25082 * t26405 * t114820 - 3.0_f64 * t7898 * t30584 + 3.0_f64 * t7898 * t30315 - 3.0_f64 * t2014 * t8108 * t22483 - 2.0_f64 * t114812 * t2056 - 6.0_f64 * t29508 * t7978 - 2.0_f64 * t651 * t508 * t114905 - 6.0_f64 * t18245 * t7988 - 12.0_f64 * t4248 * t30578 - 6.0_f64 * t115358 * t508 - 6.0_f64 * t30589 * t1843 - 6.0_f64 * t4248 * t30558 - 6.0_f64 * t7732 * t30558;
    t115962
}

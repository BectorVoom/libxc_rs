//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1328/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1328(t114226: f64, t1799: f64, t22633: f64, t22635: f64, t120213: f64, t120218: f64, t120221: f64, t120226: f64, t120229: f64, t120232: f64, t120239: f64, t120244: f64, t120247: f64, t120248: f64, t1386: f64, t16022: f64, t16439: f64, t31217: f64, t32758: f64, t3882: f64, t5215: f64, t8476: f64, t8486: f64) -> f64 {
    let t120253 = 0.3289868133696452873e-1_f64 * t22633 * t22635 * t114226 * t1799;
    let t120254 = -t120248 * t1386 + 2.0_f64 * t16022 * t8476 + 2.0_f64 * t16439 * t8476 - t16439 * t8486 - t31217 * t5215 - t32758 * t3882 + t120213 - t120218 - t120221 + t120226 + t120229 - t120232 - t120239 - t120244 + t120247 + t120253;
    t120254
}

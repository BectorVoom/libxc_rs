//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1880/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1880(t13087: f64, t13182: f64, t13234: f64, t16848: f64, t16877: f64, t16879: f64, t20882: f64, t20887: f64, t20891: f64, t20896: f64, t20958: f64, t20998: f64, t21011: f64, t2643: f64, t843: f64) -> f64 {
    let t21013 = -35.0_f64 / 72.0_f64 * t13087 - 119.0_f64 / 4608.0_f64 * t13182 + t2643 * t20882 / 256.0_f64 + t2643 * t20887 / 256.0_f64 - t2643 * t20891 / 1024.0_f64 - 7.0_f64 / 192.0_f64 * t16848 - 5.0_f64 / 128.0_f64 * t843 * t20896 + 119.0_f64 / 4608.0_f64 * t13234 + 7.0_f64 / 768.0_f64 * t16877 - 7.0_f64 / 768.0_f64 * t16879 + t20958 + t20998 + t21011;
    t21013
}

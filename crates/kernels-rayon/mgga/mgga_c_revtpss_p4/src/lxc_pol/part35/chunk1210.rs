//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1210/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1210(t101785: f64, t101929: f64, t109926: f64, t114260: f64, t114270: f64, t114288: f64, t114296: f64, t114301: f64, t2048: f64, t26175: f64, t28602: f64, t29538: f64, t29544: f64, t29548: f64, t29562: f64, t30543: f64, t7343: f64, t7706: f64, t7709: f64, t7964: f64) -> f64 {
    let t115348 = -2.0_f64 * t7709 * t30543 + 88.0_f64 / 9.0_f64 * t101929 + 30.0_f64 * t101785 * t29562 + 30.0_f64 * t26175 * t114260 - 5.0_f64 * t109926 * t7706 - 10.0_f64 * t28602 * t29544 - 5.0_f64 * t28602 * t29548 - 2.0_f64 * t114270 * t2048 - 2.0_f64 * t114296 * t2048 - 4.0_f64 * t29538 * t7964 - 5.0_f64 * t7343 * t114288 - 5.0_f64 * t7343 * t114301;
    t115348
}

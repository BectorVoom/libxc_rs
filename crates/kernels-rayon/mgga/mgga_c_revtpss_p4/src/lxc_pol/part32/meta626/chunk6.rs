//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1996/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1996(t60673: f64, t7342: f64, t101243: f64, t101935: f64, t101938: f64, t108762: f64, t108769: f64, t108792: f64, t108864: f64, t2048: f64, t26175: f64, t28133: f64, t28141: f64, t28154: f64, t28602: f64, t28628: f64, t29562: f64, t30543: f64, t6960: f64, t6963: f64, t7343: f64, t7964: f64, t95276: f64) -> f64 {
    let t109926 = t60673 * t7342;
    let t109945 = 20.0_f64 / 3.0_f64 * t101243 * t28628 + 20.0_f64 / 3.0_f64 * t28154 * t101935 + 20.0_f64 / 3.0_f64 * t28154 * t101938 - 5.0_f64 / 3.0_f64 * t109926 * t6960 - 2.0_f64 / 3.0_f64 * t108769 * t2048 - 10.0_f64 / 3.0_f64 * t28602 * t28133 - 4.0_f64 / 3.0_f64 * t28141 * t7964 - 5.0_f64 / 3.0_f64 * t7343 * t108792 - 2.0_f64 / 3.0_f64 * t6963 * t30543 + 10.0_f64 * t95276 * t29562 + 10.0_f64 * t26175 * t108864 - 2.0_f64 / 3.0_f64 * t108762 * t2048;
    t109945
}

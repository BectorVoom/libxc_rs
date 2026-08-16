//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 742/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk742(t13187: f64, t17104: f64, t17360: f64, t17362: f64, t17422: f64, t1901: f64, t20875: f64, t20880: f64, t20884: f64, t20888: f64, t20894: f64, t20899: f64, t20904: f64, t20909: f64, t20912: f64, t20916: f64, t446: f64) -> f64 {
    let t20919 = -2.0_f64 / 3.0_f64 * t1901 * t20875 - 2.0_f64 / 9.0_f64 * t17104 - 2.0_f64 / 3.0_f64 * t446 * t20880 + 2.0_f64 / 3.0_f64 * t446 * t20884 - 2.0_f64 * t446 * t20888 + t17360 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t17362 + 2.0_f64 * t446 * t20894 - 2.0_f64 * t446 * t20899 + 2.0_f64 * t446 * t20904 + 2.0_f64 / 3.0_f64 * t17422 - t446 * t20909 - t446 * t20912 / 3.0_f64 - t446 * t20916 - 4.0_f64 / 9.0_f64 * t13187;
    t20919
}

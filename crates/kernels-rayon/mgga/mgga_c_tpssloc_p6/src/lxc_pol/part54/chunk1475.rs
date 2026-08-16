//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1475/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1475(t122667: f64, t122671: f64, t122678: f64, t122681: f64, t122692: f64, t122696: f64, t2114: f64, t2165: f64, t26114: f64, t26179: f64, t26870: f64, t26967: f64, t32318: f64, t32365: f64, t4028: f64, t7156: f64, t7264: f64, t7458: f64, t7890: f64, t7983: f64, t8835: f64) -> f64 {
    let t125017 = -t2114 * t26870 - t2165 * t26967 - 2.0_f64 * t26114 * t8835 - 2.0_f64 * t26179 * t8835 - 2.0_f64 * t32318 * t7458 - 2.0_f64 * t32365 * t4028 - t7156 * t7983 - t7264 * t7890 + t122667 + t122671 + t122678 - t122681 - t122692 + t122696;
    t125017
}

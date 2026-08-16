//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1094/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1094(t28307: f64, t22542: f64, t822: f64, t20671: f64, t22629: f64, t10007: f64, t1865: f64, t825: f64, t9438: f64, t10012: f64, t2684: f64, t22623: f64, t7427: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t28308 = 0.76685851907841499352e0_f64 * t28307;
    let t28309 = t822 * t22542;
    let t28312 = 0.34082600847929555268e0_f64 * t28309 * t20671 * t22629;
    let t28357 = t825 * t9438 * t10007 * t1865;
    let t28361 = t2684 * t9438 * t10012 * t1865;
    let t28366 = t7427 * t9438 * t22623 * t1865;
    (t28308, t28309, t28312, t28357, t28361, t28366)
}

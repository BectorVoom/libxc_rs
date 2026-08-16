//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1005/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1005(t31608: f64, t6883: f64, t1377: f64, t7213: f64, t1307: f64, t22633: f64, t22635: f64, t1992: f64, t31558: f64, t3911: f64, t22716: f64, t8622: f64) -> (f64, f64, f64, f64) {
    let t115294 = t6883 * t31608;
    let t115296 = t1377 * t7213;
    let t115299 = t22633 * t22635 * t115296 * t1307;
    let t115303 = t1992 * t22635 * t31558 * t3911;
    let t115305 = t22716 * t8622;
    (t115294, t115299, t115303, t115305)
}

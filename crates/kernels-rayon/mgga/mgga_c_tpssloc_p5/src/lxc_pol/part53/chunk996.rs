//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 996/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk996(t115352: f64, t6897: f64, t7700: f64, t1377: f64, t7936: f64, t1307: f64, t22633: f64, t22635: f64, t1992: f64, t31558: f64, t5353: f64, t33310: f64, t6883: f64) -> (f64, f64, f64, f64) {
    let t122121 = t6897 * t115352 * t7700;
    let t122124 = t1377 * t7936;
    let t122127 = t22633 * t22635 * t122124 * t1307;
    let t122131 = t1992 * t22635 * t31558 * t5353;
    let t122133 = t6883 * t33310;
    (t122121, t122127, t122131, t122133)
}

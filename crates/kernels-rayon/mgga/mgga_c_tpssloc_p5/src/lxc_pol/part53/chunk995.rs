//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 995/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk995(t33273: f64, t81159: f64, t115545: f64, t22633: f64, t26215: f64, t33272: f64, t80650: f64, t33250: f64, t6914: f64, t115614: f64, t1842: f64, t1992: f64, t22635: f64) -> (f64, f64, f64, f64, f64) {
    let t122102 = t81159 * t33273;
    let t122107 = t22633 * t115545 * t26215;
    let t122110 = t22633 * t80650 * t33272;
    let t122112 = t6914 * t33250;
    let t122117 = t1992 * t22635 * t115614 * t1842;
    (t122102, t122107, t122110, t122112, t122117)
}

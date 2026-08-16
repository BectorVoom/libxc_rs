//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 980/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk980(t115296: f64, t1307: f64, t22633: f64, t22635: f64, t1992: f64, t31558: f64, t3911: f64, t22716: f64, t8622: f64, t6897: f64, t80645: f64, t8621: f64) -> (f64, f64, f64, f64) {
    let t115299 = t22633 * t22635 * t115296 * t1307;
    let t115303 = t1992 * t22635 * t31558 * t3911;
    let t115305 = t22716 * t8622;
    let t115306 = 0.63969658155208805863e-1_f64 * t115305;
    let t115308 = t6897 * t80645 * t8621;
    (t115299, t115303, t115306, t115308)
}

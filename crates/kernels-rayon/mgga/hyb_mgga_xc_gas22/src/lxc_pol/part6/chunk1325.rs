//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1325/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1325(t28903: f64, t8669: f64, t20703: f64, t20706: f64, t20714: f64, t24556: f64, t24559: f64, t24562: f64, t28853: f64, t28856: f64, t28859: f64, t796: f64) -> (f64, f64, f64) {
    let t28907 = t8669 * t28903;
    let t28916 = t20714 - 56.0_f64 / 27.0_f64 * t20703 + 4.0_f64 / 9.0_f64 * t20706 - 56.0_f64 / 27.0_f64 * t24556 + 16.0_f64 / 9.0_f64 * t24559 - 2.0_f64 / 3.0_f64 * t24562 + 4.0_f64 / 9.0_f64 * t28859 - 2.0_f64 / 3.0_f64 * t28853 + t28856;
    let t28917 = t796 * t28916;
    (t28907, t28916, t28917)
}

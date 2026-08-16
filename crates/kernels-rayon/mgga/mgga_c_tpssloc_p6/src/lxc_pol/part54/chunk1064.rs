//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1064/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1064(t1484: f64, t1530: f64, t16596: f64, t1877: f64, t193: f64, t202: f64, t2057: f64, t24339: f64, t24344: f64, t2522: f64, t25365: f64, t25374: f64, t26739: f64, t26744: f64, t4119: f64, t4255: f64, t4303: f64, t4314: f64, t7110: f64, t7114: f64, t776: f64, t7845: f64, t868: f64, t870: f64) -> f64 {
    let t26806 = t193 * t202 * t26739 * t870 + 3.0_f64 * t1484 * t2522 * t7110 - t1530 * t1877 * t24339 - 3.0_f64 * t16596 * t2522 * t7114 + 2.0_f64 * t1877 * t24344 * t25374 - t1877 * t26744 * t868 - t1877 * t4303 * t7114 + 3.0_f64 * t2057 * t2522 * t4119 + 6.0_f64 * t2057 * t4255 * t4314 - 3.0_f64 * t2522 * t25365 * t7114 + 3.0_f64 * t2522 * t776 * t7845;
    t26806
}

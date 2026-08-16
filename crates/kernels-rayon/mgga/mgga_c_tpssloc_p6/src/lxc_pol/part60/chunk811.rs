//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 811/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk811(t2057: f64, t5527: f64, t1484: f64, t1530: f64, t1877: f64, t193: f64, t202: f64, t24344: f64, t2522: f64, t26744: f64, t28248: f64, t29105: f64, t4314: f64, t5544: f64, t5660: f64, t5664: f64, t7114: f64, t7845: f64, t870: f64) -> f64 {
    let t29125 = t2057 * t5527;
    let t29148 = t193 * t202 * t29105 * t870 + 6.0_f64 * t1484 * t2522 * t7845 - 2.0_f64 * t1530 * t1877 * t26744 + 2.0_f64 * t1877 * t24344 * t5664 - t1877 * t5660 * t7114 + 3.0_f64 * t2057 * t2522 * t5544 - 6.0_f64 * t2522 * t28248 * t7114 + 6.0_f64 * t29125 * t4314;
    t29148
}

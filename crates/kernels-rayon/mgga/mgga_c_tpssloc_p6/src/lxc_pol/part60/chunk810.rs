//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 810/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk810(t29075: f64, t29104: f64, t870: f64, t1408: f64, t1877: f64, t2057: f64, t24191: f64, t24344: f64, t25: f64, t2522: f64, t26744: f64, t28249: f64, t28252: f64, t28256: f64, t28456: f64, t28459: f64, t28462: f64, t28972: f64, t4314: f64, t5397: f64, t7114: f64, t7475: f64, t7545: f64, t7845: f64) -> (f64, f64, f64) {
    let t29105 = t29075 + t29104;
    let t29106 = t29105 * t870;
    let t29124 = 3.0_f64 * t4314 * t28972 + 3.0_f64 * t2522 * t7845 * t7475 - 3.0_f64 * t24191 * t28249 + 3.0_f64 * t2522 * t2057 * t28252 + 3.0_f64 / 2.0_f64 * t2522 * t2057 * t28256 + t1877 * t29106 * t25 / 2.0_f64 - t1877 * t26744 * t7545 + t1877 * t7845 * t1408 + t1877 * t24344 * t28456 - t1877 * t7114 * t28459 - t1877 * t7114 * t28462 / 2.0_f64 + t1877 * t2057 * t5397 / 2.0_f64;
    (t29105, t29106, t29124)
}

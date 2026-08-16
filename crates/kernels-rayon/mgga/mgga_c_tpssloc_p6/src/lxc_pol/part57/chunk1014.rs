//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 1014/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk1014(t115262: f64, t1983: f64, t28826: f64, t128393: f64, t128397: f64, t128401: f64, t128404: f64, t128406: f64, t128413: f64, t128415: f64, t128418: f64, t128420: f64, t128422: f64, t2036: f64, t2039: f64, t24999: f64, t28811: f64, t29211: f64, t33133: f64, t33204: f64, t6517: f64, t652: f64, t7458: f64, t7670: f64, t7787: f64, t7806: f64, t7943: f64) -> f64 {
    let t128429 = 6.0_f64 * t1983 * t115262 * t28826;
    let t128433 = -2.0_f64 * t2039 * t28811 * t652 - t2036 * t28811 - 4.0_f64 * t24999 * t7806 - 2.0_f64 * t29211 * t6517 - 2.0_f64 * t33133 * t7943 - 4.0_f64 * t33204 * t7458 - 2.0_f64 * t7670 * t7787 + t128393 + t128397 - t128401 - t128404 - t128406 - t128413 - t128415 - t128418 - t128420 - t128422 + t128429;
    t128433
}

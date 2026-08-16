//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 814/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk814(t113: f64, t1442: f64, t1459: f64, t1774: f64, t1849: f64, t1983: f64, t2075: f64, t2096: f64, t27188: f64, t28821: f64, t28943: f64, t28952: f64, t28959: f64, t28969: f64, t29197: f64, t29201: f64, t29205: f64, t4028: f64, t510: f64, t5450: f64, t5457: f64, t5460: f64, t5494: f64, t652: f64, t7042: f64, t7458: f64, t7685: f64, t7787: f64, t7802: f64, t7806: f64, t7890: f64, t7900: f64, t7941: f64) -> f64 {
    let t29210 = -4.0_f64 * t27188 * t1459 - 2.0_f64 * t7042 * t5494 - t28943 * t510 + 2.0_f64 * t7685 * t7941 - 2.0_f64 * t652 * t28952 - 4.0_f64 * t4028 * t7806 - 4.0_f64 * t7042 * t5460 - 2.0_f64 * t28959 * t510 - t5450 * t2075 - 2.0_f64 * t1442 * t7890 - 2.0_f64 * t7787 * t1774 + 2.0_f64 * t7900 * t1849 + 3.0_f64 * t1983 * t28969 - t113 * t29197 - 2.0_f64 * t5457 * t2075 - 2.0_f64 * t1983 * t29201 + t28821 * t2096 - 4.0_f64 * t652 * t29205 - 4.0_f64 * t7458 * t7802;
    t29210
}

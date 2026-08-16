//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 814/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk814<F: Float>(t113: F, t1442: F, t1459: F, t1774: F, t1849: F, t1983: F, t2075: F, t2096: F, t27188: F, t28821: F, t28943: F, t28952: F, t28959: F, t28969: F, t29197: F, t29201: F, t29205: F, t4028: F, t510: F, t5450: F, t5457: F, t5460: F, t5494: F, t652: F, t7042: F, t7458: F, t7685: F, t7787: F, t7802: F, t7806: F, t7890: F, t7900: F, t7941: F) -> F {
    let t29210 = -F::cast_from(4.0_f64) * t27188 * t1459 - F::cast_from(2.0_f64) * t7042 * t5494 - t28943 * t510 + F::cast_from(2.0_f64) * t7685 * t7941 - F::cast_from(2.0_f64) * t652 * t28952 - F::cast_from(4.0_f64) * t4028 * t7806 - F::cast_from(4.0_f64) * t7042 * t5460 - F::cast_from(2.0_f64) * t28959 * t510 - t5450 * t2075 - F::cast_from(2.0_f64) * t1442 * t7890 - F::cast_from(2.0_f64) * t7787 * t1774 + F::cast_from(2.0_f64) * t7900 * t1849 + F::cast_from(3.0_f64) * t1983 * t28969 - t113 * t29197 - F::cast_from(2.0_f64) * t5457 * t2075 - F::cast_from(2.0_f64) * t1983 * t29201 + t28821 * t2096 - F::cast_from(4.0_f64) * t652 * t29205 - F::cast_from(4.0_f64) * t7458 * t7802;
    t29210
}

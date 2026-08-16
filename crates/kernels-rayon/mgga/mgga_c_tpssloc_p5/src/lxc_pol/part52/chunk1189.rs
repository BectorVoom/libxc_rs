//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1189/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1189(t1266: f64, t1393: f64, t1976: f64, t2114: f64, t2165: f64, t31078: f64, t31080: f64, t31082: f64, t31088: f64, t31089: f64, t31223: f64, t31877: f64, t31892: f64, t510: f64, t574: f64, t6515: f64, t6862: f64, t7264: f64, t8667: f64, t8687: f64) -> f64 {
    let t31895 = -t1266 * t8667 + t1393 * t8687 - t1976 * t7264 - t2114 * t6862 - t2165 * t6515 - t31877 * t510 + t31892 * t574 - 2.0_f64 * t31078 - 2.0_f64 * t31080 - 2.0_f64 * t31082 - t31088 + t31089 + t31223;
    t31895
}

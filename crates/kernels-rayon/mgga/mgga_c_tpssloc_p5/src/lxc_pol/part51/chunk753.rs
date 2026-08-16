//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 753/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk753(t2019: f64, t6999: f64, t1983: f64, t113: f64, t1266: f64, t1393: f64, t1869: f64, t1976: f64, t1980: f64, t510: f64, t574: f64, t650: f64, t6515: f64, t6517: f64, t652: f64, t6522: f64, t6524: f64, t6527: f64, t6537: f64, t6539: f64, t672: f64, t6862: f64, t6872: f64, t6877: f64, t6882: f64, t6998: f64) -> (f64, f64) {
    let t7000 = t2019 * t6999;
    let t7001 = t1983 * t7000;
    let t7002 = -t113 * t6862 - t1266 * t1869 + t1393 * t1980 - t1976 * t650 - t510 * t6515 + t574 * t6872 - 2.0_f64 * t6517 * t672 - 2.0_f64 * t652 * t6539 - t6522 - t6524 - t6527 - t6537 + t6877 + t6882 + t6998 - t7001;
    (t7000, t7002)
}

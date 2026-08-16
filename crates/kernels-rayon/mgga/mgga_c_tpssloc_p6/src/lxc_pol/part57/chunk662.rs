//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 662/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk662(t533: f64, t8639: f64, t1390: f64, t1983: f64, t2018: f64, t3701: f64, t2095: f64, t1869: f64, t1976: f64, t2036: f64, t2040: f64, t2075: f64, t2096: f64, t510: f64, t574: f64, t6517: f64, t652: f64, t8329: f64, t8450: f64, t8519: f64, t8522: f64, t8528: f64, t8529: f64, t8535: f64, t8596: f64, t8604: f64, t8608: f64) -> (f64, f64, f64, f64, f64) {
    let t8640 = t533 * t8639;
    let t8641 = t8640 * t1390;
    let t8642 = t1983 * t8641;
    let t8643 = t3701 * t2018;
    let t8644 = t2095 * t8643;
    let t8645 = t1983 * t8644;
    let t8646 = -t1869 * t2075 - t1976 * t2036 - 2.0_f64 * t2040 * t6517 + t2096 * t8450 - t510 * t8519 + t574 * t8604 - 2.0_f64 * t652 * t8529 - t8329 - t8522 - t8528 - t8535 - t8596 + t8608 + t8642 - t8645;
    (t8640, t8641, t8643, t8644, t8646)
}

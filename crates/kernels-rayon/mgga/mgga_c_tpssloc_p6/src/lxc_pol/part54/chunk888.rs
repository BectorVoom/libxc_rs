//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 888/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk888(t2039: f64, t7266: f64, t8446: f64, t8598: f64, t8603: f64, t8829: f64, t2036: f64, t2040: f64, t2075: f64, t2096: f64, t2114: f64, t2165: f64, t510: f64, t574: f64, t652: f64, t8329: f64, t8522: f64, t8528: f64, t8535: f64, t8596: f64, t8608: f64, t8642: f64, t8645: f64, t8690: f64, t8835: f64) -> (f64, f64) {
    let t8840 = 2.0_f64 * t2039 * t7266 + t8446 + t8598 + t8603 + t8829;
    let t8843 = -t2036 * t2165 - 2.0_f64 * t2040 * t7266 - t2075 * t2114 + t2096 * t8690 - t510 * t8829 + t574 * t8840 - 2.0_f64 * t652 * t8835 - t8329 - t8522 - t8528 - t8535 - t8596 + t8608 + t8642 - t8645;
    (t8840, t8843)
}

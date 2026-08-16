//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1196/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1196(t1266: f64, t31055: f64, t31057: f64, t31060: f64, t31671: f64, t31733: f64, t31737: f64, t31746: f64, t31749: f64, t31750: f64, t32349: f64, t510: f64, t7171: f64, t7220: f64, t8329: f64, t8690: f64, t8829: f64) -> f64 {
    let t32378 = -t1266 * t8829 - t32349 * t510 + 3.0_f64 * t7171 * t8690 - t7220 * t8690 - t31055 - t31057 - t31060 + t31671 - t31733 + t31737 - t31746 - t31749 - t31750 - t8329;
    t32378
}

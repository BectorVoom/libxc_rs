//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1993/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1993(t100638: f64, t100641: f64, t100644: f64, t100656: f64, t100669: f64, t100696: f64, t100731: f64, t100769: f64, t100791: f64, t101833: f64, t101840: f64, t1081: f64, t1877: f64, t24191: f64, t24344: f64, t25928: f64, t25930: f64, t26563: f64, t26744: f64, t26756: f64, t28: f64, t28771: f64, t29106: f64, t7114: f64, t84797: f64) -> f64 {
    let t101981 = 2.0_f64 * t101840 * t25928 - 3.0_f64 * t24191 * t100769 - 3.0_f64 / 2.0_f64 * t24191 * t100731 + t1877 * t101833 * t28 / 2.0_f64 - 3.0_f64 * t26563 * t100638 - 3.0_f64 * t24191 * t100656 + t26756 * t100644 + t1877 * t24344 * t100669 - 3.0_f64 * t84797 * t28771 + t1877 * t29106 * t1081 / 2.0_f64 - t1877 * t7114 * t100696 + 3.0_f64 * t26563 * t100641 + 3.0_f64 * t24191 * t100791 - t1877 * t26744 * t25930;
    t101981
}

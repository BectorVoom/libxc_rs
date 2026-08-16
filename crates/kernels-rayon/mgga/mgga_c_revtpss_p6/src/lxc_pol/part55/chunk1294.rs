//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1294/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1294(t128937: f64, t128945: f64, t128959: f64, t128960: f64, t128964: f64, t128965: f64, t128966: f64, t27060: f64, t28760: f64, t28932: f64, t29427: f64, t29432: f64, t29456: f64, t32822: f64, t7359: f64, t7378: f64, t7586: f64, t7978: f64, t8109: f64, t8764: f64) -> f64 {
    let t131080 = -2.0_f64 * t27060 * t7978 - 2.0_f64 * t28760 * t7586 + 3.0_f64 * t28932 * t8764 - 2.0_f64 * t29427 * t7378 - 2.0_f64 * t29432 * t7978 - 2.0_f64 * t29456 * t7359 + t32822 * t8109 + t128937 + t128945 + t128959 + t128960 + t128964 + t128965 + t128966;
    t131080
}

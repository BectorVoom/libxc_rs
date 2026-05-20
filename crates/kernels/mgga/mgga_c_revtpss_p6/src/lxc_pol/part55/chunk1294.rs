//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1294/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1294<F: Float>(t128937: F, t128945: F, t128959: F, t128960: F, t128964: F, t128965: F, t128966: F, t27060: F, t28760: F, t28932: F, t29427: F, t29432: F, t29456: F, t32822: F, t7359: F, t7378: F, t7586: F, t7978: F, t8109: F, t8764: F) -> F {
    let t131080 = -F::new(2.0) * t27060 * t7978 - F::new(2.0) * t28760 * t7586 + F::new(3.0) * t28932 * t8764 - F::new(2.0) * t29427 * t7378 - F::new(2.0) * t29432 * t7978 - F::new(2.0) * t29456 * t7359 + t32822 * t8109 + t128937 + t128945 + t128959 + t128960 + t128964 + t128965 + t128966;
    t131080
}

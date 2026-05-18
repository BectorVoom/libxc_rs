//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 986/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk986<F: Float>(t13695: F, t1480: F, t133: F, t168: F, t3111: F, t4807: F, t1060: F, t355: F, t4099: F, t721: F, t145: F, t4875: F) -> (F, F, F, F, F) {
    let t16294 = t13695 * t1480;
    let t16296 = t133 * t168;
    let t16300 = t3111 * t4807;
    let t16304 = t1060 * t355 * t4099 * t721;
    let t16314 = t4875 * t145;
    (t16294, t16296, t16300, t16304, t16314)
}

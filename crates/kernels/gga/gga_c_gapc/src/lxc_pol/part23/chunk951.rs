//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 951/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk951<F: Float>(t11371: F, t11461: F, t11531: F, t11607: F, t576: F, t3797: F, t699: F, t932: F, t996: F, t3723: F, t787: F, t876: F) -> (F, F, F, F, F, F, F) {
    let t11609 = t11371 + t11461 + t11531 + t11607;
    let t11610 = t576 * t11609;
    let t11611 = t699 * t3797;
    let t11612 = t996 * t932;
    let t11613 = t3723 * t787;
    let t11614 = t11612 * t11613;
    let t11616 = t3723 * t876;
    (t11609, t11610, t11611, t11612, t11613, t11614, t11616)
}

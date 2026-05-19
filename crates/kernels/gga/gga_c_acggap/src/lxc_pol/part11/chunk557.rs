//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 557/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk557<F: Float>(t1089: F, t175: F, t3809: F, t384: F, t1080: F, t330: F, t363: F, t987: F, t3243: F, t453: F, t1210: F, t159: F) -> (F, F, F, F, F, F) {
    let t3811 = t1089 * t175 * t3809;
    let t3812 = t384 * t3811;
    let t3814 = t330 * t1080;
    let t3816 = t987 * t363;
    let t3827 = F::cast_from(0.19756347548806534796e1_f64) * t3243 * t453;
    let t3828 = t159 * t1210;
    (t3811, t3812, t3814, t3816, t3827, t3828)
}

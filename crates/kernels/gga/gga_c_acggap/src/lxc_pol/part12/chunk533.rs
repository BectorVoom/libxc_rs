//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 533/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk533<F: Float>(t3786: F, t384: F, t1032: F, t1103: F, t175: F, t3044: F, t398: F, t1036: F, t301: F, t879: F, t1089: F, t1080: F, t330: F, t363: F, t987: F, t3243: F, t453: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3787 = t384 * t3786;
    let t3793 = t1032 * t1103;
    let t3806 = t398 * t175 * t3044;
    let t3808 = 0.12862205435420921092e-2 * t1036 * t3806;
    let t3809 = t879 * t301;
    let t3811 = t1089 * t175 * t3809;
    let t3812 = t384 * t3811;
    let t3814 = t330 * t1080;
    let t3816 = t987 * t363;
    let t3827 = 0.19756347548806534796e1 * t3243 * t453;
    (t3787, t3793, t3806, t3808, t3809, t3811, t3812, t3814, t3816, t3827)
}

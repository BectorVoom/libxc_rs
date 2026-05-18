//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1163/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1163<F: Float>(t2001: F, t5539: F, t31346: F, t6328: F, t6140: F, t5891: F, t7561: F, t1809: F, t2020: F, t422: F, t5784: F, t598: F, t599: F, t6: F) -> (F, F, F, F, F, F) {
    let t40131 = t2001 * t5539;
    let t40134 = t31346 * t6328;
    let t40136 = t31346 * t6140;
    let t40145 = t7561 * t5891;
    let t40147 = t2020 * t1809;
    let t40152 = t598 * t422 * t6 * t5784 * t599;
    (t40131, t40134, t40136, t40145, t40147, t40152)
}

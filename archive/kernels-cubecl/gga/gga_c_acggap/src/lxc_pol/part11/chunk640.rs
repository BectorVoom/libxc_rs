//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 640/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk640<F: Float>(t2655: F, t2658: F, t2669: F, t2695: F, t2828: F, t2840: F, t4038: F, t4040: F, t4041: F, t4042: F, t4043: F, t4044: F, t4046: F, t4049: F, t4050: F, t4058: F) -> F {
    let t5020 = t2828 + t2655 - t2658 + t4038 + t2840 + t4040 - t4041 - t4042 - t4043 - t4044 + t2669 + t2695 + t4046 - t4049 - t4050 - t4058;
    t5020
}

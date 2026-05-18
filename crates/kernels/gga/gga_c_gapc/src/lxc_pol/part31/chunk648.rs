//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 648/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk648<F: Float>(t277: F, t3768: F, t334: F, t1084: F, t3687: F, t1089: F, t1026: F) -> (F, F, F, F, F) {
    let t3769 = t277 * t3768;
    let t3770 = t3769 * t334;
    let t3772 = t1084 * t3687;
    let t3773 = t3772 * t1089;
    let t3775 = t277 * t1026;
    (t3769, t3770, t3772, t3773, t3775)
}

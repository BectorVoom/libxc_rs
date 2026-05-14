//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 505/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk505<F: Float>(t1237: F, t3077: F, t1236: F, t955: F, t1160: F, t929: F, t944: F, t1159: F, t862: F) -> (F, F, F, F) {
    let t3078 = t3077 * t1237;
    let t3080 = t1236 * t955;
    let t3081 = t1160 * t3080;
    let t3084 = t944 * t929;
    let t3088 = t862 * t1159;
    (t3078, t3081, t3084, t3088)
}

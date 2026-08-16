//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 682/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk682<F: Float>(t481: F, t818: F, t4: F, t5: F, t2188: F, t2546: F, t186: F, t932: F) -> (F, F, F, F) {
    let t7208 = t481 * t818;
    let t7216 = t4 * t5;
    let t7241 = t2546 * t2188;
    let t7259 = t932 * t186;
    (t7208, t7216, t7241, t7259)
}

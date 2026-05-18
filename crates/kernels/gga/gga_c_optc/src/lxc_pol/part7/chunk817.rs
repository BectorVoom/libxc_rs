//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 817/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk817<F: Float>(t7523: F, t7525: F, t7527: F, t7529: F, t7531: F, t7535: F, t7538: F, t7541: F, t7544: F, t7547: F, t7550: F, t232: F) -> (F, F) {
    let t7713 = F::new(0.55403703703703703703e-1) * t7523;
    let t7724 = -t7713 - F::new(0.23744444444444444444e-1) * t7525 + F::new(0.11872222222222222222e-1) * t7527 - F::new(0.35616666666666666666e-1) * t7529 + F::new(0.17808333333333333333e-1) * t7531 - F::new(0.19787037037037037037e-1) * t7535 + F::new(0.71233333333333333332e-1) * t7538 - F::new(0.35616666666666666666e-1) * t7541 - F::new(0.10685e0) * t7544 + F::new(0.10685e0) * t7547 - F::new(0.17808333333333333333e-1) * t7550;
    let t7726 = F::new(0.62182e-1) * t7724 * t232;
    (t7724, t7726)
}

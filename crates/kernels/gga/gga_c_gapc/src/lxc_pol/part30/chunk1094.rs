//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1094/1135 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1094<F: Float>(t12327: F, t575: F, t687: F, t12339: F, t23726: F, t12346: F, t4908: F, t3537: F, t1616: F, t1617: F, t3873: F, t4915: F, t10529: F, t10541: F, t15430: F, t3859: F) -> (F, F, F, F, F, F, F) {
    let t37340 = t12327 * t575;
    let t37342 = 2.0 * t37340 * t687;
    let t37344 = 12.0 * t23726 * t12339;
    let t37346 = 4.0 * t4908 * t12346;
    let t37347 = t3537 * t3537;
    let t37349 = 4.0 * t1616 * t37347;
    let t37352 = 6.0 * t4915 * t3873 * t1617;
    let t37354 = 8.0 * t10529 * t10541;
    let t37356 = 2.0 * t15430 * t3859;
    (t37342, t37344, t37346, t37349, t37352, t37354, t37356)
}

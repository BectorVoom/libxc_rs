//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 711/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk711<F: Float>(t2957: F, t8400: F, t1574: F, t2938: F, t1600: F, t2958: F, t1573: F, t2932: F, t152: F, t3638: F, t5918: F, t434: F) -> (F, F, F, F, F) {
    let t8401 = t2957 * t8400;
    let t8403 = t1574 * t2938;
    let t8406 = t1600 * t2958;
    let t8408 = t2932 * t1573;
    let t8409 = t8408 * t2938;
    let t8411 = t3638 * t152;
    let t8412 = t8411 * t5918;
    let t8413 = t434 * t8412;
    (t8401, t8403, t8406, t8409, t8413)
}

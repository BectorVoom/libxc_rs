//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 498/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk498<F: Float>(t701: F, t9595: F, t1901: F, t2541: F, t2586: F, t3209: F, t835: F) -> (F, F, F, F) {
    let t9596 = t9595 * t701;
    let t9597 = t1901 * t9596;
    let t9600 = t2541 * t2586;
    let t9603 = t835 * t3209;
    (t9596, t9597, t9600, t9603)
}

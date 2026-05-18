//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 757/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk757<F: Float>(t7174: F, t943: F, t5230: F, t883: F, t732: F, t2553: F, t2060: F, t2558: F, t2559: F, t731: F, t2547: F, t481: F, t685: F) -> (F, F, F, F, F, F) {
    let t7175 = t943 * t7174;
    let t7177 = t883 * t5230;
    let t7178 = t732 * t7177;
    let t7179 = t2553 * t7178;
    let t7181 = t2060 * t2558;
    let t7182 = t943 * t7181;
    let t7184 = t731 * t2559;
    let t7187 = t481 * t2547 * t685;
    (t7175, t7177, t7179, t7182, t7184, t7187)
}

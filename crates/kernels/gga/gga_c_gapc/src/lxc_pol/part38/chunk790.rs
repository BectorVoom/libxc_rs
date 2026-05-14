//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 790/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk790<F: Float>(t462: F, t762: F, t3193: F, t126: F, t818: F, t787: F, t3187: F, t297: F, t3727: F, t7371: F, t771: F, t2316: F, t3188: F, t284: F, t2902: F, t3216: F) -> (F, F, F, F, F, F) {
    let t10139 = t462 * t762;
    let t10140 = t10139 * t3193;
    let t10142 = t126 * t818;
    let t10143 = t10142 * t787;
    let t10144 = t3187 * t10143;
    let t10146 = t3727 * t297;
    let t10147 = t10146 * t7371;
    let t10148 = t771 * t10147;
    let t10150 = t3188 * t2316;
    let t10151 = t284 * t10150;
    let t10153 = t2902 * t3216;
    (t10140, t10142, t10144, t10148, t10151, t10153)
}

//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 836/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk836<F: Float>(t314: F, t9739: F, t8957: F, t9738: F, t197: F, t7764: F, t1077: F, t3171: F, t820: F, t3443: F, t869: F, t896: F) -> (F, F, F, F, F, F) {
    let t9740 = t9739 * t314;
    let t9741 = t8957 * t9740;
    let t9742 = t9738 * t9741;
    let t9744 = t197 * t7764;
    let t9745 = t1077 * t9744;
    let t9747 = t3171 * t820;
    let t9748 = t3443 * t9747;
    let t9750 = t869 * t896;
    (t9740, t9741, t9742, t9745, t9748, t9750)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2242/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2242<F: Float>(t12050: F, t3151: F, t357: F, t15907: F, t3133: F, t3302: F, t4893: F, t3059: F, t4975: F, t4781: F, t12132: F, t1647: F, t3316: F) -> (F, F, F, F, F, F, F, F) {
    let t16568 = t12050 * t3151 * t357;
    let t16569 = t15907 * t16568;
    let t16573 = t3302 * t3133 * t357;
    let t16574 = t4893 * t16573;
    let t16577 = t4975 * t3059;
    let t16578 = t4781 * t16577;
    let t16581 = t4893 * t12132;
    let t16584 = t1647 * t3316;
    (t16568, t16569, t16573, t16574, t16577, t16578, t16581, t16584)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1228/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1228<F: Float>(t15907: F, t16554: F, t12077: F, t378: F, t342: F, t12050: F, t3154: F, t3151: F, t12046: F, t357: F, t3133: F, t3302: F, t4893: F, t3059: F, t4975: F, t4781: F) -> (F, F, F, F, F, F, F) {
    let t16555 = t15907 * t16554;
    let t16558 = t12077 * t378;
    let t16559 = t342 * t16558;
    let t16560 = t12050 * t3154;
    let t16561 = t16560 * t3151;
    let t16562 = t15907 * t16561;
    let t16565 = t12046 * t378;
    let t16566 = t342 * t16565;
    let t16568 = t12050 * t3151 * t357;
    let t16569 = t15907 * t16568;
    let t16573 = t3302 * t3133 * t357;
    let t16574 = t4893 * t16573;
    let t16577 = t4975 * t3059;
    let t16578 = t4781 * t16577;
    (t16555, t16559, t16562, t16566, t16569, t16574, t16578)
}

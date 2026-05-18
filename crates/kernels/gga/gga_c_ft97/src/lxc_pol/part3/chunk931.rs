//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 931/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk931<F: Float>(t2469: F, t5147: F, t242: F, t1091: F, t3842: F, t10007: F, t14196: F, t17790: F, t14200: F, t17794: F, t5073: F, t684: F) -> (F, F, F, F, F, F) {
    let t18398 = t2469 * t5147;
    let t18399 = t242 * t18398;
    let t18402 = t1091 * t3842;
    let t18403 = t10007 * t18402;
    let t18406 = t14196 * t17790;
    let t18409 = t14200 * t17794;
    let t18412 = t5073 * t684;
    (t18398, t18399, t18403, t18406, t18409, t18412)
}

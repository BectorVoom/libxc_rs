//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 813/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk813<F: Float>(t248: F, t3570: F, t6230: F, t3515: F, t1243: F, t19045: F, t225: F, t6151: F, t6153: F, t6239: F, t3640: F, t6270: F) -> (F, F, F, F, F, F) {
    let t19095 = t248 * t3570 * t6230;
    let t19096 = t3515 * t19095;
    let t19201 = t19045 * t1243;
    let t19232 = t6151 * t225;
    let t19234 = t6153 * t225;
    let t19249 = t6239 * t225;
    let t19267 = t6270 * t3640;
    (t19096, t19201, t19232, t19234, t19249, t19267)
}

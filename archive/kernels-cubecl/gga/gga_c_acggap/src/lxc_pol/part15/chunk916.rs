//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 916/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk916<F: Float>(t2082: F, t30044: F, t2087: F, t7610: F, t381: F, t7779: F, t2100: F, t31228: F, t7556: F, t30984: F, t7495: F, t151: F, t394: F, t592: F, t7510: F) -> (F, F, F, F, F, F, F) {
    let t31283 = t30044 * t2082;
    let t31285 = t7610 * t2087;
    let t31289 = t381 * t7779;
    let t31290 = t31289 * t2100;
    let t31295 = t31228 * t7556;
    let t31297 = t30984 * t7495;
    let t31309 = t151 * t394 * t592 * t7510;
    (t31283, t31285, t31289, t31290, t31295, t31297, t31309)
}

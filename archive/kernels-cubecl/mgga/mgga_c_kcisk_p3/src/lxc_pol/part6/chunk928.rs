//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 928/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk928<F: Float>(t29439: F, t29490: F, t716: F, t736: F, t2576: F, t9082: F, t2567: F, t9035: F, t734: F, t17936: F, t9047: F, t28950: F, t719: F, sigma2: F) -> (F, F, F, F, F) {
    let t29491 = t29439 + t29490;
    let t29492 = t29491 * t716;
    let t29493 = t29492 * sigma2;
    let t29494 = t29493 * t736;
    let t29496 = t2576 * t9082;
    let t29498 = t2567 * t9035;
    let t29499 = t734 * t29498;
    let t29501 = t17936 * t9047;
    let t29503 = t719 * t28950;
    (t29494, t29496, t29499, t29501, t29503)
}

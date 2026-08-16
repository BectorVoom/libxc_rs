//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1706/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1706<F: Float>(t140: F, t3698: F, t5047: F, t1222: F, t1012: F, t13026: F, t16715: F, t16720: F, t5312: F, t1774: F, t3601: F, t3611: F) -> (F, F, F, F, F) {
    let t17471 = t140 * t3698;
    let t17472 = t17471 * t5047;
    let t17474 = t1222 * t17472 / F::cast_from(324.0_f64);
    let t17475 = t1012 * t13026;
    let t17476 = t17475 * t16715;
    let t17479 = t5312 * t16720;
    let t17482 = t1774 * t3601;
    let t17483 = t17482 * t3611;
    (t17474, t17476, t17479, t17482, t17483)
}

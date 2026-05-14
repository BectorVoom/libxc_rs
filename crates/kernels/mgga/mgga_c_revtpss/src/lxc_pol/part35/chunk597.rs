//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 597/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk597<F: Float>(t1312: F, t1518: F, t4248: F, t5877: F, t5883: F, t5920: F, t93: F, t5545: F, t5547: F, t5570: F, t5572: F, t1907: F) -> (F, F, F, F, F, F) {
    let t6773 = 2.0 * t1312 * t5920 + 4.0 * t1518 * t4248 + 2.0 * t5883 * t93 + t5877;
    let t6777 = 8.0 * t5545;
    let t6778 = 8.0 * t5547;
    let t6779 = 2.0 * t5570;
    let t6780 = 0.11696447245269292414e1 * t5572;
    let t6781 = t1907 * t1907;
    (t6773, t6777, t6778, t6779, t6780, t6781)
}

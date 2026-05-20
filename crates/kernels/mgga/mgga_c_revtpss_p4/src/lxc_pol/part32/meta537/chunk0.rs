//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1847/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1847<F: Float>(t45963: F, t7342: F, t10301: F, t26178: F, t2247: F, t239: F, t38: F, t6960: F, t25163: F, t7348: F, t26205: F, t6963: F) -> (F, F, F, F, F, F) {
    let t95276 = t45963 * t7342;
    let t95283 = t10301 * t26178;
    let t95293 = t2247 * t38 * t239;
    let t95294 = t95293 * t6960;
    let t95296 = t7348 * t25163;
    let t95314 = t6963 * t26205;
    (t95276, t95283, t95293, t95294, t95296, t95314)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 939/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk939<F: Float>(t12039: F, t3269: F, t11804: F, t996: F, t1035: F, t11239: F, t342: F, t11247: F, t378: F, t3145: F, t334: F, t11249: F) -> (F, F, F, F, F) {
    let t12040 = t3269 * t12039;
    let t12043 = t996 * t11804;
    let t12046 = t11239 * t1035;
    let t12047 = t342 * t12046;
    let t12048 = t378 * t11247;
    let t12050 = F::new(1.0) / t3145 / t334;
    let t12051 = t11249 * t12050;
    (t12040, t12043, t12047, t12048, t12051)
}

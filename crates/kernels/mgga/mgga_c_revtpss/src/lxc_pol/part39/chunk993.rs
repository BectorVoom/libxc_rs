//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 993/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk993<F: Float>(t2772: F, t779: F, t689: F, t781: F, t9292: F, t861: F, t867: F, t786: F, t2467: F, t2828: F, t676: F, t123: F, t2465: F, t2410: F, t261: F, t2832: F, t892: F) -> (F, F, F, F, F, F, F) {
    let t11036 = t779 * t2772;
    let t11037 = t689 * t11036;
    let t11040 = 0.17073386770573548589e-1 * t9292 * t781;
    let t11043 = t861 * t867;
    let t11044 = t786 * t11043;
    let t11045 = t11044 * t2467;
    let t11049 = t676 * t2828;
    let t11050 = t123 * t11049;
    let t11051 = t2465 * t11050;
    let t11064 = 1.0 / t2410 / t261;
    let t11075 = t2832 * t892;
    (t11037, t11040, t11044, t11045, t11051, t11064, t11075)
}

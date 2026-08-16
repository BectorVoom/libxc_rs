//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1180/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1180<F: Float>(t138: F, t9675: F, t9674: F, t4075: F, t556: F, t786: F, t1444: F, t2434: F, t123: F, t3915: F, t1359: F, t9292: F) -> (F, F, F, F, F, F, F) {
    let t9676 = t138 * t9675;
    let t9677 = t9674 * t9676;
    let t9679 = t556 * t4075;
    let t9680 = t786 * t9679;
    let t9685 = t2434 * t1444;
    let t9686 = t123 * t9685;
    let t9687 = t3915 * t9686;
    let t9691 = F::cast_from(0.17073386770573548589e-1_f64) * t9292 * t1359;
    (t9676, t9677, t9680, t9685, t9686, t9687, t9691)
}

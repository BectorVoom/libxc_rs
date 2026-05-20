//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2128/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2128<F: Float>(t2033: F, t3829: F, t2014: F, t7900: F, t28067: F, t95088: F, t14468: F, t30: F, t2: F, t2411: F, t580: F, t890: F) -> (F, F, F, F) {
    let t98618 = t3829 * t2033;
    let t98621 = F::new(6.0) * t2014 * t98618 * t7900;
    let t98623 = F::new(6.0) * t95088 * t28067;
    let t98627 = t30 * t14468;
    let t98631 = t2411 * t2;
    let t98633 = t98631 * t580 * t890;
    (t98621, t98623, t98627, t98633)
}

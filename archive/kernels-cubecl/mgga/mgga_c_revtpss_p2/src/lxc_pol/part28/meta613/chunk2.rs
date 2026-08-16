//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2143/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2143<F: Float>(t25856: F, t4248: F, t2014: F, t2034: F, t49564: F, t2033: F, t3829: F, t7900: F, t28067: F, t95088: F, t14468: F, t30: F) -> (F, F, F, F, F) {
    let t98615 = F::cast_from(2.0_f64) * t4248 * t25856;
    let t98617 = t2014 * t2034 * t49564;
    let t98618 = t3829 * t2033;
    let t98621 = F::cast_from(6.0_f64) * t2014 * t98618 * t7900;
    let t98623 = F::cast_from(6.0_f64) * t95088 * t28067;
    let t98627 = t30 * t14468;
    (t98615, t98617, t98621, t98623, t98627)
}

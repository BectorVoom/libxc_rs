//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 432/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk432<F: Float>(t680: F, t130: F, t146: F, t2566: F) -> (F,) {
    let t2580 = t680 * t680;
    let t2581 = 1.0 / t2580;
    let t2582 = t130 * t2581;
    let t2583 = t146 * t146;
    let t2584 = 1.0 / t2583;
    let t2585 = t2566 * t2584;
    let t2587 = 0.16081979498692535067e2 * t2582 * t2585;
    (t2587,)
}

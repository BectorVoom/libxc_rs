//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 432/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk432<F: Float>(t2470: F, t874: F, t875: F, t822: F, t860: F, t1941: F, t268: F, t271: F) -> (F, F, F) {
    let t2810 = 0.13009920719177044025e-1 * t874 * t875 * t2470;
    let t2815 = t822 * t860;
    let t2846 = t268 * t1941 * t271;
    (t2810, t2815, t2846)
}

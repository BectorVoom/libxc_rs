//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 234/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk234<F: Float>(t251: F, t72: F, t686: F, t874: F, t822: F, t261: F) -> (F, F, F, F) {
    let t875 = t251 * t72;
    let t878 = 0.9757440539382783019e-2 * t874 * t875 * t686;
    let t879 = t822 * t251;
    let t892 = 1.0 / t261;
    (t875, t878, t879, t892)
}

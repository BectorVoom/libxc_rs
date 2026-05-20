//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1696/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1696<F: Float>(t88041: F, t88046: F, t88048: F, t88050: F, t88052: F, t88054: F, t88140: F, t88358: F, t88361: F, t88363: F, t88368: F, t88573: F) -> F {
    let t88981 = -t88041 - t88046 + t88048 + t88050 + t88052 + t88054 + t88573 - t88140 + t88358 - t88361 + t88363 - t88368;
    t88981
}

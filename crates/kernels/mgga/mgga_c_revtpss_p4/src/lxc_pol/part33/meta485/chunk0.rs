//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1768/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1768<F: Float>(t122: F, t1949: F, t72: F, t2466: F, t25375: F, t1955: F, t25308: F) -> (F, F, F, F) {
    let t25377 = t1949 * t72 * t122;
    let t25378 = t25377 * t2466;
    let t25379 = t25375 * t25378;
    let t25383 = t1955 * t25308;
    (t25377, t25378, t25379, t25383)
}

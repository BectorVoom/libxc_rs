//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1155/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1155<F: Float>(t126101: F, t126153: F, t126205: F, t126241: F, t126290: F, t126333: F, t126367: F, t126408: F, t892: F, t198: F, t205: F, t8489: F) -> (F, F, F) {
    let t126411 = t126101 + t126153 + t126205 + t126241 + t126290 + t126333 + t126367 + t126408;
    let t126412 = t126411 * t892;
    let t126422 = t198 * t205 * t8489;
    (t126411, t126412, t126422)
}

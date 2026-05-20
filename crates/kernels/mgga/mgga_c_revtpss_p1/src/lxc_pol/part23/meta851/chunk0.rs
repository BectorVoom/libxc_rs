//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2735/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2735<F: Float>(t3718: F, t44546: F, t6689: F, t1222: F, t17240: F, t20318: F, t1263: F, t372: F, t6622: F) -> (F, F, F) {
    let t71294 = t3718 * t44546 * t6689;
    let t71297 = t1222 * t17240 * t20318;
    let t71300 = t372 * t1263 * t6622;
    (t71294, t71297, t71300)
}

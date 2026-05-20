//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2782/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2782<F: Float>(t14110: F, t49471: F, t136: F, t2457: F, t47480: F, t6895: F, t22414: F, t686: F, t72: F, t9680: F, t22386: F, t3915: F) -> (F, F, F, F) {
    let t74763 = t49471 * t14110;
    let t74770 = t47480 * t6895 * t136 * t2457;
    let t74782 = t9680 * t22414 * t72 * t686;
    let t74794 = t3915 * t22386 * t72 * t686;
    (t74763, t74770, t74782, t74794)
}

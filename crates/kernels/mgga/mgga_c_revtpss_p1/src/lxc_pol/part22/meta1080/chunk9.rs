//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3891/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3891<F: Float>(t2439: F, t3895: F, t6896: F, t14110: F, t49471: F, t136: F, t2457: F, t47480: F, t6895: F, t22414: F, t686: F, t72: F, t9680: F) -> (F, F, F, F) {
    let t74757 = t2439 * t3895 * t6896;
    let t74763 = t49471 * t14110;
    let t74770 = t47480 * t6895 * t136 * t2457;
    let t74782 = t9680 * t22414 * t72 * t686;
    (t74757, t74763, t74770, t74782)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 828/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk828<F: Float>(t18814: F, t689: F, t6042: F, t786: F, t789: F, t6049: F, t779: F, t14987: F, t4481: F, t6075: F, t892: F, t262: F, t5962: F) -> (F, F, F, F, F, F) {
    let t18815 = t689 * t18814;
    let t18821 = t786 * t6042;
    let t18822 = t18821 * t789;
    let t18825 = t779 * t6049;
    let t18826 = t689 * t18825;
    let t18828 = t14987 * t4481;
    let t18850 = t6075 * t892;
    let t18860 = t262 * t5962;
    (t18815, t18822, t18826, t18828, t18850, t18860)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2940/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2940<F: Float>(t1882: F, t3923: F, t4003: F, t10022: F, t2782: F, t10014: F, t14242: F, t10073: F, t14225: F, t1892: F, t5744: F, t786: F) -> (F, F, F, F, F, F) {
    let t48073 = t1882 * t3923;
    let t48074 = t48073 * t4003;
    let t48076 = t2782 * t10022 * t48074;
    let t48079 = t10014 * t14242;
    let t48081 = t10073 * t14225;
    let t48083 = t5744 * t1892;
    let t48084 = t786 * t48083;
    (t48073, t48076, t48079, t48081, t48083, t48084)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2746/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2746<F: Float>(t1209: F, t1284: F, t6695: F, t20849: F, t3754: F, t3781: F, t6564: F, t20800: F, t3302: F, t13141: F, t1811: F, t460: F) -> (F, F, F, F, F) {
    let t72267 = t1209 * t1284 * t6695;
    let t72270 = t20849 * t3754;
    let t72326 = t6564 * t3781;
    let t72329 = t20800 * t3302;
    let t72343 = t460 * t13141 * t1811;
    (t72267, t72270, t72326, t72329, t72343)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1776/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1776<F: Float>(t1269: F, t1770: F, t1811: F, t3555: F, t460: F, t5412: F, t17306: F, t487: F) -> (F, F, F, F) {
    let t18005 = t1770 * t1269;
    let t18037 = t3555 * t1811;
    let t18054 = t460 * t5412;
    let t18059 = t17306 * t487;
    (t18005, t18037, t18054, t18059)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2599/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2599<F: Float>(t5603: F, t9692: F, t1904: F, t689: F, t9634: F, t1364: F, t14067: F, t786: F, t136: F, t2457: F, t5774: F, t9674: F) -> (F, F, F, F) {
    let t47863 = t5603 * t9692;
    let t47873 = t689 * t9634 * t1904;
    let t47876 = t786 * t14067 * t1364;
    let t47885 = t9674 * t5774 * t136 * t2457;
    (t47863, t47873, t47876, t47885)
}

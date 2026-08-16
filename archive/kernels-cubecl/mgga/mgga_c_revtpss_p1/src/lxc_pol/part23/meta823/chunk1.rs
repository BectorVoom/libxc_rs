//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2676/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2676<F: Float>(t1011: F, t15987: F, t18942: F, t15905: F, t55599: F, t6258: F, t905: F, t11710: F, t16089: F, t19706: F, t16095: F, t20095: F) -> (F, F, F, F, F) {
    let t66423 = t1011 * t15987 * t18942;
    let t66431 = t55599 * t15905;
    let t66434 = t6258 * t905;
    let t66467 = t16089 * t11710 * t19706;
    let t66470 = t16095 * t11710 * t20095;
    (t66423, t66431, t66434, t66467, t66470)
}

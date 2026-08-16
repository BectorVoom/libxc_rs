//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2448/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2448<F: Float>(t1063: F, t11160: F, t247: F, t3109: F, t11620: F, t73: F, t12166: F, t15905: F, t994: F, t11662: F, t11710: F, t4892: F) -> (F, F, F, F) {
    let t42606 = t1063 * t247 * t3109 * t11160;
    let t42610 = t11620 * t73;
    let t42621 = t994 * t12166 * t15905;
    let t42637 = t4892 * t11710 * t11662;
    (t42606, t42610, t42621, t42637)
}

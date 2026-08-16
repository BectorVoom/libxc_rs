//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3178/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3178<F: Float>(t12226: F, t1719: F, t12470: F, t1744: F, t12555: F, t5180: F, t12486: F, t300: F, t12553: F, t3521: F, t1261: F, t1715: F, t247: F, t44701: F) -> (F, F, F, F, F, F, F) {
    let t58473 = t1719 * t12226;
    let t58592 = t12470 * t1744;
    let t58647 = t5180 * t12555;
    let t58665 = t300 * t12486;
    let t58672 = t300 * t12553;
    let t58708 = t300 * t3521;
    let t58777 = t1261 * t247 * t44701 * t1715;
    (t58473, t58592, t58647, t58665, t58672, t58708, t58777)
}

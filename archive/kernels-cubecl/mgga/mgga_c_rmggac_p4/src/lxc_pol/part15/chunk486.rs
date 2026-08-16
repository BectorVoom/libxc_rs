//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 486/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk486<F: Float>(t1228: F, t1904: F, t1184: F, t1888: F, t476: F, t221: F, t1867: F, t4522: F, t6108: F, t1475: F, t1494: F, t209: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6125 = t1228 * t1904;
    let t6129 = t1888 * t1184;
    let t6130 = t6129 * t476;
    let t6131 = t221 * t6130;
    let t6134 = t1867 * t4522;
    let t6135 = t6134 * t476;
    let t6136 = t221 * t6135;
    let t6139 = t6108 * t476;
    let t6140 = t221 * t6139;
    let t6144 = t1475 * t1494;
    let t6145 = t221 * t6144;
    let t6148 = t1867 * t476;
    let t6149 = t6148 * t209;
    (t6125, t6130, t6131, t6135, t6136, t6139, t6140, t6144, t6145, t6149)
}

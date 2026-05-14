//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1019/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1019<F: Float>(t3060: F, t983: F, t3229: F, t506: F, t9937: F, t529: F, t3190: F, t938: F) -> (F, F, F, F, F) {
    let t9964 = t3060 * t983;
    let t9967 = t983 * t3229;
    let t9977 = t506 * t9937;
    let t9978 = t529 * t9977;
    let t9981 = t3190 * t938;
    (t9964, t9967, t9977, t9978, t9981)
}

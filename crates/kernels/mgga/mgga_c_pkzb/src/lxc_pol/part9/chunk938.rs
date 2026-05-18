//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 938/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk938<F: Float>(t702: F, t7240: F, t1932: F, t2819: F, t1095: F, t5873: F, t1917: F, t2849: F, t721: F, t1108: F, t1971: F, t1956: F, t2852: F) -> (F, F, F, F, F, F, F) {
    let t7241 = t7240 * t702;
    let t7244 = t2819 * t1932;
    let t7247 = t1095 * t5873;
    let t7248 = t7247 * t1917;
    let t7255 = t2849 * t721;
    let t7258 = t1108 * t1971;
    let t7261 = t2852 * t1956;
    (t7241, t7244, t7247, t7248, t7255, t7258, t7261)
}

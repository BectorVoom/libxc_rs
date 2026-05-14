//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 868/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk868<F: Float>(t7306: F, t7987: F, t2122: F, t2132: F, t7885: F, t864: F, t1219: F, t615: F, t7911: F, t862: F, t865: F, t15407: F, t7942: F, t9033: F, t322: F, t7896: F, t7979: F) -> (F, F, F, F, F, F) {
    let t31951 = t7987 * t7306;
    let t31955 = t7885 * t2132 * t2122 * t864;
    let t31965 = t615 * t7911 * t1219;
    let t31969 = t862 * t2122 * t865;
    let t31972 = t7942 * t9033 * t15407;
    let t31976 = t7896 * t2132 * t7979 * t322;
    (t31951, t31955, t31965, t31969, t31972, t31976)
}

//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1311/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1311<F: Float>(t18657: F, t2380: F, t8376: F, t19107: F, t22971: F, t19109: F, t6460: F, t19116: F, t6517: F, t3185: F, t6475: F, t8350: F) -> (F, F, F, F, F, F) {
    let t23061 = t2380 * t18657 * t8376;
    let t23075 = t19107 * t22971;
    let t23076 = t19109 * t6460;
    let t23081 = t19116 * t22971;
    let t23082 = t6517 * t6460;
    let t23088 = t3185 * t6475 * t8350;
    (t23061, t23075, t23076, t23081, t23082, t23088)
}

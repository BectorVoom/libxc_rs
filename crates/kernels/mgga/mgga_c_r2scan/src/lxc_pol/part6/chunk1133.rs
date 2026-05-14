//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1133/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1133<F: Float>(t20541: F, t2158: F, t2096: F, t122: F, t163: F, t252: F, t277: F, t6077: F, t783: F, t2098: F, t23: F, t116: F, t118: F, t507: F, t6325: F, t20319: F, t538: F, t6155: F) -> (F, F, F, F, F) {
    let t20542 = t20541 * t2158;
    let t20544 = t2096 * t2096;
    let t20552 = 0.57269714369731807609e-4 * t783 * t252 * t20544 * t122 / t6077 / t163 * t277;
    let t20557 = 1.0 / t23 / t2098;
    let t20561 = 0.58191567502592392719e-5 * t116 / t6325 / t118 * t122 * t20557 * t507;
    let t20563 = t6155 * t538 * t20319;
    (t20542, t20552, t20557, t20561, t20563)
}

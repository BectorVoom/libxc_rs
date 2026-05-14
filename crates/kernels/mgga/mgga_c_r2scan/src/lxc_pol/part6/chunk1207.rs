//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1207/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1207<F: Float>(t1673: F, t5465: F, t1266: F, t1680: F, t1684: F, t21: F, t159: F, t1678: F, t1686: F, t1783: F, t5917: F, t5967: F, t1762: F, t2021: F, t5916: F, t5710: F, t5960: F) -> (F, F, F, F, F, F) {
    let t22250 = t1673 * t5465;
    let t22255 = 0.75272354370370370365e-2 * t1680 * t1684 * t21 * t1266;
    let t22258 = t159 * t1783 * t1678 * t1686;
    let t22260 = t5967 * t5917;
    let t22264 = 0.26024595120724175621e0 * t1762 * t5916 * t2021;
    let t22267 = 0.52049190241448351242e0 * t1762 * t5960 * t5710;
    (t22250, t22255, t22258, t22260, t22264, t22267)
}

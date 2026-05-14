//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 954/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk954<F: Float>(t1987: F, t2875: F, t2866: F, t1972: F, t2865: F, t730: F, t1116: F, t5754: F, t237: F, t2826: F) -> (F, F, F, F, F, F) {
    let t7552 = 0.34631718211362927518e2 * t1987 * t2875;
    let t7554 = 0.23392894490538584828e1 * t1987 * t2866;
    let t7555 = t2865 * t1972;
    let t7557 = 0.11696447245269292414e1 * t730 * t7555;
    let t7559 = 0.5848223622634646207e0 * t5754 * t1116;
    let t7560 = t237 * t2826;
    (t7552, t7554, t7555, t7557, t7559, t7560)
}

//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1085/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1085<F: Float>(t237: F, t5838: F, t1971: F, t721: F, t2852: F, t2149: F, t803: F, t1987: F, t7555: F, t2860: F, t5809: F, t2865: F, t5842: F, t730: F, t2866: F, t5754: F) -> (F, F, F, F, F, F, F) {
    let t20637 = t237 * t5838;
    let t20638 = t1971 * t721;
    let t20641 = 0.31168546390226634765e3 * t20637 * t2852 * t20638;
    let t20642 = t2149 * t803;
    let t20647 = 0.35089341735807877242e1 * t1987 * t7555;
    let t20649 = 0.35089341735807877242e1 * t2860 * t5809;
    let t20652 = 0.11696447245269292414e1 * t730 * t2865 * t5842;
    let t20654 = 0.35089341735807877242e1 * t5754 * t2866;
    (t20638, t20641, t20642, t20647, t20649, t20652, t20654)
}

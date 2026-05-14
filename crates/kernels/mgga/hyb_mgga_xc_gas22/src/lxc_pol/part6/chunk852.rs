//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 852/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk852<F: Float>(t1110: F, t7269: F, t1101: F, t2754: F, t2757: F, t1068: F, t2679: F, t7: F, t132: F, t2687: F, t1046: F, t2713: F, t1047: F, t2729: F, t2751: F, t1067: F, t2813: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t7271 = 0.35089341735807877242e1 * t1110 * t7269;
    let t7272 = t2754 * t1101;
    let t7274 = t2757 * t1101;
    let t7276 = t2754 * t1068;
    let t7278 = t2757 * t1068;
    let t7281 = 1.0 / t2679 / t7;
    let t7292 = 1.0 / t2687 / t132;
    let t7307 = t2713 * t1046;
    let t7308 = t7307 * t1047;
    let t7310 = 6.0 * t2729 * t7308;
    let t7312 = 60.0 * t2751 * t1101;
    let t7313 = t1067 * t2813;
    (t7271, t7272, t7274, t7276, t7278, t7281, t7292, t7307, t7308, t7310, t7312, t7313)
}

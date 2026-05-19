//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 229/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk229<F: Float>(t696: F, t698: F, t248: F, t283: F, t619: F, t636: F, t640: F, t645: F, t653: F, t654: F, t688: F, t695: F) -> (F, F) {
    let t700 = F::cast_from(0.5848223622634646_f64) * t696 * t698;
    let t701 = t619 + t636 + t640 - t645 + t248 * t654 + t688 + F::cast_from(0.0197516734986138_f64) * t653 * t283 - t695 - t700;
    (t700, t701)
}

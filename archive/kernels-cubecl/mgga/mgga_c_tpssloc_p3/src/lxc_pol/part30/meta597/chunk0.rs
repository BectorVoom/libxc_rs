//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1980/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1980<F: Float>(t81074: F, t22724: F, t22727: F, t22894: F, t80670: F, t3787: F, t6955: F, t154: F, t9533: F, t131: F, t3748: F, t2009: F, t9537: F) -> (F, F, F, F, F, F, F) {
    let t81075 = F::cast_from(0.16220877603642232915e0_f64) * t81074;
    let t81076 = t22724 * t22727;
    let t81080 = t80670 * t22894;
    let t81105 = t3787 * t6955;
    let t81142 = t9533 * t154;
    let t81144 = t81142 * t3748 * t131;
    let t81146 = t81144 * t9537 * t2009;
    (t81075, t81076, t81080, t81105, t81142, t81144, t81146)
}

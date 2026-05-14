//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1235/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1235<F: Float>(t10874: F, t8195: F, t10885: F, t676: F, t10853: F, t136: F, t549: F, t10545: F, t1237: F, t2035: F, t25652: F, t26179: F, t26189: F, t26211: F, t26217: F, t26228: F, t26234: F, t30108: F, t30111: F, t30114: F, t30128: F, t30132: F, t30137: F, t30139: F, t30141: F, t3155: F, t8523: F, t8528: F, t8531: F, t8538: F) -> (F,) {
    let t30143 = t8195 * t10874;
    let t30145 = t676 * t10885;
    let t30148 = t136 * t549 * t10853;
    let t30150 = -t30108 / 96.0 - t30111 / 72.0 - t30114 / 96.0 + t2035 * t25652 * t1237 / 12.0 + t26179 / 54.0 + t26189 / 18.0 - t26211 / 36.0 - t26217 / 72.0 - 7.0 / 216.0 * t26228 + t26234 / 24.0 - t3155 * t10545 * t8523 / 24.0 - 7.0 / 72.0 * t8528 * t30128 * t8531 + t3155 * t30132 * t8538 / 6.0 + t30137 / 48.0 - 7.0 / 16.0 * t30139 - t30141 / 16.0 - 7.0 / 16.0 * t30143 - t30145 / 16.0 - t30148 / 16.0;
    (t30150,)
}

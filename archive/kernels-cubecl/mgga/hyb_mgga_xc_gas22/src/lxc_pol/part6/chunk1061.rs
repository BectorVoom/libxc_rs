//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1061/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1061<F: Float>(t1782: F, t3804: F, t1787: F, t559: F, t9909: F, t555: F, t558: F, t6162: F, t8159: F, t8162: F, t8183: F, t8187: F, t8199: F, t8210: F, t8216: F, t8218: F) -> (F, F, F, F) {
    let t10137 = t1782 * t3804;
    let t10141 = t1787 * t3804;
    let t10145 = t559 * t9909;
    let t10151 = -t555 * t558 * t10137 / F::cast_from(64.0_f64) - t555 * t558 * t10141 / F::cast_from(64.0_f64) - t555 * t558 * t10145 / F::cast_from(64.0_f64) - t8159 - t8162 + t6162 / F::cast_from(288.0_f64) - t8183 - t8187 / F::cast_from(48.0_f64) - t8199 - t8210 - t8216 - t8218;
    (t10137, t10141, t10145, t10151)
}

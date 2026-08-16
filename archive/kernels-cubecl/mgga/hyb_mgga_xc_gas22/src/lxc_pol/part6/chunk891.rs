//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 891/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk891<F: Float>(t1139: F, t2867: F, t1143: F, t2874: F, t1166: F, t2880: F, t526: F, t530: F, t1128: F, t2938: F, t511: F, t519: F) -> (F, F, F, F, F, F) {
    let t7721 = t2867 * t1139;
    let t7734 = t1143 * t2874;
    let t7739 = t1166 * t2880;
    let t7744 = F::cast_from(1.0_f64) / t530 / t526 / F::cast_from(2.0_f64);
    let t7764 = t2938 * t1128;
    let t7768 = F::cast_from(1.0_f64) / t519 / t511;
    (t7721, t7734, t7739, t7744, t7764, t7768)
}

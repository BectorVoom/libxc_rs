//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 866/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk866<F: Float>(t7501: F, t7504: F, t7507: F, t7510: F, t7514: F, t7516: F, t7518: F, t7521: F) -> (F,) {
    let t7523 = -0.34523333333333333333e1 * t7501 + 0.23015555555555555556e1 * t7504 - 0.26851481481481481482e1 * t7507 - 0.93932222222222222223e0 * t7510 + 0.73355e-1 * t7514 - 0.14671e0 * t7516 - 0.17116166666666666667e0 * t7518 - 0.36793333333333333333e0 * t7521;
    (t7523,)
}

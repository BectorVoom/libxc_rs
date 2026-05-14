//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1171/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1171<F: Float>(t2751: F, t3647: F, t3616: F, t7249: F, t7255: F, t2649: F, t9404: F, t2757: F, t3649: F, t2754: F, t1057: F, t9374: F, t1134: F, t9620: F, t2903: F, t9611: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t26038 = t2751 * t3647;
    let t26042 = t3616 * t7249;
    let t26044 = t3616 * t7255;
    let t26046 = t9404 * t2649;
    let t26048 = t2757 * t3649;
    let t26050 = t2754 * t3649;
    let t26052 = t2751 * t3649;
    let t26054 = t1057 * t9374;
    let t26093 = t1134 * t9620;
    let t26096 = t2903 * t9611;
    (t26038, t26042, t26044, t26046, t26048, t26050, t26052, t26054, t26093, t26096)
}

//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 862/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk862<F: Float>(t7189: F, t7291: F, t2515: F, t936: F, t345: F) -> (F, F, F, F) {
    let t7384 = 0.93011851851851851854e0 * t7189;
    let t7391 = 0.36514074074074074075e0 * t7291;
    let t7402 = 1.0 / t2515 / t936;
    let t7403 = t345 * t7402;
    (t7384, t7391, t7402, t7403)
}

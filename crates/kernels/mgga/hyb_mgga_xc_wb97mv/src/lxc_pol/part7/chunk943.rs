//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 943/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk943<F: Float>(t2206: F, t8951: F, t1330: F, t6802: F, t1336: F, t2224: F, t238: F) -> (F, F, F, F) {
    let t8952 = t8951 * t2206;
    let t8954 = t6802 * t1330;
    let t8955 = t8954 * t2206;
    let t8958 = t238 * t2224 * t1336;
    (t8952, t8954, t8955, t8958)
}

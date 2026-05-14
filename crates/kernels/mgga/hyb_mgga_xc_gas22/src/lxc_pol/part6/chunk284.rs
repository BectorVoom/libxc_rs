//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 284/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk284<F: Float>(t935: F, t961: F, t942: F, t953: F, t958: F, t965: F) -> (F, F, F) {
    let t981 = 0.516475e0 * t935;
    let t984 = 0.104195e0 * t961;
    let t986 = 0.3529725e1 * t953 - t981 + 0.1549425e1 * t942 + 0.6311625e0 * t958 - t984 + 0.312585e0 * t965;
    (t981, t984, t986)
}

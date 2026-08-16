//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 718/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk718<F: Float>(t3047: F, t49: F, t35688: F, t7935: F, t14362: F, t2190: F, t3144: F, t25561: F, t29: F, t3117: F, t3132: F, t3136: F) -> (F, F, F, F, F) {
    let t70171 = t3047 * t49;
    let t70173 = t35688 * t70171 * t7935;
    let t70176 = t2190 * t14362 * t3144;
    let t70186 = t3117 * t25561 * t29;
    let t70188 = t3132 * t70186 * t3136;
    (t70171, t70173, t70176, t70186, t70188)
}

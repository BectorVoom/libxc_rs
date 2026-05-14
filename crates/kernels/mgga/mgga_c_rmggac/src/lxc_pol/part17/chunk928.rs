//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 928/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk928<F: Float>(t46454: F, t7192: F, t2186: F, t9932: F, t3928: F, t6441: F, t645: F, t4044: F, t6421: F, t2060: F, t45622: F, t903: F, t34847: F, t9971: F, t1614: F, t1971: F, t511: F, t615: F, t7230: F) -> (F, F, F, F, F, F, F) {
    let t47478 = t7192 * t46454;
    let t47484 = t2186 * t9932;
    let t47487 = t3928 * t645 * t6441;
    let t47490 = t4044 * t645 * t6421;
    let t47493 = t903 * t2060 * t45622;
    let t47495 = t34847 * t9971;
    let t47500 = t7230 * t1971 * t511 * t1614 * t615;
    (t47478, t47484, t47487, t47490, t47493, t47495, t47500)
}

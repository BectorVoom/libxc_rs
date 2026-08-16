//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 912/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk912<F: Float>(t1971: F, t352: F, t7230: F, t875: F, t9843: F, t8577: F, t9171: F, t1910: F, t1970: F, t209: F, t236: F, t476: F, t7231: F) -> (F, F, F) {
    let t45264 = t7230 * t1971 * t875 * t9843 * t352;
    let t45266 = t8577 * t9171;
    let t45272 = t1970 * t7231 * t236 * t1910 * t476 * t209;
    (t45264, t45266, t45272)
}

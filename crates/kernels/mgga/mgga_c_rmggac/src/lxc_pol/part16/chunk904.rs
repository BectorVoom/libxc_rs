//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 904/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk904<F: Float>(t1494: F, t1970: F, t1971: F, t209: F, t515: F, t570: F, t352: F, t6172: F, t118: F, t128: F, t1888: F, t1986: F) -> (F, F, F) {
    let t45032 = t1970 * t1971 * t515 * t570 * t1494 * t209;
    let t45038 = t1970 * t1971 * t515 * t6172 * t352;
    let t45043 = t1986 * t118 * t128 * t1888 * t209;
    (t45032, t45038, t45043)
}

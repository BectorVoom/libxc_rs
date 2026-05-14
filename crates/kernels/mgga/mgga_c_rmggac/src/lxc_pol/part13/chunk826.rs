//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 826/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk826<F: Float>(t3351: F, t3352: F, t511: F, t5218: F, t1971: F, t5184: F, t880: F, t2144: F, t31125: F, t2010: F, t8342: F, t935: F, t2415: F, t4029: F, t1240: F, t515: F, t570: F, t7230: F) -> (F, F, F, F, F, F) {
    let t40533 = t3351 * t3352 * t511 * t5218;
    let t40537 = t3351 * t1971 * t880 * t5184;
    let t40541 = t3351 * t1971 * t2144 * t31125;
    let t40544 = t2010 * t8342 * t935;
    let t40547 = t2010 * t2415 * t4029;
    let t40554 = t7230 * t1971 * t515 * t570 * t1240;
    (t40533, t40537, t40541, t40544, t40547, t40554)
}

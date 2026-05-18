//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 676/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk676<F: Float>(t515: F, t9044: F, t1971: F, t7230: F, t498: F, t570: F, t7231: F, t3351: F, t5144: F, t3352: F, t9008: F, t903: F) -> (F, F, F, F, F, F, F, F) {
    let t9045 = t515 * t9044;
    let t9046 = t1971 * t9045;
    let t9047 = t7230 * t9046;
    let t9049 = t570 * t498;
    let t9050 = t515 * t9049;
    let t9051 = t7231 * t9050;
    let t9052 = t3351 * t9051;
    let t9054 = t515 * t5144;
    let t9055 = t3352 * t9054;
    let t9056 = t3351 * t9055;
    let t9060 = t903 * t9008;
    (t9046, t9047, t9049, t9051, t9052, t9055, t9056, t9060)
}

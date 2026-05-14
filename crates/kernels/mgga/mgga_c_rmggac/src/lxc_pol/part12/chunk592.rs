//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 592/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk592<F: Float>(t236: F, t8420: F, t7231: F, t1970: F, t1475: F, t321: F, t3352: F, t333: F, t511: F, t1971: F, t352: F, t515: F, t128: F, t605: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8421 = t236 * t8420;
    let t8422 = t7231 * t8421;
    let t8423 = t1970 * t8422;
    let t8425 = t1475 * t321;
    let t8426 = t236 * t8425;
    let t8427 = t3352 * t8426;
    let t8428 = t1970 * t8427;
    let t8430 = t1475 * t333;
    let t8431 = t511 * t8430;
    let t8432 = t1971 * t8431;
    let t8433 = t1970 * t8432;
    let t8435 = t1475 * t352;
    let t8436 = t515 * t8435;
    let t8437 = t1971 * t8436;
    let t8438 = t1970 * t8437;
    let t8440 = t128 * t605;
    (t8422, t8423, t8427, t8428, t8432, t8433, t8437, t8438, t8440)
}

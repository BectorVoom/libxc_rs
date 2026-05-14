//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 564/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk564<F: Float>(t515: F, t8435: F, t1971: F, t1970: F, t128: F, t605: F, t209: F, t118: F, t1986: F) -> (F, F, F) {
    let t8436 = t515 * t8435;
    let t8437 = t1971 * t8436;
    let t8438 = t1970 * t8437;
    let t8440 = t128 * t605;
    let t8441 = t8440 * t209;
    let t8442 = t118 * t8441;
    let t8443 = t1986 * t8442;
    (t8437, t8438, t8443)
}

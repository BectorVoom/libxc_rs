//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 408/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk408<F: Float>(t1469: F, t236: F, t1475: F, t498: F, t321: F, t333: F, t511: F, t352: F, t515: F, t128: F, t605: F, t209: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t8416 = t236 * t1469;
    let t8420 = t1475 * t498;
    let t8421 = t236 * t8420;
    let t8425 = t1475 * t321;
    let t8426 = t236 * t8425;
    let t8430 = t1475 * t333;
    let t8431 = t511 * t8430;
    let t8435 = t1475 * t352;
    let t8436 = t515 * t8435;
    let t8440 = t128 * t605;
    let t8441 = t8440 * t209;
    (t8416, t8420, t8421, t8425, t8426, t8430, t8431, t8435, t8436, t8440, t8441)
}

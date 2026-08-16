//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 666/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk666<F: Float>(t1652: F, t2060: F, t739: F, t321: F, t615: F, t236: F, t3352: F, t7230: F, t333: F, t511: F, t1971: F, t352: F, t515: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8821 = t2060 * t1652;
    let t8822 = t739 * t8821;
    let t8829 = t615 * t321;
    let t8830 = t236 * t8829;
    let t8831 = t3352 * t8830;
    let t8832 = t7230 * t8831;
    let t8834 = t615 * t333;
    let t8835 = t511 * t8834;
    let t8836 = t1971 * t8835;
    let t8837 = t7230 * t8836;
    let t8842 = t515 * t615 * t352;
    (t8821, t8822, t8829, t8831, t8832, t8834, t8836, t8837, t8842)
}

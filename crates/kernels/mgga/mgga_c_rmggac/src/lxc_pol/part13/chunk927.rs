//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 927/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk927<F: Float>(t34847: F, t8836: F, t1525: F, t1971: F, t333: F, t511: F, t7230: F, t615: F, t848: F, t8843: F, t352: F, t515: F) -> (F, F, F, F, F) {
    let t40379 = t34847 * t8836;
    let t40384 = t7230 * t1971 * t511 * t1525 * t333;
    let t40389 = t7230 * t1971 * t511 * t615 * t848;
    let t40391 = t34847 * t8843;
    let t40396 = t7230 * t1971 * t515 * t1525 * t352;
    (t40379, t40384, t40389, t40391, t40396)
}

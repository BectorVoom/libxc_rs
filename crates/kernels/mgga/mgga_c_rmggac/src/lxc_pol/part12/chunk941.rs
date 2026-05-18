//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 941/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk941<F: Float>(t1970: F, t1971: F, t352: F, t515: F, t5605: F, t36634: F, t5156: F, t656: F, t36629: F, t5163: F, t36471: F, t5166: F) -> (F, F, F, F) {
    let t40182 = t1970 * t1971 * t515 * t5605 * t352;
    let t40185 = t36634 * t656 * t5156;
    let t40188 = t36629 * t656 * t5163;
    let t40191 = t36471 * t656 * t5166;
    (t40182, t40185, t40188, t40191)
}

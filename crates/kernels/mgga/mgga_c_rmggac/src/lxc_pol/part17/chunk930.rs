//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 930/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk930<F: Float>(t7255: F, t9985: F, t3351: F, t3352: F, t511: F, t6441: F, t1971: F, t6421: F, t880: F, t2144: F, t45622: F, t7720: F, t9817: F, t39277: F, t9046: F, t17859: F, t8812: F) -> (F, F, F, F, F, F, F) {
    let t47516 = t7255 * t9985;
    let t47520 = t3351 * t3352 * t511 * t6441;
    let t47524 = t3351 * t1971 * t880 * t6421;
    let t47528 = t3351 * t1971 * t2144 * t45622;
    let t47530 = t7720 * t9817;
    let t47532 = t39277 * t9046;
    let t47534 = t17859 * t8812;
    (t47516, t47520, t47524, t47528, t47530, t47532, t47534)
}

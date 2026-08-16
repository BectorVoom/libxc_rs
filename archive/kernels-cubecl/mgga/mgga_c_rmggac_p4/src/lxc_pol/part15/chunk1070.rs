//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 1070/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk1070<F: Float>(t132: F, t1811: F, t7933: F, t7934: F, t575: F, t577: F, t3351: F, t511: F, t6403: F, t9188: F, t47124: F, t515: F) -> (F, F, F, F) {
    let t47549 = t7933 * t7934 * t1811 * t132;
    let t47553 = t7933 * t7934 * t577 * t575;
    let t47557 = t3351 * t9188 * t511 * t6403;
    let t47561 = t3351 * t9188 * t515 * t47124;
    (t47549, t47553, t47557, t47561)
}

//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 900/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk900<F: Float>(t535: F, t577: F, t7933: F, t7934: F, t132: F, t1811: F, t575: F, t3351: F, t511: F, t6403: F, t9188: F, t47124: F, t515: F, t236: F, t6412: F, t2305: F, t39393: F) -> (F, F, F, F, F, F, F) {
    let t47545 = t7933 * t7934 * t577 * t535;
    let t47549 = t7933 * t7934 * t1811 * t132;
    let t47553 = t7933 * t7934 * t577 * t575;
    let t47557 = t3351 * t9188 * t511 * t6403;
    let t47561 = t3351 * t9188 * t515 * t47124;
    let t47565 = t3351 * t9188 * t236 * t6412;
    let t47570 = t39393 * t2305;
    (t47545, t47549, t47553, t47557, t47561, t47565, t47570)
}

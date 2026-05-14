//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 977/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk977<F: Float>(t26422: F, t395: F, t1300: F, t8021: F, t1305: F, t8022: F, t1322: F, t7736: F, t13504: F, t1309: F, t1315: F, t13805: F, t13861: F, t13868: F, t20097: F, t20151: F, t20162: F, t20169: F, t20177: F, t20182: F, t20185: F, t20202: F, t20206: F, t2164: F, t26008: F, t26086: F, t26090: F, t3935: F, t3970: F, t405: F, t8033: F, sigma0: F) -> (F,) {
    let t26423 = t26422 * sigma0;
    let t26424 = t26423 * t395;
    let t26427 = t8021 * t1300;
    let t26430 = t8022 * t1305;
    let t26432 = t7736 * t1322;
    let t26433 = t13504 * t26432;
    let t26445 = 0.17990788716177317213e-1 * t26008 * t1315 - 0.63967248768630461203e-1 * t3970 * t8033 + 0.79959060960788076503e-2 * t26086 + 0.10794473229706390328e0 * t1309 * t26090 + 0.35981577432354634426e-1 * t20097 * t2164 + 0.5397236614853195164e-1 * t26424 * t405 - 0.14392630972941853771e0 * t26427 * t405 + 0.17990788716177317213e-1 * t26430 - 0.23987718288236422951e-1 * t3935 * t26433 + 0.11993859144118211475e-1 * t13805 + 0.2398771828823642295e-1 * t20151 + 0.71963154864709268852e-1 * t20162 - 0.23987718288236422951e-1 * t20169 - t20177 + 0.95950873152945691803e-1 * t20182 + 0.23987718288236422951e-1 * t20185 - t20202 - 0.95950873152945691803e-1 * t20206 - 0.11993859144118211475e-1 * t13861 + 0.319836243843152306e-1 * t13868;
    (t26445,)
}

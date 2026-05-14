//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1028/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1028<F: Float>(t16595: F, t16597: F, t16615: F, t10469: F, t10474: F, t10484: F, t10495: F, t10497: F, t11983: F, t16593: F, t16602: F, t16606: F, t16611: F, t16613: F, t16620: F, t16625: F, t16629: F, t16633: F, t16640: F) -> (F,) {
    let t18226 = 0.15476481481481481481e-2 * t16595;
    let t18227 = 0.23214722222222222222e-2 * t16597;
    let t18232 = 0.23214722222222222222e-2 * t16615;
    let t18243 = -0.46429444444444444444e-2 * t16593 - t18226 - t18227 + 0.77382407407407407407e-3 * t16602 - 0.23214722222222222222e-2 * t16606 - 0.11607361111111111111e-1 * t16611 - 0.61905925925925925924e-2 * t16613 + t18232 - 0.38691203703703703703e-3 * t16620 + 0.46429444444444444444e-2 * t16625 - 0.11607361111111111111e-2 * t16629 - 0.19345601851851851852e-2 * t16633 - 0.15476481481481481481e-2 * t10469 - 0.51588271604938271604e-3 * t10474 - 0.15476481481481481481e-2 * t10484 + 0.12897067901234567901e-2 * t10495 + 0.77382407407407407407e-3 * t10497 + t11983 - 0.38691203703703703703e-3 * t16640;
    (t18243,)
}

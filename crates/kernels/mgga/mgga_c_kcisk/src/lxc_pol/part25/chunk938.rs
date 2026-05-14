//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 938/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk938<F: Float>(t4581: F, t6715: F, t6713: F, t4811: F, t6704: F, t6953: F, t2063: F, t4803: F, t5193: F, t5192: F, t5182: F, t15921: F, t6666: F, t10487: F, t719: F, t15930: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t16592 = t4581 * t6715;
    let t16593 = t6713 * t16592;
    let t16595 = t4811 * t6704;
    let t16596 = 0.22109259259259259258e-2 * t16595;
    let t16597 = t4811 * t6953;
    let t16598 = 0.33163888888888888888e-2 * t16597;
    let t16600 = t5193 * t2063 * t4803;
    let t16601 = t5192 * t16600;
    let t16602 = t5182 * t16601;
    let t16604 = t6666 * t15921;
    let t16605 = t5192 * t16604;
    let t16606 = t5182 * t16605;
    let t16608 = t719 * t10487;
    let t16609 = t16608 * t15930;
    (t16593, t16595, t16596, t16597, t16598, t16600, t16602, t16604, t16606, t16609)
}

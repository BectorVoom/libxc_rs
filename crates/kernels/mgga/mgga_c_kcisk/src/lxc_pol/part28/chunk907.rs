//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 907/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk907<F: Float>(t15993: F, t45: F, t6847: F, t15991: F, t16398: F, t2368: F, t4703: F, t5191: F, t5203: F, t5074: F, t6720: F, t4811: F, t6704: F, t6953: F, t10487: F, t719: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t16448 = 0.12361111111111111111e-1 * t15993;
    let t16462 = t45 * t6847;
    let t16485 = 0.13418888888888888889e0 * t15991;
    let t16500 = 0.22076e0 * t16398;
    let t16528 = 0.23744444444444444444e-1 * t15993;
    let t16541 = t2368 * t4703;
    let t16580 = t5191 * t5203;
    let t16588 = t5074 * t6720;
    let t16595 = t4811 * t6704;
    let t16596 = 0.22109259259259259258e-2 * t16595;
    let t16597 = t4811 * t6953;
    let t16598 = 0.33163888888888888888e-2 * t16597;
    let t16608 = t719 * t10487;
    (t16448, t16462, t16485, t16500, t16528, t16541, t16580, t16588, t16595, t16596, t16597, t16598, t16608)
}

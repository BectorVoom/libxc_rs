//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1292/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1292<F: Float>(t2736: F, t55345: F, t79: F, t33870: F, t9512: F, t114136: F, t114138: F, t33850: F, t9515: F, t32417: F, t33873: F, t115078: F, t9536: F, t32353: F, t9850: F, t32342: F, t33941: F) -> (F, F, F, F, F, F, F, F, F) {
    let t115471 = t55345 * t79 * t2736;
    let t115489 = 0.34722222222222222222e-2 * t9512 * t33870;
    let t115493 = 0.15476481481481481481e-2 * t114136;
    let t115496 = 0.10317654320987654321e-2 * t114138;
    let t115500 = t9515 * t33850;
    let t115504 = 0.13402777777777777778e-2 * t32417 * t33873;
    let t115515 = t9536 * t115078;
    let t115526 = t9850 * t32353;
    let t115531 = 0.11574074074074074074e-2 * t33941 * t32342;
    (t115471, t115489, t115493, t115496, t115500, t115504, t115515, t115526, t115531)
}

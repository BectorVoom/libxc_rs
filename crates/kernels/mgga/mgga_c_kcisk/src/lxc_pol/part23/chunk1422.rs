//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1422/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1422<F: Float>(t114136: F, t114138: F, t33850: F, t9515: F, t32417: F, t33873: F, t115078: F, t9536: F, t109749: F, t109910: F, t114134: F, t114145: F, t114148: F, t114151: F, t2740: F, t32370: F, t32380: F, t33807: F, t33823: F, t9512: F, t9519: F, t9523: F, t9850: F, t9860: F, t9864: F) -> (F,) {
    let t115493 = 0.15476481481481481481e-2 * t114136;
    let t115496 = 0.10317654320987654321e-2 * t114138;
    let t115500 = t9515 * t33850;
    let t115504 = 0.13402777777777777778e-2 * t32417 * t33873;
    let t115515 = t9536 * t115078;
    let t115519 = 0.15476481481481481481e-2 * t114134 - t115493 - 0.10416666666666666667e-1 * t9860 * t32380 + t115496 - 0.92858888888888888888e-2 * t114145 - 0.41270617283950617284e-2 * t114148 + 0.12381185185185185185e-1 * t114151 - 0.10722222222222222222e-1 * t115500 * t9519 + t115504 + 0.10416666666666666667e-1 * t9512 * t33823 - 0.10416666666666666667e-1 * t33807 * t9523 * t2740 - 0.52083333333333333333e-2 * t9850 * t32370 * t2740 + 0.92592592592592592592e-2 * t109910 * t9864 - 0.69444444444444444445e-2 * t115515 + 0.92592592592592592592e-2 * t109749 * t9864;
    (t115519,)
}

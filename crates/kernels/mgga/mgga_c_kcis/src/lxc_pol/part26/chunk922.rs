//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 922/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk922<F: Float>(t1364: F, t15896: F, t21015: F, t21018: F, t21023: F, t21027: F, t21030: F, t21033: F, t21036: F, t21041: F, t21044: F, t21048: F, t21052: F, t21055: F, t21059: F, t21500: F, t21508: F, t21512: F, t21514: F, t3964: F, t5738: F, t5742: F, t7092: F) -> F {
    let t21516 = -F::new(0.66327777777777777776e-2) * t21015 + F::new(0.16581944444444444444e-2) * t21018 + F::new(0.16581944444444444444e-2) * t21023 + F::new(0.17687407407407407407e-1) * t21027 - F::new(0.33163888888888888888e-2) * t21030 - F::new(0.49745833333333333332e-2) * t21033 + F::new(0.13265555555555555555e-1) * t21036 - F::new(0.55273148148148148147e-3) * t21041 + F::new(0.99491666666666666664e-2) * t21044 + F::new(0.88437037037037037034e-2) * t21048 + F::new(0.29479012345679012345e-2) * t21052 - F::new(0.58958024691358024689e-2) * t15896 + F::new(0.22109259259259259259e-2) * t21055 - F::new(0.16581944444444444444e-2) * t21059 - F::new(0.66725e-1) * t1364 * t21500 - F::new(0.66725e-1) * t3964 * t7092 - F::new(0.13345e0) * t5742 * t5738 - F::new(0.58958024691358024689e-2) * t21508 + F::new(0.11054629629629629629e-2) * t21512 - F::new(0.33163888888888888888e-2) * t21514;
    t21516
}

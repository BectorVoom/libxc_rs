//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1302/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1302(t1364: f64, t15896: f64, t21015: f64, t21018: f64, t21023: f64, t21027: f64, t21030: f64, t21033: f64, t21036: f64, t21041: f64, t21044: f64, t21048: f64, t21052: f64, t21055: f64, t21059: f64, t21500: f64, t21508: f64, t21512: f64, t21514: f64, t3964: f64, t5738: f64, t5742: f64, t7092: f64) -> f64 {
    let t21516 = -0.66327777777777777776e-2_f64 * t21015 + 0.16581944444444444444e-2_f64 * t21018 + 0.16581944444444444444e-2_f64 * t21023 + 0.17687407407407407407e-1_f64 * t21027 - 0.33163888888888888888e-2_f64 * t21030 - 0.49745833333333333332e-2_f64 * t21033 + 0.13265555555555555555e-1_f64 * t21036 - 0.55273148148148148147e-3_f64 * t21041 + 0.99491666666666666664e-2_f64 * t21044 + 0.88437037037037037034e-2_f64 * t21048 + 0.29479012345679012345e-2_f64 * t21052 - 0.58958024691358024689e-2_f64 * t15896 + 0.22109259259259259259e-2_f64 * t21055 - 0.16581944444444444444e-2_f64 * t21059 - 0.66725e-1_f64 * t1364 * t21500 - 0.66725e-1_f64 * t3964 * t7092 - 0.13345e0_f64 * t5742 * t5738 - 0.58958024691358024689e-2_f64 * t21508 + 0.11054629629629629629e-2_f64 * t21512 - 0.33163888888888888888e-2_f64 * t21514;
    t21516
}

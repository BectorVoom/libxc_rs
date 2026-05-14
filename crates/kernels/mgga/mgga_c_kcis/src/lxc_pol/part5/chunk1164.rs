//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1164/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1164<F: Float>(t21101: F, t21148: F, t21467: F, t21497: F, t1386: F, t17292: F, t5637: F, t4160: F, t1307: F, t7313: F, t4170: F, t17298: F, t5668: F, t1364: F, t15896: F, t21015: F, t21018: F, t21023: F, t21027: F, t21030: F, t21033: F, t21036: F, t21041: F, t21044: F, t21048: F, t21052: F, t21055: F, t21059: F, t3964: F, t5738: F, t5742: F, t7092: F) -> (F, F, F, F) {
    let t21499 = t21101 + t21148 + t21467 + t21497;
    let t21500 = t21499 * t1386;
    let t21507 = t17292 * t5637;
    let t21508 = t4160 * t21507;
    let t21510 = t7313 * t1307;
    let t21511 = t4170 * t21510;
    let t21512 = t4160 * t21511;
    let t21514 = t17298 * t5668;
    let t21516 = -0.66327777777777777776e-2 * t21015 + 0.16581944444444444444e-2 * t21018 + 0.16581944444444444444e-2 * t21023 + 0.17687407407407407407e-1 * t21027 - 0.33163888888888888888e-2 * t21030 - 0.49745833333333333332e-2 * t21033 + 0.13265555555555555555e-1 * t21036 - 0.55273148148148148147e-3 * t21041 + 0.99491666666666666664e-2 * t21044 + 0.88437037037037037034e-2 * t21048 + 0.29479012345679012345e-2 * t21052 - 0.58958024691358024689e-2 * t15896 + 0.22109259259259259259e-2 * t21055 - 0.16581944444444444444e-2 * t21059 - 0.66725e-1 * t1364 * t21500 - 0.66725e-1 * t3964 * t7092 - 0.13345e0 * t5742 * t5738 - 0.58958024691358024689e-2 * t21508 + 0.11054629629629629629e-2 * t21512 - 0.33163888888888888888e-2 * t21514;
    (t21508, t21512, t21514, t21516)
}

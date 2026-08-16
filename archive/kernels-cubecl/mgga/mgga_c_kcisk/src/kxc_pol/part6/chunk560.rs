//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 560/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk560<F: Float>(t1212: F, t3696: F, t7802: F, t3704: F, t3711: F, t5668: F, t5736: F, t7738: F, t7742: F, t7746: F, t7758: F, t7765: F, t7771: F, t7773: F, t7777: F, t7780: F, t7783: F) -> (F, F) {
    let t7804 = t3696 * t7802 * t1212;
    let t7819 = -F::cast_from(0.1294625e1_f64) * t7758 + F::cast_from(0.258925e1_f64) * t7765 + t3704 + F::cast_from(0.20128333333333333334e0_f64) * t5668 - F::cast_from(0.20128333333333333333e0_f64) * t7738 + F::cast_from(0.60385e0_f64) * t7742 - F::cast_from(0.301925e0_f64) * t7746 + F::cast_from(0.82524375e-1_f64) * t7771 + F::cast_from(0.16504875e0_f64) * t7773 + t3711 + F::cast_from(0.11038e0_f64) * t5736 - F::cast_from(0.27595e-1_f64) * t7777 + F::cast_from(0.16557e0_f64) * t7780 - F::cast_from(0.82785e-1_f64) * t7783;
    (t7804, t7819)
}

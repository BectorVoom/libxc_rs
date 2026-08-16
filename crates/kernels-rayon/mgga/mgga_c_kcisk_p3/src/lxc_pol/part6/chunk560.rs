//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 560/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk560(t1212: f64, t3696: f64, t7802: f64, t3704: f64, t3711: f64, t5668: f64, t5736: f64, t7738: f64, t7742: f64, t7746: f64, t7758: f64, t7765: f64, t7771: f64, t7773: f64, t7777: f64, t7780: f64, t7783: f64) -> (f64, f64) {
    let t7804 = t3696 * t7802 * t1212;
    let t7819 = -0.1294625e1_f64 * t7758 + 0.258925e1_f64 * t7765 + t3704 + 0.20128333333333333334e0_f64 * t5668 - 0.20128333333333333333e0_f64 * t7738 + 0.60385e0_f64 * t7742 - 0.301925e0_f64 * t7746 + 0.82524375e-1_f64 * t7771 + 0.16504875e0_f64 * t7773 + t3711 + 0.11038e0_f64 * t5736 - 0.27595e-1_f64 * t7777 + 0.16557e0_f64 * t7780 - 0.82785e-1_f64 * t7783;
    (t7804, t7819)
}

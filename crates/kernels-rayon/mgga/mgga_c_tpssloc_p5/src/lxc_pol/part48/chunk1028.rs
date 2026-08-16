//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 1028/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk1028(t115704: f64, t115708: f64, t115712: f64, t115716: f64, t115718: f64, t115721: f64, t115725: f64, t115727: f64, t115728: f64, t115732: f64, t115738: f64, t117445: f64, t2075: f64, t24167: f64, t24543: f64, t24552: f64, t24935: f64, t31832: f64, t3929: f64, t510: f64, t7042: f64, t7220: f64, t8690: f64, t8840: f64) -> f64 {
    let t117634 = -2.0_f64 * t117445 * t510 - t2075 * t24543 - 2.0_f64 * t2075 * t24935 + t24167 * t8690 - 2.0_f64 * t24552 * t7042 - 2.0_f64 * t31832 * t7220 + t3929 * t8840 - t115704 - t115708 - t115712 + t115716 - t115718 + t115721 - t115725 - t115727 - t115728 - t115732 - t115738;
    t117634
}

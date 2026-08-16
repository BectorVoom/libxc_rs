//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2439/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2439(t21194: f64, t2888: f64, t41684: f64, t48799: f64, t48800: f64, t48809: f64, t59657: f64, t68442: f64, t68444: f64, t68446: f64, t68448: f64, t68479: f64, t68483: f64, t68486: f64, t68489: f64, t68492: f64, t68494: f64, t68498: f64, t68571: f64, t68577: f64, t68580: f64, t68583: f64) -> (f64, f64) {
    let t69380 = t21194 * t2888;
    let t69425 = 0.34246666666666666667e-1_f64 * t68442 + 0.57077777777777777777e-2_f64 * t68444 + 0.63419753086419753083e-2_f64 * t68446 - 0.2283111111111111111e-1_f64 * t68448 + t48799 - t48800 - t48809 + 0.17757530864197530864e-1_f64 * t41684 - 0.50735802469135802467e-1_f64 * t68479 - 0.41095999999999999999e0_f64 * t68483 + 0.20547999999999999999e0_f64 * t68486 - 0.34246666666666666665e-1_f64 * t68489 - 0.34246666666666666665e-1_f64 * t68492 + 0.11415555555555555555e-1_f64 * t68494 - 0.34246666666666666667e-1_f64 * t68498 - 0.1522074074074074074e-1_f64 * t59657 - 0.17123333333333333333e-1_f64 * t68571 + 0.41096e0_f64 * t68577 - 0.30822e0_f64 * t68580 + 0.10274e0_f64 * t68583;
    (t69380, t69425)
}

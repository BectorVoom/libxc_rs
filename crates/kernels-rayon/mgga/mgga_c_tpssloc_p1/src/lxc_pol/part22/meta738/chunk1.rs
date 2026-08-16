//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2423/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2423(t48103: f64, t49304: f64, t49306: f64, t49317: f64, t49322: f64, t68442: f64, t68444: f64, t68446: f64, t68448: f64, t68452: f64, t68454: f64, t41684: f64, t41863: f64, t68460: f64, t68464: f64, t68468: f64, t68472: f64, t68479: f64, t68483: f64, t68486: f64, t68489: f64, t68492: f64, t68494: f64) -> (f64, f64) {
    let t69066 = 0.103295e1_f64 * t68442 + 0.17215833333333333333e0_f64 * t68444 + 0.19128703703703703704e0_f64 * t68446 - 0.68863333333333333333e0_f64 * t68448 + t49304 - t49306 - t49317 - t49322 - 0.41678e0_f64 * t68452 + 0.69463333333333333333e-1_f64 * t68454 + 0.92617777777777777779e0_f64 * t48103;
    let t69079 = 0.62517e0_f64 * t68460 + 0.62517e0_f64 * t68464 - 0.104195e0_f64 * t68468 - 0.104195e0_f64 * t68472 + 0.5356037037037037037e0_f64 * t41684 + 0.30872592592592592592e0_f64 * t41863 - 0.15302962962962962963e1_f64 * t68479 - 0.123954e2_f64 * t68483 + 0.61977e1_f64 * t68486 - 0.103295e1_f64 * t68489 - 0.103295e1_f64 * t68492 + 0.34431666666666666667e0_f64 * t68494;
    (t69066, t69079)
}

//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 186/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk186(t608: f64, t65: f64, t34: f64, t36: f64, t43: f64, t607: f64, t55: f64, t583: f64, t61: f64, t59: f64, t39: f64, t44: f64, t51: f64, rho0: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t609 = t608 * t65;
    let t612 = t34 * rho0;
    let t614 = 1.0_f64 / t36 / t612;
    let t615 = sigma0 * t614;
    let t618 = t43 * t607;
    let t621 = t55 * t607;
    let t625 = 1.0_f64 / t61 / t583;
    let t626 = t59 * t625;
    let t627 = 8.0_f64 / 3.0_f64 * t626;
    let t628 = -8.0_f64 / 3.0_f64 * t615 * t44 + 5.0_f64 / 6.0_f64 * t39 * t618 - 5.0_f64 / 6.0_f64 * t51 * t621 + t627;
    (t609, t615, t621, t625, t626, t627, t628)
}

//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 540/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk540(t3990: f64, t607: f64, t3966: f64, t55: f64, t1414: f64, t1420: f64, t2282: f64, t39: f64, t3982: f64, t3985: f64, t51: f64, t615: f64, t621: f64) -> f64 {
    let t3991 = t3990 * t607;
    let t3994 = t55 * t3966;
    let t3997 = -20.0_f64 / 9.0_f64 * t615 * t1414 + 5.0_f64 / 18.0_f64 * t39 * t3982 + 5.0_f64 / 6.0_f64 * t39 * t3985 + 20.0_f64 / 9.0_f64 * t1420 * t621 + 5.0_f64 / 18.0_f64 * t51 * t3991 - 5.0_f64 / 6.0_f64 * t51 * t3994 - t2282;
    t3997
}

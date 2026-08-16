//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1333/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1333(t29754: f64, t85853: f64, t29624: f64, t7327: f64, t24667: f64, t6252: f64, t1653: f64, t8039: f64, t85822: f64, t24574: f64, t29741: f64, t29614: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t103610 = t85853 * t29754;
    let t103687 = t29624 * t7327;
    let t103694 = t24667 * t6252;
    let t103699 = t85822 * t1653 * t8039;
    let t103710 = t24574 * t29741;
    let t103723 = t29614 * t7327;
    (t103610, t103687, t103694, t103699, t103710, t103723)
}

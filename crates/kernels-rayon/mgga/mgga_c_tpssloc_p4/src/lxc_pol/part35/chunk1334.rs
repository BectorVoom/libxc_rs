//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1334/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1334(t24574: f64, t29702: f64, t6260: f64, t7327: f64, t24660: f64, t6252: f64, t27736: f64, t7999: f64, t24826: f64, t29716: f64, t8070: f64, t94490: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t103744 = t24574 * t29702;
    let t103767 = t7327 * t6260;
    let t103774 = t24660 * t6252;
    let t103799 = t7999 * t27736;
    let t103810 = t24826 * t29716;
    let t103830 = t94490 * t8070;
    (t103744, t103767, t103774, t103799, t103810, t103830)
}

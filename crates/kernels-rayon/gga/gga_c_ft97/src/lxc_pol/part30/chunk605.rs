//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 605/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk605(t27805: f64, t27807: f64, t375: f64, t6903: f64, t89: f64, t1131: f64, t747: f64, t2574: f64, t6119: f64, t24437: f64, t24447: f64, t92: f64) -> (f64, f64, f64, f64, f64) {
    let t27808 = t27805 * t27807;
    let t27811 = t89 * t375 * t6903;
    let t27814 = t1131 * t747;
    let t27816 = t2574 * t6119 * t27814;
    let t27817 = t24437 * t27816;
    let t27819 = t24447 * t92;
    (t27808, t27811, t27814, t27817, t27819)
}

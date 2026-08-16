//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 759/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk759(t5395: f64, t8624: f64, t5727: f64, t5743: f64, t5692: f64) -> (f64, f64, f64) {
    let t8704 = t5395 * t8624;
    let t8705 = t8704 * t5727;
    let t8707 = t8704 * t5743;
    let t8709 = 1.0_f64 / t5692;
    (t8705, t8707, t8709)
}

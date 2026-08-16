//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 971/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk971(t614: f64, t7339: f64, t32967: f64, t378: f64, t32706: f64, t5766: f64, t1349: f64, t1637: f64, t7314: f64, t23405: f64, t32881: f64, t165: f64, t32869: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t138438 = t7339 * t614;
    let t138445 = t378 * t32967;
    let t138476 = t5766 * t32706;
    let t138480 = 4.0_f64 / 27.0_f64 * t1349 * t1637 * t7314;
    let t138493 = t23405 * t32881;
    let t138511 = t32869 * t165;
    (t138438, t138445, t138476, t138480, t138493, t138511)
}

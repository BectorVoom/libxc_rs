//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 658/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk658(t3806: f64, t3807: f64, t3810: f64, t3829: f64, t3830: f64, t3831: f64, t3834: f64, t3904: f64, t3910: f64, t3914: f64, t884: f64, t125: f64, t1458: f64) -> (f64, f64) {
    let t3916 = -t3914 * t884 + t3806 + t3807 - t3810 + t3829 - t3830 - t3831 + t3834 - t3904 + t3910;
    let t3938 = t1458 * t125;
    (t3916, t3938)
}

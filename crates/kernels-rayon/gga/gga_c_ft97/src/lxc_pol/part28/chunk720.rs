//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 720/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk720(t26867: f64, t26922: f64, t26976: f64, t27196: f64, t27238: f64, t27285: f64, t27312: f64, t27403: f64, t609: f64, t6708: f64, t160: f64, t27391: f64) -> (f64, f64, f64) {
    let t27406 = t26867 + t26922 + t26976 + t27196 + t27238 + t27285 + t27312 + t27403;
    let t27411 = t6708 * t609;
    let t27414 = t27391 * t160;
    (t27406, t27411, t27414)
}

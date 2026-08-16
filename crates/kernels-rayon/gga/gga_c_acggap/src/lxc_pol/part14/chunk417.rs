//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 417/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk417(t1980: f64, t1994: f64, t382: f64, t587: f64) -> (f64, f64) {
    let t1995 = t1980 * t1994;
    let t1996 = 0.10482697429868050689e-3_f64 * t1995;
    let t1997 = t587 * t382;
    (t1996, t1997)
}

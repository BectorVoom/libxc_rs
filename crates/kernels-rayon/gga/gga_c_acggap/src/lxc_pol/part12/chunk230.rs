//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 230/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk230(t180: f64, t851: f64, t323: f64, t310: f64, t443: f64) -> (f64, f64, f64, f64) {
    let t852 = t851 * t180;
    let t854 = 0.13170898365871023197e1_f64 * t852 * t323;
    let t855 = t310 * t443;
    let t857 = t310 * t180;
    (t852, t854, t855, t857)
}

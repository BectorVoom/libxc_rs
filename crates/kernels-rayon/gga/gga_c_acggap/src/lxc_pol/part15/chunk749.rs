//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 749/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk749(t464: f64, t8331: f64, t633: f64, t864: f64, t2132: f64, t7885: f64, t862: f64, t865: f64, t103: f64, t566: f64, t95: f64, t1298: f64, t469: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8332 = t8331 * t464;
    let t8336 = t633 * t864;
    let t8337 = t2132 * t8336;
    let t8339 = 0.26020884564615598386e1_f64 * t7885 * t8337;
    let t8347 = t862 * t633;
    let t8349 = 0.13170898365871023197e1_f64 * t8347 * t865;
    let t8372 = t566 * t95 * t103;
    let t8382 = t469 * t1298;
    (t8332, t8336, t8337, t8339, t8347, t8349, t8372, t8382)
}

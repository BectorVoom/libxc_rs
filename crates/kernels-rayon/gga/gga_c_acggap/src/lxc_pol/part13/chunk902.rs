//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 902/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk902(t1998: f64, t3756: f64, t3761: f64, t141: f64, t167: f64, t2035: f64, t1096: f64, t1992: f64, t1089: f64, t2079: f64, t429: f64, t7542: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t30775 = t1998 * t3756;
    let t30777 = t1998 * t3761;
    let t30779 = t167 * t141;
    let t30780 = t2035 * t30779;
    let t30781 = t1992 * t1096;
    let t30782 = t30780 * t30781;
    let t30786 = t2079 * t1089 * t429 * t7542;
    (t30775, t30777, t30779, t30780, t30781, t30782, t30786)
}

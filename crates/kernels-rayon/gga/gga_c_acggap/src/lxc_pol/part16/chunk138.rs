//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 138/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk138(t161: f64, t413: f64, t151: f64, t177: f64, t383: f64) -> (f64, f64, f64, f64) {
    let t414 = t161 * t413;
    let t415 = t151 * t414;
    let t417 = 0.10003937560882938627e-2_f64 * t415 * t177;
    let t418 = t151 * t383;
    (t414, t415, t417, t418)
}

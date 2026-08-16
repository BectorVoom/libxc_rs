//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 355/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk355(t1423: f64, t384: f64, t1025: f64, t171: f64) -> (f64, f64) {
    let t1424 = t384 * t1423;
    let t1426 = t1025 * t171;
    (t1424, t1426)
}

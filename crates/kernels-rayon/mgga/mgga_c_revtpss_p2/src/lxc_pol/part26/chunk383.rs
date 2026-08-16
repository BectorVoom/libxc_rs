//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 383/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk383(t1945: f64, t213: f64, t248: f64, t209: f64, t785: f64) -> (f64, f64, f64, f64) {
    let t1946 = t213 * t1945;
    let t1947 = t1946 * t248;
    let t1954 = t209 * t209;
    let t1955 = t1954 * t785;
    (t1946, t1947, t1954, t1955)
}

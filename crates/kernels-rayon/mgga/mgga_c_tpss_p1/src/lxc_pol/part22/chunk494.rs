//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 494/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk494(t1953: f64, t1955: f64, t1957: f64, t1960: f64, t1962: f64, t1964: f64, t1967: f64, t1969: f64, t1973: f64, t574: f64, t577: f64) -> (f64, f64) {
    let t1974 = t1953 - t1955 + t1957 + t1960 - t1962 + t1964 + t1967 - t1969 + t1973;
    let t1976 = t574 * t577;
    (t1974, t1976)
}

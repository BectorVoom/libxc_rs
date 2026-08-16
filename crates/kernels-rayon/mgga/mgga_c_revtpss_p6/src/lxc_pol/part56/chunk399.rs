//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 399/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk399(t1949: f64, t233: f64, t1957: f64, t1951: f64, t1956: f64, t213: f64) -> (f64, f64, f64) {
    let t1958 = t233 * t1949;
    let t1959 = t1957 * t1958;
    let t1962 = 0.65854491829355115987e0_f64 * t213 * t1951 - 0.4336814094102599731e0_f64 * t1956 * t1959;
    (t1958, t1959, t1962)
}

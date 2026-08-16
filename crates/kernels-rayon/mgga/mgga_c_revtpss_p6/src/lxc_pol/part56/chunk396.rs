//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 396/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk396(t1941: f64, t218: f64, t816: f64, t234: f64, t64: f64, t213: f64, t248: f64) -> (f64, f64, f64) {
    let t1943 = t1941 * t218 * t816;
    let t1945 = t234 * t64;
    let t1946 = t213 * t1945;
    let t1947 = t1946 * t248;
    let t1949 = t1943 / 96.0_f64 + 0.42874018118069736972e-3_f64 * t1947;
    (t1945, t1946, t1949)
}

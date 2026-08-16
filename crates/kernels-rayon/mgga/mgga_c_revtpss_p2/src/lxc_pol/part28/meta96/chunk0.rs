//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 616/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk616(t2014: f64, t2035: f64, t118: f64, t1932: f64, t1939: f64, t2007: f64, t2011: f64, t508: f64, t569: f64, t3: f64, param_d: f64) -> (f64, f64, f64) {
    let t2036 = t2014 * t2035;
    let t2037 = -t118 * t2007 - t1932 * t508 + t2011 * t569 - t1939 + t2036;
    let t2038 = t3 * t2037;
    let t2040 = param_d * t2037;
    (t2037, t2038, t2040)
}

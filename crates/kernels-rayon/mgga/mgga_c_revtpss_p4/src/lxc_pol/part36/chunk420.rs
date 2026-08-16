//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 420/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk420(t1450: f64, t2034: f64, t2014: f64, t117: f64, t1936: f64, t572: f64, t55: f64, t61: f64, t68: f64, t72: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2035 = t2034 * t1450;
    let t2036 = t2014 * t2035;
    let t2042 = t117 * t1936;
    let t2044 = 3.0_f64 * t572 * t2042;
    let t2121 = t55 * t61 - t68;
    let t2122 = t2121 * t72;
    (t2035, t2036, t2042, t2044, t2121, t2122)
}

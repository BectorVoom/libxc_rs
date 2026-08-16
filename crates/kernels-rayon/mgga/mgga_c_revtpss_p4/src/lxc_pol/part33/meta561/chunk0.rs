//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1957/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1957(t30128: f64, t651: f64, t18245: f64, t1936: f64, t1501: f64, t1518: f64) -> (f64, f64, f64) {
    let t30130 = 2.0_f64 * t651 * t30128;
    let t30137 = 2.0_f64 * t18245 * t1936;
    let t30138 = t1501 * t1518;
    (t30130, t30137, t30138)
}

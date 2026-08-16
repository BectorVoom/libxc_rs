//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1766/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1766(t1032: f64, t2769: f64, t233: f64) -> (f64, f64) {
    let t25373 = t1032 * t2769;
    let t25374 = t25373 * t233;
    (t25373, t25374)
}

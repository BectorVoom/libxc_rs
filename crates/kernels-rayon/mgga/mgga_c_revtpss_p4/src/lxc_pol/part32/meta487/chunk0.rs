//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1734/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1734(t197: f64, t530: f64, t2013: f64, t1450: f64, t5591: f64, t8995: f64) -> (f64, f64, f64, f64) {
    let t28166 = t197 * t530;
    let t28167 = t2013 * t28166;
    let t28176 = t1450 * t5591;
    let t28196 = t2013 * t8995;
    (t28166, t28167, t28176, t28196)
}

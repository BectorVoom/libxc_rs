//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1968/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1968(t27879: f64, t27907: f64, t27984: f64, t28017: f64, t532: f64, t1450: f64, t2014: f64, t1931: f64, t670: f64) -> (f64, f64, f64, f64, f64) {
    let t28019 = t27879 + t27907 + t27984 + t28017;
    let t28020 = t532 * t28019;
    let t28021 = t28020 * t1450;
    let t28022 = t2014 * t28021;
    let t28025 = t1931 * t670;
    (t28019, t28020, t28021, t28022, t28025)
}

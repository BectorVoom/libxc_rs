//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1995/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1995(t1940: f64, t2071: f64, t9342: f64, t28309: f64, t686: f64, t72: f64, t25375: f64, t1957: f64, t28425: f64, t25372: f64, t98809: f64, t25386: f64) -> (f64, f64, f64, f64, f64) {
    let t102917 = 3.0_f64 * t1940 * t2071 * t9342;
    let t102928 = t28309 * t72 * t686;
    let t102930 = 0.28912093960683998208e-1_f64 * t25375 * t102928;
    let t102931 = t1957 * t28425;
    let t102934 = 0.28912093960683998208e-1_f64 * t25372 * t102931 * t98809;
    let t102937 = 0.51405703062096148812e-1_f64 * t25386 * t102931 * t98809;
    (t102917, t102928, t102930, t102934, t102937)
}

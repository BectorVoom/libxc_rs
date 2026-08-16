//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2006/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2006(t3063: f64, t8521: f64, t11200: f64, t7143: f64, t1035: f64, t1983: f64, t36870: f64, t1096: f64, t19482: f64, t27668: f64, t995: f64, t4982: f64, t988: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t94042 = t3063 * t8521;
    let t94053 = t11200 * t7143;
    let t94063 = t1983 * t36870 * t1035;
    let t94064 = t19482 * t1096;
    let t94080 = t995 * t27668;
    let t94081 = t4982 * t988;
    (t94042, t94053, t94063, t94064, t94080, t94081)
}

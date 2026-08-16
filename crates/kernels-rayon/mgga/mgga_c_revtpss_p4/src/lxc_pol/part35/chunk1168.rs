//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1168/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1168(t29513: f64, t7349: f64, t28640: f64, t7702: f64, t1923: f64, t29532: f64, t7348: f64, t29551: f64, t101788: f64, t7706: f64, t29538: f64, t26179: f64, t29544: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t109983 = t29513 * t7349;
    let t109985 = t7702 * t28640;
    let t109988 = t1923 * t7348 * t29532;
    let t109990 = t29551 * t7349;
    let t110008 = t101788 * t7706;
    let t110010 = t29538 * t7349;
    let t110014 = t26179 * t29544;
    (t109983, t109985, t109988, t109990, t110008, t110010, t110014)
}

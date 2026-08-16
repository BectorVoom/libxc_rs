//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1998/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1998(t2247: f64, t5819: f64, t68: f64, t1469: f64, t603: f64, t29513: f64, t7349: f64, t28640: f64, t7702: f64, t1923: f64, t29532: f64, t7348: f64) -> (f64, f64, f64, f64, f64) {
    let t109976 = t2247 * t5819 * t68;
    let t109980 = t603 * t1469 * t68;
    let t109983 = t29513 * t7349;
    let t109985 = t7702 * t28640;
    let t109988 = t1923 * t7348 * t29532;
    (t109976, t109980, t109983, t109985, t109988)
}

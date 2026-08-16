//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2249/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2249(t27833: f64, t7935: f64, t1448: f64, t6922: f64, t28196: f64, t28197: f64, t28067: f64, t98450: f64, t7897: f64, t8995: f64, t28199: f64, t25082: f64, t27153: f64, t33651: f64) -> (f64, f64, f64, f64, f64) {
    let t109262 = 2.0_f64 * t27833 * t7935;
    let t109263 = t6922 * t1448;
    let t109266 = 2.0_f64 * t28196 * t28197 * t109263;
    let t109268 = 6.0_f64 * t98450 * t28067;
    let t109269 = t7897 * t8995;
    let t109271 = 4.0_f64 * t109269 * t28199;
    let t109274 = 6.0_f64 * t25082 * t33651 * t27153;
    (t109262, t109266, t109268, t109271, t109274)
}

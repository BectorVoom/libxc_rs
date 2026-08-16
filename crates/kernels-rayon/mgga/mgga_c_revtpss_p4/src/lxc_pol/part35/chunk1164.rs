//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1164/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1164(t30261: f64, t689: f64, t25904: f64, t25899: f64, t1358: f64, t212: f64, t30247: f64, t1904: f64, t28824: f64, t109407: f64, t7289: f64, t27884: f64, t28845: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t109457 = t30261 * t689;
    let t109458 = t25904 * t109457;
    let t109460 = t25899 * t109457;
    let t109488 = t689 * t212 * t30247 * t1358;
    let t109505 = t689 * t28824 * t1904;
    let t109512 = t7289 * t109407;
    let t109514 = t27884 * t28845;
    (t109458, t109460, t109488, t109505, t109512, t109514)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2176/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2176(t25759: f64, t77425: f64, t100987: f64, t27375: f64, t106625: f64, t29598: f64, t94245: f64, t1711: f64, t4343: f64, t106561: f64, t27799: f64, t105923: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t107882 = t25759 * t77425;
    let t107885 = t100987 * t27375;
    let t107892 = t25759 * t106625;
    let t107895 = t94245 * t29598;
    let t107901 = t1711 * t4343;
    let t107908 = t27799 * t106561;
    let t107919 = t25759 * t105923;
    (t107882, t107885, t107892, t107895, t107901, t107908, t107919)
}

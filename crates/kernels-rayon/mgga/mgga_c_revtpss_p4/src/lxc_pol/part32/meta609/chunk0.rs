//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1948/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1948(t106625: f64, t25207: f64, t27375: f64, t63185: f64, t11064: f64, t1544: f64, t27384: f64, t25759: f64, t77425: f64, t100987: f64, t29598: f64, t94245: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t106626 = t25207 * t106625;
    let t107793 = t63185 * t27375;
    let t107805 = t11064 * t1544 * t27384;
    let t107882 = t25759 * t77425;
    let t107885 = t100987 * t27375;
    let t107892 = t25759 * t106625;
    let t107895 = t94245 * t29598;
    (t106626, t107793, t107805, t107882, t107885, t107892, t107895)
}

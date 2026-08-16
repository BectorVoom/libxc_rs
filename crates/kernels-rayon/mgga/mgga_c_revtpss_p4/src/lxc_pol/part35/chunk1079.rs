//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1079/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1079(t1955: f64, t6041: f64, t30: f64, t6079: f64, t1468: f64, t1583: f64, t6075: f64, t33: f64, t5966: f64, t25759: f64, t29598: f64, t1544: f64, t1711: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t29698 = t1955 * t6041;
    let t29713 = t30 * t6079;
    let t29716 = t1468 * t1583;
    let t29719 = t30 * t6075;
    let t29939 = t33 * t5966;
    let t29946 = t25759 * t29598;
    let t29949 = t1711 * t1544;
    (t29698, t29713, t29716, t29719, t29939, t29946, t29949)
}

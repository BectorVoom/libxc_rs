//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 531/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk531(t1132: f64, t1723: f64, t1139: f64, t1145: f64, t1715: f64, t141: f64, t1137: f64, t1144: f64, t1717: f64, t1150: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1724 = t1132 * t1723;
    let t1727 = t1139 * t1723;
    let t1729 = t1145 * t1715;
    let t1730 = t141 * t1729;
    let t1732 = 0.1898925e1_f64 * t1724 - t1137 + 0.29896666666666666667e0_f64 * t1717 + 0.3071625e0_f64 * t1727 - t1144 + 0.82156666666666666667e-1_f64 * t1730;
    let t1733 = t1732 * t1150;
    (t1724, t1727, t1729, t1730, t1732, t1733)
}

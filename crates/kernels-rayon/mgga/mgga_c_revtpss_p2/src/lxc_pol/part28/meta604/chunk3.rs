//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2087/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2087(t25875: f64, t97703: f64, t97705: f64, t122: f64, t3916: f64, t72: f64, t7910: f64, t25895: f64, t1398: f64, t543: f64, t5774: f64, t1903: f64, t4056: f64) -> (f64, f64, f64, f64, f64) {
    let t97719 = 0.25702851531048074406e-1_f64 * t25875 * t97703 * t97705;
    let t97732 = t7910 * t72 * t122 * t3916;
    let t97734 = 0.28912093960683998208e-1_f64 * t25895 * t97732;
    let t97737 = t5774 * t1398 * t543;
    let t97742 = t1903 * t4056 * t543;
    (t97719, t97732, t97734, t97737, t97742)
}

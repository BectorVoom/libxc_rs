//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1046/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1046(t247: f64, t2858: f64, t3109: f64, t1063: f64, t140: f64, t3247: f64, t1011: f64, t3254: f64, t3237: f64, t245: f64, t3089: f64, t3088: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11744 = t247 * t3109 * t2858;
    let t11745 = t1063 * t11744;
    let t11752 = t140 * t3247;
    let t11753 = t1011 * t11752;
    let t11755 = t140 * t3254;
    let t11756 = t1011 * t11755;
    let t11762 = t140 * t3237;
    let t11763 = t1011 * t11762;
    let t11772 = t3089 * t245;
    let t11773 = t3088 * t11772;
    (t11745, t11753, t11756, t11763, t11772, t11773)
}

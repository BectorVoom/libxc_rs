//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2041/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2041(t11782: f64, t1972: f64, t1007: f64, t25532: f64, t3080: f64, t7106: f64, t11735: f64, t1968: f64, t11772: f64, t25515: f64, t3114: f64, t11923: f64, t25580: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t93736 = t11782 * t1972;
    let t93743 = t25532 * t1007;
    let t93745 = t7106 * t3080;
    let t93750 = 5.0_f64 / 1296.0_f64 * t1968 * t11735;
    let t93751 = t25515 * t11772;
    let t93752 = t3114 * t93751;
    let t93755 = t25580 * t11923;
    (t93736, t93743, t93745, t93750, t93751, t93752, t93755)
}

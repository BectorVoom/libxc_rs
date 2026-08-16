//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1281/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1281(t11735: f64, t1968: f64, t11772: f64, t25515: f64, t3114: f64, t11923: f64, t25580: f64, t11240: f64, t11244: f64, t7120: f64, t12020: f64, t7121: f64) -> (f64, f64, f64, f64, f64) {
    let t93750 = 5.0_f64 / 1296.0_f64 * t1968 * t11735;
    let t93751 = t25515 * t11772;
    let t93752 = t3114 * t93751;
    let t93755 = t25580 * t11923;
    let t93758 = t11240 * t7120 * t11244;
    let t93761 = t12020 * t7121;
    (t93750, t93752, t93755, t93758, t93761)
}

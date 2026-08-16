//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1352/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1352(t508: f64, t651: f64, t94991: f64, t2014: f64, t25177: f64, t7312: f64, t25178: f64, t7235: f64, t10416: f64, t7003: f64, t1937: f64, t49693: f64) -> (f64, f64, f64, f64, f64) {
    let t95049 = 2.0_f64 * t651 * t508 * t94991;
    let t95056 = 6.0_f64 * t2014 * t7312 * t25177;
    let t95058 = 6.0_f64 * t7235 * t25178;
    let t95066 = 6.0_f64 * t10416 * t7003;
    let t95068 = 6.0_f64 * t49693 * t1937;
    (t95049, t95056, t95058, t95066, t95068)
}

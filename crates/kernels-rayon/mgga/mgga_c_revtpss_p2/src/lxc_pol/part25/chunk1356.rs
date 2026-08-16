//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1356/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1356(t94365: f64, t95026: f64, t95071: f64, t95117: f64, t1464: f64, t7318: f64, t26093: f64, t575: f64, t10259: f64, t572: f64, t7330: f64, t117: f64, t94991: f64) -> (f64, f64, f64, f64, f64) {
    let t95119 = t94365 + t95026 + t95071 + t95117;
    let t95125 = t7318 * t1464;
    let t95127 = t26093 * t575;
    let t95131 = 6.0_f64 * t572 * t7330 * t10259;
    let t95136 = 3.0_f64 * t572 * t117 * t94991;
    (t95119, t95125, t95127, t95131, t95136)
}

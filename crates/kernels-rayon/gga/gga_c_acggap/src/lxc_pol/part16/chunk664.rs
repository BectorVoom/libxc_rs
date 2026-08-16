//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 664/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk664(t6421: f64, t6441: f64, t6453: f64, t6574: f64, t1717: f64, t3952: f64, t1941: f64, t814: f64, t157: f64, t513: f64, t524: f64, t506: f64) -> (f64, f64, f64, f64, f64) {
    let t6576 = t6421 + t6441 + t6453 + t6574;
    let t6596 = t1717 * t3952;
    let t6614 = t1941 * t814;
    let t6841 = t513 * t524 * t157;
    let t6847 = t506 * t524 * t157;
    (t6576, t6596, t6614, t6841, t6847)
}

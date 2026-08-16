//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1917/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1917(t1214: f64, t7637: f64, t8201: f64, t8197: f64, t2142: f64, t5497: f64, t7652: f64, t1209: f64, t29135: f64) -> (f64, f64, f64, f64, f64) {
    let t29264 = t7637 * t8201 * t1214;
    let t29268 = t7637 * t8197 * t1214;
    let t29271 = t2142 * t5497;
    let t29272 = t7652 * t29271;
    let t29275 = t1209 * t29135;
    (t29264, t29268, t29271, t29272, t29275)
}

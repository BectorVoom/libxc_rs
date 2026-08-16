//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2098/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2098(t104852: f64, t3767: f64, t3782: f64, t1224: f64, t139: f64, t29047: f64, t5052: f64, t3698: f64, t5047: f64, t26866: f64, t5436: f64, t17225: f64, t7624: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t104853 = t3767 * t104852;
    let t104856 = t3782 * t104852;
    let t104863 = t29047 * t139 * t1224 * t5052 / 216.0_f64;
    let t104872 = t29047 * t139 * t3698 * t5047 / 324.0_f64;
    let t104888 = t5436 * t26866;
    let t104894 = t7624 * t17225;
    (t104853, t104856, t104863, t104872, t104888, t104894)
}

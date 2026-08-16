//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3605/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3605(t68297: f64, t68301: f64, t68305: f64, t68310: f64, t68332: f64, t68334: f64, t68336: f64, t68342: f64, t68347: f64, t68350: f64, t68353: f64, t68357: f64, t68360: f64) -> f64 {
    let t68429 = 4.0_f64 / 3.0_f64 * t68297 + 2.0_f64 / 3.0_f64 * t68301 + 2.0_f64 * t68305 - 80.0_f64 / 81.0_f64 * t68310 + 4.0_f64 / 27.0_f64 * t68332 + 8.0_f64 / 27.0_f64 * t68334 + 8.0_f64 / 9.0_f64 * t68336 + 10.0_f64 / 27.0_f64 * t68342 + 40.0_f64 / 9.0_f64 * t68347 - 4.0_f64 / 3.0_f64 * t68350 - 8.0_f64 * t68353 - 4.0_f64 / 9.0_f64 * t68357 + 8.0_f64 * t68360;
    t68429
}

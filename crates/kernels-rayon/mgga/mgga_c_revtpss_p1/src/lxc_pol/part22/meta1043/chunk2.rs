//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3648/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3648(t68297: f64, t68301: f64, t68305: f64, t68310: f64, t68332: f64, t68334: f64, t68336: f64, t68342: f64, t68347: f64, t68350: f64, t68353: f64, t68357: f64, t68360: f64) -> f64 {
    let t68997 = 0.37083333333333333334e-1_f64 * t68297 + 0.18541666666666666667e-1_f64 * t68301 + 0.55625000000000000001e-1_f64 * t68305 - 0.27469135802469135803e-1_f64 * t68310 + 0.41203703703703703704e-2_f64 * t68332 + 0.82407407407407407407e-2_f64 * t68334 + 0.24722222222222222222e-1_f64 * t68336 + 0.10300925925925925926e-1_f64 * t68342 + 0.12361111111111111111e0_f64 * t68347 - 0.37083333333333333333e-1_f64 * t68350 - 0.22249999999999999999e0_f64 * t68353 - 0.12361111111111111111e-1_f64 * t68357 + 0.2225e0_f64 * t68360;
    t68997
}

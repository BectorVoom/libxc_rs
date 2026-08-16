//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3070/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3070(t43888: f64, t56176: f64, t56184: f64, t56229: f64, t56236: f64, t68332: f64, t68334: f64, t68336: f64, t68389: f64, t68399: f64, t68454: f64, t68456: f64, t81224: f64, t81228: f64, t81230: f64, t81232: f64, t81234: f64, t81236: f64, t81242: f64, t81245: f64) -> f64 {
    let t81250 = 0.61805555555555555556e-2_f64 * t68332 + 0.12361111111111111111e-1_f64 * t68334 + 0.37083333333333333333e-1_f64 * t68336 - 0.82407407407407407407e-2_f64 * t56176 + t56184 + 0.166875e0_f64 * t81224 + 0.92708333333333333333e-2_f64 * t81228 - 0.34336419753086419753e-2_f64 * t81230 + 0.12361111111111111111e-1_f64 * t81232 - 0.18541666666666666667e-1_f64 * t81234 - 0.30902777777777777778e-2_f64 * t81236 + t56229 - 0.28842592592592592592e-1_f64 * t56236 - 0.92708333333333333334e-2_f64 * t68389 + 0.24722222222222222223e-1_f64 * t68399 + 0.30902777777777777777e-1_f64 * t81242 - 0.11125e0_f64 * t81245 - 0.96141975308641975307e-2_f64 * t43888 - 0.37083333333333333334e-1_f64 * t68454 - 0.55625000000000000001e-1_f64 * t68456;
    t81250
}

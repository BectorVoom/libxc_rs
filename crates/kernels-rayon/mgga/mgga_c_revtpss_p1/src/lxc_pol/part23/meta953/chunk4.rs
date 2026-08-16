//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3167/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3167(t43888: f64, t56176: f64, t56236: f64, t56447: f64, t56462: f64, t68332: f64, t68334: f64, t68336: f64, t68389: f64, t68399: f64, t68454: f64, t68456: f64, t81224: f64, t81228: f64, t81230: f64, t81232: f64, t81234: f64, t81236: f64, t81242: f64, t81245: f64) -> f64 {
    let t83230 = 0.55555555555555555556e-2_f64 * t68332 + 0.11111111111111111111e-1_f64 * t68334 + 0.33333333333333333333e-1_f64 * t68336 - 0.74074074074074074073e-2_f64 * t56176 + t56447 + 0.15e0_f64 * t81224 + 0.83333333333333333333e-2_f64 * t81228 - 0.30864197530864197531e-2_f64 * t81230 + 0.11111111111111111111e-1_f64 * t81232 - 0.16666666666666666667e-1_f64 * t81234 - 0.27777777777777777778e-2_f64 * t81236 + t56462 - 0.25925925925925925926e-1_f64 * t56236 - 0.83333333333333333334e-2_f64 * t68389 + 0.22222222222222222223e-1_f64 * t68399 + 0.27777777777777777777e-1_f64 * t81242 - 0.99999999999999999998e-1_f64 * t81245 - 0.86419753086419753087e-2_f64 * t43888 - 0.33333333333333333334e-1_f64 * t68454 - 0.50000000000000000001e-1_f64 * t68456;
    t83230
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2896/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2896(t51973: f64, t41281: f64, t41283: f64, t41285: f64, t41287: f64, t41289: f64, t41292: f64, t41690: f64, t51961: f64, t51965: f64, t51967: f64, t51971: f64) -> f64 {
    let t52573 = 0.68863333333333333332e0_f64 * t51973;
    let t52574 = 0.69463333333333333333e0_f64 * t41281 - 0.13892666666666666667e0_f64 * t41283 - 0.34731666666666666666e0_f64 * t41285 - 0.11577222222222222222e0_f64 * t41287 + 0.69463333333333333333e-1_f64 * t41289 + 0.30872592592592592592e-1_f64 * t41292 + t41690 + 0.61977e1_f64 * t51961 - 0.17215833333333333333e1_f64 * t51965 + 0.51647499999999999999e0_f64 * t51967 - 0.516475e0_f64 * t51971 - t52573;
    t52574
}

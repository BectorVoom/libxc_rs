//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2845/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2845(t51973: f64, t41281: f64, t41283: f64, t41285: f64, t41287: f64, t41289: f64, t41292: f64, t41307: f64, t51961: f64, t51965: f64, t51967: f64, t51971: f64) -> f64 {
    let t51974 = 0.40256666666666666668e0_f64 * t51973;
    let t51975 = 0.55190000000000000001e0_f64 * t41281 - 0.11038e0_f64 * t41283 - 0.27595e0_f64 * t41285 - 0.91983333333333333335e-1_f64 * t41287 + 0.5519e-1_f64 * t41289 + 0.24528888888888888889e-1_f64 * t41292 + t41307 + 0.36231e1_f64 * t51961 - 0.10064166666666666667e1_f64 * t51965 + 0.30192500000000000001e0_f64 * t51967 - 0.301925e0_f64 * t51971 - t51974;
    t51975
}

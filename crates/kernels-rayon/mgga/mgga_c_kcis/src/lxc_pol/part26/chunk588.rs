//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 588/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk588(t3795: f64, t3881: f64, t4338: f64, t4345: f64, t5469: f64, t5472: f64, t5475: f64, t5479: f64, t5514: f64, t5516: f64, t5557: f64, t5559: f64, t5562: f64, t5565: f64, t5568: f64, t5571: f64) -> f64 {
    let t6097 = -0.17648625e1_f64 * t5514 + 0.3529725e1_f64 * t5516 + t4338 + 0.17215833333333333333e0_f64 * t3795 + 0.17215833333333333333e0_f64 * t5469 - 0.34431666666666666667e0_f64 * t5472 + 0.103295e1_f64 * t5475 + 0.103295e1_f64 * t5479 + 0.31558125e0_f64 * t5557 + 0.6311625e0_f64 * t5559 + t4345 + 0.69463333333333333333e-1_f64 * t3881 + 0.69463333333333333333e-1_f64 * t5562 - 0.34731666666666666667e-1_f64 * t5565 + 0.20839e0_f64 * t5568 + 0.20839e0_f64 * t5571;
    t6097
}

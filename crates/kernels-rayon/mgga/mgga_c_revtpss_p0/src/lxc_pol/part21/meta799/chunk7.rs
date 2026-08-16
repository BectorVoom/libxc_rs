//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2900/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2900(t52126: f64, t41308: f64, t41330: f64, t41332: f64, t41334: f64, t41336: f64, t41365: f64, t41367: f64, t41441: f64, t52112: f64, t52128: f64, t52130: f64) -> f64 {
    let t52623 = 0.34731666666666666667e0_f64 * t52126;
    let t52628 = -0.103295e1_f64 * t41365 + 0.34431666666666666666e0_f64 * t41367 + 0.103295e1_f64 * t41308 - 0.68863333333333333332e0_f64 * t41330 - 0.45908888888888888888e0_f64 * t41332 + 0.17215833333333333333e0_f64 * t41334 + 0.19128703703703703703e0_f64 * t41336 - t52623 + 0.30872592592592592592e0_f64 * t52128 + 0.3529725e1_f64 * t52130 - 0.929655e1_f64 * t52112 + 0.92617777777777777776e0_f64 * t41441;
    t52628
}

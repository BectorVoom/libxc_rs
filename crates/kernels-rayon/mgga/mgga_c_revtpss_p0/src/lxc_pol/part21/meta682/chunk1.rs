//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2496/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2496(t13037: f64, t472: f64, t3603: f64, t482: f64, t675: f64, t828: f64) -> (f64, f64, f64, f64) {
    let t44531 = 1.0_f64 / t13037 / t472;
    let t44535 = t3603 * t3603;
    let t44545 = t675 * t482;
    let t44546 = t828 * t44545;
    (t44531, t44535, t44545, t44546)
}

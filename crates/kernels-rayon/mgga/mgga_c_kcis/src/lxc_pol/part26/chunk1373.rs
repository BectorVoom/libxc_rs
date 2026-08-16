//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1373/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1373(t21499: f64, t6176: f64, t7899: f64, t102475: f64, t102478: f64, t102481: f64, t102484: f64, t102498: f64, t2237: f64, t28535: f64, t8144: f64, t98574: f64, t98587: f64, t98598: f64, t98604: f64) -> (f64, f64) {
    let t103582 = t6176 * t7899 * t21499;
    let t103586 = 0.13901041666666666667e-2_f64 * t8144 * t28535 + t98574 + t98587 - 0.24320185185185185185e-1_f64 * t102475 + 0.1621345679012345679e-1_f64 * t102478 - 0.88437037037037037034e-2_f64 * t102481 + 0.16581944444444444444e-2_f64 * t102484 - t98598 - t98604 + 0.69505208333333333333e-3_f64 * t2237 * t103582 - 0.16581944444444444444e-2_f64 * t102498;
    (t103582, t103586)
}

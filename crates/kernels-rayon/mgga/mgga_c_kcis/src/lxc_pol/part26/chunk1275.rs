//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1275/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1275(t101849: f64, t101853: f64, t27607: f64, t28714: f64, t28749: f64, t28755: f64, t28760: f64, t28772: f64, t29510: f64, t7971: f64, t8213: f64, t98978: f64, t98986: f64, t98988: f64, t99013: f64, t99331: f64) -> f64 {
    let t101862 = 0.34752604166666666667e-3_f64 * t27607 * t29510 + 0.69505208333333333334e-3_f64 * t99013 * t8213 - 0.82448622685185185187e-4_f64 * t101849 + 0.69505208333333333334e-3_f64 * t28714 * t28772 - t98978 - t98986 - t98988 - 0.18534722222222222222e-2_f64 * t101853 * t7971 - 0.61782407407407407408e-3_f64 * t99331 * t28749 - 0.61782407407407407408e-3_f64 * t99331 * t28755 - 0.12356481481481481482e-2_f64 * t99331 * t28760;
    t101862
}

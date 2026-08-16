//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1295/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1295(t100522: f64, t100525: f64, t100528: f64, t100531: f64, t101208: f64, t14554: f64, t19399: f64, t26679: f64, t26692: f64, t27812: f64, t27822: f64, t27832: f64, t27950: f64, t27954: f64, t28928: f64, t7703: f64, t93366: f64, t95524: f64) -> f64 {
    let t101303 = -0.66327777777777777776e-2_f64 * t100522 + 0.18534722222222222222e-2_f64 * t7703 * t14554 * t26679 * t19399 + 0.61836467013888888889e-4_f64 * t93366 * t28928 - 0.16581944444444444444e-2_f64 * t100525 - 0.61890573922526041667e-5_f64 * t27812 * t101208 + 0.66327777777777777776e-2_f64 * t100528 + 0.11054629629629629629e-2_f64 * t100531 + 0.61836467013888888889e-4_f64 * t95524 * t27822 - 0.61782407407407407408e-3_f64 * t27832 * t27950 - 0.12356481481481481482e-2_f64 * t26692 * t28928 + 0.46336805555555555557e-3_f64 * t27832 * t27954;
    t101303
}

//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1305/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1305(t16732: f64, t12119: f64, t16349: f64, t16676: f64, t16679: f64, t16682: f64, t16688: f64, t16697: f64, t16702: f64, t16704: f64, t16706: f64, t16708: f64, t16713: f64, t16717: f64, t16720: f64, t16724: f64, t16728: f64, t16731: f64, t3961: f64, t507: f64) -> f64 {
    let t16733 = 0.14739506172839506172e-2_f64 * t16732;
    let t16734 = -0.33163888888888888888e-2_f64 * t16676 - 0.88437037037037037034e-2_f64 * t16679 + 0.178089025e-1_f64 * t3961 * t16682 + 0.27636574074074074073e-2_f64 * t16688 + 0.73697530864197530861e-2_f64 * t16697 - 0.22109259259259259258e-2_f64 * t12119 - 0.33163888888888888888e-2_f64 * t16702 - 0.33163888888888888888e-2_f64 * t16704 + 0.22109259259259259258e-2_f64 * t16706 - 0.66327777777777777776e-2_f64 * t16708 + 0.55273148148148148146e-2_f64 * t16713 + t16349 * t507 - 0.66327777777777777776e-2_f64 * t16717 - t16720 + 0.33163888888888888888e-2_f64 * t16724 - 0.73697530864197530862e-2_f64 * t16728 + t16731 + t16733;
    t16734
}

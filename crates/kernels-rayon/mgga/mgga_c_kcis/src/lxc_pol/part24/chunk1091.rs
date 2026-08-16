//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1091/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1091(t26685: f64, t27799: f64, t27804: f64, t27849: f64, t27857: f64, t28905: f64, t28909: f64, t28913: f64, t28917: f64, t28920: f64, t28925: f64, t28928: f64) -> f64 {
    let t28931 = 0.22109259259259259258e-2_f64 * t27799 - 0.15445601851851851852e-3_f64 * t27804 + 0.33163888888888888888e-2_f64 * t28905 + 0.16581944444444444444e-2_f64 * t28909 + 0.27636574074074074073e-2_f64 * t28913 - 0.33163888888888888888e-2_f64 * t28917 + 0.24872916666666666666e-2_f64 * t28920 + 0.22109259259259259258e-2_f64 * t27849 + 0.46336805555555555556e-3_f64 * t27857 - 0.33163888888888888888e-2_f64 * t28925 + 0.61836467013888888889e-4_f64 * t26685 * t28928;
    t28931
}

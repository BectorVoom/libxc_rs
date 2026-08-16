//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1246/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1246(t20839: f64, t20851: f64, t44: f64, t230: f64, t18435: f64, t18438: f64, t18441: f64, t18447: f64, t18449: f64, t18451: f64, t18454: f64, t18456: f64, t20819: f64, t20821: f64, t20824: f64, t20826: f64, t8524: f64, t9272: f64, t9313: f64, t9315: f64) -> f64 {
    let t20853 = (t20839 + t20851) * t44;
    let t20854 = t20853 * t230;
    let t20855 = -t18435 / 16.0_f64 + t8524 + t9315 - t18438 / 8.0_f64 - t18441 / 8.0_f64 - t18447 / 16.0_f64 - t9313 - t18449 / 8.0_f64 + t18451 / 16.0_f64 - t18454 / 16.0_f64 + t18456 / 8.0_f64 - t20819 / 16.0_f64 - t9272 + t20821 / 8.0_f64 - t20824 / 8.0_f64 + t20826 / 8.0_f64 + t20854;
    t20855
}

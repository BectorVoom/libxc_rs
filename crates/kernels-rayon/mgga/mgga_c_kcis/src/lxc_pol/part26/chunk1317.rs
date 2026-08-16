//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1317/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1317(t102081: f64, t102280: f64, t102563: f64, t102568: f64, t102575: f64, t102582: f64, t102586: f64, t20984: f64, t27567: f64, t27583: f64, t28758: f64, t28765: f64, t28807: f64, t4440: f64, t6159: f64, t8222: f64, t99219: f64, t99301: f64, t99556: f64) -> f64 {
    let t102594 = -0.17411041666666666666e-2_f64 * t102563 + 0.61782407407407407408e-3_f64 * t99219 * t8222 - 0.30918233506944444444e-4_f64 * t27567 * t102568 - 0.92754700520833333333e-4_f64 * t27567 * t102280 - 0.61836467013888888888e-4_f64 * t27567 * t102081 + t99556 - 0.23168402777777777778e-3_f64 * t27583 * t4440 * t28758 * t102575 - 0.23168402777777777778e-3_f64 * t27583 * t102582 - 0.51588271604938271603e-3_f64 * t102586 + 0.23168402777777777778e-3_f64 * t99301 * t28807 - 0.69505208333333333334e-3_f64 * t27583 * t6159 * t28765 * t20984;
    t102594
}

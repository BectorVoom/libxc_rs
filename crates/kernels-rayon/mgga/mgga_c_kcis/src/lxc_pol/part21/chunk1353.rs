//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1353/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1353(t27014: f64, t28160: f64, t15573: f64, t28210: f64, t7788: f64, t15255: f64, t92693: f64, t96736: f64, t26955: f64, t26966: f64, t27090: f64, t28190: f64, t28211: f64, t8091: f64, t92590: f64, t92814: f64, t92816: f64, t92818: f64, t92820: f64, t92822: f64, t93050: f64, t96720: f64) -> (f64, f64) {
    let t96952 = 0.23168402777777777778e-3_f64 * t27014 * t28160;
    let t96957 = 0.23168402777777777778e-3_f64 * t7788 * t15573 * t28210;
    let t96968 = t92693 * t96736 * t15255;
    let t96971 = 0.34752604166666666667e-3_f64 * t28190 * t27090 + t96952 - 0.18534722222222222222e-2_f64 * t26966 * t28211 + t96957 + 0.11584201388888888889e-3_f64 * t92814 + 0.30918233506944444444e-4_f64 * t92816 + 0.77382407407407407407e-3_f64 * t92818 + 0.12897067901234567901e-2_f64 * t92820 - 0.61782407407407407408e-3_f64 * t92822 + 0.24777891269883300782e-5_f64 * t93050 * t96720 - 0.11584201388888888889e-3_f64 * t92590 * t8091 - 0.30918233506944444444e-4_f64 * t26955 * t96968;
    (t96968, t96971)
}

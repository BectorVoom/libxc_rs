//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 920/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk920(t28241: f64, t28273: f64, t28323: f64, t28728: f64, t28754: f64, t28785: f64, t28955: f64, t29346: f64, t752: f64, t24081: f64, t2594: f64, t17775: f64, t8968: f64) -> (f64, f64, f64) {
    let t29349 = t28241 + t28273 + t28323 + t28728 + t28754 + t28785 + t28955 + t29346;
    let t29350 = t29349 * t752;
    let t29352 = 3.0_f64 * t24081 * t2594;
    let t29354 = 6.0_f64 * t17775 * t8968;
    (t29350, t29352, t29354)
}

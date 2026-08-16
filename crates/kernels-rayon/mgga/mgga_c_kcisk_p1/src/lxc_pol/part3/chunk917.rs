//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 917/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk917(t13523: f64, t13526: f64, t13530: f64, t13533: f64, t13536: f64, t13540: f64, t13543: f64, t13546: f64, t13549: f64, t13552: f64, t13555: f64, t1265: f64, t4125: f64) -> (f64, f64) {
    let t13557 = -t13523 - 0.23744444444444444444e-1_f64 * t13526 + 0.11872222222222222222e-1_f64 * t13530 - 0.35616666666666666666e-1_f64 * t13533 + 0.17808333333333333333e-1_f64 * t13536 - 0.19787037037037037037e-1_f64 * t13540 + 0.71233333333333333332e-1_f64 * t13543 - 0.35616666666666666666e-1_f64 * t13546 - 0.10685e0_f64 * t13549 + 0.10685e0_f64 * t13552 - 0.17808333333333333333e-1_f64 * t13555;
    let t13561 = 1.0_f64 / t4125 / t1265;
    (t13557, t13561)
}

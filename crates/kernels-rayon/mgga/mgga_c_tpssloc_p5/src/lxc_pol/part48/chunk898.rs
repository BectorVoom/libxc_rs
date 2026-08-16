//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 898/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk898(t12823: f64, t8327: f64, t31058: f64, t4034: f64, t9348: f64, t22947: f64, t3701: f64, t31054: f64, t31056: f64, t31059: f64, t214: f64, t6624: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t112535 = 2.0_f64 * t12823 * t8327;
    let t112537 = 4.0_f64 * t4034 * t31058;
    let t112542 = 2.0_f64 * t9348 * t8327;
    let t112611 = t3701 * t22947;
    let t112620 = 4.0_f64 * t31054;
    let t112621 = 4.0_f64 * t31056;
    let t112622 = 4.0_f64 * t31059;
    let t112660 = t214 * t6624;
    (t112535, t112537, t112542, t112611, t112620, t112621, t112622, t112660)
}

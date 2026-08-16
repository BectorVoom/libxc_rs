//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1210/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1210(t17454: f64, t27544: f64, t28594: f64, t4262: f64, t3734: f64, t6034: f64, t17409: f64, t7948: f64, t11783: f64, t2055: f64, t17471: f64, t28629: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t97742 = t27544 * t17454;
    let t97744 = t28594 * t4262;
    let t97746 = t3734 * t6034;
    let t97748 = t7948 * t17409;
    let t97750 = t11783 * t2055;
    let t97752 = t28629 * t17471;
    (t97742, t97744, t97746, t97748, t97750, t97752)
}

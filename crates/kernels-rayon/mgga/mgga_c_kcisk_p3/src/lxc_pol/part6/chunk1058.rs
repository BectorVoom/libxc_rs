//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1058/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1058(t14484: f64, t30153: f64, t4271: f64, t1471: f64, t21230: f64, t7706: f64, t196: f64, t30738: f64, t6298: f64, t7710: f64, t1472: f64, t30158: f64) -> (f64, f64, f64, f64, f64) {
    let t31332 = t4271 * t14484 * t30153;
    let t31336 = t1471 * t21230 * t7706;
    let t31339 = t30738 * t196;
    let t31343 = t1471 * t6298 * t7710;
    let t31347 = t1471 * t1472 * t30158;
    (t31332, t31336, t31339, t31343, t31347)
}

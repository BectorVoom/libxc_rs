//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 144/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk144(t338: f64, t41: f64, t382: f64, t525: f64, t79: f64, t20: f64, t469: f64) -> (f64, f64, f64, f64, f64) {
    let t526 = t338 * t41;
    let t529 = 10.0_f64 / 9.0_f64 * t525 * t526 * t382;
    let t530 = t529 < -0.66725e-1_f64;
    let t532 = piecewise3(t530, 0.0_f64, 0.66725e-1_f64 + t529);
    let t533 = t79 * t532;
    let t534 = t469 * t20;
    let t535 = t533 * t534;
    (t526, t533, t534, t535, t529)
}

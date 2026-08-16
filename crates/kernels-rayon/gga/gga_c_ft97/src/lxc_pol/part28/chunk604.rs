//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 604/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk604(t22767: f64, t6449: f64, t22632: f64, t5611: f64, t1598: f64, t409: f64, t11119: f64, t47: f64, t9: f64, t12486: f64, t420: f64, t422: f64, t938: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t25746 = t22767 * t6449;
    let t25749 = t22632 * t6449;
    let t25750 = t5611 * t25749;
    let t25752 = t1598 * t409;
    let t25753 = t11119 * t25752;
    let t25754 = t9 * t47;
    let t25755 = t420 * t12486;
    let t25756 = t25754 * t25755;
    let t25759 = t422 * t938;
    (t25746, t25749, t25750, t25752, t25753, t25755, t25756, t25759)
}

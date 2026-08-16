//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 537/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk537(t1614: f64, t22563: f64, t7837: f64, t410: f64, t70: f64, t414: f64, t5569: f64, t5572: f64, t47: f64, t9: f64, t1624: f64, t373: f64, t422: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t22564 = t22563 * t1614;
    let t22565 = t7837 * t22564;
    let t22568 = t410 * t70;
    let t22572 = t414 * t70;
    let t22574 = t5569 * t22572 * t5572;
    let t22581 = t1614 * t47;
    let t22582 = t9 * t22581;
    let t22583 = t1624 * t22582;
    let t22584 = t422 * t373;
    (t22565, t22568, t22572, t22574, t22581, t22583, t22584)
}

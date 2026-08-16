//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 701/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk701(t13596: f64, t13555: f64, t1457: f64, t2103: f64, t11724: f64, t935: f64, t1445: f64, t813: f64, t3470: f64, t3651: f64, t11798: f64, t1645: f64, t2624: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13597 = 0.14896037479937677779e-1_f64 * t13596;
    let t13598 = t1457 * t13555;
    let t13600 = 0.71500979903700853338e0_f64 * t2103 * t13598;
    let t13601 = t11724 * t935;
    let t13602 = t1445 * t13601;
    let t13604 = 0.92023022289409799224e1_f64 * t813 * t13602;
    let t13606 = 0.25025342966295298669e1_f64 * t3651 * t3470;
    let t13608 = 0.10725146985555128001e1_f64 * t11798 * t3470;
    let t13609 = t1645 * t2624;
    (t13597, t13598, t13600, t13601, t13602, t13604, t13606, t13608, t13609)
}

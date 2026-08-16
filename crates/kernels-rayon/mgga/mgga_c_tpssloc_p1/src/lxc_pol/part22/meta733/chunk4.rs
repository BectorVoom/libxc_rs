//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2408/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2408(t48157: f64, t60192: f64, t60194: f64, t60202: f64, t68571: f64, t68577: f64, t68580: f64, t68583: f64, t68586: f64, t68589: f64, t68592: f64, t42086: f64, t59663: f64, t59665: f64, t59680: f64, t59688: f64, t59694: f64, t60204: f64, t68596: f64, t68599: f64, t68602: f64, t68605: f64, t68608: f64) -> (f64, f64) {
    let t68839 = -0.91285185185185185187e-1_f64 * t48157 - 0.29896666666666666667e0_f64 * t68571 + 0.98587999999999999998e0_f64 * t60192 - 0.65725333333333333332e0_f64 * t60194 - 0.32862666666666666666e0_f64 * t60202 + 0.71752e1_f64 * t68577 - 0.53814e1_f64 * t68580 + 0.17938e1_f64 * t68583 + 0.17938e1_f64 * t68586 + 0.59793333333333333334e0_f64 * t68589 - 0.19931111111111111111e0_f64 * t68592;
    let t68851 = 0.39862222222222222223e1_f64 * t68596 - 0.99655555555555555554e0_f64 * t68599 + 0.35876e1_f64 * t68602 - 0.99655555555555555555e0_f64 * t68605 - 0.53814e1_f64 * t68608 - 0.91285185185185185184e-1_f64 * t60204 - 0.59793333333333333334e0_f64 * t59663 + 0.19931111111111111111e0_f64 * t59665 + 0.29896666666666666667e0_f64 * t59680 + 0.79724444444444444444e0_f64 * t59688 - 0.39862222222222222223e0_f64 * t59694 + t42086;
    (t68839, t68851)
}

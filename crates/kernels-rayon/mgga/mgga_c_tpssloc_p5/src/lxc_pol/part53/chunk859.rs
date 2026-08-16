//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 859/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk859(t6585: f64, t8339: f64, t1894: f64, t59: f64, t776: f64, t6591: f64, t6600: f64, t6599: f64, t6612: f64, t829: f64, t6605: f64, t808: f64, t8342: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t30697 = t6585 * t8339;
    let t30700 = t1894 * t59 * t776;
    let t30701 = t6591 * t30700;
    let t30703 = t6600 * t8339;
    let t30704 = t6599 * t30703;
    let t30706 = t6612 * t829;
    let t30707 = t6605 * t30706;
    let t30709 = t808 * t8342;
    (t30697, t30700, t30701, t30703, t30704, t30706, t30707, t30709)
}

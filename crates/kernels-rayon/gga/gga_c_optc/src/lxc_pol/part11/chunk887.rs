//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 887/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk887(t16741: f64, t16769: f64, t818: f64, t16708: f64, t7801: f64, t10478: f64, t1355: f64, t1367: f64, t14102: f64, t14235: f64, t16654: f64, t16657: f64, t16672: f64, t16676: f64, t16677: f64, t16680: f64, t16683: f64, t16686: f64, t16699: f64, t16709: f64, t2493: f64, t2518: f64, t252: f64, t2530: f64, t2537: f64, t3716: f64, t4885: f64, t4888: f64, t7759: f64, t7799: f64, t810: f64) -> (f64, f64, f64, f64) {
    let t16770 = t16741 + t16769;
    let t16771 = t16770 * t818;
    let t16774 = t16708 * t7801;
    let t16779 = t16654 + t16657 - 0.19751789702565206229e-1_f64 * t16672 - t16676 - 6.0_f64 * t2493 * t16677 + 0.96494049533612093922e2_f64 * t2518 * t16680 - 0.35089340384731224426e1_f64 * t2530 * t16683 + 0.51947267698127589897e2_f64 * t2537 * t16686 - 0.3109e-1_f64 * t16699 * t252 + 3.0_f64 * t14235 * t1355 + 3.0_f64 * t3716 * t4885 + 0.96494049533612093922e2_f64 * t10478 * t4888 - 0.19298809906722418785e3_f64 * t7759 * t16709 + 1.0_f64 * t810 * t16771 + 0.20691336878655965246e4_f64 * t7799 * t16774 + 0.17544670192365612213e1_f64 * t14102 * t1367;
    (t16770, t16771, t16774, t16779)
}

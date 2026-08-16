//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1787/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1787(t9303: f64, t9641: f64, t2435: f64, t9635: f64, t213: f64, t225: f64, t4071: f64, t47343: f64, t47568: f64, t47570: f64, t47574: f64, t47580: f64, t47591: f64, t47593: f64, t47595: f64, t47601: f64, t47606: f64, t47608: f64, t47612: f64, t47616: f64, t561: f64, t9652: f64) -> f64 {
    let t47618 = t9303 * t9641;
    let t47620 = t2435 * t9635;
    let t47622 = 0.44178176337912614788e-3_f64 * t47568 - 0.18505311230957427423e-1_f64 * t47570 - 0.78548797528808629095e-3_f64 * t47574 + 0.15805078039045227836e2_f64 * t4071 * t9652 - 0.1561190486301245283e0_f64 * t47580 + 0.65854491829355115987e0_f64 * t213 * t47343 * t225 * t561 - t47591 + 0.65854491829355115985e-1_f64 * t47593 - 0.43902994552903410657e-1_f64 * t47595 + t47601 - 0.23417857294518679245e0_f64 * t47606 + 0.87805989105806821314e-1_f64 * t47608 + 0.23417857294518679246e0_f64 * t47612 - 0.39029762157531132075e-2_f64 * t47616 + 0.1040793657534163522e-1_f64 * t47618 + 0.43902994552903410657e-1_f64 * t47620;
    t47622
}

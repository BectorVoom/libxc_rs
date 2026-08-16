//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 568/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk568(t43: f64, t2413: f64, t2827: f64, t1891: f64, t1210: f64, t1214: f64, t429: f64, t529: f64, t496: f64, t492: f64, t490: f64, t149: f64, t209: f64, t371: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t44 = t43 <= zeta_threshold;
    let t2828 = t2413 + t2827;
    let t2832 = piecewise3(t44, 0.0_f64, t1891);
    let t2835 = t1210 * t1214;
    let t2837 = t529 * t429;
    let t2838 = t2837 * t496;
    let t2839 = t492 * t2838;
    let t2841 = t490 * t2839 / 9.0_f64;
    let t2843 = t209 * t149 * t371;
    (t2828, t2832, t2835, t2837, t2839, t2841, t2843)
}

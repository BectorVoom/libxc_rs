//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1235/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1235(t7906: f64, t909: f64, t911: f64, t2742: f64, t2773: f64, t2774: f64, t2778: f64, t2780: f64, t115: f64, t2341: f64, t2770: f64, t2769: f64) -> (f64, f64, f64, f64, f64) {
    let t25504 = t909 * t7906 * t911;
    let t25508 = t2773 * t2742 * t2774;
    let t25511 = t2778 * t2742 * t2780;
    let t25514 = t2341 * t2770 * t115;
    let t25515 = t2769 * t25514;
    (t25504, t25508, t25511, t25514, t25515)
}

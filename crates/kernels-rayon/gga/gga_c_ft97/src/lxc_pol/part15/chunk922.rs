//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 922/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk922(t66902: f64, t1636: f64, t4935: f64, t89: f64, t4926: f64, t9733: f64, t4930: f64, t5106: f64, t8282: f64, t5099: f64, t5110: f64, t5102: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t66903 = 4.0_f64 / 9.0_f64 * t66902;
    let t66905 = t89 * t1636 * t4935;
    let t66906 = 8.0_f64 / 9.0_f64 * t66905;
    let t66934 = t89 * t9733 * t4926;
    let t66935 = 8.0_f64 / 27.0_f64 * t66934;
    let t66945 = t89 * t9733 * t4930;
    let t66946 = 4.0_f64 / 27.0_f64 * t66945;
    let t67078 = t8282 * t5106;
    let t67097 = t8282 * t5099;
    let t67103 = t8282 * t5110;
    let t67288 = t8282 * t5102;
    (t66903, t66905, t66906, t66934, t66935, t66945, t66946, t67078, t67097, t67103, t67288)
}

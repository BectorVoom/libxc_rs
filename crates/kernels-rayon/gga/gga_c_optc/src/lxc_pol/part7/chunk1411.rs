//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1411/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1411(t1220: f64, t2367: f64, t8421: f64, t3277: f64, t8410: f64, t8416: f64, t11786: f64, t11894: f64, t26488: f64, t26490: f64, t26493: f64, t26560: f64, t26853: f64, t27346: f64, t3281: f64, t3286: f64, t4281: f64, t4289: f64, t9241: f64) -> f64 {
    let t28066 = t1220 * t2367 * t8421;
    let t28068 = t8410 * t3277;
    let t28071 = t1220 * t2367 * t8416;
    let t28082 = 4.0_f64 / 3.0_f64 * t28066 + 2.0_f64 / 3.0_f64 * t28068 + t26488 + t26490 + t26493 - t26560 - 16.0_f64 / 9.0_f64 * t28071 + t8410 * t3281 + 4.0_f64 / 3.0_f64 * t8410 * t3286 - 4.0_f64 * t11786 * t9241 + t26853 - 8.0_f64 * t4281 * t4289 * t11894 * t27346;
    t28082
}

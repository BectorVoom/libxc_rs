//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1347/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1347(t241: f64, t26582: f64, t26686: f64, t26721: f64, t26850: f64, t3067: f64, t8701: f64, t26792: f64, t26476: f64, t26479: f64, t26482: f64, t26484: f64, t26488: f64, t26490: f64, t26493: f64, t26560: f64, t26849: f64) -> (f64, f64, f64, f64) {
    let t26853 = t241 * (t26582 + t26686 + t26721 + t26850);
    let t26855 = 0.41015588084031179722e4_f64 * t3067 * t8701;
    let t26857 = 0.19751789702565206229e-1_f64 * t241 * t26792;
    let t26858 = t26476 - t26479 - t26482 + t26484 + t26488 + t26490 + t26493 - t26560 + t26853 - t26855 + t26857 - t26849;
    (t26853, t26855, t26857, t26858)
}

//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1170/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1170(t1805: f64, t7329: f64, t2001: f64, t5539: f64, t31346: f64, t6328: f64, t6140: f64, t31525: f64, t31526: f64, t31530: f64, t31532: f64, t31543: f64, t31544: f64, t35723: f64, t35737: f64, t35748: f64, t35756: f64, t37694: f64, t37697: f64, t37698: f64, t37700: f64) -> f64 {
    let t40126 = t7329 * t1805;
    let t40131 = t2001 * t5539;
    let t40134 = t31346 * t6328;
    let t40136 = t31346 * t6140;
    let t40138 = 7.0_f64 / 144.0_f64 * t40126 + t31525 + 0.19812298142450615803e-1_f64 * t31526 + 0.17149607247227894789e-2_f64 * t31530 - 0.17149607247227894789e-2_f64 * t31532 + t31543 + t35723 - t37694 + 0.51448821741683684366e-2_f64 * t40131 + 0.33020496904084359672e-1_f64 * t31544 - t35737 + t37697 + 0.13719685797782315831e-1_f64 * t40134 - 0.20579528696673473747e-1_f64 * t40136 + t37698 - t37700 - t35748 + t35756;
    t40138
}

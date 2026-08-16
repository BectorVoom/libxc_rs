//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 901/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk901(t373: f64, t1040: f64, t2942: f64, t376: f64, t383: f64, t3145: f64, t56: f64, t8429: f64, t11: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8611 = 1.0_f64/pow_3_2(t373);
    let t8612 = t2942 * t1040;
    let t8613 = t8611 * t8612;
    let t8617 = 1.0_f64 / t376 / t383 / 4.0_f64;
    let t8618 = t8617 * t8612;
    let t8620 = t56 * t3145;
    let t8621 = t8620 * t8429;
    let t8622 = t11 * t8621;
    (t8611, t8613, t8617, t8618, t8620, t8621, t8622)
}

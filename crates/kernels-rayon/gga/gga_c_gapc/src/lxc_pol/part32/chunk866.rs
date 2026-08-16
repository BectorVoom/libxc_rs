//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 866/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk866(t3375: f64, t9673: f64, t320: f64, t8700: f64, t3379: f64, t3402: f64, t8838: f64, t3406: f64, t7115: f64, t9921: f64, t2598: f64, t3404: f64) -> (f64, f64, f64, f64, f64) {
    let t10024 = t9673 * t3375;
    let t10026 = t320 * t8700;
    let t10027 = t10026 * t3379;
    let t10029 = t3402 * t8838;
    let t10030 = t7115 * t3406;
    let t10031 = t9921 * t10030;
    let t10032 = t10029 * t10031;
    let t10034 = t3404 * t2598;
    (t10024, t10027, t10031, t10032, t10034)
}

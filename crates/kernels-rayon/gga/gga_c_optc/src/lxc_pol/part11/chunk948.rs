//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 948/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk948(t12366: f64, t5190: f64, t17426: f64, t3020: f64, t8582: f64, t1221: f64, t17348: f64, t914: f64, t17336: f64, t8426: f64, t4305: f64, t5308: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t17435 = 0.48245472966453314466e2_f64 * t12366 * t5190;
    let t17436 = t17426 * t3020;
    let t17438 = 0.96490945932906628932e2_f64 * t8582 * t17436;
    let t17439 = t1221 * t17348;
    let t17440 = t914 * t17439;
    let t17442 = t8426 * t17336;
    let t17443 = t914 * t17442;
    let t17447 = 0.51947267698127589899e2_f64 * t4305 * t5308;
    (t17435, t17436, t17438, t17439, t17440, t17442, t17443, t17447)
}

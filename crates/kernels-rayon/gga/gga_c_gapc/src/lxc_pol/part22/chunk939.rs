//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 939/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk939(t9513: f64, t9516: f64, t9518: f64, t9521: f64, t9523: f64, t9526: f64, t9530: f64, t9533: f64, t9536: f64, t9539: f64, t9541: f64, t9544: f64, t9546: f64) -> f64 {
    let t10856 = 0.12974218172834570556e-1_f64 * t9513 + 0.27801896084645508334e-2_f64 * t9516 + 0.55603792169291016668e-2_f64 * t9518 - 0.14492726735651760868e-5_f64 * t9521 - 0.10136107947527008247e-3_f64 * t9523 - 0.10136107947527008247e-3_f64 * t9526 + 0.30361328125000000002e-3_f64 * t9530 - 0.10120442708333333334e-3_f64 * t9533 + 0.6746961805555555556e-5_f64 * t9536 + 0.28985453471303521736e-5_f64 * t9539 + 0.2471588561924985691e-3_f64 * t9541 + 0.2471588561924985691e-3_f64 * t9544 - 0.6746961805555555556e-5_f64 * t9546;
    t10856
}

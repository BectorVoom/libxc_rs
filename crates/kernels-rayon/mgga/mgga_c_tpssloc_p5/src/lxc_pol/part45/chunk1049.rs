//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1049/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1049(t1983: f64, t31669: f64, t6999: f64, t115824: f64, t115914: f64, t115942: f64, t115946: f64, t115948: f64, t115959: f64, t115965: f64, t1869: f64, t2040: f64, t22461: f64, t2314: f64, t24167: f64, t24169: f64, t24428: f64, t31734: f64, t3652: f64, t3929: f64, t510: f64, t6515: f64, t7050: f64, t7061: f64, t7156: f64, t8450: f64, t8519: f64, t8604: f64, t90041: f64) -> f64 {
    let t115968 = 2.0_f64 * t1983 * t31669 * t6999;
    let t115969 = -2.0_f64 * t115824 * t510 - t115914 * t510 - t1869 * t24428 - 4.0_f64 * t2040 * t90041 - 4.0_f64 * t22461 * t7050 - 4.0_f64 * t22461 * t7061 - 4.0_f64 * t2314 * t31734 + t24167 * t8450 + 2.0_f64 * t24169 * t8450 - t3652 * t8519 + t3929 * t8604 - 2.0_f64 * t6515 * t7156 - t115942 - t115946 - t115948 + t115959 + t115965 - t115968;
    t115969
}

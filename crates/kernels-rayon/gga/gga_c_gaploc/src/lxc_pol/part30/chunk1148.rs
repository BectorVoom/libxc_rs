//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1148/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1148(t31021: f64, t20018: f64, t7026: f64, t4779: f64, t584: f64, t9419: f64, t20551: f64, t20671: f64, t20669: f64, t20556: f64, t1537: f64, t9268: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t31022 = 0.76685851907841499352e0_f64 * t31021;
    let t31024 = 0.17875244975925213335e0_f64 * t20018 * t7026;
    let t31037 = t584 * t4779 * t9419;
    let t31040 = 0.51123901271894332902e0_f64 * t31037 * t20671 * t20551;
    let t31041 = t584 * t20669;
    let t31044 = 0.34082600847929555268e0_f64 * t31041 * t20671 * t20556;
    let t31045 = t1537 * t9268;
    (t31022, t31024, t31037, t31040, t31041, t31044, t31045)
}

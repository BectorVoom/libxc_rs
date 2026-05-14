//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1037/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1037<F: Float>(t31021: F, t20018: F, t7026: F, t4779: F, t584: F, t9419: F, t20551: F, t20671: F, t20669: F, t20556: F, t1537: F, t9268: F, t20687: F, t20561: F, t1406: F, t6582: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t31022 = 0.76685851907841499352e0 * t31021;
    let t31024 = 0.17875244975925213335e0 * t20018 * t7026;
    let t31037 = t584 * t4779 * t9419;
    let t31040 = 0.51123901271894332902e0 * t31037 * t20671 * t20551;
    let t31041 = t584 * t20669;
    let t31044 = 0.34082600847929555268e0 * t31041 * t20671 * t20556;
    let t31045 = t1537 * t9268;
    let t31046 = 0.51123901271894332901e1 * t31045;
    let t31047 = t584 * t20687;
    let t31050 = 0.85206502119823888169e0 * t31047 * t20671 * t20561;
    let t31051 = t1406 * t6582;
    (t31022, t31024, t31037, t31040, t31041, t31044, t31046, t31047, t31050, t31051)
}

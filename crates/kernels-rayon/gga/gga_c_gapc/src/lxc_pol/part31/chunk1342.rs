//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1342/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1342(t2011: f64, t3808: f64, t4915: f64, t30472: f64, t3483: f64, t3480: f64, t9370: f64, t15430: f64, t10538: f64, t28182: f64, t12058: f64, t687: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t36103 = 6.0_f64 * t4915 * t3808 * t2011;
    let t36105 = 4.0_f64 * t30472 * t3483;
    let t36109 = t3480 * t9370;
    let t36111 = 2.0_f64 * t15430 * t3808;
    let t36113 = 6.0_f64 * t28182 * t10538;
    let t36116 = 12.0_f64 * t4915 * t12058 * t687;
    (t36103, t36105, t36109, t36111, t36113, t36116)
}

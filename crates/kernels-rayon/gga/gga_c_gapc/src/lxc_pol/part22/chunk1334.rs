//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1334/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1334(t3480: f64, t9370: f64, t15430: f64, t3808: f64, t10538: f64, t28182: f64, t12058: f64, t4915: f64, t687: f64, t1112: f64, t1616: f64, t2011: f64, t3822: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t36109 = t3480 * t9370;
    let t36111 = 2.0_f64 * t15430 * t3808;
    let t36113 = 6.0_f64 * t28182 * t10538;
    let t36116 = 12.0_f64 * t4915 * t12058 * t687;
    let t36119 = 2.0_f64 * t1616 * t1112 * t9370;
    let t36122 = 2.0_f64 * t1616 * t3822 * t2011;
    (t36109, t36111, t36113, t36116, t36119, t36122)
}

//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1420/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1420(t30830: f64, t7967: f64, t913: f64, t10609: f64, t31054: f64, t2754: f64, t4130: f64, t2482: f64, t9272: f64, t10608: f64, t6895: f64, t20671: f64, t26328: f64, t31037: f64) -> (f64, f64, f64, f64, f64) {
    let t35074 = t30830 * t913 * t7967;
    let t35075 = 0.59584149919750711116e-1_f64 * t35074;
    let t35089 = t31054 * t10609;
    let t35090 = 0.11502877786176224903e1_f64 * t35089;
    let t35091 = t4130 * t2754;
    let t35093 = t9272 * t35091 * t2482;
    let t35094 = 0.11502877786176224903e1_f64 * t35093;
    let t35096 = t9272 * t10608 * t6895;
    let t35097 = 0.57514388930881124514e0_f64 * t35096;
    let t35099 = t31037 * t20671 * t26328;
    (t35075, t35090, t35094, t35097, t35099)
}

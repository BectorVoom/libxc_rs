//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1097/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1097(t6891: f64, t6895: f64, t167: f64, t168: f64, t17033: f64, t16421: f64, t2591: f64, t1037: f64, t16406: f64, t2667: f64, t5296: f64, t17051: f64, t175: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20010 = t6895 * t6891;
    let t20060 = t167 * t168 * t17033;
    let t20065 = t16421 * t168 * t2591;
    let t20155 = t16406 * t1037;
    let t20164 = t5296 * t2667;
    let t20199 = t17051 * t175;
    (t20010, t20060, t20065, t20155, t20164, t20199)
}

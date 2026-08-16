//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1948/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1948(t101226: f64, t2047: f64, t7706: f64, t95283: f64, t26179: f64, t28105: f64, t28109: f64, t101156: f64, t101323: f64, t2048: f64, t25102: f64, t25110: f64, t25114: f64, t25162: f64, t26187: f64, t28133: f64, t28141: f64, t28602: f64, t28635: f64, t6963: f64, t7343: f64, t7352: f64, t7964: f64) -> f64 {
    let t101850 = t2047 * t101226;
    let t101870 = 80.0_f64 / 9.0_f64 * t95283 * t7706;
    let t101872 = 80.0_f64 / 9.0_f64 * t26179 * t28105;
    let t101874 = 80.0_f64 / 9.0_f64 * t26179 * t28109;
    let t101875 = 20.0_f64 / 3.0_f64 * t25162 * t101850 - 4.0_f64 / 3.0_f64 * t101323 * t2048 - 10.0_f64 / 3.0_f64 * t28602 * t25110 - 4.0_f64 / 3.0_f64 * t28141 * t7352 - 10.0_f64 / 3.0_f64 * t26187 * t28133 - 4.0_f64 / 3.0_f64 * t25102 * t7964 - 10.0_f64 / 3.0_f64 * t7343 * t101156 - 4.0_f64 / 3.0_f64 * t6963 * t28635 - 5.0_f64 / 3.0_f64 * t28602 * t25114 + t101870 + t101872 + t101874;
    t101875
}

//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1275/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1275(t3409: f64, t6153: f64, t15482: f64, t6339: f64, t1165: f64, t1531: f64, t1533: f64, t18153: f64, t18155: f64, t18157: f64, t20906: f64, t23568: f64, t23572: f64, t23574: f64, t23584: f64, t23586: f64, t3084: f64, t5862: f64) -> f64 {
    let t23588 = t3409 * t6153;
    let t23590 = t15482 * t6339;
    let t23592 = -0.17149607247227894789e-2_f64 * t18153 + 0.17149607247227894789e-2_f64 * t18155 - 0.85748036236139473944e-3_f64 * t18157 - 0.12004725073059526352e-1_f64 * t23568 - 0.68598428988911579156e-2_f64 * t23572 + 0.85748036236139473944e-3_f64 * t23574 + 0.85748036236139473944e-3_f64 * t1531 * t1165 * t20906 * t1533 + 0.42874018118069736972e-3_f64 * t1531 * t1165 * t5862 * t3084 - 0.80031500487063509014e-2_f64 * t23584 - 0.80031500487063509014e-2_f64 * t23586 - 0.80031500487063509014e-2_f64 * t23588 - 0.48018900292238105409e-1_f64 * t23590;
    t23592
}

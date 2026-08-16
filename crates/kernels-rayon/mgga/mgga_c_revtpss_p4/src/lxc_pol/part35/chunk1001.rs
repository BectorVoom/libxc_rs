//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1001/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1001(t12839: f64, t1469: f64, t20795: f64, t3626: f64, t6638: f64, t17304: f64, t17340: f64, t17342: f64, t17438: f64, t1791: f64, t20817: f64, t20843: f64, t20847: f64, t20851: f64, t20917: f64, t20927: f64, t20966: f64, t21177: f64, t5331: f64, t5340: f64, t6611: f64) -> f64 {
    let t24567 = t12839 * t1469;
    let t24568 = t20795 * t24567;
    let t24569 = t3626 * t24568;
    let t24572 = t20795 * t6638;
    let t24573 = t3626 * t24572;
    let t24587 = 0.42874018118069736972e-3_f64 * t20817 - 0.42874018118069736972e-3_f64 * t20843 + 0.85748036236139473944e-3_f64 * t20847 + 0.14291339372689912324e-3_f64 * t17304 - 0.85748036236139473944e-3_f64 * t5340 * t24569 + 0.42874018118069736972e-3_f64 * t5331 * t24573 + 0.85748036236139473944e-3_f64 * t20917 + 0.7622047665434619906e-3_f64 * t17340 - 0.14291339372689912324e-3_f64 * t17342 - 0.21722835846488666732e-1_f64 * t21177 * t1791 - 0.68598428988911579154e-2_f64 * t17438 * t6611 - 0.85748036236139473944e-3_f64 * t20927 + 11.0_f64 / 108.0_f64 * t20966 - 0.64311027177104605458e-3_f64 * t20851 * t1791;
    t24587
}

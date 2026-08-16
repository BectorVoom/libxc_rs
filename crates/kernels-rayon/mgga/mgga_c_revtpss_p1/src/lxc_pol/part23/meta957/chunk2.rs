//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3203/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3203(t1256: f64, t24700: f64, t1791: f64, t21107: f64, t5287: f64, t70210: f64, t71931: f64, t71971: f64, t71974: f64, t71976: f64, t72000: f64, t72005: f64, t72017: f64, t84082: f64) -> f64 {
    let t84084 = t24700 * t1256;
    let t84094 = t71931 / 216.0_f64 + 0.85748036236139473944e-3_f64 * t71971 - 0.42874018118069736972e-3_f64 * t71974 + 0.14481890564325777821e-1_f64 * t84082 + 0.14291339372689912324e-3_f64 * t84084 - 0.57165357490759649295e-3_f64 * t71976 - 7.0_f64 / 648.0_f64 * t72000 - 0.68598428988911579154e-2_f64 * t21107 * t5287 - 0.91464571985215438872e-2_f64 * t72005 - 0.42874018118069736972e-3_f64 * t72017 - 0.64311027177104605458e-3_f64 * t70210 * t1791;
    t84094
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3138/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3138(t21102: f64, t5265: f64, t20816: f64, t5274: f64, t1042: f64, t1261: f64, t17569: f64, t17609: f64, t20825: f64, t20907: f64, t20914: f64, t21143: f64, t24808: f64, t3647: f64, t5268: f64, t5270: f64, t5279: f64, t5381: f64, t6625: f64, t69906: f64, t80045: f64, t80050: f64) -> f64 {
    let t82441 = t21102 * t5265;
    let t82457 = t5274 * t20816;
    let t82467 = 0.14481890564325777821e-1_f64 * t82441 - 0.85748036236139473944e-3_f64 * t3647 * t24808 - 0.85748036236139473944e-3_f64 * t1261 * t1042 * t5268 * t80045 - 0.85748036236139473944e-3_f64 * t1261 * t1042 * t5268 * t80050 + 0.85748036236139473944e-3_f64 * t17569 * t20914 + 0.64311027177104605458e-3_f64 * t17609 * t6625 + 0.42874018118069736972e-3_f64 * t82457 - 0.85748036236139473944e-3_f64 * t5381 * t20907 - 0.85748036236139473944e-3_f64 * t21143 * t5270 - 0.7145669686344956162e-3_f64 * t17569 * t20825 + 0.42874018118069736972e-3_f64 * t69906 * t5279;
    t82467
}

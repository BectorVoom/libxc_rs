//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1263/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1263(t20112: f64, t380: f64, t1043: f64, t1089: f64, t6343: f64, t1668: f64, t4930: f64, t16449: f64, t1651: f64, t4772: f64, t5004: f64, t20089: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20113 = t380 * t20112;
    let t20119 = t6343 * t1043 * t1089;
    let t20123 = t4930 * t1668 * t1089;
    let t20128 = t16449 * t1651;
    let t20133 = t5004 * t4772;
    let t20136 = t20089 * t1089;
    (t20113, t20119, t20123, t20128, t20133, t20136)
}

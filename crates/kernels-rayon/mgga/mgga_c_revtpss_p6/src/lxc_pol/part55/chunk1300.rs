//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1300/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1300(t2110: f64, t8249: f64, t2172: f64, t8113: f64, t2167: f64, t8130: f64, t127453: f64, t129018: f64, t129026: f64, t129029: f64, t129032: f64, t131119: f64, t2170: f64, t28987: f64, t28990: f64, t32377: f64, t573: f64, t5805: f64, t7557: f64, t7696: f64, t8124: f64, t8245: f64, t8905: f64, param_d: f64) -> (f64, f64, f64, f64) {
    let t131133 = t2110 * t8249;
    let t131134 = t8113 * t2172;
    let t131135 = t2167 * t8130;
    let t131148 = t131119 * t573 * param_d + 6.0_f64 * t2170 * t28987 + 3.0_f64 * t2170 * t28990 + 3.0_f64 * t5805 * t8905 + 3.0_f64 * t7557 * t8245 + 6.0_f64 * t7696 * t8124 + t127453 + t129018 + t129026 + t129029 + t129032 + t32377;
    (t131133, t131134, t131135, t131148)
}

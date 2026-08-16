//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2240/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2240(t104624: f64, t104626: f64, t104640: f64, t104651: f64, t104653: f64, t20806: f64, t20811: f64, t20876: f64, t21153: f64, t21166: f64, t21259: f64, t26870: f64, t26880: f64, t29100: f64, t6690: f64, t7624: f64, t97182: f64) -> f64 {
    let t112175 = 0.28582678745379824648e-3_f64 * t26880 * t20811 - 0.85748036236139473944e-3_f64 * t97182 * t6690 - 0.85748036236139473944e-3_f64 * t26870 * t21166 - t104624 + t104626 + 0.57165357490759649296e-3_f64 * t26880 * t20876 - 0.28582678745379824648e-3_f64 * t7624 * t21153 - t104640 + t104651 - t104653 - 0.85748036236139473944e-3_f64 * t26870 * t21259 - 0.42874018118069736972e-3_f64 * t29100 * t20806;
    t112175
}

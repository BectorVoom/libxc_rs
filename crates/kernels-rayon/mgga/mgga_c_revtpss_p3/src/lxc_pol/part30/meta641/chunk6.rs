//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2234/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2234(t26827: f64, t5362: f64, t17435: f64, t7613: f64, t3670: f64, t8184: f64, t12702: f64, t12744: f64, t17391: f64, t17602: f64, t17744: f64, t26870: f64, t29062: f64, t29096: f64, t3663: f64, t3674: f64, t5335: f64, t5343: f64, t5348: f64, t97182: f64, t97191: f64) -> f64 {
    let t104815 = 0.57165357490759649296e-3_f64 * t26827 * t5362;
    let t104817 = 0.57165357490759649296e-3_f64 * t7613 * t17435;
    let t104818 = t3670 * t8184;
    let t104821 = -0.42874018118069736972e-3_f64 * t26870 * t17744 + 0.17149607247227894789e-2_f64 * t12702 * t29096 * t5343 - 0.85748036236139473944e-3_f64 * t12744 * t29096 * t5335 - 0.85748036236139473944e-3_f64 * t97182 * t5348 - 0.85748036236139473944e-3_f64 * t26870 * t17391 - 0.42874018118069736972e-3_f64 * t26870 * t17602 - 0.57165357490759649296e-3_f64 * t97191 + 0.22866142996303859718e-2_f64 * t29062 * t3663 - t104815 - t104817 - 0.45732285992607719436e-2_f64 * t104818 * t3674;
    t104821
}

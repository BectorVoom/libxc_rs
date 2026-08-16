//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1352/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1352(t104888: f64, t112258: f64, t112260: f64, t112279: f64, t112301: f64, t112322: f64, t112356: f64, t1808: f64, t24535: f64, t24569: f64, t24573: f64, t24640: f64, t26880: f64, t29037: f64, t29097: f64, t29100: f64, t6640: f64, t6673: f64, t6679: f64, t6683: f64, t7624: f64) -> f64 {
    let t116160 = -0.17149607247227894789e-2_f64 * t104888 * t6640 + 0.91464571985215438873e-2_f64 * t112260 * t1808 - 0.57165357490759649295e-3_f64 * t112258 - 0.28963781128651555642e-1_f64 * t112356 * t1808 + 0.14291339372689912324e-2_f64 * t29037 * t6673 - 0.85748036236139473944e-3_f64 * t29037 * t6679 - 0.17149607247227894789e-2_f64 * t29037 * t6683 - 0.11433071498151929859e-2_f64 * t112279 - 0.14291339372689912324e-2_f64 * t26880 * t24640 - 0.1270341277572436651e-2_f64 * t7624 * t24535 + 11.0_f64 / 108.0_f64 * t112301 + 0.85748036236139473944e-3_f64 * t112322 - 0.17149607247227894789e-2_f64 * t29097 * t24569 + 0.85748036236139473944e-3_f64 * t29100 * t24573;
    t116160
}

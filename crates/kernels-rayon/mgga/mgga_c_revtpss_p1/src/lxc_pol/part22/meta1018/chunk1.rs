//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3522/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3522(t11250: f64, t16027: f64, t16103: f64, t16104: f64, t16228: f64, t19722: f64, t19986: f64, t3095: f64, t43069: f64, t53402: f64, t54490: f64, t54497: f64, t54500: f64, t54521: f64, t54533: f64, t54801: f64, t54811: f64, t55046: f64, t55141: f64, t66187: f64, t66686: f64, t66689: f64, t66702: f64, t66712: f64, t66714: f64) -> f64 {
    let t66716 = 0.28582678745379824648e-3_f64 * t54490 - 0.57165357490759649296e-3_f64 * t54497 + t66686 / 432.0_f64 + 0.11433071498151929859e-2_f64 * t43069 * t66689 * t16103 + 0.57165357490759649296e-3_f64 * t54521 - 0.57165357490759649296e-3_f64 * t55141 * t16104 + 0.30488190661738479624e-2_f64 * t53402 * t19986 - 0.17149607247227894789e-2_f64 * t54801 * t66187 * t11250 * t16228 + 0.28582678745379824648e-3_f64 * t54811 * t66187 * t66702 * t3095 + 0.57165357490759649296e-3_f64 * t54533 - 0.22866142996303859718e-2_f64 * t55046 * t19722 + 0.85748036236139473944e-3_f64 * t54500 * t16027 - 0.57165357490759649296e-3_f64 * t66712 - t66714 / 162.0_f64;
    t66716
}

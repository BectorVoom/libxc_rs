//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1353/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1353(t104682: f64, t104685: f64, t104825: f64, t112328: f64, t112334: f64, t112336: f64, t24649: f64, t24731: f64, t24736: f64, t24741: f64, t24804: f64, t24808: f64, t24836: f64, t26867: f64, t26880: f64, t29010: f64, t29097: f64, t29100: f64, t6625: f64, t6631: f64, t6635: f64, t7624: f64, t97149: f64, t97179: f64) -> f64 {
    let t116185 = -0.11433071498151929859e-2_f64 * t112328 + 0.25724410870841842183e-2_f64 * t29097 * t24731 - 0.12862205435420921092e-2_f64 * t29100 * t24736 + 0.25724410870841842183e-2_f64 * t97179 * t24741 - 0.25724410870841842183e-2_f64 * t97149 * t24836 - 0.85748036236139473944e-3_f64 * t112334 + 0.17149607247227894789e-2_f64 * t112336 + 0.12862205435420921092e-2_f64 * t29010 * t6625 + 0.25724410870841842183e-2_f64 * t104682 * t6631 - 0.12862205435420921092e-2_f64 * t104685 * t6635 + 0.28582678745379824648e-3_f64 * t104825 + 0.85748036236139473944e-3_f64 * t26880 * t24649 - 0.17149607247227894789e-2_f64 * t7624 * t24808 + 0.14291339372689912324e-2_f64 * t26867 * t24804;
    t116185
}

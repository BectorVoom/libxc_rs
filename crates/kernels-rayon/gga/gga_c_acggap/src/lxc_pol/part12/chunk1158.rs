//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1158/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1158(t30085: f64, t30091: f64, t30094: f64, t30099: f64, t30106: f64, t30110: f64, t30118: f64, t30121: f64, t30123: f64, t30125: f64, t30130: f64, t30132: f64, t30139: f64, t30151: f64, t32352: f64, t33914: f64, t33916: f64) -> f64 {
    let t36860 = 0.17149607247227894789e-2_f64 * t30085 + t32352 + 0.85748036236139473944e-3_f64 * t30091 + 0.42874018118069736972e-3_f64 * t30094 - 0.31448092289604152069e-3_f64 * t30099 - 0.64311027177104605458e-2_f64 * t33914 - 0.62896184579208304138e-3_f64 * t33916 + 0.75475421495049964964e-2_f64 * t30106 - 0.31448092289604152069e-3_f64 * t30110 + 0.20965394859736101379e-3_f64 * t30118 + 0.42874018118069736972e-3_f64 * t30121 + 0.17149607247227894789e-2_f64 * t30123 + 0.31448092289604152069e-3_f64 * t30125 - 0.41930789719472202758e-3_f64 * t30130 - 0.42874018118069736972e-3_f64 * t30132 - 0.28582678745379824648e-3_f64 * t30139 - 0.25158473831683321655e-2_f64 * t30151;
    t36860
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 967/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk967(t11696: f64, t3093: f64, t3092: f64, t11644: f64, t11649: f64, t11653: f64, t11656: f64, t11663: f64, t11667: f64, t11672: f64, t11675: f64, t11680: f64, t11684: f64, t11689: f64, t11693: f64, t3091: f64, t3097: f64, t3130: f64, t3136: f64, t3169: f64, t4837: f64, t4892: f64, t4899: f64) -> f64 {
    let t11697 = t3093 * t11696;
    let t11698 = t3092 * t11697;
    let t11701 = -0.57165357490759649295e-3_f64 * t11644 - 0.34299214494455789577e-2_f64 * t3169 * t3136 + 0.42874018118069736972e-3_f64 * t11649 + 0.85748036236139473944e-3_f64 * t4837 * t11653 + 0.45732285992607719436e-2_f64 * t11656 * t3130 + 0.85748036236139473944e-3_f64 * t4892 * t11663 - 0.42874018118069736972e-3_f64 * t4899 * t11667 - 0.45732285992607719436e-2_f64 * t11672 * t3097 + 0.85748036236139473944e-3_f64 * t11675 * t3097 + 0.42874018118069736972e-3_f64 * t3091 * t11680 - 0.85748036236139473944e-3_f64 * t3091 * t11684 + 0.12862205435420921092e-2_f64 * t4892 * t11689 - 0.64311027177104605458e-3_f64 * t4899 * t11693 + 0.42874018118069736972e-3_f64 * t3091 * t11698;
    t11701
}

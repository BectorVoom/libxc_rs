//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 839/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk839(t8133: f64, t8144: f64, t8145: f64, t8580: f64, t8582: f64, t8607: f64, t9532: f64, t9534: f64, t9539: f64, t9544: f64, t9546: f64, t9550: f64, t9555: f64, t9557: f64, t9561: f64, t9566: f64, t9568: f64, t9570: f64, t9574: f64, t9578: f64) -> f64 {
    let t9871 = -0.37737710747524982482e-2_f64 * t8580 - 0.42874018118069736972e-3_f64 * t8582 + 0.64311027177104605458e-2_f64 * t9532 - 0.85748036236139473944e-3_f64 * t9534 + 0.15724046144802076034e-2_f64 * t9539 - 0.62896184579208304138e-3_f64 * t9544 - 0.85748036236139473944e-3_f64 * t9546 - 0.21437009059034868486e-2_f64 * t9550 - 0.37737710747524982483e-2_f64 * t9555 + 0.17149607247227894789e-2_f64 * t9557 - 0.85748036236139473944e-3_f64 * t9561 - 0.42874018118069736972e-3_f64 * t9566 - 0.13719685797782315831e-1_f64 * t9568 + 0.13719685797782315831e-1_f64 * t9570 - t8133 + 0.85748036236139473944e-3_f64 * t8607 + 0.12862205435420921092e-2_f64 * t9574 - 0.94344276868812456204e-2_f64 * t9578 + t8144 - t8145;
    t9871
}

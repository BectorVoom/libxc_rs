//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 839/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk839<F: Float>(t8133: F, t8144: F, t8145: F, t8580: F, t8582: F, t8607: F, t9532: F, t9534: F, t9539: F, t9544: F, t9546: F, t9550: F, t9555: F, t9557: F, t9561: F, t9566: F, t9568: F, t9570: F, t9574: F, t9578: F) -> F {
    let t9871 = -F::cast_from(0.37737710747524982482e-2_f64) * t8580 - F::cast_from(0.42874018118069736972e-3_f64) * t8582 + F::cast_from(0.64311027177104605458e-2_f64) * t9532 - F::cast_from(0.85748036236139473944e-3_f64) * t9534 + F::cast_from(0.15724046144802076034e-2_f64) * t9539 - F::cast_from(0.62896184579208304138e-3_f64) * t9544 - F::cast_from(0.85748036236139473944e-3_f64) * t9546 - F::cast_from(0.21437009059034868486e-2_f64) * t9550 - F::cast_from(0.37737710747524982483e-2_f64) * t9555 + F::cast_from(0.17149607247227894789e-2_f64) * t9557 - F::cast_from(0.85748036236139473944e-3_f64) * t9561 - F::cast_from(0.42874018118069736972e-3_f64) * t9566 - F::cast_from(0.13719685797782315831e-1_f64) * t9568 + F::cast_from(0.13719685797782315831e-1_f64) * t9570 - t8133 + F::cast_from(0.85748036236139473944e-3_f64) * t8607 + F::cast_from(0.12862205435420921092e-2_f64) * t9574 - F::cast_from(0.94344276868812456204e-2_f64) * t9578 + t8144 - t8145;
    t9871
}

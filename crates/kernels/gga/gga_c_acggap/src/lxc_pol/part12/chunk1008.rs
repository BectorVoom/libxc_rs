//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1008/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1008<F: Float>(t30085: F, t30091: F, t30094: F, t30099: F, t30106: F, t30110: F, t30118: F, t30121: F, t30123: F, t30125: F, t30130: F, t30132: F, t30139: F, t30151: F, t32352: F, t33914: F, t33916: F) -> (F,) {
    let t36860 = 0.17149607247227894789e-2 * t30085 + t32352 + 0.85748036236139473944e-3 * t30091 + 0.42874018118069736972e-3 * t30094 - 0.31448092289604152069e-3 * t30099 - 0.64311027177104605458e-2 * t33914 - 0.62896184579208304138e-3 * t33916 + 0.75475421495049964964e-2 * t30106 - 0.31448092289604152069e-3 * t30110 + 0.20965394859736101379e-3 * t30118 + 0.42874018118069736972e-3 * t30121 + 0.17149607247227894789e-2 * t30123 + 0.31448092289604152069e-3 * t30125 - 0.41930789719472202758e-3 * t30130 - 0.42874018118069736972e-3 * t30132 - 0.28582678745379824648e-3 * t30139 - 0.25158473831683321655e-2 * t30151;
    (t36860,)
}

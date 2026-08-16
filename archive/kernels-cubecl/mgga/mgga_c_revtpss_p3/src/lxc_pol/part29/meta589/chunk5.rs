//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1956/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1956<F: Float>(t96236: F, t97685: F, t136: F, t2457: F, t8103: F, t25944: F, t25950: F, t28845: F, t102081: F, t102084: F, t102086: F, t102090: F, t102093: F, t102096: F, t14268: F, t2097: F, t7295: F, t7296: F, t96188: F, t96193: F, t96195: F, t96197: F) -> (F, F) {
    let t102098 = F::cast_from(0.51405703062096148812e-1_f64) * t96236 * t97685;
    let t102100 = t8103 * t136 * t2457;
    let t102101 = t25944 * t102100;
    let t102104 = F::cast_from(0.25702851531048074406e-1_f64) * t25950 * t28845;
    let t102111 = F::cast_from(0.28912093960683998208e-1_f64) * t96188 + t102081 - t102084 - t102086 - t102090 + t102093 - F::cast_from(0.14456046980341999104e-1_f64) * t96193 + t102096 - t102098 + F::cast_from(0.17135234354032049604e-2_f64) * t102101 - t102104 + F::cast_from(0.25702851531048074406e-1_f64) * t96195 + F::cast_from(0.8673628188205199462e0_f64) * t7295 * t7296 * t2097 * t14268 + F::cast_from(0.14634331517634470219e-1_f64) * t96197;
    (t102100, t102111)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1956/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1956(t96236: f64, t97685: f64, t136: f64, t2457: f64, t8103: f64, t25944: f64, t25950: f64, t28845: f64, t102081: f64, t102084: f64, t102086: f64, t102090: f64, t102093: f64, t102096: f64, t14268: f64, t2097: f64, t7295: f64, t7296: f64, t96188: f64, t96193: f64, t96195: f64, t96197: f64) -> (f64, f64) {
    let t102098 = 0.51405703062096148812e-1_f64 * t96236 * t97685;
    let t102100 = t8103 * t136 * t2457;
    let t102101 = t25944 * t102100;
    let t102104 = 0.25702851531048074406e-1_f64 * t25950 * t28845;
    let t102111 = 0.28912093960683998208e-1_f64 * t96188 + t102081 - t102084 - t102086 - t102090 + t102093 - 0.14456046980341999104e-1_f64 * t96193 + t102096 - t102098 + 0.17135234354032049604e-2_f64 * t102101 - t102104 + 0.25702851531048074406e-1_f64 * t96195 + 0.8673628188205199462e0_f64 * t7295 * t7296 * t2097 * t14268 + 0.14634331517634470219e-1_f64 * t96197;
    (t102100, t102111)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2157/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2157(t19912: f64, t7111: f64, t100121: f64, t100146: f64, t19626: f64, t19641: f64, t19731: f64, t19819: f64, t19873: f64, t19944: f64, t25517: f64, t25539: f64, t25569: f64, t27498: f64, t27536: f64, t4788: f64, t6268: f64, t6293: f64, t6327: f64, t93548: f64, t93821: f64) -> f64 {
    let t107169 = t7111 * t19912;
    let t107183 = 0.47637797908966374413e-3_f64 * t25569 * t6327 - 0.25724410870841842183e-2_f64 * t100121 * t19819 + 0.17149607247227894789e-2_f64 * t27536 * t19944 - t25539 * t6293 / 81.0_f64 + t107169 / 648.0_f64 + 0.85748036236139473944e-3_f64 * t93548 * t19641 - 0.57165357490759649296e-3_f64 * t25517 * t19873 + 0.57165357490759649296e-3_f64 * t100146 * t4788 + 0.57165357490759649296e-3_f64 * t93821 * t6268 + 0.57165357490759649296e-3_f64 * t25517 * t19731 - 0.28582678745379824648e-3_f64 * t27498 * t19626;
    t107183
}

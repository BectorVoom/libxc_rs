//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1215/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1215(t107331: f64, t107335: f64, t107339: f64, t107343: f64, t107348: f64, t107353: f64, t107367: f64, t107862: f64, t544: f64, t553: f64, t90980: f64, t90993: f64, t91000: f64, t97070: f64, t97095: f64, t97108: f64, t97111: f64, t97124: f64, t97137: f64, t97142: f64) -> f64 {
    let t107928 = 0.9869604401089358619e-1_f64 * t97070 - 0.9869604401089358619e-1_f64 * t107331 + 0.19739208802178717238e0_f64 * t107335 + 0.9869604401089358619e-1_f64 * t107339 + 0.9869604401089358619e-1_f64 * t107343 + 0.46058153871750340221e0_f64 * t97095 - 0.9869604401089358619e-1_f64 * t107348 + 0.49348022005446793095e-1_f64 * t90980 + 0.29608813203268075857e0_f64 * t107353 + 0.23029076935875170111e0_f64 * t97108 - 0.24674011002723396548e-1_f64 * t97111 - 0.49348022005446793095e-1_f64 * t90993 + t544 * t553 * t107862 - 0.46058153871750340221e0_f64 * t97124 + 0.23029076935875170111e0_f64 * t97137 + 0.9869604401089358619e-1_f64 * t107367 + 0.24674011002723396548e-1_f64 * t97142 - 0.38381794893125283518e0_f64 * t91000;
    t107928
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1960/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1960(t96187: f64, t97688: f64, t28791: f64, t689: f64, t25899: f64, t25921: f64, t26257: f64, t26282: f64, t26347: f64, t27837: f64, t28806: f64, t4077: f64, t543: f64, t5658: f64, t5775: f64, t7295: f64, t7301: f64, t7506: f64, t8094: f64, t94656: f64, t96210: f64, t96211: f64, t96218: f64, t96222: f64, t96226: f64, t96230: f64) -> (f64, f64) {
    let t102164 = 0.28912093960683998208e-1_f64 * t96187 * t97688;
    let t102165 = t28791 * t689;
    let t102167 = 0.25702851531048074406e-1_f64 * t25899 * t102165;
    let t102175 = -t96210 - 0.19274729307122665471e-1_f64 * t96211 - t96218 + 0.10408353825846239354e2_f64 * t7295 * t94656 * t8094 * t4077 - 0.13170898365871023197e1_f64 * t26282 * t5775 + 0.17347256376410398924e1_f64 * t27837 * t26347 + 0.45699670022203476294e-2_f64 * t96222 + 0.17347256376410398924e1_f64 * t25921 * t28806 - 0.54878743191129263322e-2_f64 * t96226 + t102164 + t96230 + t102167 + 0.4336814094102599731e0_f64 * t27837 * t26257 + 0.8673628188205199462e0_f64 * t7295 * t7301 * t7506 * t5658 * t543;
    (t102165, t102175)
}

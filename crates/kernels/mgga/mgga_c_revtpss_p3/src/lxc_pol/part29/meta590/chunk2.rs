//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1960/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1960<F: Float>(t96187: F, t97688: F, t28791: F, t689: F, t25899: F, t25921: F, t26257: F, t26282: F, t26347: F, t27837: F, t28806: F, t4077: F, t543: F, t5658: F, t5775: F, t7295: F, t7301: F, t7506: F, t8094: F, t94656: F, t96210: F, t96211: F, t96218: F, t96222: F, t96226: F, t96230: F) -> (F, F) {
    let t102164 = F::cast_from(0.28912093960683998208e-1_f64) * t96187 * t97688;
    let t102165 = t28791 * t689;
    let t102167 = F::cast_from(0.25702851531048074406e-1_f64) * t25899 * t102165;
    let t102175 = -t96210 - F::cast_from(0.19274729307122665471e-1_f64) * t96211 - t96218 + F::cast_from(0.10408353825846239354e2_f64) * t7295 * t94656 * t8094 * t4077 - F::cast_from(0.13170898365871023197e1_f64) * t26282 * t5775 + F::cast_from(0.17347256376410398924e1_f64) * t27837 * t26347 + F::cast_from(0.45699670022203476294e-2_f64) * t96222 + F::cast_from(0.17347256376410398924e1_f64) * t25921 * t28806 - F::cast_from(0.54878743191129263322e-2_f64) * t96226 + t102164 + t96230 + t102167 + F::cast_from(0.4336814094102599731e0_f64) * t27837 * t26257 + F::cast_from(0.8673628188205199462e0_f64) * t7295 * t7301 * t7506 * t5658 * t543;
    (t102165, t102175)
}

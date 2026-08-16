//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 848/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk848(t76: f64, t8590: f64, t3142: f64, t745: f64, t595: f64, t8589: f64, t1655: f64, t236: f64, t3137: f64, t5678: f64, t5682: f64, t5689: f64, t5709: f64, t5889: f64, t598: f64, t7797: f64, t7798: f64, t7802: f64) -> f64 {
    let t8967 = t8590 * t76;
    let t8970 = t3142 * t745;
    let t8972 = t595 * t8589;
    let t8977 = -t5678 - t5682 - t5689 + t7797 + 0.11696447245269292414e1_f64 * t7798 + t7802 - t5889 - t5709 + 0.5848223622634646207e0_f64 * t8967 * t236 + 0.5848223622634646207e0_f64 * t8970 - 0.675260332e-1_f64 * t8972 * t598 - 0.675260332e-1_f64 * t3137 * t1655;
    t8977
}

//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 848/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk848<F: Float>(t76: F, t8590: F, t3142: F, t745: F, t595: F, t8589: F, t1655: F, t236: F, t3137: F, t5678: F, t5682: F, t5689: F, t5709: F, t5889: F, t598: F, t7797: F, t7798: F, t7802: F) -> F {
    let t8967 = t8590 * t76;
    let t8970 = t3142 * t745;
    let t8972 = t595 * t8589;
    let t8977 = -t5678 - t5682 - t5689 + t7797 + F::new(0.11696447245269292414e1) * t7798 + t7802 - t5889 - t5709 + F::new(0.5848223622634646207e0) * t8967 * t236 + F::new(0.5848223622634646207e0) * t8970 - F::new(0.675260332e-1) * t8972 * t598 - F::new(0.675260332e-1) * t3137 * t1655;
    t8977
}

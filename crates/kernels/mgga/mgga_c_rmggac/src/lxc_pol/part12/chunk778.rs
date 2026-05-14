//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 778/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk778<F: Float>(t34922: F, t34927: F, t34931: F, t39079: F, t39084: F, t39089: F, t39094: F, t39099: F, t39104: F, t39108: F, t39112: F, t39119: F, t39122: F, t39127: F, t39132: F, t39137: F, t39142: F, t39147: F) -> (F,) {
    let t39149 = 0.71827762319940103985e-4 * t39079 + 0.42564599893297839398e-5 * t39084 - 0.12769379967989351819e-4 * t39089 + 0.95770349759920138644e-4 * t39094 + 0.31923449919973379548e-4 * t39099 - 0.31923449919973379548e-4 * t39104 - 0.25538759935978703638e-4 * t39108 - 0.85129199786595678796e-5 * t39112 - t34922 + 0.68400385060046895006e-6 * t34927 + 0.68400385060046895006e-6 * t34931 + 0.10248087766267884742e-3 * t39119 + 0.33105799917009430643e-4 * t39122 + 0.25538759935978703638e-4 * t39127 - 0.1064114997332445985e-4 * t39132 + 0.31923449919973379548e-4 * t39137 + 0.31923449919973379548e-4 * t39142 - 0.63846899839946759096e-4 * t39147;
    (t39149,)
}

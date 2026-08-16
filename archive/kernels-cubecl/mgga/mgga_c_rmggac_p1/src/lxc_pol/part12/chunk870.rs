//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 870/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk870<F: Float>(t39141: F, t7717: F, t1624: F, t236: F, t495: F, t7230: F, t9188: F, t34922: F, t34927: F, t34931: F, t39079: F, t39084: F, t39089: F, t39094: F, t39099: F, t39104: F, t39108: F, t39112: F, t39119: F, t39122: F, t39127: F, t39132: F, t39137: F) -> F {
    let t39142 = t7717 * t39141;
    let t39147 = t7230 * t9188 * t236 * t1624 * t495;
    let t39149 = F::cast_from(0.71827762319940103985e-4_f64) * t39079 + F::cast_from(0.42564599893297839398e-5_f64) * t39084 - F::cast_from(0.12769379967989351819e-4_f64) * t39089 + F::cast_from(0.95770349759920138644e-4_f64) * t39094 + F::cast_from(0.31923449919973379548e-4_f64) * t39099 - F::cast_from(0.31923449919973379548e-4_f64) * t39104 - F::cast_from(0.25538759935978703638e-4_f64) * t39108 - F::cast_from(0.85129199786595678796e-5_f64) * t39112 - t34922 + F::cast_from(0.68400385060046895006e-6_f64) * t34927 + F::cast_from(0.68400385060046895006e-6_f64) * t34931 + F::cast_from(0.10248087766267884742e-3_f64) * t39119 + F::cast_from(0.33105799917009430643e-4_f64) * t39122 + F::cast_from(0.25538759935978703638e-4_f64) * t39127 - F::cast_from(0.1064114997332445985e-4_f64) * t39132 + F::cast_from(0.31923449919973379548e-4_f64) * t39137 + F::cast_from(0.31923449919973379548e-4_f64) * t39142 - F::cast_from(0.63846899839946759096e-4_f64) * t39147;
    t39149
}

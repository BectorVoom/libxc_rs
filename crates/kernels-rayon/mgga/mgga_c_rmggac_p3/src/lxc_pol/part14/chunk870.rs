//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 870/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk870(t39141: f64, t7717: f64, t1624: f64, t236: f64, t495: f64, t7230: f64, t9188: f64, t34922: f64, t34927: f64, t34931: f64, t39079: f64, t39084: f64, t39089: f64, t39094: f64, t39099: f64, t39104: f64, t39108: f64, t39112: f64, t39119: f64, t39122: f64, t39127: f64, t39132: f64, t39137: f64) -> f64 {
    let t39142 = t7717 * t39141;
    let t39147 = t7230 * t9188 * t236 * t1624 * t495;
    let t39149 = 0.71827762319940103985e-4_f64 * t39079 + 0.42564599893297839398e-5_f64 * t39084 - 0.12769379967989351819e-4_f64 * t39089 + 0.95770349759920138644e-4_f64 * t39094 + 0.31923449919973379548e-4_f64 * t39099 - 0.31923449919973379548e-4_f64 * t39104 - 0.25538759935978703638e-4_f64 * t39108 - 0.85129199786595678796e-5_f64 * t39112 - t34922 + 0.68400385060046895006e-6_f64 * t34927 + 0.68400385060046895006e-6_f64 * t34931 + 0.10248087766267884742e-3_f64 * t39119 + 0.33105799917009430643e-4_f64 * t39122 + 0.25538759935978703638e-4_f64 * t39127 - 0.1064114997332445985e-4_f64 * t39132 + 0.31923449919973379548e-4_f64 * t39137 + 0.31923449919973379548e-4_f64 * t39142 - 0.63846899839946759096e-4_f64 * t39147;
    t39149
}

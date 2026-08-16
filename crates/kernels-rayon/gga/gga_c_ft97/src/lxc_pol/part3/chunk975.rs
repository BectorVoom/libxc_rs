//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 975/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk975(t10363: f64, t5284: f64, t5295: f64, t816: f64, t2724: f64, t5260: f64, t820: f64, t14721: f64, t14729: f64, t14742: f64, t14763: f64, t14766: f64, t19039: f64, t19045: f64, t19049: f64, t19050: f64, t19053: f64, t19057: f64, t19066: f64, t19069: f64, t19073: f64, t19076: f64, t2691: f64, t4113: f64, t4114: f64, t4125: f64, t5239: f64, t5265: f64, t5266: f64, t811: f64, t812: f64, t821: f64) -> f64 {
    let t19080 = t10363 * t5284;
    let t19087 = t816 * t5295;
    let t19091 = t2724 * t5295;
    let t19095 = t816 * t5260;
    let t19096 = t19095 * t820;
    let t19099 = -0.72985269132393279984e0_f64 * t5265 * t5266 * t821 + 0.14597053826478655997e1_f64 * t19039 * t5266 * t812 - 0.2416365355361531912e1_f64 * t14742 * t19045 - 0.2416365355361531912e1_f64 * t19049 * t19050 + 0.2416365355361531912e1_f64 * t19053 * t19050 - 0.2416365355361531912e1_f64 * t14766 * t19057 + 0.2416365355361531912e1_f64 * t14729 * t19045 + 0.2416365355361531912e1_f64 * t14721 * t19057 - 4.0_f64 * t14763 * t5239 - 4.0_f64 * t2691 * t19066 - 4.0_f64 * t2691 * t19069 + 8.0_f64 * t2691 * t19073 + 4.0_f64 * t2691 * t19076 * t811 - 6.0_f64 * t4113 * t19080 * t820 + 4.0_f64 * t4113 * t4114 * t4125 - 2.0_f64 * t2691 * t19087 * t811 + 2.0_f64 * t4113 * t19091 * t820 - 2.0_f64 * t2691 * t19096;
    t19099
}

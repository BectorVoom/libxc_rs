//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 975/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk975<F: Float>(t10363: F, t5284: F, t5295: F, t816: F, t2724: F, t5260: F, t820: F, t14721: F, t14729: F, t14742: F, t14763: F, t14766: F, t19039: F, t19045: F, t19049: F, t19050: F, t19053: F, t19057: F, t19066: F, t19069: F, t19073: F, t19076: F, t2691: F, t4113: F, t4114: F, t4125: F, t5239: F, t5265: F, t5266: F, t811: F, t812: F, t821: F) -> F {
    let t19080 = t10363 * t5284;
    let t19087 = t816 * t5295;
    let t19091 = t2724 * t5295;
    let t19095 = t816 * t5260;
    let t19096 = t19095 * t820;
    let t19099 = -F::cast_from(0.72985269132393279984e0_f64) * t5265 * t5266 * t821 + F::cast_from(0.14597053826478655997e1_f64) * t19039 * t5266 * t812 - F::cast_from(0.2416365355361531912e1_f64) * t14742 * t19045 - F::cast_from(0.2416365355361531912e1_f64) * t19049 * t19050 + F::cast_from(0.2416365355361531912e1_f64) * t19053 * t19050 - F::cast_from(0.2416365355361531912e1_f64) * t14766 * t19057 + F::cast_from(0.2416365355361531912e1_f64) * t14729 * t19045 + F::cast_from(0.2416365355361531912e1_f64) * t14721 * t19057 - F::cast_from(4.0_f64) * t14763 * t5239 - F::cast_from(4.0_f64) * t2691 * t19066 - F::cast_from(4.0_f64) * t2691 * t19069 + F::cast_from(8.0_f64) * t2691 * t19073 + F::cast_from(4.0_f64) * t2691 * t19076 * t811 - F::cast_from(6.0_f64) * t4113 * t19080 * t820 + F::cast_from(4.0_f64) * t4113 * t4114 * t4125 - F::cast_from(2.0_f64) * t2691 * t19087 * t811 + F::cast_from(2.0_f64) * t4113 * t19091 * t820 - F::cast_from(2.0_f64) * t2691 * t19096;
    t19099
}

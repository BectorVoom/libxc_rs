//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1079/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1079<F: Float>(t34847: F, t9123: F, t16043: F, t9213: F, t9218: F, t9106: F, t10792: F, t2301: F, t37018: F, t42228: F, t42234: F, t42239: F, t42243: F, t42247: F, t42248: F, t42250: F, t42255: F, t42259: F, t42260: F, t42262: F, t5211: F, t665: F, t903: F) -> F {
    let t42264 = t34847 * t9123;
    let t42266 = t16043 * t9213;
    let t42268 = t16043 * t9218;
    let t42270 = t16043 * t9106;
    let t42272 = t10792 * t2301;
    let t42274 = F::cast_from(0.13637330827122670864e-1_f64) * t42228 + F::cast_from(0.35922725105591425692e0_f64) * t903 * t665 * t5211 - F::cast_from(0.19211284388664477842e-2_f64) * t42234 + t42239 + t42243 + t42247 + F::cast_from(0.36021158228745895953e-3_f64) * t42248 - F::cast_from(0.36021158228745895953e-3_f64) * t42250 + F::cast_from(0.42564599893297839398e-5_f64) * t42255 - t42259 - t37018 + F::cast_from(0.85129199786595678796e-5_f64) * t42260 - F::cast_from(0.25538759935978703638e-4_f64) * t42262 + F::cast_from(0.1064114997332445985e-4_f64) * t42264 + F::cast_from(0.17025839957319135759e-4_f64) * t42266 - F::cast_from(0.25538759935978703638e-4_f64) * t42268 + F::cast_from(0.25538759935978703638e-4_f64) * t42270 + F::cast_from(0.14967802127329760705e-1_f64) * t42272;
    t42274
}

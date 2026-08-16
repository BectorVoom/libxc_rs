//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1079/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1079(t34847: f64, t9123: f64, t16043: f64, t9213: f64, t9218: f64, t9106: f64, t10792: f64, t2301: f64, t37018: f64, t42228: f64, t42234: f64, t42239: f64, t42243: f64, t42247: f64, t42248: f64, t42250: f64, t42255: f64, t42259: f64, t42260: f64, t42262: f64, t5211: f64, t665: f64, t903: f64) -> f64 {
    let t42264 = t34847 * t9123;
    let t42266 = t16043 * t9213;
    let t42268 = t16043 * t9218;
    let t42270 = t16043 * t9106;
    let t42272 = t10792 * t2301;
    let t42274 = 0.13637330827122670864e-1_f64 * t42228 + 0.35922725105591425692e0_f64 * t903 * t665 * t5211 - 0.19211284388664477842e-2_f64 * t42234 + t42239 + t42243 + t42247 + 0.36021158228745895953e-3_f64 * t42248 - 0.36021158228745895953e-3_f64 * t42250 + 0.42564599893297839398e-5_f64 * t42255 - t42259 - t37018 + 0.85129199786595678796e-5_f64 * t42260 - 0.25538759935978703638e-4_f64 * t42262 + 0.1064114997332445985e-4_f64 * t42264 + 0.17025839957319135759e-4_f64 * t42266 - 0.25538759935978703638e-4_f64 * t42268 + 0.25538759935978703638e-4_f64 * t42270 + 0.14967802127329760705e-1_f64 * t42272;
    t42274
}

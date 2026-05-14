//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 944/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk944<F: Float>(t2185: F, t678: F, t9086: F, t16043: F, t9051: F, t9055: F, t34847: F, t9123: F, t9213: F, t9218: F, t9106: F, t10792: F, t2301: F, t37018: F, t42228: F, t42234: F, t42239: F, t42243: F, t42247: F, t42248: F, t42250: F, t42255: F, t5211: F, t665: F, t903: F) -> (F,) {
    let t42258 = t9086 * t2185 * t678;
    let t42259 = 0.19863479950205658386e-4 * t42258;
    let t42260 = t16043 * t9051;
    let t42262 = t16043 * t9055;
    let t42264 = t34847 * t9123;
    let t42266 = t16043 * t9213;
    let t42268 = t16043 * t9218;
    let t42270 = t16043 * t9106;
    let t42272 = t10792 * t2301;
    let t42274 = 0.13637330827122670864e-1 * t42228 + 0.35922725105591425692e0 * t903 * t665 * t5211 - 0.19211284388664477842e-2 * t42234 + t42239 + t42243 + t42247 + 0.36021158228745895953e-3 * t42248 - 0.36021158228745895953e-3 * t42250 + 0.42564599893297839398e-5 * t42255 - t42259 - t37018 + 0.85129199786595678796e-5 * t42260 - 0.25538759935978703638e-4 * t42262 + 0.1064114997332445985e-4 * t42264 + 0.17025839957319135759e-4 * t42266 - 0.25538759935978703638e-4 * t42268 + 0.25538759935978703638e-4 * t42270 + 0.14967802127329760705e-1 * t42272;
    (t42274,)
}

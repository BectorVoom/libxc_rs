//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 785/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk785<F: Float>(t3351: F, t3352: F, t511: F, t5199: F, t3350: F, t39207: F, t7751: F, t674: F, t7715: F, t8687: F, t1997: F, t7243: F, t8576: F, t1973: F, t39231: F, t39234: F, t39238: F, t39243: F, t39248: F, t39250: F, t39252: F, t39256: F, t39258: F, t39262: F, t39265: F, t39266: F, t39271: F) -> (F, F) {
    let t39275 = t3351 * t3352 * t511 * t5199;
    let t39277 = t39207 * t3350;
    let t39278 = t39277 * t7751;
    let t39281 = t8687 * t7715 * t674;
    let t39282 = t39281 * t1997;
    let t39284 = t8576 * t7243;
    let t39285 = t39284 * t1973;
    let t39286 = 0.19863479950205658386e-4 * t39285;
    let t39287 = -0.31923449919973379548e-4 * t39231 - t39234 - 0.85129199786595678796e-5 * t39238 - 0.51077519871957407276e-4 * t39243 - 0.25538759935978703638e-4 * t39248 - 0.59590439850616975157e-4 * t39250 + 0.59590439850616975157e-4 * t39252 - t39256 - 0.54549323308490683457e-1 * t39258 + 0.25538759935978703638e-4 * t39262 + t39265 + 0.51077519871957407276e-4 * t39266 - 0.12769379967989351819e-4 * t39271 - 0.38308139903968055457e-4 * t39275 + 0.1064114997332445985e-4 * t39278 - 0.1064114997332445985e-4 * t39282 + t39286;
    (t39277, t39287)
}

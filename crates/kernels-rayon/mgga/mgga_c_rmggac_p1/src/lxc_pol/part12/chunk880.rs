//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 880/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk880(t3350: f64, t39207: f64, t7751: f64, t674: f64, t7715: f64, t8687: f64, t1997: f64, t7243: f64, t8576: f64, t1973: f64, t39231: f64, t39234: f64, t39238: f64, t39243: f64, t39248: f64, t39250: f64, t39252: f64, t39256: f64, t39258: f64, t39262: f64, t39265: f64, t39266: f64, t39271: f64, t39275: f64) -> (f64, f64) {
    let t39277 = t39207 * t3350;
    let t39278 = t39277 * t7751;
    let t39281 = t8687 * t7715 * t674;
    let t39282 = t39281 * t1997;
    let t39284 = t8576 * t7243;
    let t39285 = t39284 * t1973;
    let t39286 = 0.19863479950205658386e-4_f64 * t39285;
    let t39287 = -0.31923449919973379548e-4_f64 * t39231 - t39234 - 0.85129199786595678796e-5_f64 * t39238 - 0.51077519871957407276e-4_f64 * t39243 - 0.25538759935978703638e-4_f64 * t39248 - 0.59590439850616975157e-4_f64 * t39250 + 0.59590439850616975157e-4_f64 * t39252 - t39256 - 0.54549323308490683457e-1_f64 * t39258 + 0.25538759935978703638e-4_f64 * t39262 + t39265 + 0.51077519871957407276e-4_f64 * t39266 - 0.12769379967989351819e-4_f64 * t39271 - 0.38308139903968055457e-4_f64 * t39275 + 0.1064114997332445985e-4_f64 * t39278 - 0.1064114997332445985e-4_f64 * t39282 + t39286;
    (t39277, t39287)
}

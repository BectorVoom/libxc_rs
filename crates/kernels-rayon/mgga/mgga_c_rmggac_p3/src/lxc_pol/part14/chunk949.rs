//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 949/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk949(t2298: f64, t26490: f64, t2604: f64, t8821: f64, t3928: f64, t5211: f64, t645: f64, t35655: f64, t35665: f64, t40251: f64, t40254: f64, t40260: f64, t40263: f64, t40266: f64, t40270: f64, t40274: f64, t40279: f64, t40283: f64, t40287: f64, t40291: f64, t40294: f64, t504: f64, t8795: f64) -> f64 {
    let t40295 = t26490 * t2298;
    let t40297 = t2604 * t8821;
    let t40302 = t3928 * t645 * t5211;
    let t40304 = -t40251 + 0.25538759935978703638e-4_f64 * t40254 + 0.19863479950205658386e-4_f64 * t35655 + t40260 + 0.59590439850616975158e-4_f64 * t35665 - t40263 + 0.31923449919973379548e-4_f64 * t40266 + 0.25538759935978703638e-4_f64 * t40270 - 0.51077519871957407276e-4_f64 * t40274 - 0.85129199786595678796e-5_f64 * t40279 + 0.76616279807936110914e-4_f64 * t40283 - 0.25538759935978703638e-4_f64 * t40287 + 0.76616279807936110914e-4_f64 * t40291 - t40294 + 0.8980681276397856423e-1_f64 * t40295 + 0.2993560425465952141e-1_f64 * t40297 - 0.39914139006212695214e-1_f64 * t504 * t8795 + 0.17961362552795712846e0_f64 * t40302;
    t40304
}

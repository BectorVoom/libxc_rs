//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 842/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk842<F: Float>(t2298: F, t26490: F, t2604: F, t8821: F, t3928: F, t5211: F, t645: F, t35655: F, t35665: F, t40251: F, t40254: F, t40260: F, t40263: F, t40266: F, t40270: F, t40274: F, t40279: F, t40283: F, t40287: F, t40291: F, t40294: F, t504: F, t8795: F) -> (F,) {
    let t40295 = t26490 * t2298;
    let t40297 = t2604 * t8821;
    let t40302 = t3928 * t645 * t5211;
    let t40304 = -t40251 + 0.25538759935978703638e-4 * t40254 + 0.19863479950205658386e-4 * t35655 + t40260 + 0.59590439850616975158e-4 * t35665 - t40263 + 0.31923449919973379548e-4 * t40266 + 0.25538759935978703638e-4 * t40270 - 0.51077519871957407276e-4 * t40274 - 0.85129199786595678796e-5 * t40279 + 0.76616279807936110914e-4 * t40283 - 0.25538759935978703638e-4 * t40287 + 0.76616279807936110914e-4 * t40291 - t40294 + 0.8980681276397856423e-1 * t40295 + 0.2993560425465952141e-1 * t40297 - 0.39914139006212695214e-1 * t504 * t8795 + 0.17961362552795712846e0 * t40302;
    (t40304,)
}

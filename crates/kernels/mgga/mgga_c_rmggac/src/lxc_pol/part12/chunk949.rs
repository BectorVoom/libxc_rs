//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 949/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk949<F: Float>(t9160: F, t9166: F, t9172: F, t9174: F, t9176: F, t9178: F, t42335: F, t7937: F, t7946: F, t8304: F, t9653: F, t9185: F, t9191: F, t9195: F, t9199: F, t9202: F) -> (F, F, F, F, F, F) {
    let t42336 = 0.25538759935978703638e-4 * t9160;
    let t42337 = 0.85129199786595678796e-5 * t9166;
    let t42338 = 0.85129199786595678796e-5 * t9172;
    let t42339 = 0.11974241701863808564e0 * t9174;
    let t42340 = 0.11974241701863808564e0 * t9176;
    let t42341 = 0.79828278012425390428e-1 * t9178;
    let t42343 = t9653 - t42335 + t42336 + t42337 + t42338 - t42339 - t42340 + t42341 + 0.14408463291498358381e-2 * t7937 - t8304 + t7946;
    let t42345 = 0.25538759935978703638e-4 * t9185;
    let t42346 = 0.51077519871957407276e-4 * t9191;
    let t42347 = 0.76616279807936110914e-4 * t9195;
    let t42348 = 0.25538759935978703638e-4 * t9199;
    let t42349 = 0.25538759935978703638e-4 * t9202;
    (t42343, t42345, t42346, t42347, t42348, t42349)
}

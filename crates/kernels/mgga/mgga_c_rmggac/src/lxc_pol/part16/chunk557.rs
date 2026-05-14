//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 557/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk557<F: Float>(t291: F, t8342: F, t2010: F, t1661: F, t2012: F, t2020: F, t2339: F, t2019: F, t1665: F, t2323: F, t2415: F, t935: F, t938: F, t623: F, t880: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t8343 = t8342 * t291;
    let t8344 = t2010 * t8343;
    let t8346 = t2012 * t1661;
    let t8347 = t2010 * t8346;
    let t8349 = t2020 * t2339;
    let t8350 = t2019 * t8349;
    let t8352 = t2012 * t1665;
    let t8353 = t2010 * t8352;
    let t8355 = t2020 * t2323;
    let t8356 = t2019 * t8355;
    let t8358 = t2415 * t935;
    let t8359 = t2010 * t8358;
    let t8362 = t2415 * t938;
    let t8363 = t2010 * t8362;
    let t8365 = t623 * t880;
    (t8343, t8344, t8346, t8347, t8349, t8350, t8352, t8353, t8355, t8356, t8358, t8359, t8362, t8363, t8365)
}

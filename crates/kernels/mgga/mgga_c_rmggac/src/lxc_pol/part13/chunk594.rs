//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 594/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk594<F: Float>(t2416: F, t7487: F, t2160: F, t2339: F, t638: F, t2323: F, t1540: F, t511: F, t650: F, t1411: F, t2011: F, t291: F, t2010: F, t1661: F, t2012: F, t2020: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t8328 = t7487 * t2416;
    let t8331 = t638 * t2160 * t2339;
    let t8334 = t638 * t2160 * t2323;
    let t8339 = t1540 * t511;
    let t8340 = t8339 * t650;
    let t8342 = t2011 * t1411;
    let t8343 = t8342 * t291;
    let t8344 = t2010 * t8343;
    let t8346 = t2012 * t1661;
    let t8347 = t2010 * t8346;
    let t8349 = t2020 * t2339;
    (t8328, t8331, t8334, t8339, t8340, t8342, t8343, t8344, t8346, t8347, t8349)
}

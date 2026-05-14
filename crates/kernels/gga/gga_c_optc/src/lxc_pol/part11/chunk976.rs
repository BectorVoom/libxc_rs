//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 976/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk976<F: Float>(t1422: F, t8344: F, t1329: F, t7668: F, t7680: F, t1359: F, t7501: F, t1347: F, t7798: F, t7341: F, t7758: F, t1378: F, t1781: F, t862: F, t1382: F, t24407: F) -> (F, F, F, F, F, F, F, F, F) {
    let t30408 = t1422 * t8344;
    let t30661 = t1329 * t7668;
    let t30827 = t1329 * t7680;
    let t31281 = t1359 * t7501;
    let t31288 = t1347 * t7798;
    let t31301 = t1359 * t7341;
    let t31304 = t1347 * t7758;
    let t31479 = t862 * t1781 * t1378;
    let t31579 = t24407 * t1382;
    (t30408, t30661, t30827, t31281, t31288, t31301, t31304, t31479, t31579)
}

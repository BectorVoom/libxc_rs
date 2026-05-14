//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 614/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk614<F: Float>(t5184: F, t6986: F, t5182: F, t1333: F, t2534: F, t2510: F, t2514: F, t3521: F, t4595: F, t708: F, t6759: F, t1648: F, t2364: F, t4604: F, t1824: F, t4609: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6987 = t5184 * t6986;
    let t6988 = t5182 * t6987;
    let t6990 = t1333 * t2534;
    let t6992 = t1333 * t2510;
    let t6998 = t3521 * t2514;
    let t7000 = t4595 * t708;
    let t7001 = t7000 * t6759;
    let t7005 = t4604 * t2364 * t1648;
    let t7009 = t4609 * t2364 * t1824;
    (t6987, t6988, t6990, t6992, t6998, t7000, t7001, t7005, t7009)
}

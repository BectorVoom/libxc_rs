//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1139/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1139<F: Float>(t10226: F, t3739: F, t11640: F, t828: F, t10230: F, t11633: F, t11629: F, t3254: F, t10102: F, t3724: F, t10292: F, t11684: F, t11691: F, t2165: F, t24761: F, t35682: F, t6857: F, t8452: F) -> (F, F, F, F, F, F, F, F) {
    let t35921 = t10226 * t3739;
    let t35923 = t828 * t11640;
    let t35925 = t10230 * t11633;
    let t35928 = t3254 * t11629;
    let t35930 = t10102 * t3724;
    let t35932 = t10292 * t11684;
    let t35934 = t2165 * t11691;
    let t35938 = t35682 * t24761 * t8452 * t6857;
    (t35921, t35923, t35925, t35928, t35930, t35932, t35934, t35938)
}

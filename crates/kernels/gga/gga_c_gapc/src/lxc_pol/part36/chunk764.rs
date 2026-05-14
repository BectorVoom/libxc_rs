//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 764/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk764<F: Float>(t10068: F, t3405: F, t3411: F, t3414: F, t9722: F, t1084: F, t8711: F, t134: F, t7877: F, t442: F, t7591: F, t941: F, t2902: F, t761: F, t3221: F, t1474: F, t277: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10069 = t3405 * t10068;
    let t10070 = t3411 * t10069;
    let t10072 = t9722 * t3414;
    let t10073 = t3411 * t10072;
    let t10075 = t1084 * t8711;
    let t10077 = t134 * t7877;
    let t10078 = t10077 * t442;
    let t10079 = t7591 * t941 * t10078;
    let t10080 = t10075 * t10079;
    let t10102 = t2902 * t761;
    let t10103 = t10102 * t3221;
    let t10105 = t1474 * t277;
    (t10069, t10070, t10072, t10073, t10078, t10079, t10080, t10102, t10103, t10105)
}

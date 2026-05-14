//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 757/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk757<F: Float>(t3727: F, t787: F, t2588: F, t876: F, t898: F, t1033: F, t7089: F, t311: F, t474: F, t919: F, t3288: F, t7165: F, t1092: F, t3293: F, t3297: F, t869: F) -> (F, F, F, F, F, F) {
    let t9969 = t3727 * t787;
    let t9970 = t2588 * t9969;
    let t9972 = t3727 * t876;
    let t9973 = t898 * t9972;
    let t9975 = t7089 * t1033;
    let t9976 = t311 * t9975;
    let t9977 = t474 * t919;
    let t9978 = t9976 * t9977;
    let t9980 = t3288 * t7165;
    let t9981 = t1092 * t9980;
    let t9986 = t869 * t3293 * t3297;
    (t9970, t9973, t9978, t9980, t9981, t9986)
}

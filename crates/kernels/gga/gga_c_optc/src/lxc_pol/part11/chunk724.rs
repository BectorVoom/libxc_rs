//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 724/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk724<F: Float>(t1382: F, t896: F, t2673: F, t2638: F, t311: F, t330: F, t1378: F, t530: F, t862: F, t8113: F, t1388: F, t7878: F, t893: F, t1384: F, t2619: F, t874: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10975 = t896 * t1382;
    let t10976 = t10975 * t2673;
    let t10990 = t2638 * t311;
    let t10991 = t330 * t10990;
    let t11007 = t530 * t1378;
    let t11008 = t862 * t11007;
    let t11018 = t330 * t8113;
    let t11073 = t7878 * t1388;
    let t11074 = t893 * t11073;
    let t11110 = t2619 * t1384;
    let t11111 = t874 * t11110;
    (t10975, t10976, t10990, t10991, t11008, t11018, t11073, t11074, t11111)
}

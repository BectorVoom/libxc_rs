//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 911/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk911<F: Float>(t24: F, t2548: F, t1013: F, t2434: F, t8384: F, t19: F, t2606: F, t3813: F, t2274: F, t2643: F, t2264: F, t123: F, t2673: F, t311: F, t7856: F, t140: F, t309: F, t883: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10838 = t24 * t2548;
    let t10845 = t2434 * t1013;
    let t10849 = t8384 * t1013;
    let t10887 = t19 * t2606;
    let t10888 = t10887 * t3813;
    let t10913 = t2643 * t2274;
    let t10918 = t2643 * t2264;
    let t10925 = t2606 * t123;
    let t10926 = t2673 * t10925;
    let t10935 = t311 * t7856;
    let t10952 = t883 * t309 * t140;
    (t10838, t10845, t10849, t10888, t10913, t10918, t10925, t10926, t10935, t10952)
}

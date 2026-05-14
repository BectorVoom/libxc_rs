//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 396/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk396<F: Float>(t2153: F, t282: F, t61: F, t268: F, t995: F, t19: F, t792: F, t1561: F, t315: F, t277: F, t825: F, t1474: F, t641: F, t919: F, t21: F, t811: F) -> (F, F, F, F, F, F, F, F) {
    let t2154 = t2153 * t282;
    let t2155 = t61 * t2154;
    let t2158 = t995 * t268;
    let t2159 = t792 * t19;
    let t2160 = t2158 * t2159;
    let t2161 = t1561 * t315;
    let t2164 = t277 * t825;
    let t2165 = t1474 * t2164;
    let t2166 = t641 * t919;
    let t2185 = t811 * t21;
    (t2155, t2158, t2160, t2161, t2164, t2165, t2166, t2185)
}

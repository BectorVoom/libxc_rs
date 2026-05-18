//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 772/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk772<F: Float>(t2842: F, t2844: F, t684: F, t2881: F, t2739: F, t312: F, t2874: F, t2878: F, t8392: F, t2885: F, t1934: F, t875: F) -> (F, F, F, F, F, F, F) {
    let t10452 = t2842 * t2844 * t684;
    let t10453 = t2881 * t10452;
    let t10457 = t312 * t2739 * t684;
    let t10458 = t2874 * t10457;
    let t10461 = t8392 * t2878;
    let t10463 = t8392 * t2885;
    let t10465 = t1934 * t875;
    (t10452, t10453, t10457, t10458, t10461, t10463, t10465)
}

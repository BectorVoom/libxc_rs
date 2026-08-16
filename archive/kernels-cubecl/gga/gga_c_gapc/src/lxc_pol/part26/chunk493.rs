//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 493/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk493<F: Float>(t2880: F, t458: F, t2879: F, t119: F, t462: F, t125: F, t4: F, t173: F, t144: F, t188: F, t152: F, t1947: F) -> (F, F, F, F, F, F, F, F) {
    let t2881 = t2880 * t458;
    let t2882 = t2879 * t2881;
    let t2884 = t462 * t119;
    let t2885 = t4 * t125;
    let t2886 = t2885 * t173;
    let t2887 = t2884 * t2886;
    let t2889 = t188 * t144;
    let t2890 = t2889 * t152;
    let t2891 = t2890 * t1947;
    (t2881, t2882, t2884, t2885, t2886, t2887, t2890, t2891)
}

//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 518/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk518<F: Float>(t1912: F, t3046: F, t1027: F, t664: F, t684: F, t185: F, t1936: F, t649: F, t128: F, t654: F, t122: F, t424: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3047 = t3046 * t1912;
    let t3049 = t1027 * t664;
    let t3051 = t1027 * t684;
    let t3053 = t185 * t1936;
    let t3054 = t3053 * t649;
    let t3056 = t654 * t128;
    let t3057 = t185 * t3056;
    let t3058 = t3057 * t649;
    let t3060 = t424 * t122;
    (t3047, t3049, t3051, t3053, t3054, t3056, t3057, t3058, t3060)
}

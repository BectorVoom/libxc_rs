//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 526/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk526<F: Float>(t3039: F, t637: F, t670: F, t87: F, t8: F, t1734: F, t1743: F, t1912: F, t1027: F, t664: F, t684: F, t185: F, t1936: F) -> (F, F, F, F, F, F, F, F) {
    let t3040 = t3039 * t637;
    let t3042 = t670 * t87;
    let t3044 = F::new(1.0) / t8 / t3042;
    let t3045 = t1734 * t3044;
    let t3046 = t1743 * t3045;
    let t3047 = t3046 * t1912;
    let t3049 = t1027 * t664;
    let t3051 = t1027 * t684;
    let t3053 = t185 * t1936;
    (t3040, t3042, t3044, t3045, t3047, t3049, t3051, t3053)
}

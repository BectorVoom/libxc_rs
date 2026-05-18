//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 423/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk423<F: Float>(t2210: F, t2212: F, t268: F, t492: F, t798: F, t1482: F, t827: F, t462: F, t760: F, t513: F, t786: F, t875: F) -> (F, F, F, F, F, F, F, F) {
    let t2213 = t2210 * t2212;
    let t2216 = t492 * t268;
    let t2217 = t2216 * t798;
    let t2220 = t1482 * t268;
    let t2221 = t2220 * t827;
    let t2224 = t462 * t760;
    let t2225 = t2224 * t798;
    let t2228 = t513 * t760;
    let t2229 = t2228 * t827;
    let t2232 = t786 * t875;
    (t2213, t2216, t2217, t2221, t2224, t2225, t2229, t2232)
}

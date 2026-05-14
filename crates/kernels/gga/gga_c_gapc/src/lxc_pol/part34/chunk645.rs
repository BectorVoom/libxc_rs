//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 645/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk645<F: Float>(t2232: F, t442: F, t1474: F, t268: F, t122: F, t2435: F, t1971: F, t291: F, t786: F, t830: F) -> (F, F, F, F, F) {
    let t6172 = t2232 * t442;
    let t6178 = t1474 * t268;
    let t6179 = t2435 * t122;
    let t6181 = t1971 * t291;
    let t6182 = t830 * t786;
    (t6172, t6178, t6179, t6181, t6182)
}

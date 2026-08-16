//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 673/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk673<F: Float>(t125: F, t2207: F, t291: F, t667: F, t2232: F, t442: F, t1474: F, t268: F, t122: F, t2435: F, t1971: F, t786: F, t830: F) -> (F, F, F, F, F, F, F) {
    let t6146 = t2207 * t125;
    let t6148 = t667 * t291;
    let t6172 = t2232 * t442;
    let t6178 = t1474 * t268;
    let t6179 = t2435 * t122;
    let t6181 = t1971 * t291;
    let t6182 = t830 * t786;
    (t6146, t6148, t6172, t6178, t6179, t6181, t6182)
}

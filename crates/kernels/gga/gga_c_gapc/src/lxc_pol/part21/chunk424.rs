//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 424/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk424<F: Float>(t2232: F, t836: F, t772: F, t1: F, t769: F, t791: F, t468: F, t892: F, t924: F, t474: F, t818: F, t801: F) -> (F, F, F, F, F, F, F) {
    let t2233 = t836 * t2232;
    let t2234 = t772 * t2233;
    let t2237 = t769 * t1;
    let t2238 = t791 * t2237;
    let t2239 = t468 * t892;
    let t2242 = t468 * t924;
    let t2245 = t474 * t818;
    let t2246 = t2245 * t801;
    (t2233, t2234, t2238, t2239, t2242, t2245, t2246)
}

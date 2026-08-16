//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 794/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk794<F: Float>(t1: F, t14292: F, t2263: F, t297: F, t4941: F, t896: F, t19: F, t7433: F, t2679: F, t2678: F, t2674: F, t2668: F) -> (F, F, F, F, F) {
    let t14293 = t14292 * t1;
    let t14294 = t297 * t2263;
    let t14299 = t896 * t4941;
    let t14300 = t14299 * t19;
    let t14306 = t7433 * t4941;
    let t14307 = t14306 * t2679;
    let t14308 = t2678 * t14307;
    let t14312 = t14306 * t2674;
    let t14313 = t2668 * t14312;
    (t14293, t14294, t14300, t14308, t14313)
}

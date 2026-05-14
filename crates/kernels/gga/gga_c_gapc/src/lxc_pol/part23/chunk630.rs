//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 630/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk630<F: Float>(t442: F, t5972: F, t5971: F, t169: F, t4605: F, t5: F, t521: F, t1403: F, t1666: F, t1388: F, t515: F, t1983: F, t618: F, t125: F, t2207: F, t291: F, t667: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5973 = t5972 * t442;
    let t5974 = t5971 * t5973;
    let t5977 = t169 * t4605;
    let t5979 = t521 * t5;
    let t5983 = t1666 * t1403;
    let t5987 = t1388 * t515;
    let t6055 = t618 * t1983;
    let t6146 = t2207 * t125;
    let t6148 = t667 * t291;
    (t5973, t5974, t5977, t5979, t5983, t5987, t6055, t6146, t6148)
}

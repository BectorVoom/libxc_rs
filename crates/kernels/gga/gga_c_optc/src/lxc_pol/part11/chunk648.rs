//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 648/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk648<F: Float>(t1188: F, t1570: F, t1575: F, t277: F, t4276: F, t4278: F, t4540: F, t490: F, t498: F, t5169: F, t5173: F, t5189: F, t5192: F, t5266: F, t5270: F, t5306: F, t5310: F, t5434: F, t5441: F, t5474: F, t95: F) -> (F,) {
    let t5478 = t5266 - t5270 - 8.0 / 9.0 * t4276 + t4278 / 3.0 + 0.25844881434903430496e-2 * t95 * t277 * t5434 * t1188 - t5310 + 44.0 / 9.0 * t490 * t5441 - 8.0 / 3.0 * t1570 * t1575 + t5474 * t498 / 2.0 + t4540 / 9.0 + t5306 + t5169 - t5173 + t5189 + t5192;
    (t5478,)
}

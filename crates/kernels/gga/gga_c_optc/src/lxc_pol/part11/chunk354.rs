//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 354/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk354<F: Float>(t1188: F, t1216: F, t1220: F, t1448: F, t1462: F, t1488: F, t1490: F, t1494: F, t1554: F, t1570: F, t1575: F, t1579: F, t1585: F, t1588: F, t277: F, t490: F, t498: F, t95: F) -> (F,) {
    let t1591 = -t1448 + t1462 + t1488 + t1490 - t1494 + 0.25844881434903430496e-2 * t95 * t277 * t1554 * t1188 + t1570 * t498 / 2.0 - 4.0 / 3.0 * t490 * t1575 + t1216 + t1220 * t1579 / 6.0 + 50.0 / 27.0 * t1585 * t1588;
    (t1591,)
}

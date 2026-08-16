//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 693/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk693<F: Float>(t123: F, t1570: F, t4183: F, t883: F, t882: F, t2344: F, t4324: F, t2343: F, t2304: F, t4807: F, t423: F, t481: F, t482: F) -> (F, F, F, F, F, F) {
    let t6485 = t1570 * t123;
    let t6486 = t883 * t4183;
    let t6487 = t6485 * t6486;
    let t6488 = t882 * t6487;
    let t6490 = t2344 * t4324;
    let t6491 = t2343 * t6490;
    let t6494 = t2304 * t4807;
    let t6498 = t481 * t482 * t423;
    (t6486, t6488, t6490, t6491, t6494, t6498)
}

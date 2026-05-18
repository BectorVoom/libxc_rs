//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 828/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk828<F: Float>(t1324: F, t999: F, t1323: F, t2778: F, t1064: F, t1328: F, t2854: F, t6320: F, t2787: F, t4324: F, t2343: F, t2765: F, t4807: F) -> (F, F, F, F, F, F, F, F) {
    let t7952 = t999 * t1324;
    let t7957 = t2778 * t1323;
    let t7958 = t1064 * t7957;
    let t7963 = t2854 * t1328;
    let t7964 = t6320 * t7963;
    let t7967 = t2787 * t4324;
    let t7968 = t2343 * t7967;
    let t7971 = t2765 * t4807;
    (t7952, t7957, t7958, t7963, t7964, t7967, t7968, t7971)
}

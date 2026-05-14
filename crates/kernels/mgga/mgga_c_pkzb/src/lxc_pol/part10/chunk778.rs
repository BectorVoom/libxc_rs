//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 778/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk778<F: Float>(t204: F, t205: F, t3730: F, t2173: F, t3017: F, t352: F, t1185: F, t3033: F, t1184: F, t852: F) -> (F, F, F, F, F, F) {
    let t3732 = t204 * t205 * t3730;
    let t3734 = t2173 - 0.35616666666666666666e-1 * t3017 + 0.53425e-1 * t3732;
    let t3736 = 0.621814e-1 * t3734 * t352;
    let t3738 = 2.0 * t3033 * t1185;
    let t3739 = t1184 * t1184;
    let t3740 = t3739 * t852;
    (t3732, t3734, t3736, t3738, t3739, t3740)
}

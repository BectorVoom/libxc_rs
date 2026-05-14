//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 945/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk945<F: Float>(t6967: F, t6969: F, t6972: F, t9008: F, t9012: F, t9029: F, t387: F, t9011: F, t7183: F, t1434: F, t2578: F, t1422: F, t2539: F, t1005: F, t3580: F, t2593: F) -> (F, F, F, F, F, F, F, F) {
    let t9031 = -t6967 + 0.24722222222222222222e-1 * t6969 - 0.92708333333333333333e-2 * t6972 + 0.12361111111111111111e-1 * t9008 - t9012 + 0.278125e-1 * t9029;
    let t9032 = t9031 * t387;
    let t9037 = 0.34246666666666666666e-1 * t9011;
    let t9039 = -t7183 + 0.45662222222222222222e-1 * t6969 - 0.17123333333333333333e-1 * t6972 + 0.22831111111111111111e-1 * t9008 - t9037 + 0.5137e-1 * t9029;
    let t9042 = t1434 * t2578;
    let t9045 = t1422 * t2539;
    let t9048 = t3580 * t1005;
    let t9051 = t1434 * t2593;
    (t9031, t9032, t9037, t9039, t9042, t9045, t9048, t9051)
}

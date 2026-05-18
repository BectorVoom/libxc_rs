//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1248/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1248<F: Float>(t1433: F, t2599: F, t7109: F, t6993: F, t1409: F, t2521: F, t7148: F, t1056: F, t3622: F, t3630: F, t2707: F, t9321: F) -> (F, F, F, F, F, F, F, F) {
    let t25813 = t2599 * t1433;
    let t25816 = t7109 * t1433;
    let t25819 = t6993 * t1433;
    let t25823 = t2521 * t1409;
    let t25826 = t7148 * t1409;
    let t25907 = F::new(32.0) * t3622 * t1056;
    let t25930 = F::new(32.0) * t3630 * t1056;
    let t25937 = t9321 * t2707;
    (t25813, t25816, t25819, t25823, t25826, t25907, t25930, t25937)
}

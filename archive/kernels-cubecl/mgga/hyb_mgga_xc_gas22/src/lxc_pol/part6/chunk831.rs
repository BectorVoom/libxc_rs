//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 831/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk831<F: Float>(t6012: F, t704: F, t1890: F, t2057: F, t2062: F, t2066: F, t697: F, t701: F, t17: F, t2053: F, t700: F) -> (F, F, F, F, F, F, F) {
    let t6279 = t6012 * t704;
    let t6281 = t1890 * t2057;
    let t6283 = t1890 * t2062;
    let t6285 = t1890 * t2066;
    let t6288 = F::cast_from(1.0_f64) / t697 / t701;
    let t6289 = t17 * t6288;
    let t6291 = F::cast_from(1.0_f64) / t2053 / t700;
    (t6279, t6281, t6283, t6285, t6288, t6289, t6291)
}

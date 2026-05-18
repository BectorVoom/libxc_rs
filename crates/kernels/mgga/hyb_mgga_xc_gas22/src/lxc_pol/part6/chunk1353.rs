//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1353/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1353<F: Float>(t10667: F, t10703: F, t2289: F, t2292: F, t24799: F, t24996: F, t260: F, t28949: F, t29068: F, t29071: F, t29072: F, t29079: F, t29081: F, t29128: F, t29177: F, t29214: F, t29316: F, t29337: F, t29380: F, t29384: F, t29387: F, t29423: F, t29478: F, t3422: F, t3443: F, t3444: F, t849: F, t856: F) -> F {
    let t29493 = F::new(0.14035736694323150897e2) * t856 * t10667 * t2292 + t29068 + t29071 - F::new(0.34631718211362927518e2) * t856 * t29072 * t3444 - t29079 - t29081 + t260 * (t29128 + t29177 + t29214 + t29316 + t29337 + t29380 + t29423 + t29478) + F::new(0.23392894490538584828e1) * t856 * t2289 * t10703 * t849 + F::new(0.4155806185363551302e3) * t24799 * t3422 * t28949 - F::new(0.34631718211362927518e2) * t856 * t3443 * t24996 + t29384 - t29387;
    t29493
}

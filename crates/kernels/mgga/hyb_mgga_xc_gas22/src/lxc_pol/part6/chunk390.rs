//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 390/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk390<F: Float>(t1014: F, t1398: F, t1412: F, t1414: F, t1422: F, t1427: F, t1434: F, t1442: F, t260: F, t374: F, t979: F, t998: F) -> F {
    let t1445 = -t1398 + t1412 + t260 * (-F::new(0.310907e-1) * t1414 * t374 + F::new(1.0) * t979 * t1422 + t1398 - t1412 - F::new(0.19751673498613801407e-1) * t1427 + F::new(0.5848223622634646207e0) * t998 * t1434) + F::new(0.19751673498613801407e-1) * t260 * t1427 - F::new(0.5848223622634646207e0) * t1014 * t1442;
    t1445
}

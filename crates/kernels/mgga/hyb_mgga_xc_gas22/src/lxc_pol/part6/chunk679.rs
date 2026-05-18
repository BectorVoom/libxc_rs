//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 679/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk679<F: Float>(t1371: F, t847: F, t2167: F, t2218: F, t2297: F, t2302: F, t3300: F, t3311: F, t3325: F, t3330: F, t3336: F, t3338: F, t3342: F, t3346: F, t3350: F) -> (F, F) {
    let t3404 = t1371 * t847;
    let t3418 = -F::new(0.1294625e1) * t3325 + F::new(0.258925e1) * t3330 + t2297 - F::new(0.301925e0) * t2167 - F::new(0.301925e0) * t3300 + F::new(0.905775e0) * t3311 + F::new(0.82524375e-1) * t3336 + F::new(0.16504875e0) * t3338 + t2302 - F::new(0.16557e0) * t2218 - F::new(0.16557e0) * t3342 + F::new(0.248355e0) * t3346 + F::new(0.248355e0) * t3350;
    (t3404, t3418)
}

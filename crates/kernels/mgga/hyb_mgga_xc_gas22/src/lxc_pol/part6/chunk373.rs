//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 373/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk373<F: Float>(t1363: F, t284: F, t1331: F, t1337: F, t1340: F, t1344: F, t842: F, t845: F) -> (F, F) {
    let t1364 = t1363 * t284;
    let t1370 = F::new(0.258925e1) * t1337 - t842 + F::new(0.905775e0) * t1331 + F::new(0.16504875e0) * t1340 - t845 + F::new(0.248355e0) * t1344;
    (t1364, t1370)
}

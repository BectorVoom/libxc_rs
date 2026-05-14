//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 383/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk383<F: Float>(t1426: F, t387: F, t1000: F, t1003: F, t1394: F, t1400: F, t1403: F, t1407: F) -> (F, F) {
    let t1427 = t1426 * t387;
    let t1433 = 0.258925e1 * t1400 - t1000 + 0.905775e0 * t1394 + 0.16504875e0 * t1403 - t1003 + 0.248355e0 * t1407;
    (t1427, t1433)
}

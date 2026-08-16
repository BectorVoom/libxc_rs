//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1013/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1013<F: Float>(t198: F, t9488: F, t3662: F, t1555: F, t2869: F, t7580: F, t1297: F, t313: F, t1834: F, sigma2: F) -> (F, F, F, F, F) {
    let t9489 = t9488 * t198;
    let t9490 = t3662 * t9489;
    let t9493 = t1555 * t2869;
    let t9501 = t7580 * sigma2;
    let t9502 = t1297 * t313;
    let t9503 = t9502 * t1834;
    (t9489, t9490, t9493, t9501, t9503)
}

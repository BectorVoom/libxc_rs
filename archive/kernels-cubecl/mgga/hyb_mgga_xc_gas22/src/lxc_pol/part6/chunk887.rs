//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 887/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk887<F: Float>(t458: F, t7543: F, t496: F, t1052: F, t2814: F, t2640: F, t2643: F, t2676: F, t1089: F, t1096: F, t7410: F, t1110: F) -> (F, F, F, F, F, F, F) {
    let t7544 = t458 * t7543;
    let t7546 = F::cast_from(120.0_f64) * t7544 * t496;
    let t7547 = t1052 * t2814;
    let t7549 = t2643 * t2640;
    let t7551 = t2643 * t2676;
    let t7554 = t1089 * t7410 * t1096;
    let t7556 = F::cast_from(0.5848223622634646207e0_f64) * t1110 * t7554;
    (t7544, t7546, t7547, t7549, t7551, t7554, t7556)
}

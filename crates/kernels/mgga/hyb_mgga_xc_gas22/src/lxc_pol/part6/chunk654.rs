//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 654/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk654<F: Float>(t1238: F, t2022: F, t3177: F, t675: F, t3: F, t699: F, t702: F, t2047: F, t2048: F, t3023: F, t3169: F, t3174: F, t572: F) -> (F, F, F, F) {
    let t3178 = t2022 * t1238;
    let t3180 = t3177 * t3178 * t675;
    let t3184 = t699 * t702 * t3;
    let t3187 = t2047 + t2048 / F::new(162.0) + t3169 / F::new(162.0) - t572 * t3174 / F::new(81.0) + t572 * t3180 / F::new(27.0) + t3023 * t3184 / F::new(27.0);
    (t3178, t3180, t3184, t3187)
}

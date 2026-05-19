//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1174/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1174<F: Float>(t2053: F, t341: F, t20686: F, t238: F, t353: F, t6611: F, t963: F, t2213: F, t2507: F, t2511: F, t20626: F, t343: F, t348: F) -> (F, F, F, F, F, F, F) {
    let t21402 = F::new(1.0) / t341 / t2053;
    let t21424 = t238 * t20686 * t353;
    let t21425 = F::cast_from(0.13490888888888888889e1_f64) * t21424;
    let t21427 = t238 * t6611 * t963;
    let t21430 = t238 * t2213 * t2507;
    let t21433 = t238 * t2213 * t2511;
    let t21462 = F::new(1.0) / t348 / t20626 / t353 / t343 / F::new(96.0);
    (t21402, t21424, t21425, t21427, t21430, t21433, t21462)
}

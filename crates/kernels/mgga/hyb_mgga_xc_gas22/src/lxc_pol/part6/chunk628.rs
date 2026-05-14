//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 628/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk628<F: Float>(t43: F, t3: F, t575: F, t578: F, t1888: F, t1891: F, t3006: F, t3011: F, t3017: F, t3023: F, t572: F) -> (F, F, F) {
    let t45 = 0.135e1 < t43;
    let t3025 = t575 * t578 * t3;
    let t3028 = t1888 + t1891 / 162.0 + t3006 / 162.0 - t572 * t3011 / 81.0 + t572 * t3017 / 27.0 - t3023 * t3025 / 27.0;
    let t3029 = piecewise3(t45, t3028, 0.0);
    (t3025, t3028, t3029)
}

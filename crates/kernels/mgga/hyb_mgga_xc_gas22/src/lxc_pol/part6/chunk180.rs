//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 180/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk180<F: Float>(t531: F, t537: F, t510: F, t513: F, t518: F, t521: F, t524: F, t532: F, t536: F, t459: F, t3: F, t5: F) -> (F, F, F, F, F) {
    let t538 = t537 * t531;
    let t541 = param_c_os_0 + t510 * t513 + t518 * t521 + t524 * t532 / F::new(2.0) + t536 * t538 / F::new(2.0);
    let t543 = F::new(1.0) / t459;
    let t544 = t3 * t543;
    let t545 = t5 - t544;
    (t538, t541, t543, t544, t545)
}

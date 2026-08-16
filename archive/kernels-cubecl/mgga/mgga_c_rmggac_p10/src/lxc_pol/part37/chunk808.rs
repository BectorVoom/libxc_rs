//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 808/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk808<F: Float>(t2808: F, t511: F, t3352: F, t14258: F, t2841: F, t495: F, t14230: F, t14249: F, t2067: F, t14124: F, t14125: F, t201: F, t457: F, t558: F) -> (F, F, F) {
    let t74555 = t511 * t2808;
    let t74556 = t3352 * t74555;
    let t74557 = t14258 * t74556;
    let t74559 = t2841 * t495;
    let t74562 = t14230 * t14249 * t2067 * t74559;
    let t74569 = t14124 * t14125 * t511 * t558 * t457 * t201;
    (t74557, t74562, t74569)
}

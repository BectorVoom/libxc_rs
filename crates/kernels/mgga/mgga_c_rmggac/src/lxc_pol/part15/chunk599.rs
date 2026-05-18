//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 599/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk599<F: Float>(t236: F, t8455: F, t1971: F, t7453: F, t2368: F, t504: F, t529: F, t7754: F) -> (F, F, F, F) {
    let t8456 = t236 * t8455;
    let t8457 = t1971 * t8456;
    let t8458 = t7453 * t8457;
    let t8460 = t504 * t2368;
    let t8465 = t7754 * t529;
    (t8457, t8458, t8460, t8465)
}

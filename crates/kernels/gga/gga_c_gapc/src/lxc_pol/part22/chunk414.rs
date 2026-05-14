//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 414/1209 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk414<F: Float>(t61: F, t760: F, t798: F, t2224: F, t793: F, t435: F, t818: F) -> (F, F, F, F) {
    let t2261 = t61 * t760;
    let t2262 = t2261 * t798;
    let t2265 = t2224 * t793;
    let t2268 = t435 * t818;
    (t2261, t2262, t2265, t2268)
}

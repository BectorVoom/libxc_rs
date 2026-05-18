//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1372/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1372<F: Float>(t15139: F, t22493: F, t11624: F, t13917: F, t51066: F, t13888: F, t353: F, t3886: F, t859: F, t2249: F, t56296: F, t11541: F) -> (F, F, F, F) {
    let t57581 = t22493 * t15139;
    let t57584 = t13917 * t51066 * t11624;
    let t57588 = t859 * t353 * t13888 * t3886;
    let t57591 = t2249 * t56296;
    let t57593 = t13917 * t57591 * t11541;
    (t57581, t57584, t57588, t57593)
}

//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 811/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk811<F: Float>(t2298: F, t322: F, t2164: F, t2197: F, t2192: F, t2331: F, t899: F, t912: F, t918: F, t2079: F, t2105: F, t4394: F, param_a_c: F) -> (F, F, F, F, F, F, F) {
    let t6429 = t322 * t2298;
    let t6445 = t2164 * t2197;
    let t6447 = t2164 * t2192;
    let t6455 = t899 * t912 * t2331;
    let t6456 = t6455 * t918;
    let t6469 = t2079 * param_a_c;
    let t6472 = t4394 * t2105;
    (t6429, t6445, t6447, t6455, t6456, t6469, t6472)
}

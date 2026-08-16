//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1189/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1189<F: Float>(t13991: F, t9270: F, t4002: F, t4453: F, t13939: F, t2367: F, t2271: F, t938: F, t6745: F, t13808: F, t13877: F, t2242: F, t4013: F) -> (F, F, F, F, F, F, F) {
    let t51102 = t9270 * t13991;
    let t51122 = t4453 * t4002;
    let t51126 = t2367 * t13939;
    let t51134 = t2271 * t938;
    let t51142 = t6745 * t4002;
    let t51153 = t13808 * t13877;
    let t51156 = t2242 * t4013;
    (t51102, t51122, t51126, t51134, t51142, t51153, t51156)
}

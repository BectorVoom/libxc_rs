//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1073/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1073<F: Float>(t13398: F, t13578: F, t2255: F, t2277: F, t37800: F, t3781: F, t44246: F, t45805: F, t49315: F, t49316: F, t49318: F, t49327: F, t49329: F, t49334: F, t49344: F, t49345: F, t6685: F, t9482: F) -> (F,) {
    let t49346 = -t49315 - t49316 - t49318 + 3.0 / 128.0 * t6685 * t2255 * t3781 * t13398 - t49327 - t49329 - t49334 - t2277 * t9482 * t45805 * t37800 / 64.0 + t2277 * t9482 * t13578 * t44246 / 64.0 + t49344 - t49345;
    (t49346,)
}

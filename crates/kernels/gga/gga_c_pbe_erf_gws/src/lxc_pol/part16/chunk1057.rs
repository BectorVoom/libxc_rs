//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1057/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1057<F: Float>(t2277: F, t2343: F, t6545: F, t902: F, t9105: F, t9110: F, t9113: F, t9114: F, t9118: F, t9121: F, t9123: F, t9570: F, t9575: F, t9579: F, t9581: F) -> F {
    let t9584 = t9105 - t9110 - t9113 - t2277 * t9570 / F::new(256.0) - t9114 + F::new(7.0) / F::new(2304.0) * t6545 - t9118 + t9121 + t9123 + t902 * t9575 / F::new(1536.0) + t9579 + t2343 * t9581 / F::new(384.0);
    t9584
}

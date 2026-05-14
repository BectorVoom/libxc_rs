//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 947/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk947<F: Float>(t3257: F, t9569: F, t1113: F, t6110: F, t905: F, t3237: F, t6627: F, t2345: F, t3219: F, t6220: F, t2277: F, t2343: F, t6545: F, t902: F, t9105: F, t9110: F, t9113: F, t9114: F, t9118: F, t9121: F, t9123: F) -> (F, F, F, F, F) {
    let t9570 = t3257 * t9569;
    let t9574 = t1113 * t6110;
    let t9575 = t905 * t9574;
    let t9579 = 7.0 / 1152.0 * t6627 * t3237;
    let t9581 = t2345 * t3219 * t6220;
    let t9584 = t9105 - t9110 - t9113 - t2277 * t9570 / 256.0 - t9114 + 7.0 / 2304.0 * t6545 - t9118 + t9121 + t9123 + t902 * t9575 / 1536.0 + t9579 + t2343 * t9581 / 384.0;
    (t9570, t9574, t9575, t9581, t9584)
}

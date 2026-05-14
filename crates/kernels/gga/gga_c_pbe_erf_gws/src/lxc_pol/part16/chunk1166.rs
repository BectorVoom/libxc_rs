//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1166/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1166<F: Float>(t54289: F, t51372: F, t54265: F, t54269: F, t54273: F, t54276: F, t54280: F, t54287: F, t55562: F, t55564: F, t55569: F, t55570: F, t54301: F, t54305: F, t51383: F, t51388: F, t51396: F, t51401: F, t54295: F, t54297: F, t54299: F, t54303: F, t54307: F, t54310: F) -> (F, F) {
    let t55572 = 7.0 / 72.0 * t54289;
    let t55573 = -t54265 / 48.0 + t55562 + t54269 / 24.0 - t55564 + t54273 / 96.0 + t54276 / 4.0 - t54280 / 32.0 - 7.0 / 72.0 * t51372 + t55569 - t55570 - t54287 / 384.0 - t55572;
    let t55580 = 7.0 / 288.0 * t54301;
    let t55582 = 119.0 / 1728.0 * t54305;
    let t55586 = -7.0 / 72.0 * t51383 - 119.0 / 864.0 * t51388 - 119.0 / 432.0 * t51396 + t54295 / 24.0 - t54297 / 12.0 + t54299 / 24.0 + t55580 + 5.0 / 96.0 * t54303 - t55582 - t54307 / 24.0 - 35.0 / 288.0 * t51401 + t54310 / 96.0;
    (t55573, t55586)
}

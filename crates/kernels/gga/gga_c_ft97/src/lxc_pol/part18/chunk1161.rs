//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1161/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1161<F: Float>(t363: F, t473: F, t100453: F, t2992: F, t93378: F, t5675: F, t8326: F, t2983: F, t100419: F, t100423: F, t100427: F, t100431: F, t100434: F, t100438: F, t100443: F, t100447: F, t100451: F, t93349: F) -> (F, F, F, F) {
    let t100454 = t473 * t363;
    let t100457 = t93378 * t100453 * t2992 * t100454;
    let t100459 = t8326 * t5675;
    let t100462 = t93378 * t100459 * t2983 * t100454;
    let t100464 = 8.0 / 9.0 * t100419 + 2.0 / 9.0 * t100423 + t93349 / 18.0 - 2.0 / 27.0 * t100427 + t100431 - 2.0 * t100434 + t100438 / 9.0 + 2.0 / 9.0 * t100443 + 8.0 * t100447 + t100451 / 9.0 + t100457 / 9.0 - t100462 / 27.0;
    (t100454, t100457, t100462, t100464)
}

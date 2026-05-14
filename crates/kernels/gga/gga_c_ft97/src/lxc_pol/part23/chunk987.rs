//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 987/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk987<F: Float>(t29369: F, t840: F, t871: F, t25246: F, t25248: F, t25252: F, t25284: F, t29340: F, t29342: F, t29346: F, t29350: F, t29354: F, t29356: F, t29359: F, t29363: F, t29366: F, t446: F) -> (F, F) {
    let t29371 = t840 * t871 * t29369;
    let t29374 = -2.0 / 9.0 * t25246 - t25248 / 9.0 + t25252 + t29340 / 9.0 - t446 * t29342 / 3.0 + t446 * t29346 / 3.0 + 2.0 / 3.0 * t446 * t29350 - t25284 / 27.0 + t29354 / 27.0 + 2.0 / 3.0 * t446 * t29356 + 2.0 / 3.0 * t446 * t29359 + t446 * t29363 / 3.0 + 2.0 / 3.0 * t446 * t29366 + t446 * t29371 / 3.0;
    (t29371, t29374)
}

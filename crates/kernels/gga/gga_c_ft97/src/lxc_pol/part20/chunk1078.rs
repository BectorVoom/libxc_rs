//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1078/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1078<F: Float>(t108395: F, t41816: F, t446: F, t108354: F, t108357: F, t108360: F, t108364: F, t108368: F, t108371: F, t108376: F, t108381: F, t108386: F, t108391: F, t108394: F, t14116: F, t24519: F) -> (F, F, F) {
    let t108397 = t446 * t41816 * t108395;
    let t108399 = -t108354 - t108357 + 2.0 * t108360 + 12.0 * t108364 - 4.0 / 9.0 * t108368 - 4.0 / 3.0 * t108371 + t108376 / 2.0 + t108381 / 4.0 + t108386 / 4.0 - 3.0 / 8.0 * t108391 + t108394 + 10.0 / 27.0 * t108397;
    let t108401 = t24519 * t14116;
    (t108397, t108399, t108401)
}

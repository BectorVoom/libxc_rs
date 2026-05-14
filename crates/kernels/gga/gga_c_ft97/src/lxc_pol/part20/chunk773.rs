//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 773/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk773<F: Float>(t2354: F, t24546: F, t684: F, t6118: F, t2413: F, t6119: F, t2373: F, t24181: F, t193: F, t89: F, t2405: F, t9744: F, t24500: F, t24505: F, t24510: F, t24514: F, t24517: F, t24522: F, t24524: F, t24529: F, t24534: F, t24538: F, t24541: F, t24544: F) -> (F, F, F, F, F, F, F, F, F) {
    let t24548 = t2354 * t24546 * t684;
    let t24549 = t6118 * t24548;
    let t24552 = t2354 * t6119 * t2413;
    let t24553 = t6118 * t24552;
    let t24555 = t24181 * t2373;
    let t24556 = t193 * t24555;
    let t24557 = t89 * t24556;
    let t24560 = t9744 * t6119 * t2405;
    let t24561 = t6118 * t24560;
    let t24563 = -4.0 / 3.0 * t24500 + t24505 / 4.0 - 3.0 * t24510 - t24514 / 2.0 + t24517 / 3.0 + 2.0 / 9.0 * t24522 - 2.0 / 9.0 * t24524 + 2.0 / 3.0 * t24529 - 2.0 / 3.0 * t24534 - t24538 + t24541 / 3.0 - t24544 / 9.0 + t24549 / 3.0 + t24553 / 6.0 - 6.0 * t24557 + t24561 / 9.0;
    (t24548, t24549, t24552, t24553, t24555, t24557, t24560, t24561, t24563)
}

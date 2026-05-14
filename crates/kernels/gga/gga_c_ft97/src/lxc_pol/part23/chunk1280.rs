//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1280/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1280<F: Float>(t124279: F, t124282: F, t124267: F, t124270: F, t124273: F, t124277: F, t124287: F, t124292: F, t124296: F, t124300: F, t124304: F, t124307: F, t109434: F, t109438: F, t109443: F, t109470: F, t109476: F, t124311: F, t124316: F, t124322: F, t124325: F, t97248: F, t97407: F, t97411: F) -> (F, F) {
    let t124638 = 2.0 / 9.0 * t124279;
    let t124639 = t124282 / 3.0;
    let t124646 = -2.0 / 3.0 * t124267 - 2.0 / 3.0 * t124270 + 4.0 / 3.0 * t124273 + t124277 / 6.0 + t124638 + t124639 + 15.0 / 16.0 * t124287 - 3.0 / 8.0 * t124292 + t124296 / 8.0 - t124300 / 6.0 - t124304 / 12.0 - 2.0 / 3.0 * t124307;
    let t124653 = -2.0 / 3.0 * t124311 + t97407 + t124316 / 3.0 - 8.0 / 9.0 * t109434 + t109438 - t97411 + t97248 + t109443 + t109470 - 4.0 / 9.0 * t109476 - 2.0 / 3.0 * t124322 + 2.0 / 9.0 * t124325;
    (t124646, t124653)
}

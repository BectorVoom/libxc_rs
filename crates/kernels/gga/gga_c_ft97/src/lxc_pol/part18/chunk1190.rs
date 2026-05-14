//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1190/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1190<F: Float>(t101573: F, t1317: F, t28: F, t469: F, t100466: F, t100469: F, t100472: F, t100475: F, t100478: F, t100480: F, t100482: F, t93414: F, t93421: F, t93424: F, t93449: F, t26007: F, t376: F, t5665: F) -> (F, F, F) {
    let t101576 = t1317 * t28 * t469 * t101573;
    let t101582 = -2.0 / 9.0 * t100466 - t100469 / 9.0 - 2.0 / 9.0 * t100472 + t100475 / 27.0 + t100478 + t100480 - t100482 - t101576 / 6.0 - t93414 / 27.0 - 2.0 / 27.0 * t93421 - 4.0 / 27.0 * t93424 - 2.0 / 81.0 * t93449;
    let t101587 = t5665 * t376 * t26007;
    (t101576, t101582, t101587)
}

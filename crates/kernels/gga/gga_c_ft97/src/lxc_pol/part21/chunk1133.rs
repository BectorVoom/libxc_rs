//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1133/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1133<F: Float>(t29482: F, t93014: F, t22632: F, t22761: F, t29506: F, t22515: F, t423: F, t4431: F, t53: F, t100763: F, t101150: F, t101243: F, t115362: F, t115658: F, t115664: F, t15797: F, t22513: F, t22767: F, t22834: F, t29483: F, t29528: F, t45237: F, t58181: F, t58185: F, t58191: F, t92414: F, t92441: F, t92773: F, t92776: F, t93016: F, t93099: F, t93103: F) -> (F, F) {
    let t115907 = t29482 * t93014;
    let t115922 = t22761 * t22632 * t29506;
    let t115939 = t22515 * t423 * t4431 * t53;
    let t115942 = 0.10357803499222879255e-4 * t115907 * t93016 - 0.12020514968855939808e-5 * t58181 * t101150 - 0.12020514968855939808e-5 * t58185 * t92773 + 0.60102574844279699039e-6 * t29483 * t92776 + 0.30030568862539529421e-7 * t58191 * t115362 + 0.30644932022222222221e0 * t22761 * t22767 * t29506 - 0.38306165027777777777e-1 * t115922 - 0.13519760450715832853e-3 * t15797 * t92414 + 0.10338048737805743098e-3 * t100763 * t92441 * t115658 + t101243 + 0.13519760450715832853e-3 * t15797 * t93099 + 0.13519760450715832853e-3 * t15797 * t93103 - 0.67552196935353456646e-5 * t45237 * t115664 - 0.23254900946437792e-1 * t22834 * t29528 - 0.15137014751914110597e-3 * t22513 * t115939;
    (t115939, t115942)
}

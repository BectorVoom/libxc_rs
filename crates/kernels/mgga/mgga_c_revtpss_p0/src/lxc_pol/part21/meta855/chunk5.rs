//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3240/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3240<F: Float>(t58322: F, t58325: F, t58327: F, t58330: F, t58333: F, t58658: F, t58660: F, t58662: F, t58664: F, t58669: F, t58671: F, t58341: F, t58344: F, t58462: F, t58464: F, t58468: F, t58472: F, t58475: F, t58675: F, t58678: F, t58683: F, t58685: F) -> (F, F) {
    let t60142 = t58658 - t58322 + t58325 + t58660 + t58327 + t58330 + t58333 - t58662 - t58664 + t58669 - t58671;
    let t60143 = -t58341 - t58344 - t58675 - t58678 - t58683 + t58462 + t58464 + t58468 - t58685 + t58472 + t58475;
    (t60142, t60143)
}

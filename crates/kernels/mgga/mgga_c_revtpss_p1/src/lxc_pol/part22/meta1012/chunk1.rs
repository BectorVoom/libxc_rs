//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3477/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3477<F: Float>(t65388: F, t65389: F, t65391: F, t65392: F, t65395: F, t65396: F, t65398: F, t65422: F, t19658: F, t3169: F, t13312: F, t1469: F) -> (F, F, F) {
    let t65425 = t65388 + t65389 + t65391 + t65392 + t65395 + t65396 + t65398 + t65422;
    let t65431 = t3169 * t19658;
    let t65433 = t1469 * t13312;
    (t65425, t65431, t65433)
}

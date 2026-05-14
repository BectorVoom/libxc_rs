//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1222/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1222<F: Float>(t101252: F, t104203: F, t104208: F, t108966: F, t108990: F, t111639: F, t111665: F, t111670: F, t114260: F, t114296: F, t2123: F, t26792: F, t28154: F, t29380: F, t29388: F, t29412: F, t29548: F, t29562: F) -> (F,) {
    let t116821 = -15.0 * t104208 * t29562 - 15.0 * t104203 * t29562 - 15.0 * t26792 * t114260 + 5.0 / 2.0 * t29388 * t29548 + t114296 * t2123 + 5.0 / 2.0 * t29412 * t29548 + 30.0 * t101252 * t111639 - 10.0 * t108966 * t29380 - 5.0 * t108990 * t29380 - 10.0 * t28154 * t111665 - 10.0 * t28154 * t111670;
    (t116821,)
}

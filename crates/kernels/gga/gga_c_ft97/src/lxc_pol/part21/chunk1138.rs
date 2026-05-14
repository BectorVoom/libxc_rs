//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1138/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1138<F: Float>(t79: F, t115369: F, t115423: F, t115477: F, t115525: F, t115576: F, t115636: F, t115683: F, t115729: F, t115773: F, t115810: F, t115855: F, t115904: F, t115942: F, t115990: F, t116036: F, t116077: F) -> (F,) {
    let t80 = 0.1e-59 < t79;
    let t116082 = piecewise3(t80, t115369 + t115423 + t115477 + t115525 + t115576 + t115636 + t115683 + t115729 + t115773 + t115810 + t115855 + t115904 + t115942 + t115990 + t116036 + t116077, 0.0);
    (t116082,)
}

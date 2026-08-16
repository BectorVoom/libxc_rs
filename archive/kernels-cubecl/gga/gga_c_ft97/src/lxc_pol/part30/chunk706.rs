//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 706/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk706<F: Float>(t2874: F, t29260: F, t28842: F, t295: F, t312: F, t28852: F, t296: F, t684: F, t7131: F, t835: F, t4176: F, t6353: F, t840: F) -> (F, F, F, F, F) {
    let t29261 = t2874 * t29260;
    let t29265 = t295 * t28842 * t312;
    let t29270 = t296 * t28852;
    let t29274 = t835 * t7131 * t684;
    let t29278 = t840 * t6353 * t4176;
    (t29261, t29265, t29270, t29274, t29278)
}

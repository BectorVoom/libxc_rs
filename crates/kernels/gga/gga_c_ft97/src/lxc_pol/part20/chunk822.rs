//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 822/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk822<F: Float>(t25165: F, t2665: F, t684: F, t6317: F, t2413: F, t6318: F, t10409: F, t2405: F, t824: F, t856: F) -> (F, F, F, F, F, F, F) {
    let t25167 = t2665 * t25165 * t684;
    let t25168 = t6317 * t25167;
    let t25171 = t2665 * t6318 * t2413;
    let t25172 = t6317 * t25171;
    let t25175 = t10409 * t6318 * t2405;
    let t25176 = t6317 * t25175;
    let t25178 = t856 * t824;
    (t25167, t25168, t25171, t25172, t25175, t25176, t25178)
}

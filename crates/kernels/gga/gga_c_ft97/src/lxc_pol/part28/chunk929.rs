//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 929/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk929<F: Float>(t1378: F, t1985: F, t23997: F, t582: F, t2097: F, t5935: F, t53: F, t925: F, t3066: F, t1851: F, t6454: F, t2178: F, t6615: F) -> (F, F, F, F, F, F, F) {
    let t107082 = t1985 * t1378;
    let t107284 = t582 * t23997;
    let t107627 = t2097 * t5935;
    let t115418 = t925 * t53;
    let t115567 = t925 * t3066;
    let t117775 = t1851 * t6454;
    let t120449 = t2178 * t6615;
    (t107082, t107284, t107627, t115418, t115567, t117775, t120449)
}

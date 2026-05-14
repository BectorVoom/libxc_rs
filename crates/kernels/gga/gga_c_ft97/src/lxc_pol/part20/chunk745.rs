//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 745/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk745<F: Float>(t13522: F, t232: F, t230: F, t2393: F, t2395: F, t1614: F, t209: F, t9: F) -> (F, F, F, F) {
    let t24266 = t232 * t13522;
    let t24269 = t230 * t2393;
    let t24270 = t24269 * t2395;
    let t24274 = t1614 * t209;
    let t24275 = t9 * t24274;
    (t24266, t24269, t24270, t24275)
}

//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 773/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk773<F: Float>(t76: F, t8050: F, t66: F, t378: F, t7241: F, t1586: F, t1642: F, t422: F, t626: F, t7763: F, t81: F, t342: F, t344: F, t8639: F, t7800: F, t37292: F) -> (F, F, F, F, F, F, F, F) {
    let t38241 = 1.0 / t8050 / t76;
    let t38242 = t66 * t38241;
    let t38262 = t378 * t7241;
    let t38268 = t1642 * t1586;
    let t38308 = t626 * t422;
    let t38327 = t81 * t7763;
    let t38355 = 5.0 / 54.0 * t342 * t8639 * t344;
    let t38357 = t81 * t7800;
    let t38392 = 280.0 / 81.0 * t37292;
    (t38242, t38262, t38268, t38308, t38327, t38355, t38357, t38392)
}

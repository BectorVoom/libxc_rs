//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 998/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk998<F: Float>(t605: F, t9114: F, t1948: F, t2252: F, t342: F, t511: F, t8639: F, t1526: F, t1944: F, t38308: F, t1614: F, t373: F, t3018: F, t534: F, t62: F, t1594: F) -> (F, F, F, F, F, F, F) {
    let t41269 = t9114 * t605;
    let t41305 = t342 * t2252 * t1948;
    let t41328 = 5.0 / 54.0 * t342 * t8639 * t511;
    let t41332 = t1526 * t38308 * t1944;
    let t44991 = t1614 * t373;
    let t45237 = t534 * t3018;
    let t45572 = t3018 * t62;
    let t45573 = t1594 * t45572;
    (t41269, t41305, t41328, t41332, t44991, t45237, t45573)
}

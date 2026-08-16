//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 817/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk817<F: Float>(t157: F, t526: F, t2178: F, t358: F, t167: F, t2101: F, t9114: F, t2179: F, t582: F, t184: F, t363: F, t2: F, t9952: F) -> (F, F, F, F, F, F, F) {
    let t13140 = t526 * t157;
    let t13165 = t2178 * t358;
    let t13208 = t2101 * t167;
    let t13212 = t9114 * t167;
    let t13220 = t582 * t2179;
    let t13255 = t184 * t363;
    let t13313 = t9952 * t2;
    (t13140, t13165, t13208, t13212, t13220, t13255, t13313)
}

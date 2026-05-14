//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 744/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk744<F: Float>(t2: F, t9224: F, t157: F, t1985: F, t2097: F, t597: F, t526: F, t2178: F, t358: F, t167: F, t2101: F, t9114: F, t2179: F, t582: F, t184: F, t363: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12823 = t9224 * t2;
    let t12968 = t1985 * t157;
    let t12982 = t2097 * t597;
    let t13140 = t526 * t157;
    let t13165 = t2178 * t358;
    let t13208 = t2101 * t167;
    let t13212 = t9114 * t167;
    let t13220 = t582 * t2179;
    let t13255 = t184 * t363;
    (t12823, t12968, t12982, t13140, t13165, t13208, t13212, t13220, t13255)
}

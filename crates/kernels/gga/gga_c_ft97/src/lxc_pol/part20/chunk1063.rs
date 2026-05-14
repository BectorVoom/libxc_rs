//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1063/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1063<F: Float>(t108142: F, t2349: F, t2354: F, t446: F, t2373: F, t6837: F, t1434: F, t193: F, t9942: F, t13672: F, t1424: F, t2506: F, t27856: F, t6109: F, t681: F, t27851: F) -> (F, F, F, F, F, F, F, F) {
    let t108145 = t446 * t2354 * t108142 * t2349;
    let t108147 = t6837 * t2373;
    let t108150 = t1434 * t193 * t9942 * t108147;
    let t108152 = t1424 * t13672;
    let t108155 = t1434 * t193 * t2506 * t108152;
    let t108157 = t6109 * t681 * t27856;
    let t108158 = t108157 / 6.0;
    let t108160 = t6109 * t681 * t27851;
    (t108145, t108147, t108150, t108152, t108155, t108157, t108158, t108160)
}

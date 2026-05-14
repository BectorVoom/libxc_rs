//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1074/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1074<F: Float>(t107937: F, t193: F, t2373: F, t89: F, t24191: F, t3821: F, t1425: F, t3704: F, t668: F, t27883: F, t681: F, t2371: F, t27742: F, t713: F, t13672: F, t6008: F) -> (F, F, F, F, F, F, F, F) {
    let t108322 = t89 * t193 * t107937 * t2373;
    let t108326 = t89 * t193 * t24191 * t3821;
    let t108330 = t89 * t3704 * t1425 * t668;
    let t108333 = t89 * t681 * t27883;
    let t108334 = 4.0 / 3.0 * t108333;
    let t108335 = t2371 * t27742;
    let t108338 = t89 * t193 * t108335 * t713;
    let t108342 = t89 * t193 * t6008 * t13672;
    (t108322, t108326, t108330, t108333, t108334, t108335, t108338, t108342)
}

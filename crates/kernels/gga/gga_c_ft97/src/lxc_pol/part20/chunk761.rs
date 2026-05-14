//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 761/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk761<F: Float>(t24412: F, t2569: F, t1449: F, t2526: F, t2568: F, t6187: F, t766: F, t10002: F, t6175: F, t713: F, t771: F, t6008: F, t193: F, t6148: F, t761: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t24413 = t24412 * t2569;
    let t24415 = t1449 * t2526;
    let t24416 = t2568 * t24415;
    let t24418 = t6187 * t766;
    let t24419 = t2568 * t24418;
    let t24421 = t10002 * t6175;
    let t24423 = t771 * t713;
    let t24424 = t6008 * t24423;
    let t24425 = t193 * t24424;
    let t24429 = t6148 * t761;
    (t24413, t24415, t24416, t24418, t24419, t24421, t24423, t24424, t24425, t24429)
}

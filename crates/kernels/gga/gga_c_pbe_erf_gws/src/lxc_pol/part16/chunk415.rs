//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 415/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk415<F: Float>(t1407: F, t476: F, t53: F, t1413: F, t1416: F, t478: F, t1524: F) -> (F, F, F, F, F) {
    let t1526 = t476 * t1407;
    let t1528 = 1.0 / t53;
    let t1529 = t1528 * t1413;
    let t1531 = t478 * t1416;
    let t1533 = -t1524 / 9.0 + t1526 / 3.0 - t1529 / 9.0 + t1531 / 3.0;
    (t1526, t1528, t1529, t1531, t1533)
}

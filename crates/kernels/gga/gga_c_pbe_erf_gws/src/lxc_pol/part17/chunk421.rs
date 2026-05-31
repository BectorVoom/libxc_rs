//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 421/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk421<F: Float>(t48: F, t1403: F, t1407: F, t476: F, t53: F, t1413: F, t1416: F, t478: F) -> (F, F, F, F, F, F, F) {
    let t1523 = F::cast_from(1.0_f64) / t48;
    let t1524 = t1523 * t1403;
    let t1526 = t476 * t1407;
    let t1528 = F::cast_from(1.0_f64) / t53;
    let t1529 = t1528 * t1413;
    let t1531 = t478 * t1416;
    let t1533 = -t1524 / F::cast_from(9.0_f64) + t1526 / F::cast_from(3.0_f64) - t1529 / F::cast_from(9.0_f64) + t1531 / F::cast_from(3.0_f64);
    (t1523, t1524, t1526, t1528, t1529, t1531, t1533)
}

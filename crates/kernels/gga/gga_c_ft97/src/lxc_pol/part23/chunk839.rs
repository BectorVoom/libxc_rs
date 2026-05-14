//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 839/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk839<F: Float>(t1486: F, t6327: F, t681: F, t1491: F, t1636: F, t89: F, t1485: F, t458: F) -> (F, F, F, F) {
    let t25146 = t1486 * t681 * t6327;
    let t25153 = t89 * t1636 * t1491;
    let t25154 = 4.0 / 9.0 * t25153;
    let t25162 = t1485 * t458;
    (t25146, t25153, t25154, t25162)
}

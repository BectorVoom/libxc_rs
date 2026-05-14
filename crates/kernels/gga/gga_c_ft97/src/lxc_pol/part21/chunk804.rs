//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 804/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk804<F: Float>(t1369: F, t1370: F, t1637: F, t358: F, t5842: F, t1359: F, t1557: F) -> (F, F, F, F) {
    let t23898 = t1369 * t1637 * t1370;
    let t23899 = 2.0 / 9.0 * t23898;
    let t23900 = t5842 * t358;
    let t23909 = t1359 * t1557;
    (t23898, t23899, t23900, t23909)
}

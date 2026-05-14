//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 210/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk210<F: Float>(t1286: F, t1310: F, t1333: F, t1337: F, t1339: F, t88: F, t2: F, t515: F, t4: F) -> (F, F, F) {
    let t1342 = t1286 * t1310 / 6.0 - t88 * t1337 + 2.0 * t1339 - 2.0 * t1333;
    let t1347 = t515 * t2;
    let t1348 = t1347 * t4;
    (t1342, t1347, t1348)
}

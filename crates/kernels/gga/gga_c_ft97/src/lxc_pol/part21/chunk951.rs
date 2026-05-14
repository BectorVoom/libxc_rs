//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 951/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk951<F: Float>(t23327: F, t4607: F, t11902: F, t6465: F, t26356: F, t925: F, t1902: F, t4462: F, t5630: F, t4454: F, t8518: F, t23339: F, t4572: F, t11810: F, t29790: F, t83: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t29904 = t23327 * t4607;
    let t29907 = t11902 * t6465;
    let t29910 = t26356 * t925;
    let t29911 = t1902 * t29910;
    let t29914 = t5630 * t4462;
    let t29915 = t1902 * t29914;
    let t29918 = t5630 * t4454;
    let t29919 = t8518 * t29918;
    let t29922 = t23339 * t4572;
    let t29923 = t11810 * t29922;
    let t29926 = t83 * t29790;
    (t29904, t29907, t29910, t29911, t29914, t29915, t29918, t29919, t29922, t29923, t29926)
}

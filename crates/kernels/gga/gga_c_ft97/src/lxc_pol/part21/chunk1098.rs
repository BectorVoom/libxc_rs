//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1098/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1098<F: Float>(t105846: F, t27154: F, t95053: F, t1369: F, t1637: F, t6669: F, t1882: F, t27040: F, t12001: F, t27044: F, t27181: F, t376: F, t89: F, t5900: F, t9114: F, t40465: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t105847 = t105846 / 4.0;
    let t105848 = t95053 * t27154;
    let t105849 = t105848 / 3.0;
    let t105856 = t1369 * t1637 * t6669;
    let t105862 = t1882 * t27040;
    let t105863 = 4.0 * t105862;
    let t105884 = t12001 * t27044;
    let t105894 = t89 * t376 * t27181;
    let t105895 = 4.0 / 3.0 * t105894;
    let t105900 = t9114 * t5900;
    let t105905 = t40465 * t5900;
    (t105847, t105848, t105849, t105856, t105862, t105863, t105884, t105894, t105895, t105900, t105905)
}

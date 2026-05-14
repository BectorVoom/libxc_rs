//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1107/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1107<F: Float>(t1882: F, t27400: F, t26830: F, t27307: F, t26902: F, t6653: F, t8232: F, t1378: F, t1985: F, t26925: F, t8392: F, t26929: F, t27229: F, t26940: F, t358: F, t6718: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t107041 = 4.0 / 9.0 * t1882 * t27400;
    let t107043 = 4.0 / 9.0 * t1882 * t26830;
    let t107059 = 2.0 / 9.0 * t1882 * t27307;
    let t107068 = 4.0 / 9.0 * t1882 * t26902;
    let t107077 = t8232 * t6653;
    let t107082 = t1985 * t1378;
    let t107111 = 4.0 / 9.0 * t8392 * t26925;
    let t107113 = 4.0 / 9.0 * t8392 * t26929;
    let t107115 = 2.0 / 27.0 * t8392 * t27229;
    let t107117 = 2.0 / 9.0 * t1882 * t26940;
    let t107141 = t6718 * t358;
    (t107041, t107043, t107059, t107068, t107077, t107082, t107111, t107113, t107115, t107117, t107141)
}

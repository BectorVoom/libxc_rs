//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 963/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk963<F: Float>(t2155: F, t30005: F, t1222: F, t7973: F, t309: F, t945: F, t7963: F, t9033: F, t1221: F, t2138: F, t2139: F, t8004: F) -> (F, F, F, F) {
    let t31926 = t30005 * t2155;
    let t31928 = t7973 * t1222;
    let t31935 = t945 * t309;
    let t31937 = t7963 * t9033 * t31935;
    let t31944 = t2138 * t8004 * t2139 * t1221;
    (t31926, t31928, t31937, t31944)
}

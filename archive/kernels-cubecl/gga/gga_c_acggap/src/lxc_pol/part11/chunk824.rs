//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 824/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk824<F: Float>(t336: F, t4630: F, t570: F, t2020: F, t2260: F, t599: F, t8791: F, t1181: F, t7413: F, t1165: F, t604: F, t8406: F) -> (F, F, F, F, F, F, F) {
    let t8942 = t336 * t4630;
    let t8943 = t570 * t8942;
    let t8945 = t2020 * t2260;
    let t8947 = t599 * t8791;
    let t8948 = t1181 * t8947;
    let t8949 = t7413 * t8948;
    let t8952 = t1165 * t604 * t8406;
    (t8942, t8943, t8945, t8947, t8948, t8949, t8952)
}

//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1176/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1176<F: Float>(t1181: F, t30282: F, t38843: F, t599: F, t1814: F, t2937: F, t406: F, t1165: F, t30856: F, t604: F, t31362: F, t9597: F) -> (F, F, F, F) {
    let t40212 = t30282 * t1181 * t599 * t38843;
    let t40215 = t1814 * t2937 * t406;
    let t40218 = t30856 * t1165 * t604 * t40215;
    let t40220 = t31362 * t9597;
    (t40212, t40215, t40218, t40220)
}

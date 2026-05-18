//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 919/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk919<F: Float>(t31038: F, t409: F, t7712: F, t957: F, t1181: F, t30806: F, t3491: F, t599: F, t1983: F, t30127: F, t7586: F, t945: F) -> (F, F, F, F) {
    let t31039 = t31038 * t409;
    let t31041 = t7712 * t957;
    let t31045 = t30806 * t1181 * t599 * t3491;
    let t31049 = t30127 * t7586 * t1983 * t945;
    (t31039, t31041, t31045, t31049)
}

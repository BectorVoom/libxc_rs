//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1203/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1203<F: Float>(t30780: F, t38956: F, t336: F, t5674: F, t578: F, t599: F, t1773: F, t2060: F, t2061: F, t6388: F, t7450: F, t7815: F) -> (F, F, F, F) {
    let t40569 = t30780 * t38956;
    let t40573 = t578 * t336 * t599 * t5674;
    let t40576 = t2060 * t1773 * t2061;
    let t40579 = t7450 * t7815 * t6388;
    (t40569, t40573, t40576, t40579)
}

//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1076/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1076<F: Float>(t142: F, t5187: F, t8888: F, t507: F, t7436: F, t961: F, t1165: F, t20138: F, t604: F, t7413: F, t1992: F, t30127: F, t7842: F, t8791: F) -> (F, F, F, F) {
    let t35154 = t8888 * t142 * t5187;
    let t35157 = t7436 * t507 * t961;
    let t35172 = t7413 * t1165 * t604 * t20138;
    let t35176 = t30127 * t7842 * t1992 * t8791;
    (t35154, t35157, t35172, t35176)
}

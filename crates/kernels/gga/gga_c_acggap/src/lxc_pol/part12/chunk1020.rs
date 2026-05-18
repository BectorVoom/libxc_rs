//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1020/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1020<F: Float>(t1181: F, t4822: F, t599: F, t8463: F, t1988: F, t8541: F, t30811: F, t4908: F, t4680: F, t7493: F, t8648: F, t1421: F, t1992: F, t30827: F, t7842: F) -> (F, F, F, F, F) {
    let t34166 = t8463 * t1181 * t599 * t4822;
    let t34170 = t1988 * t8541;
    let t34172 = t30811 * t4908;
    let t34175 = t7493 * t4680 * t8648;
    let t34179 = t30827 * t7842 * t1992 * t1421;
    (t34166, t34170, t34172, t34175, t34179)
}

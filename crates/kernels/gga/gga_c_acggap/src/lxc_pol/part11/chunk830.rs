//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 830/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk830<F: Float>(t30937: F, t7566: F, t1181: F, t3730: F, t604: F, t7426: F, t1170: F, t1171: F, t30538: F, t1177: F, t3529: F, t4680: F, t7569: F, t1983: F, t7585: F, t7586: F, t930: F) -> (F, F, F, F, F, F, F) {
    let t30938 = t30937 * t7566;
    let t30945 = t7426 * t1181 * t604 * t3730;
    let t30948 = t1170 * t30538 * t1171;
    let t30949 = t30948 * t1177;
    let t30956 = t7426 * t1181 * t604 * t3529;
    let t30963 = t7426 * t4680 * t7569;
    let t30967 = t7585 * t7586 * t1983 * t930;
    (t30938, t30945, t30948, t30949, t30956, t30963, t30967)
}

//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 795/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk795<F: Float>(t2394: F, t463: F, t8004: F, t2147: F, t322: F, t2138: F, t309: F, t2131: F, t8306: F, t9025: F, t8440: F, t9029: F) -> (F, F, F, F, F, F, F, F) {
    let t9150 = t8004 * t2394 * t463;
    let t9154 = t2147 * t2394 * t322;
    let t9155 = t2138 * t9154;
    let t9159 = t2147 * t2394 * t309;
    let t9160 = t2131 * t9159;
    let t9162 = t8306 * t9025;
    let t9165 = t8306 * t8440;
    let t9168 = t8306 * t9029;
    (t9150, t9154, t9155, t9159, t9160, t9162, t9165, t9168)
}

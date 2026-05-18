//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 872/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk872<F: Float>(t1181: F, t30209: F, t3650: F, t604: F, t1170: F, t2066: F, t592: F, t7634: F, t7844: F, t10098: F, t7336: F) -> (F, F, F, F) {
    let t30212 = t30209 * t1181 * t604 * t3650;
    let t30216 = t1170 * t592 * t7634 * t2066;
    let t30217 = t30216 * t7844;
    let t30219 = t10098 * t7336;
    (t30212, t30216, t30217, t30219)
}

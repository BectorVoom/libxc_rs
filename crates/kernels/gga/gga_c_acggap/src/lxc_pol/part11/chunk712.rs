//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 712/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk712<F: Float>(t1096: F, t604: F, t1181: F, t7575: F, t2069: F, t4680: F, t2068: F, t1977: F, t592: F, t2066: F) -> (F, F, F, F, F, F, F) {
    let t7576 = t604 * t1096;
    let t7577 = t1181 * t7576;
    let t7578 = t7575 * t7577;
    let t7580 = t4680 * t2069;
    let t7581 = t2068 * t7580;
    let t7583 = t592 * t1977;
    let t7584 = t7583 * t2066;
    (t7576, t7577, t7578, t7580, t7581, t7583, t7584)
}

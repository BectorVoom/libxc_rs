//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 862/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk862<F: Float>(t13364: F, t31115: F, t33938: F, t1: F, t1170: F, t2065: F, t8461: F, t1530: F, t31114: F, t137: F, t524: F) -> (F, F, F, F) {
    let t33940 = t31115 * t13364 * t33938;
    let t33944 = t1170 * t2065 * t8461 * t1;
    let t33952 = t1530 * t31114;
    let t33953 = t137 * t524;
    (t33940, t33944, t33952, t33953)
}

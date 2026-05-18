//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 909/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk909<F: Float>(t1165: F, t3290: F, t7426: F, t8600: F, t3360: F, t8462: F, t3491: F, t604: F, t7560: F) -> (F, F, F, F) {
    let t30804 = t7426 * t1165 * t8600 * t3290;
    let t30806 = t3360 * t8462;
    let t30809 = t30806 * t1165 * t604 * t3491;
    let t30811 = t3360 * t7560;
    (t30804, t30806, t30809, t30811)
}

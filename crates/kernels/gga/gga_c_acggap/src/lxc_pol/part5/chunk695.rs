//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 695/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk695<F: Float>(t1658: F, t322: F, t449: F, t316: F, t3892: F, t557: F, t181: F, t315: F) -> (F, F, F, F, F) {
    let t5378 = t1658 * t322;
    let t5379 = t449 * t5378;
    let t5381 = 0.13170898365871023197e1 * t316 * t5379;
    let t5382 = t3892 * t557;
    let t5384 = t315 * t181;
    (t5378, t5379, t5381, t5382, t5384)
}

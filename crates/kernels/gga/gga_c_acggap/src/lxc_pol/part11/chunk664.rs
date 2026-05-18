//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 664/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk664<F: Float>(t316: F, t5368: F, t545: F, t862: F, t865: F, t150: F, t187: F, t5299: F, t1658: F, t322: F, t449: F, t3892: F, t557: F) -> (F, F, F, F, F, F) {
    let t5369 = t316 * t5368;
    let t5371 = t862 * t545;
    let t5372 = t5371 * t865;
    let t5375 = t5299 * t150 * t187;
    let t5378 = t1658 * t322;
    let t5379 = t449 * t5378;
    let t5381 = F::new(0.13170898365871023197e1) * t316 * t5379;
    let t5382 = t3892 * t557;
    (t5369, t5372, t5375, t5379, t5381, t5382)
}

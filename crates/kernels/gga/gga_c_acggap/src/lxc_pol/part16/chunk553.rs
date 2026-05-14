//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 553/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk553<F: Float>(t323: F, t5360: F, t1614: F, t868: F, t1308: F, t880: F, t449: F, t556: F, t879: F, t316: F, t545: F, t862: F, t865: F, t1658: F, t322: F, t3892: F, t557: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5361 = t5360 * t323;
    let t5364 = 0.13170898365871023197e1 * t868 * t1614;
    let t5365 = t1308 * t880;
    let t5368 = t449 * t556 * t879;
    let t5369 = t316 * t5368;
    let t5371 = t862 * t545;
    let t5372 = t5371 * t865;
    let t5378 = t1658 * t322;
    let t5379 = t449 * t5378;
    let t5381 = 0.13170898365871023197e1 * t316 * t5379;
    let t5382 = t3892 * t557;
    (t5361, t5364, t5365, t5368, t5369, t5372, t5379, t5381, t5382)
}

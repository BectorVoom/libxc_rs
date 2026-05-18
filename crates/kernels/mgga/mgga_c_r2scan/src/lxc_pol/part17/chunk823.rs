//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 823/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk823<F: Float>(t4873: F, t5039: F, t7097: F, t7126: F, t7156: F, t8646: F, t8647: F, t8649: F, t8651: F, t8652: F, t8653: F, t8654: F, t8655: F, t8656: F, t8657: F, t8658: F) -> F {
    let t8659 = -t7097 + t8646 - t8647 + t8649 - t8651 + t7126 + t8652 - t8653 - t8654 - t8655 + t4873 - t7156 - t8656 - t8657 + t8658 + t5039;
    t8659
}

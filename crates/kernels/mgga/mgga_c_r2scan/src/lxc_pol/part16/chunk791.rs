//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 791/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk791<F: Float>(t2483: F, t955: F, t2461: F, t898: F, t6030: F, t7126: F, t765: F, t7898: F, t7904: F, t8649: F, t8651: F, t8652: F, t9063: F, t9066: F, t4873: F, t5039: F, t6039: F, t6047: F, t7156: F, t8653: F, t8654: F, t8655: F, t8656: F, t8657: F, t8658: F) -> (F, F, F, F) {
    let t9069 = t2483 * t955;
    let t9072 = t898 * t2461;
    let t9075 = -0.1143056e0 * t7898 + 0.1350520664e0 * t6030 - t8649 + t8651 + 0.675260332e-1 * t765 * t9063 + 0.675260332e-1 * t765 * t9066 + 0.1350520664e0 * t765 * t9069 + 0.1350520664e0 * t765 * t9072 - t7126 - t8652 - t7904;
    let t9077 = t8653 + t8654 + t8655 - t4873 + 0.285764e-1 * t6039 + t6047 + t7156 + t8656 + t8657 - t8658 - t5039;
    (t9069, t9072, t9075, t9077)
}

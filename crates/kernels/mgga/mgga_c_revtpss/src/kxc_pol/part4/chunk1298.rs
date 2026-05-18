//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1298/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1298<F: Float>(t15625: F, t15676: F, t15722: F, t15755: F, t15779: F, t15814: F, t15855: F, t15913: F, t15949: F, t15991: F, t16034: F, t16073: F, t16114: F, t16136: F, t16189: F, t16233: F) -> F {
    let t16237 = t15625 + t15676 + t15722 + t15755 + t15779 + t15814 + t15855 + t15913 + t15949 + t15991 + t16034 + t16073 + t16114 + t16136 + t16189 + t16233;
    t16237
}

//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 730/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk730<F: Float>(t8155: F, t8168: F, t2233: F, t658: F, t2192: F, t694: F, t7795: F, t7797: F, t7799: F, t7801: F, t7805: F, t7809: F, t7811: F, t7814: F, t7817: F, t7834: F, t7838: F, t7842: F, t7846: F) -> (F, F, F, F) {
    let t8169 = t8155 + t8168;
    let t8171 = t2233 * t658;
    let t8176 = t2192 * t694;
    let t8192 = -8.0 * t7795 + 8.0 * t7797 + 8.0 * t7799 - 0.337177226155986 * t7801 - 0.505765839233979 * t7805 - 0.505765839233979 * t7809 - 0.505765839233979 * t7811 - 0.505765839233979 * t7814 - 0.505765839233979 * t7817 - 0.505765839233979 * t7834 - 6.0 * t7838 + 6.0 * t7842 + 6.0 * t7846;
    (t8169, t8171, t8176, t8192)
}

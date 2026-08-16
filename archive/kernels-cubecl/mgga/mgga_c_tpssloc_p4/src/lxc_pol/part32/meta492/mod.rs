//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta492 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1801;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1802;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta492<F: Float>(t25306: F, t6637: F, t6552: F, t23168: F, t7521: F, t4119: F, t6638: F, t22893: F, t7520: F, t23164: F, t1519: F, t234: F, t776: F, t1894: F, t4265: F, t214: F, t1880: F, t1909: F, t226: F, t23187: F, t25277: F, t25281: F, t25285: F, t25289: F, t25293: F, t25295: F, t25297: F, t25301: F, t25304: F, t4162: F, t4166: F, t4281: F, t6658: F, t7535: F, t808: F, t812: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t25307, t25308, t25310, t25312, t25313, t25314, t25316, t25317, t25319) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1801::<F>(t25306, t6637, t6552, t23168, t7521, t4119, t6638, t22893, t7520, t23164, t1519, t234);
        let (t25320, t25321, t25324, t25325, t25328) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1802::<F>(t25319, t776, t6637, t6552, t1894, t4265, t214, t1880, t1909, t226, t23187, t25277, t25281, t25285, t25289, t25293, t25295, t25297, t25301, t25304, t25308, t25310, t25314, t25317, t4162, t4166, t4281, t6658, t7535, t808, t812);
    (t25307, t25310, t25312, t25313, t25316, t25317, t25319, t25320, t25321, t25324, t25325, t25328)
}

//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta507 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1902;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1903;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta507(t25306: f64, t6637: f64, t6552: f64, t23168: f64, t7521: f64, t4119: f64, t6638: f64, t22893: f64, t7520: f64, t23164: f64, t1519: f64, t234: f64, t776: f64, t1894: f64, t4265: f64, t214: f64, t1880: f64, t1909: f64, t226: f64, t23187: f64, t25277: f64, t25281: f64, t25285: f64, t25289: f64, t25293: f64, t25295: f64, t25297: f64, t25301: f64, t25304: f64, t4162: f64, t4166: f64, t4281: f64, t6658: f64, t7535: f64, t808: f64, t812: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25307, t25308, t25310, t25312, t25313, t25314, t25316, t25317, t25319) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1902(t25306, t6637, t6552, t23168, t7521, t4119, t6638, t22893, t7520, t23164, t1519, t234);
        let (t25320, t25321, t25324, t25325, t25328) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1903(t25319, t776, t6637, t6552, t1894, t4265, t214, t1880, t1909, t226, t23187, t25277, t25281, t25285, t25289, t25293, t25295, t25297, t25301, t25304, t25308, t25310, t25314, t25317, t4162, t4166, t4281, t6658, t7535, t808, t812);
    (t25307, t25312, t25313, t25316, t25319, t25320, t25321, t25324, t25325, t25328)
}

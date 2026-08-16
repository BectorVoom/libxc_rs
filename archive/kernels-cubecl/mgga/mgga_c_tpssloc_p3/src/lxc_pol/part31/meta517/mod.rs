//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta517 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1715;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1716;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1717;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1718;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1719;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta517<F: Float>(t28951: F, t510: F, t2035: F, t5456: F, t28834: F, t7170: F, t2057: F, t28241: F, t1510: F, t26661: F, t24255: F, t5585: F, t24246: F, t24250: F, t25246: F, t25259: F, t28323: F, t28331: F, t28335: F, t28339: F, t28343: F, t28347: F, t4166: F, t7837: F, t812: F, t5612: F, t7101: F, t24218: F, t24220: F, t24221: F, t25065: F, t25077: F, t25080: F, t28357: F, t28360: F, t28362: F, t28364: F, t28366: F, t28368: F, t28370: F, t28373: F, t28376: F, t24230: F, t24231: F, t25109: F, t25126: F, t25133: F, t25140: F, t25144: F, t28380: F, t28384: F, t28386: F, t28390: F, t28397: F, t28399: F, t28401: F, t28403: F, t235: F, t5617: F, t1499: F, t2051: F, t226: F, t24265: F, t25277: F, t25293: F, t25310: F, t25317: F, t28420: F, t28424: F, t28428: F, t5575: F, t7839: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t28952, t28959, t28969, t28972, t28997, t29000) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1715::<F>(t28951, t510, t2035, t5456, t28834, t7170, t2057, t28241, t1510, t26661, t24255, t5585);
        let t29009 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1716::<F>(t24246, t24250, t25246, t25259, t28323, t28331, t28335, t28339, t28343, t28347, t28997, t29000, t4166, t7837, t812);
        let (t29010, t29025) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1717::<F>(t5612, t7101, t24218, t24220, t24221, t25065, t25077, t25080, t28357, t28360, t28362, t28364, t28366, t28368, t28370, t28373, t28376);
        let t29039 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1718::<F>(t24230, t24231, t25109, t25126, t25133, t25140, t25144, t28380, t28384, t28386, t28390, t28397, t28399, t28401, t28403);
        let (t29040, t29041, t29052, t29054) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1719::<F>(t29025, t29039, t235, t5617, t7101, t1499, t2051, t226, t24265, t25277, t25293, t25310, t25317, t28420, t28424, t28428, t29010, t5575, t7839, t812);
    (t28952, t28959, t28969, t28972, t28997, t29000, t29009, t29010, t29040, t29041, t29052, t29054)
}

//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta517 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1715;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1716;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1717;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1718;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1719;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta517(t28951: f64, t510: f64, t2035: f64, t5456: f64, t28834: f64, t7170: f64, t2057: f64, t28241: f64, t1510: f64, t26661: f64, t24255: f64, t5585: f64, t24246: f64, t24250: f64, t25246: f64, t25259: f64, t28323: f64, t28331: f64, t28335: f64, t28339: f64, t28343: f64, t28347: f64, t4166: f64, t7837: f64, t812: f64, t5612: f64, t7101: f64, t24218: f64, t24220: f64, t24221: f64, t25065: f64, t25077: f64, t25080: f64, t28357: f64, t28360: f64, t28362: f64, t28364: f64, t28366: f64, t28368: f64, t28370: f64, t28373: f64, t28376: f64, t24230: f64, t24231: f64, t25109: f64, t25126: f64, t25133: f64, t25140: f64, t25144: f64, t28380: f64, t28384: f64, t28386: f64, t28390: f64, t28397: f64, t28399: f64, t28401: f64, t28403: f64, t235: f64, t5617: f64, t1499: f64, t2051: f64, t226: f64, t24265: f64, t25277: f64, t25293: f64, t25310: f64, t25317: f64, t28420: f64, t28424: f64, t28428: f64, t5575: f64, t7839: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t28952, t28959, t28969, t28972, t28997, t29000) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1715(t28951, t510, t2035, t5456, t28834, t7170, t2057, t28241, t1510, t26661, t24255, t5585);
        let t29009 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1716(t24246, t24250, t25246, t25259, t28323, t28331, t28335, t28339, t28343, t28347, t28997, t29000, t4166, t7837, t812);
        let (t29010, t29025) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1717(t5612, t7101, t24218, t24220, t24221, t25065, t25077, t25080, t28357, t28360, t28362, t28364, t28366, t28368, t28370, t28373, t28376);
        let t29039 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1718(t24230, t24231, t25109, t25126, t25133, t25140, t25144, t28380, t28384, t28386, t28390, t28397, t28399, t28401, t28403);
        let (t29040, t29041, t29052, t29054) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1719(t29025, t29039, t235, t5617, t7101, t1499, t2051, t226, t24265, t25277, t25293, t25310, t25317, t28420, t28424, t28428, t29010, t5575, t7839, t812);
    (t28952, t28959, t28969, t28972, t28997, t29000, t29009, t29010, t29040, t29041, t29052, t29054)
}

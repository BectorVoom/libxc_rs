//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta460 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1685;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1686;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1687;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1688;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1689;
use chunk5::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1690;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta460<F: Float>(t5854: F, t607: F, t10355: F, t5819: F, t606: F, t4186: F, t4201: F, t2275: F, t5825: F, t18281: F, t48: F, t10368: F, t4210: F, t2282: F, t60: F, t10379: F, t1480: F, t4211: F, t4214: F, t44: F, t56: F, t5835: F, t5838: F, t5843: F, t614: F, t620: F, t38: F, t10389: F, t2299: F, t10398: F, t2306: F, t4227: F, t4232: F, t633: F, t637: F, t77: F, t1471: F, t1487: F, t1494: F, t4188: F, t4191: F, t4196: F, t4218: F, t4238: F, t5855: F, t5869: F, t608: F, t628: F, t641: F, t71: F, t85: F, t21720: F, t10301: F, t10309: F, t13269: F, t13272: F, t1497: F, t21661: F, t21663: F, t21674: F, t21677: F, t21682: F, t2242: F, t2247: F, t4173: F, t4178: F, t4241: F, t5816: F, t5872: F, t603: F, t644: F, t91: F, t5: F, t117: F, t5892: F, t625: F, t10208: F, t5891: F, t665: F, t4263: F, t4287: F, t5916: F, t2339: F, t5915: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t21727, t21733, t21736, t21742, t21745, t21754) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1685::<F>(t5854, t607, t10355, t5819, t606, t4186, t4201, t2275, t5825, t18281, t48, t10368);
        let t21768 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1686::<F>(t21754, t606, t4186, t4210, t2282, t5825, t18281, t60, t10379, t1480, t21733, t21736, t21742, t21745, t4211, t4214, t44, t56, t5835, t5838, t5843, t614, t620);
        let (t21769, t21804) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1687::<F>(t21768, t38, t10389, t5819, t2299, t5825, t10398, t2306, t18281, t4186, t4227, t4232, t606, t633, t637);
        let t21808 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1688::<F>(t21804, t77, t1471, t1487, t1494, t21727, t21769, t4188, t4191, t4196, t4218, t4238, t5855, t5869, t608, t628, t641, t71, t85);
        let (t21809, t21812) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1689::<F>(t21720, t21808, t10301, t10309, t13269, t13272, t1497, t21661, t21663, t21674, t21677, t21682, t2242, t2247, t4173, t4178, t4241, t5816, t5872, t603, t644, t91);
        let (t21813, t21814, t21818, t21821, t21824, t21827, t21829) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1690::<F>(t5, t21812, t117, t5892, t625, t10208, t5891, t665, t4263, t4287, t5916, t2339, t5915);
    (t21804, t21809, t21813, t21814, t21818, t21821, t21824, t21827, t21829)
}

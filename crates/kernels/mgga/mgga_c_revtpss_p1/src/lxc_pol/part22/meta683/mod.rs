//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta683 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2669;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2670;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2671;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta683<F: Float>(t21768: F, t38: F, t10389: F, t5819: F, t2299: F, t5825: F, t10398: F, t2306: F, t18281: F, t4186: F, t4227: F, t4232: F, t606: F, t633: F, t637: F, t77: F, t1471: F, t1487: F, t1494: F, t21727: F, t4188: F, t4191: F, t4196: F, t4218: F, t4238: F, t5855: F, t5869: F, t608: F, t628: F, t641: F, t71: F, t85: F, t21720: F, t10301: F, t10309: F, t13269: F, t13272: F, t1497: F, t21661: F, t21663: F, t21674: F, t21677: F, t21682: F, t2242: F, t2247: F, t4173: F, t4178: F, t4241: F, t5816: F, t5872: F, t603: F, t644: F, t91: F) -> (F, F, F, F, F, F, F, F) {
        let (t21769, t21784, t21789, t21794, t21799, t21804) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2669::<F>(t21768, t38, t10389, t5819, t2299, t5825, t10398, t2306, t18281, t4186, t4227, t4232, t606, t633, t637);
        let (t21805, t21808) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2670::<F>(t21804, t77, t1471, t1487, t1494, t21727, t21769, t4188, t4191, t4196, t4218, t4238, t5855, t5869, t608, t628, t641, t71, t85);
        let (t21809, t21812) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2671::<F>(t21720, t21808, t10301, t10309, t13269, t13272, t1497, t21661, t21663, t21674, t21677, t21682, t2242, t2247, t4173, t4178, t4241, t5816, t5872, t603, t644, t91);
    (t21769, t21784, t21789, t21794, t21799, t21805, t21809, t21812)
}

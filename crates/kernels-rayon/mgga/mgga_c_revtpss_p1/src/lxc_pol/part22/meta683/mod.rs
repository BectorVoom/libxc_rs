//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta683 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2669;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2670;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2671;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta683(t21768: f64, t38: f64, t10389: f64, t5819: f64, t2299: f64, t5825: f64, t10398: f64, t2306: f64, t18281: f64, t4186: f64, t4227: f64, t4232: f64, t606: f64, t633: f64, t637: f64, t77: f64, t1471: f64, t1487: f64, t1494: f64, t21727: f64, t4188: f64, t4191: f64, t4196: f64, t4218: f64, t4238: f64, t5855: f64, t5869: f64, t608: f64, t628: f64, t641: f64, t71: f64, t85: f64, t21720: f64, t10301: f64, t10309: f64, t13269: f64, t13272: f64, t1497: f64, t21661: f64, t21663: f64, t21674: f64, t21677: f64, t21682: f64, t2242: f64, t2247: f64, t4173: f64, t4178: f64, t4241: f64, t5816: f64, t5872: f64, t603: f64, t644: f64, t91: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t21769, t21784, t21789, t21794, t21799, t21804) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2669(t21768, t38, t10389, t5819, t2299, t5825, t10398, t2306, t18281, t4186, t4227, t4232, t606, t633, t637);
        let (t21805, t21808) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2670(t21804, t77, t1471, t1487, t1494, t21727, t21769, t4188, t4191, t4196, t4218, t4238, t5855, t5869, t608, t628, t641, t71, t85);
        let (t21809, t21812) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2671(t21720, t21808, t10301, t10309, t13269, t13272, t1497, t21661, t21663, t21674, t21677, t21682, t2242, t2247, t4173, t4178, t4241, t5816, t5872, t603, t644, t91);
    (t21769, t21784, t21789, t21794, t21799, t21805, t21809, t21812)
}

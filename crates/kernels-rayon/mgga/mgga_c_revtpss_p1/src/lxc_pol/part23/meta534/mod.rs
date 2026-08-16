//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta534 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2064;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2065;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2066;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta534(t21804: f64, t77: f64, t1471: f64, t1487: f64, t1494: f64, t21727: f64, t21769: f64, t4188: f64, t4191: f64, t4196: f64, t4218: f64, t4238: f64, t5855: f64, t5869: f64, t608: f64, t628: f64, t641: f64, t71: f64, t85: f64, t21720: f64, t10301: f64, t10309: f64, t13269: f64, t13272: f64, t1497: f64, t21661: f64, t21663: f64, t21674: f64, t21677: f64, t21682: f64, t2242: f64, t2247: f64, t4173: f64, t4178: f64, t4241: f64, t5816: f64, t5872: f64, t603: f64, t644: f64, t91: f64, t5: f64, t117: f64, t5892: f64, t625: f64, t10208: f64, t5891: f64, t665: f64, t4263: f64, t4287: f64, t5916: f64, t2339: f64, t5915: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t21805, t21808) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2064(t21804, t77, t1471, t1487, t1494, t21727, t21769, t4188, t4191, t4196, t4218, t4238, t5855, t5869, t608, t628, t641, t71, t85);
        let (t21809, t21812) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2065(t21720, t21808, t10301, t10309, t13269, t13272, t1497, t21661, t21663, t21674, t21677, t21682, t2242, t2247, t4173, t4178, t4241, t5816, t5872, t603, t644, t91);
        let (t21813, t21814, t21818, t21820, t21821, t21824, t21827, t21829) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2066(t5, t21812, t117, t5892, t625, t10208, t5891, t665, t4263, t4287, t5916, t2339, t5915);
    (t21805, t21809, t21813, t21814, t21818, t21820, t21821, t21824, t21827, t21829)
}

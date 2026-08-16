//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta362 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1604;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1605;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1606;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta362(t17297: f64, t942: f64, t951: f64, t959: f64, t2940: f64, t5812: f64, t5811: f64, t952: f64, t10296: f64, t10556: f64, t10784: f64, t10785: f64, t13552: f64, t13566: f64, t14287: f64, t14291: f64, t17173: f64, t17180: f64, t17185: f64, t13598: f64, t14324: f64, t17149: f64, t17165: f64, t17175: f64, t17189: f64, t17280: f64, t17286: f64, t17288: f64, t17290: f64, t17293: f64, t13642: f64, t14321: f64, t17154: f64, t17159: f64, t17163: f64, t17169: f64, t17211: f64, t17213: f64, t17216: f64, t17219: f64, t17221: f64, t17224: f64, t17241: f64, t17244: f64, t17247: f64, t17250: f64, t17253: f64, t17256: f64, t17272: f64, t17274: f64, t932: f64, t5769: f64, t13515: f64, t1557: f64, t4354: f64, t4396: f64, t10747: f64, t10765: f64, t10825: f64, t14332: f64, t1581: f64, t17197: f64, t2900: f64, t4449: f64, t4472: f64, t5762: f64, t5775: f64, t5791: f64, t5794: f64, t924: f64, t943: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17299, t17301, t17303, t17304, t17306, t17325) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1604(t17297, t942, t951, t959, t2940, t5812, t5811, t952, t10296, t10556, t10784, t10785, t13552, t13566, t14287, t14291, t17173, t17180, t17185);
        let t17349 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1605(t13598, t14324, t17149, t17165, t17175, t17189, t17280, t17286, t17288, t17290, t17293, t13642, t14321, t17154, t17159, t17163, t17169, t17211, t17213, t17216, t17219, t17221, t17224, t17241, t17244, t17247, t17250, t17253, t17256, t17272, t17274, t17325);
        let (t17350, t17355, t17366, t17372, t17374, t17375) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1606(t17349, t932, t5769, t942, t17297, t951, t13515, t1557, t4354, t4396, t10747, t10765, t10825, t14332, t1581, t17197, t2900, t4449, t4472, t5762, t5775, t5791, t5794, t924, t943, t952);
    (t17299, t17301, t17303, t17304, t17306, t17349, t17350, t17355, t17366, t17372, t17374, t17375)
}

//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta505 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2145;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2146;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2147;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta505<F: Float>(t17297: F, t942: F, t951: F, t959: F, t2940: F, t5812: F, t5811: F, t952: F, t10296: F, t10556: F, t10784: F, t10785: F, t13552: F, t13566: F, t14287: F, t14291: F, t17173: F, t17180: F, t17185: F, t13598: F, t14324: F, t17149: F, t17165: F, t17175: F, t17189: F, t17280: F, t17286: F, t17288: F, t17290: F, t17293: F, t13642: F, t14321: F, t17154: F, t17159: F, t17163: F, t17169: F, t17211: F, t17213: F, t17216: F, t17219: F, t17221: F, t17224: F, t17241: F, t17244: F, t17247: F, t17250: F, t17253: F, t17256: F, t17272: F, t17274: F, t932: F, t5769: F, t13515: F, t1557: F, t4354: F, t4396: F, t10747: F, t10765: F, t10825: F, t14332: F, t1581: F, t17197: F, t2900: F, t4449: F, t4472: F, t5762: F, t5775: F, t5791: F, t5794: F, t924: F, t943: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t17299, t17301, t17303, t17304, t17306, t17325) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2145::<F>(t17297, t942, t951, t959, t2940, t5812, t5811, t952, t10296, t10556, t10784, t10785, t13552, t13566, t14287, t14291, t17173, t17180, t17185);
        let t17349 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2146::<F>(t13598, t14324, t17149, t17165, t17175, t17189, t17280, t17286, t17288, t17290, t17293, t13642, t14321, t17154, t17159, t17163, t17169, t17211, t17213, t17216, t17219, t17221, t17224, t17241, t17244, t17247, t17250, t17253, t17256, t17272, t17274, t17325);
        let (t17350, t17355, t17366, t17372, t17374, t17375) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2147::<F>(t17349, t932, t5769, t942, t17297, t951, t13515, t1557, t4354, t4396, t10747, t10765, t10825, t14332, t1581, t17197, t2900, t4449, t4472, t5762, t5775, t5791, t5794, t924, t943, t952);
    (t17299, t17301, t17303, t17304, t17306, t17349, t17350, t17355, t17366, t17372, t17374, t17375)
}

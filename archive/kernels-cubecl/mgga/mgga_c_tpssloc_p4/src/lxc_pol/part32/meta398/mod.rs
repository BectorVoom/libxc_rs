//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta398 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1503;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1504;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1505;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1506;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1507;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1508;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta398<F: Float>(t17349: F, t932: F, t5769: F, t942: F, t17297: F, t951: F, t13515: F, t1557: F, t4354: F, t4396: F, t10747: F, t10765: F, t10825: F, t14332: F, t1581: F, t17197: F, t2900: F, t4449: F, t4472: F, t5762: F, t5775: F, t5791: F, t5794: F, t924: F, t943: F, t952: F, t10817: F, t5695: F, t2787: F, t5727: F, t10296: F, t10556: F, t10675: F, t10676: F, t13551: F, t13552: F, t13563: F, t13567: F, t17173: F, t17180: F, t17185: F, t13598: F, t13650: F, t17149: F, t17165: F, t17175: F, t17189: F, t17280: F, t17286: F, t17288: F, t17290: F, t17293: F, t13642: F, t13645: F, t17154: F, t17159: F, t17163: F, t17169: F, t17211: F, t17213: F, t17216: F, t17219: F, t17221: F, t17224: F, t17241: F, t17244: F, t17247: F, t17250: F, t17253: F, t17256: F, t17272: F, t17274: F, t913: F, t893: F, t10655: F, t5730: F, t5737: F, t923: F, t950: F, t4471: F, t10740: F, t14263: F, t14266: F, t14337: F, t1569: F, t2856: F, t2905: F, t2930: F, t4411: F, t4434: F, t4454: F, t4476: F, t5743: F, t5759: F, t933: F, t10832: F, t14409: F, t14410: F, t10636: F, t14245: F, t14246: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t17372, t17374, t17375) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1503::<F>(t17349, t932, t5769, t942, t17297, t951, t13515, t1557, t4354, t4396, t10747, t10765, t10825, t14332, t1581, t17197, t2900, t4449, t4472, t5762, t5775, t5791, t5794, t924, t943, t952);
        let (t17377, t17379, t17398) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1504::<F>(t10817, t5695, t2787, t5727, t10296, t10556, t10675, t10676, t13551, t13552, t13563, t13567, t17173, t17180, t17185);
        let t17422 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1505::<F>(t13598, t13650, t17149, t17165, t17175, t17189, t17280, t17286, t17288, t17290, t17293, t13642, t13645, t17154, t17159, t17163, t17169, t17211, t17213, t17216, t17219, t17221, t17224, t17241, t17244, t17247, t17250, t17253, t17256, t17272, t17274, t17398);
        let (t17425, t17427, t17449) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1506::<F>(t17422, t913, t893, t10655, t5730, t5737, t923, t5775, t950, t1581, t4471, t10740, t14263, t14266, t14337, t1569, t17377, t17379, t2856, t2905, t2930, t4411, t4434, t4454, t4476, t5743, t5759, t933);
        let (t17451, t17454, t17471) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1507::<F>(t5794, t950, t5791, t10556, t10832, t13563, t13598, t14409, t14410, t17149, t17154, t17159, t17163, t17165, t17169, t17173, t17175, t17180, t17185, t17189);
        let t17488 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1508::<F>(t10556, t10636, t13563, t13598, t14245, t14246, t17149, t17154, t17159, t17163, t17165, t17169, t17173, t17175, t17180, t17185, t17189);
    (t17372, t17374, t17375, t17377, t17379, t17425, t17427, t17449, t17451, t17454, t17471, t17488)
}

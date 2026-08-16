//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta506 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2148;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2149;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2150;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta506(t10817: f64, t5695: f64, t2787: f64, t5727: f64, t10296: f64, t10556: f64, t10675: f64, t10676: f64, t13551: f64, t13552: f64, t13563: f64, t13567: f64, t17173: f64, t17180: f64, t17185: f64, t13598: f64, t13650: f64, t17149: f64, t17165: f64, t17175: f64, t17189: f64, t17280: f64, t17286: f64, t17288: f64, t17290: f64, t17293: f64, t13642: f64, t13645: f64, t17154: f64, t17159: f64, t17163: f64, t17169: f64, t17211: f64, t17213: f64, t17216: f64, t17219: f64, t17221: f64, t17224: f64, t17241: f64, t17244: f64, t17247: f64, t17250: f64, t17253: f64, t17256: f64, t17272: f64, t17274: f64, t913: f64, t893: f64, t10655: f64, t5730: f64, t5737: f64, t923: f64, t5775: f64, t950: f64, t1581: f64, t4471: f64, t10740: f64, t14263: f64, t14266: f64, t14337: f64, t1569: f64, t2856: f64, t2905: f64, t2930: f64, t4411: f64, t4434: f64, t4454: f64, t4476: f64, t5743: f64, t5759: f64, t933: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17377, t17379, t17398) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2148(t10817, t5695, t2787, t5727, t10296, t10556, t10675, t10676, t13551, t13552, t13563, t13567, t17173, t17180, t17185);
        let t17422 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2149(t13598, t13650, t17149, t17165, t17175, t17189, t17280, t17286, t17288, t17290, t17293, t13642, t13645, t17154, t17159, t17163, t17169, t17211, t17213, t17216, t17219, t17221, t17224, t17241, t17244, t17247, t17250, t17253, t17256, t17272, t17274, t17398);
        let (t17423, t17425, t17427, t17428, t17443, t17446, t17449) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2150(t17422, t913, t893, t10655, t5730, t5737, t923, t5775, t950, t1581, t4471, t10740, t14263, t14266, t14337, t1569, t17377, t17379, t2856, t2905, t2930, t4411, t4434, t4454, t4476, t5743, t5759, t933);
    (t17377, t17379, t17422, t17423, t17425, t17427, t17428, t17443, t17446, t17449)
}

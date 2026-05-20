//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta554 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2378;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2379;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2380;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta554<F: Float>(t17794: F, t3363: F, t1042: F, t372: F, t5268: F, t17695: F, t13086: F, t13090: F, t13092: F, t17693: F, t17781: F, t17786: F, t17791: F, t17792: F, t3640: F, t3644: F, t3711: F, t5331: F, t5381: F, t1247: F, t1261: F, t12774: F, t12866: F, t12907: F, t12918: F, t12942: F, t12949: F, t12960: F, t17199: F, t17204: F, t17211: F, t17214: F, t17219: F, t17222: F, t17227: F, t17268: F, t17299: F, t17358: F, t17399: F, t17432: F, t17470: F, t17493: F, t17502: F, t17505: F, t17509: F, t17515: F, t17561: F, t17587: F, t17614: F, t17665: F, t17718: F, t17772: F, t3591: F, t3647: F, t3701: F, t3714: F, t5270: F, t5274: F, t5373: F, t5384: F, t489: F, t3759: F, t5230: F, t1811: F, t3601: F, t3769: F, t16695: F, t17454: F, t473: F, t5412: F, t1214: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t17795, t17796, t17799, t17800, t17803) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2378::<F>(t17794, t3363, t1042, t372, t5268, t17695, t13086, t13090, t13092, t17693, t17781, t17786, t17791, t17792, t3640, t3644, t3711, t5331, t5381);
        let t17807 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2379::<F>(t1247, t1261, t12774, t12866, t12907, t12918, t12942, t12949, t12960, t17199, t17204, t17211, t17214, t17219, t17222, t17227, t17268, t17299, t17358, t17399, t17432, t17470, t17493, t17502, t17505, t17509, t17515, t17561, t17587, t17614, t17665, t17718, t17772, t17803, t3591, t3647, t3701, t3711, t3714, t5270, t5274, t5373, t5384);
        let (t17808, t17811, t17814, t17815, t17818, t17821, t17822) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2380::<F>(t17807, t489, t3759, t5230, t1811, t3601, t3769, t16695, t17454, t473, t5412, t1214);
    (t17795, t17796, t17799, t17800, t17807, t17808, t17811, t17814, t17815, t17818, t17821, t17822)
}

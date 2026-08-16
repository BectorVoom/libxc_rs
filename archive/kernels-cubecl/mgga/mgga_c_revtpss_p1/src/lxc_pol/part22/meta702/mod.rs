//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta702 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2715;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2716;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2717;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta702<F: Float>(t545: F, t6888: F, t869: F, t689: F, t22005: F, t4003: F, t5744: F, t2782: F, t21981: F, t4086: F, t543: F, t22009: F, t72: F, t1432: F, t686: F, t10049: F, t10117: F, t10126: F, t10129: F, t10137: F, t10143: F, t1399: F, t14252: F, t1437: F, t22253: F, t5659: F, t5735: F, t5755: F, t6862: F, t820: F, t21998: F, t22325: F, t22344: F, t1427: F, t213: F, t13727: F, t13733: F, t13737: F, t1424: F, t1445: F, t4071: F, t5715: F, t5775: F, t6896: F, t9632: F, t9639: F, t9642: F, t9650: F, t9666: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t22351, t22352, t22353, t22361, t22362, t22365, t22366, t22369) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2715::<F>(t545, t6888, t869, t689, t22005, t4003, t5744, t2782, t21981, t4086, t543, t22009);
        let (t22373, t22379, t22384) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2716::<F>(t22369, t2782, t22005, t4086, t543, t6888, t72, t1432, t686, t10049, t10117, t10126, t10129, t10137, t10143, t1399, t14252, t1437, t22009, t22253, t22353, t22362, t22366, t5659, t5735, t5755, t6862, t820);
        let (t22386, t22387, t22390, t22393) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2717::<F>(t21998, t22325, t22344, t22384, t1427, t213, t6888, t13727, t13733, t13737, t1424, t1445, t4071, t5715, t5775, t6896, t9632, t9639, t9642, t9650, t9666);
    (t22351, t22352, t22361, t22365, t22369, t22373, t22379, t22386, t22387, t22390, t22393)
}

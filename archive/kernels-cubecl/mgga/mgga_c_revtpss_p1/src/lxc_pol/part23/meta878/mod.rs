//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta878 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2784;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2785;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta878<F: Float>(t1444: F, t2782: F, t4075: F, t556: F, t6918: F, t22453: F, t47530: F, t5599: F, t5775: F, t689: F, t1426: F, t6889: F, t786: F, t3917: F, t14090: F, t14100: F, t22432: F, t47603: F, t686: F, t72: F, t22427: F, t2435: F, t1358: F, t212: F, t22307: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t74824, t74826, t74829, t74835) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2784::<F>(t1444, t2782, t4075, t556, t6918, t22453, t47530, t5599, t5775, t689, t1426, t6889, t786);
        let (t74836, t74838, t74843, t74849, t74853) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2785::<F>(t3917, t74835, t14090, t14100, t22432, t47603, t686, t72, t22427, t2435, t1358, t212, t22307, t689);
    (t74824, t74826, t74829, t74835, t74836, t74838, t74843, t74849, t74853)
}

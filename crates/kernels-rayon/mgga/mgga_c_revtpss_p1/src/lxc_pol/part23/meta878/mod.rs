//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta878 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2784;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2785;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta878(t1444: f64, t2782: f64, t4075: f64, t556: f64, t6918: f64, t22453: f64, t47530: f64, t5599: f64, t5775: f64, t689: f64, t1426: f64, t6889: f64, t786: f64, t3917: f64, t14090: f64, t14100: f64, t22432: f64, t47603: f64, t686: f64, t72: f64, t22427: f64, t2435: f64, t1358: f64, t212: f64, t22307: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t74824, t74826, t74829, t74835) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2784(t1444, t2782, t4075, t556, t6918, t22453, t47530, t5599, t5775, t689, t1426, t6889, t786);
        let (t74836, t74838, t74843, t74849, t74853) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2785(t3917, t74835, t14090, t14100, t22432, t47603, t686, t72, t22427, t2435, t1358, t212, t22307, t689);
    (t74824, t74826, t74829, t74835, t74836, t74838, t74843, t74849, t74853)
}

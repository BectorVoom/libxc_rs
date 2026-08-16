//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta848 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2730;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2731;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta848(t12772: f64, t21160: f64, t3625: f64, t11249: f64, t6622: f64, t12832: f64, t20926: f64, t15904: f64, t17394: f64, t13127: f64, t3682: f64, t6667: f64, t20900: f64, t73: f64, t12987: f64, t5390: f64, t17736: f64, t21309: f64, t3767: f64, t70629: f64, t474: f64, t6593: f64, t3089: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t70857, t70890, t70914, t70916, t70917, t70942) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2730(t12772, t21160, t3625, t11249, t6622, t12832, t20926, t15904, t17394, t13127, t3682, t6667);
        let (t70944, t70959, t70982, t70990, t70993, t70994) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2731(t20900, t73, t12987, t5390, t12772, t17736, t21309, t3767, t70629, t474, t6593, t3089);
    (t70857, t70890, t70914, t70916, t70917, t70942, t70944, t70959, t70982, t70990, t70993, t70994)
}

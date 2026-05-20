//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta848 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2730;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2731;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta848<F: Float>(t12772: F, t21160: F, t3625: F, t11249: F, t6622: F, t12832: F, t20926: F, t15904: F, t17394: F, t13127: F, t3682: F, t6667: F, t20900: F, t73: F, t12987: F, t5390: F, t17736: F, t21309: F, t3767: F, t70629: F, t474: F, t6593: F, t3089: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t70857, t70890, t70914, t70916, t70917, t70942) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2730::<F>(t12772, t21160, t3625, t11249, t6622, t12832, t20926, t15904, t17394, t13127, t3682, t6667);
        let (t70944, t70959, t70982, t70990, t70993, t70994) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2731::<F>(t20900, t73, t12987, t5390, t12772, t17736, t21309, t3767, t70629, t474, t6593, t3089);
    (t70857, t70890, t70914, t70916, t70917, t70942, t70944, t70959, t70982, t70990, t70993, t70994)
}

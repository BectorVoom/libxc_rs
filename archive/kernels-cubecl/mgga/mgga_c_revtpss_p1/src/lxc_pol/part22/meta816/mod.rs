//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta816 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2925;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2926;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta816<F: Float>(t4146: F, t1455: F, t5808: F, t1892: F, t9646: F, t9648: F, t1904: F, t47567: F, t14110: F, t47530: F, t1427: F, t1903: F, t22: F, t9647: F, t2453: F, t3908: F, t5711: F, t14296: F, t9303: F, t13738: F, t686: F, t72: F, t9680: F, t213: F, t556: F, t9656: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t47672, t47730, t47764, t47772, t47777, t47781) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2925::<F>(t4146, t1455, t5808, t1892, t9646, t9648, t1904, t47567, t14110, t47530, t1427, t1903, t22, t9647);
        let (t47784, t47786, t47791, t47793, t47794) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2926::<F>(t2453, t3908, t5711, t14296, t9303, t13738, t686, t72, t9680, t213, t556, t1903, t9656);
    (t47672, t47730, t47764, t47772, t47777, t47781, t47784, t47786, t47791, t47793, t47794)
}

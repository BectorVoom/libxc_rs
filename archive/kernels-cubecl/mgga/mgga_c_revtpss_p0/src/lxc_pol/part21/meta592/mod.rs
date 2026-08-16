//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta592 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2309;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta592<F: Float>(t1214: F, t21471: F, t5464: F, t1770: F, t5462: F, t5477: F, t4003: F, t5658: F, t1398: F, t9994: F, t1877: F, t73: F) -> (F, F, F, F, F, F, F) {
        let (t21472, t21483, t21500, t21579, t21990, t22016, t22229) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2309::<F>(t1214, t21471, t5464, t1770, t5462, t5477, t4003, t5658, t1398, t9994, t1877, t73);
    (t21472, t21483, t21500, t21579, t21990, t22016, t22229)
}

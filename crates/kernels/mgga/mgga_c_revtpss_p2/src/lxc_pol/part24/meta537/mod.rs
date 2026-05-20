//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta537 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1581;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1582;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta537<F: Float>(t22841: F, t2661: F, t74026: F, t9934: F, t14100: F, t22399: F, t5722: F, t74835: F, t1357: F, t23043: F, t689: F, t1364: F, t22965: F, t786: F, t22975: F, t5599: F, t6896: F, t6919: F, t5741: F, t74892: F, t22315: F, t48084: F, t22858: F, t47372: F, t686: F, t72: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t86274, t86285, t86296, t86300, t86311) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1581::<F>(t22841, t2661, t74026, t9934, t14100, t22399, t5722, t74835, t1357, t23043, t689, t1364, t22965, t786);
        let (t86314, t86317, t86346, t86350, t86354, t86358) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1582::<F>(t1357, t22975, t689, t5599, t6896, t6919, t5741, t74892, t22315, t48084, t22858, t47372, t686, t72);
    (t86274, t86285, t86296, t86300, t86311, t86314, t86317, t86346, t86350, t86354, t86358)
}

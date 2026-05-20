//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta886 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3072;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3073;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta886<F: Float>(t15199: F, t698: F, t2852: F, t373: F, t2439: F, t4628: F, t1606: F, t9303: F, t11387: F, t4631: F, t15513: F, t914: F, t2923: F, t4587: F, t11384: F, t1596: F, t11466: F, t300: F, t11452: F, t4669: F, t11450: F, t1621: F, t11507: F, t1633: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t52065, t52110, t52126, t52128, t52163, t52214) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3072::<F>(t15199, t698, t2852, t373, t2439, t4628, t1606, t9303, t11387, t4631, t15513, t914);
        let (t52219, t52224, t52238, t52264, t52320, t52370) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3073::<F>(t2923, t4587, t11384, t1596, t11466, t300, t11452, t4669, t11450, t1621, t11507, t1633);
    (t52065, t52110, t52126, t52128, t52163, t52214, t52219, t52224, t52238, t52264, t52320, t52370)
}

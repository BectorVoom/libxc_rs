//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta244 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1088;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1089;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta244<F: Float>(t1477: F, t476: F, t52: F, t475: F, t467: F, t1785: F, t1803: F, t225: F, t6564: F, t480: F, t482: F, t6573: F, t371: F, t372: F, t1715: F, t5277: F, t1042: F, t6435: F, t6437: F, t6441: F, t6473: F, t6476: F, t6542: F, t6544: F, t6546: F, t6550: F, t6554: F, t6558: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t6593, t6594, t6595, t6598, t6601, t6602, t6609) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1088::<F>(t1477, t476, t52, t475, t467, t1785, t1803, t225, t6564, t480, t482, t6573);
        let (t6611, t6618, t6619, t6622) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1089::<F>(t371, t372, t6609, t1715, t5277, t1042, t6435, t6437, t6441, t6473, t6476, t6542, t6544, t6546, t6550, t6554, t6558);
    (t6593, t6594, t6595, t6598, t6601, t6602, t6609, t6611, t6618, t6619, t6622)
}

//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta387 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1954;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta387<F: Float>(t13665: F, t2630: F, t1857: F, t3860: F, t3863: F, t13581: F, t189: F, t512: F, t1907: F, t9593: F, t5566: F, t749: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t13666, t13667, t13668, t13669, t13671, t13672, t13673, t13674, t13680) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1954::<F>(t13665, t2630, t1857, t3860, t3863, t13581, t189, t512, t1907, t9593, t5566, t749);
    (t13666, t13667, t13668, t13669, t13671, t13672, t13673, t13674, t13680)
}

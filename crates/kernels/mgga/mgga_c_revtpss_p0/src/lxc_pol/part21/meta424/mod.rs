//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta424 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1915;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1916;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta424<F: Float>(t9597: F, t123: F, t1856: F, t2630: F, t1857: F, t3860: F, t3863: F, t13581: F, t189: F, t512: F, t1907: F, t9593: F, t5566: F, t749: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t13664, t13665) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1915::<F>(t9597, t123, t1856);
        let (t13667, t13669, t13671, t13672, t13673, t13674, t13680) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1916::<F>(t13665, t2630, t1857, t3860, t3863, t13581, t189, t512, t1907, t9593, t5566, t749);
    (t13664, t13665, t13667, t13669, t13671, t13672, t13673, t13674, t13680)
}

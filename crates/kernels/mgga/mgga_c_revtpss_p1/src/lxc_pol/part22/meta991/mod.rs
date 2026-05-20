//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta991 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3376;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3377;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta991<F: Float>(t63412: F, t63426: F, t63440: F, t63466: F, t923: F, t18979: F, t2889: F, t52035: F, t52037: F, t52039: F, t52041: F, t52045: F, t52047: F, t52049: F, t52051: F, t52065: F, t63393: F, t63396: F, t63399: F, t15220: F, t4598: F, t18984: F, t18987: F, t4614: F, t18992: F, t18950: F, t2880: F, t918: F, t2897: F, t2881: F, t41401: F, t6113: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t63468, t63469, t63471, t63473) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3376::<F>(t63412, t63426, t63440, t63466, t923, t18979, t2889, t52035, t52037, t52039, t52041, t52045, t52047, t52049, t52051, t52065, t63393, t63396, t63399);
        let (t63474, t63476, t63478, t63480, t63482, t63485, t63488, t63491) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3377::<F>(t15220, t4598, t18984, t2889, t18987, t4614, t18992, t18950, t2880, t918, t2897, t2881, t41401, t6113);
    (t63468, t63469, t63471, t63473, t63474, t63476, t63478, t63480, t63482, t63485, t63488, t63491)
}

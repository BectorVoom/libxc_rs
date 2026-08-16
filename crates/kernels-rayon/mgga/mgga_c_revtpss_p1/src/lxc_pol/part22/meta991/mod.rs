//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta991 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3376;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3377;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta991(t63412: f64, t63426: f64, t63440: f64, t63466: f64, t923: f64, t18979: f64, t2889: f64, t52035: f64, t52037: f64, t52039: f64, t52041: f64, t52045: f64, t52047: f64, t52049: f64, t52051: f64, t52065: f64, t63393: f64, t63396: f64, t63399: f64, t15220: f64, t4598: f64, t18984: f64, t18987: f64, t4614: f64, t18992: f64, t18950: f64, t2880: f64, t918: f64, t2897: f64, t2881: f64, t41401: f64, t6113: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t63468, t63469, t63471, t63473) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3376(t63412, t63426, t63440, t63466, t923, t18979, t2889, t52035, t52037, t52039, t52041, t52045, t52047, t52049, t52051, t52065, t63393, t63396, t63399);
        let (t63474, t63476, t63478, t63480, t63482, t63485, t63488, t63491) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3377(t15220, t4598, t18984, t2889, t18987, t4614, t18992, t18950, t2880, t918, t2897, t2881, t41401, t6113);
    (t63468, t63469, t63471, t63473, t63474, t63476, t63478, t63480, t63482, t63485, t63488, t63491)
}

//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta537 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2344;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2345;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta537(t17471: f64, t5047: f64, t1222: f64, t1012: f64, t13026: f64, t16715: f64, t16720: f64, t5312: f64, t1774: f64, t3601: f64, t3611: f64, t3720: f64, t12809: f64, t12882: f64, t12887: f64, t12893: f64, t12895: f64, t12900: f64, t12902: f64, t12905: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17472, t17474, t17475, t17476, t17479, t17482, t17483, t17484) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2344(t17471, t5047, t1222, t1012, t13026, t16715, t16720, t5312, t1774, t3601, t3611, t3720);
        let t17493 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2345(t1222, t12809, t12882, t12887, t12893, t12895, t12900, t12902, t12905, t17474, t17476, t17479, t17484);
    (t17472, t17474, t17475, t17476, t17479, t17482, t17483, t17484, t17493)
}

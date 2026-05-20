//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta607 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2032;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2033;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta607<F: Float>(t2149: F, t97312: F, t1294: F, t5464: F, t1210: F, t29199: F, t1203: F, t21471: F, t3596: F, t7627: F, t26936: F, t3566: F, t13181: F, t3140: F, t1243: F, t2147: F, t44841: F, t7635: F, t3572: F, t8945: F, t45551: F, t473: F, t37885: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t97313, t97314, t97318, t97319, t97332, t97343) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2032::<F>(t2149, t97312, t1294, t5464, t1210, t29199, t1203, t21471, t3596, t7627, t26936, t3566);
        let (t97348, t97358, t97363, t97377, t97397) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2033::<F>(t13181, t3140, t1243, t2149, t2147, t44841, t7635, t3572, t8945, t45551, t473, t37885);
    (t97313, t97314, t97318, t97319, t97332, t97343, t97348, t97358, t97363, t97377, t97397)
}

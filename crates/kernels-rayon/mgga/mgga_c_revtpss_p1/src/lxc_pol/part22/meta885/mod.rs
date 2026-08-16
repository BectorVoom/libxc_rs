//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta885 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3062;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3063;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3064;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3065;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3066;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3067;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3068;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3069;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3070;
use chunk9::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3071;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta885(t138: f64, t140: f64, t240: f64, t2852: f64, t346: f64, t15159: f64, t689: f64, t2435: f64, t4580: f64, t4575: f64, t15146: f64, t15150: f64, t15155: f64, t15131: f64, t15136: f64, t15141: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t52011 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3062(t138, t140, t240);
        let (t52018, t52033) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3063(t2852, t346, t15159, t689);
        let t52035 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3064(t2435, t4580);
        let t52037 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3065(t2435, t4575);
        let t52039 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3066(t15146, t689);
        let t52041 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3067(t15150, t689);
        let t52045 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3068(t15155, t689);
        let t52047 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3069(t15131, t689);
        let t52049 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3070(t15136, t689);
        let t52051 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3071(t15141, t689);
    (t52011, t52018, t52033, t52035, t52037, t52039, t52041, t52045, t52047, t52049, t52051)
}

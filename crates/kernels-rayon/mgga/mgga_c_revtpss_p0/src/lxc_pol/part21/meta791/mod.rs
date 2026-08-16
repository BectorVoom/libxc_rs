//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta791 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2850;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2851;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2852;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2853;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2854;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2855;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2856;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta791(t42518: f64, t51959: f64, t52011: f64, t42731: f64, t2852: f64, t346: f64, t2889: f64, t918: f64, t15107: f64, t15110: f64, t128: f64, t2850: f64, t51993: f64, t11142: f64, t51998: f64, t15159: f64, t689: f64, t2435: f64, t4580: f64, t4575: f64, t15146: f64, t15150: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t52013, t52016, t52020, t52023, t52025, t52028) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2850(t42518, t51959, t52011, t42731, t2852, t346, t2889, t918, t15107, t15110, t128, t2850, t51993);
        let t52031 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2851(t11142, t128, t51998);
        let t52033 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2852(t15159, t689);
        let t52035 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2853(t2435, t4580);
        let t52037 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2854(t2435, t4575);
        let t52039 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2855(t15146, t689);
        let t52041 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2856(t15150, t689);
    (t52013, t52016, t52020, t52023, t52025, t52028, t52031, t52033, t52035, t52037, t52039, t52041)
}

//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta791 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2850;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2851;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2852;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2853;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2854;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2855;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2856;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta791<F: Float>(t42518: F, t51959: F, t52011: F, t42731: F, t2852: F, t346: F, t2889: F, t918: F, t15107: F, t15110: F, t128: F, t2850: F, t51993: F, t11142: F, t51998: F, t15159: F, t689: F, t2435: F, t4580: F, t4575: F, t15146: F, t15150: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t52013, t52016, t52020, t52023, t52025, t52028) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2850::<F>(t42518, t51959, t52011, t42731, t2852, t346, t2889, t918, t15107, t15110, t128, t2850, t51993);
        let t52031 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2851::<F>(t11142, t128, t51998);
        let t52033 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2852::<F>(t15159, t689);
        let t52035 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2853::<F>(t2435, t4580);
        let t52037 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2854::<F>(t2435, t4575);
        let t52039 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2855::<F>(t15146, t689);
        let t52041 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2856::<F>(t15150, t689);
    (t52013, t52016, t52020, t52023, t52025, t52028, t52031, t52033, t52035, t52037, t52039, t52041)
}

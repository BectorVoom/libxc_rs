//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta358 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1868;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1869;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1870;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1871;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1872;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1873;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1874;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1875;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta358<F: Float>(t3361: F, t635: F, t1146: F, t2439: F, t3424: F, t698: F, t3421: F, t57: F, t268: F, t404: F, t7021: F, t1123: F, t2435: F, t3364: F, t689: F, t3369: F, t3373: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t12256 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1868::<F>(t3361, t635);
        let t12261 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1869::<F>(t1146, t2439);
        let (t12263, t12265, t12267, t12268) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1870::<F>(t3424, t698, t3421, t3361, t57);
        let t12295 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1871::<F>(t268, t404, t7021);
        let (t12296, t12297) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1872::<F>(t12295, t1123, t2435);
        let t12299 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1873::<F>(t3364, t689);
        let t12301 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1874::<F>(t3369, t689);
        let t12303 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1875::<F>(t3373, t689);
    (t12256, t12261, t12263, t12265, t12267, t12268, t12295, t12296, t12297, t12299, t12301, t12303)
}

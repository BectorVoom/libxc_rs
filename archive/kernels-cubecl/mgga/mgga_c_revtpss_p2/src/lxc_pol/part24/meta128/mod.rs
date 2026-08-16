//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta128 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk675;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk676;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk677;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk678;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk679;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk680;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta128<F: Float>(t1065: F, t905: F, t1032: F, t1647: F, t1040: F, t3147: F, t72: F, t3088: F, t3299: F, t1668: F, t3153: F, t3317: F, t1012: F, t1014: F, t3252: F, t140: F, t1655: F, t1011: F, t1678: F, t342: F, t1086: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t4872, t4879) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk675::<F>(t1065, t905, t1032, t1647, t1040);
        let (t4890, t4891) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk676::<F>(t3147, t72, t3088);
        let t4892 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk677::<F>(t3299, t4891);
        let (t4893, t4899) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk678::<F>(t1668, t3153, t3317, t4891);
        let (t4915, t4919, t4925, t4935) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk679::<F>(t1012, t1014, t3252, t140, t1655, t1011, t1678, t342);
        let t4954 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk680::<F>(t1086, t1647);
    (t4872, t4879, t4890, t4891, t4892, t4893, t4899, t4915, t4919, t4925, t4935, t4954)
}

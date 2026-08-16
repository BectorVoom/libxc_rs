//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta128 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk675;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk676;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk677;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk678;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk679;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk680;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta128(t1065: f64, t905: f64, t1032: f64, t1647: f64, t1040: f64, t3147: f64, t72: f64, t3088: f64, t3299: f64, t1668: f64, t3153: f64, t3317: f64, t1012: f64, t1014: f64, t3252: f64, t140: f64, t1655: f64, t1011: f64, t1678: f64, t342: f64, t1086: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4872, t4879) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk675(t1065, t905, t1032, t1647, t1040);
        let (t4890, t4891) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk676(t3147, t72, t3088);
        let t4892 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk677(t3299, t4891);
        let (t4893, t4899) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk678(t1668, t3153, t3317, t4891);
        let (t4915, t4919, t4925, t4935) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk679(t1012, t1014, t3252, t140, t1655, t1011, t1678, t342);
        let t4954 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk680(t1086, t1647);
    (t4872, t4879, t4890, t4891, t4892, t4893, t4899, t4915, t4919, t4925, t4935, t4954)
}

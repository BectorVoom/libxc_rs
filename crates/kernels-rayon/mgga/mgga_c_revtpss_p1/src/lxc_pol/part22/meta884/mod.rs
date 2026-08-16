//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta884 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3058;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3059;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3060;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3061;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta884(t2439: f64, t4622: f64, t15186: f64, t698: f64, t15177: f64, t15180: f64, t15162: f64, t15165: f64, t123: f64, t127: f64, t159: f64, t1065: f64, t11150: f64, t11144: f64, t3181: f64, t15194: f64, t689: f64, t2435: f64, t4584: f64, t1593: f64, t9292: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t51915, t51917, t51921, t51923, t51937, t51942, t51957) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3058(t2439, t4622, t15186, t698, t15177, t15180, t15162, t15165, t123, t127, t159);
        let (t51958, t51963, t51967) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3059(t1065, t11150, t11144, t3181, t15194, t689);
        let t51973 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3060(t2435, t4584);
        let t51978 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3061(t1593, t9292);
    (t51915, t51917, t51921, t51923, t51937, t51942, t51957, t51958, t51963, t51967, t51973, t51978)
}

//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta884 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3058;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3059;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3060;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3061;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta884<F: Float>(t2439: F, t4622: F, t15186: F, t698: F, t15177: F, t15180: F, t15162: F, t15165: F, t123: F, t127: F, t159: F, t1065: F, t11150: F, t11144: F, t3181: F, t15194: F, t689: F, t2435: F, t4584: F, t1593: F, t9292: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t51915, t51917, t51921, t51923, t51937, t51942, t51957) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3058::<F>(t2439, t4622, t15186, t698, t15177, t15180, t15162, t15165, t123, t127, t159);
        let (t51958, t51963, t51967) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3059::<F>(t1065, t11150, t11144, t3181, t15194, t689);
        let t51973 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3060::<F>(t2435, t4584);
        let t51978 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3061::<F>(t1593, t9292);
    (t51915, t51917, t51921, t51923, t51937, t51942, t51957, t51958, t51963, t51967, t51973, t51978)
}

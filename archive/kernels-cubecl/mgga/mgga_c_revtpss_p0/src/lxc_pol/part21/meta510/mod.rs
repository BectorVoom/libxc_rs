//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta510 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2134;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2135;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2136;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2137;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta510<F: Float>(t11922: F, t4895: F, t4892: F, t140: F, t4886: F, t1011: F, t3241: F, t4924: F, t12047: F, t15905: F, t3151: F, t357: F, t15907: F, t3117: F, t11883: F, t11888: F, t16037: F, t16040: F, t16045: F, t16049: F, t16052: F, t1656: F, t3115: F, t4887: F, t4896: F, t4902: F, t1651: F, t3133: F, t1045: F, t12167: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t16055, t16057, t16060, t16062, t16064, t16067, t16068) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2134::<F>(t11922, t4895, t4892, t140, t4886, t1011, t3241, t4924, t12047, t15905, t3151, t357);
        let (t16069, t16070, t16073) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2135::<F>(t15907, t16068, t3117, t11883, t11888, t16037, t16040, t16045, t16049, t16052, t16057, t16062, t16064, t16067, t1656, t3115, t3241, t4887, t4896, t4902);
        let t16076 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2136::<F>(t1651, t3133);
        let (t16077, t16078, t16081) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2137::<F>(t1045, t16076, t3117, t12167, t15905);
    (t16055, t16060, t16067, t16068, t16069, t16070, t16073, t16076, t16077, t16078, t16081)
}

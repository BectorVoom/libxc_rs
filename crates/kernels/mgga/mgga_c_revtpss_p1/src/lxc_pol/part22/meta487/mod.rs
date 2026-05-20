//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta487 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2208;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2209;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2210;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta487<F: Float>(t4757: F, t906: F, t3092: F, t380: F, t994: F, t16088: F, t606: F, t999: F, t4578: F, t905: F, t1045: F, t15691: F, t11774: F, t11917: F, t11924: F, t11938: F, t11952: F, t11954: F, t11956: F, t11965: F, t16078: F, t16081: F, t16084: F, t16089: F, t3115: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t16090, t16091, t16094, t16095) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2208::<F>(t4757, t906, t3092, t380, t994, t16088);
        let t16096 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2209::<F>(t606, t999);
        let (t16097, t16098, t16102, t16103, t16104, t16114) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2210::<F>(t16096, t4578, t3092, t905, t999, t606, t1045, t15691, t11774, t11917, t11924, t11938, t11952, t11954, t11956, t11965, t16078, t16081, t16084, t16089, t16091, t16095, t3115);
    (t16090, t16091, t16094, t16095, t16096, t16097, t16098, t16102, t16103, t16104, t16114)
}
